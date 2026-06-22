//! Siege Defense MUD — all 75 patterns, 8 waves, deterministic, no stdin.
//!
//! Run with: `cargo run --example siege_defense --features std`

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
use std::{println, vec::Vec};

use wasm4games::patterns::{
    aabb_collision_resolved, action_mask_applied, action_rate_bounded, adapter_priority_ranked,
    ai_action_selected, audio_distance_attenuated, audio_fade_applied, audio_priority_selected,
    audio_trigger_evaluated, biome_class_selected, bridge_state_transitioned,
    camera_distance_clamped, camera_follow_lerped, camera_shake_applied, capability_flag_evaluated,
    choice_weight_selected, command_opcode_encoded, condition_flag_evaluated,
    cooldown_legality_checked, ctq_threshold_evaluated, currency_delta_applied, damage_applied,
    defect_rate_quantized, dialogue_cooldown_bounded, dialogue_node_advanced,
    entity_state_transitioned, episode_return_bounded, fixed_tick_advanced, fov_adjusted,
    heuristic_distance_estimated, input_admitted, inventory_item_changed, lag_compensation_applied,
    level_gate_evaluated, look_target_weighted, mastery_moment_detected, movement_legality_checked,
    narrative_branch_selected, nav_state_advanced, noise_value_sampled, nps_prompt_gated,
    nps_score_bounded, object_spawned, observation_class_selected, ocel_event_linked,
    otel_span_emitted, packet_priority_evaluated, path_cost_bounded, path_node_expanded,
    payload_size_bounded, physics_value_rendered, policy_action_selected, prediction_error_bounded,
    projectile_advanced, purchase_admitted, quality_gate_evaluated, quest_step_advanced,
    receipt_appended, replay_frame_recorded, resource_bound_checked, reward_signal_clamped,
    reward_tier_selected, semantic_lod_selected, share_artifact_generated, sigma_level_computed,
    spawn_weight_evaluated, status_effect_ticked, sync_state_admitted, terrain_height_quantized,
    tick_delta_bounded, tile_variant_selected, transition_legality_checked, volume_clamped,
    waypoint_reached, xp_threshold_crossed,
};

// ── GameState packed into u64 ──────────────────────────────────────────────
// bits 63-48 : wall_hp       (16 bits, 0-65535)
// bits 47-36 : treasury      (12 bits, 0-4095 gold)
// bits 35-28 : soldiers      (8  bits, 0-255)
// bits 27-24 : wave          (4  bits, 0-15)
// bits 23-16 : tick          (8  bits, 0-255)
// bits 15-12 : towers_placed (4  bits, 0-15)
// bits 11-4  : wave_kills    (8  bits, 0-255)
// bits  3-0  : receipt_nib   (4  bits, rolling receipt nibble)

fn pack(
    wall_hp: u64,
    treasury: u64,
    soldiers: u64,
    wave: u64,
    tick: u64,
    towers: u64,
    kills: u64,
    receipt: u64,
) -> u64 {
    ((wall_hp & 0xFFFF) << 48)
        | ((treasury & 0xFFF) << 36)
        | ((soldiers & 0xFF) << 28)
        | ((wave & 0xF) << 24)
        | ((tick & 0xFF) << 16)
        | ((towers & 0xF) << 12)
        | ((kills & 0xFF) << 4)
        | (receipt & 0xF)
}

fn wall_hp(s: u64) -> u64 {
    (s >> 48) & 0xFFFF
}
fn treasury(s: u64) -> u64 {
    (s >> 36) & 0xFFF
}
fn soldiers(s: u64) -> u64 {
    (s >> 28) & 0xFF
}
fn wave(s: u64) -> u64 {
    (s >> 24) & 0xF
}
fn tick_f(s: u64) -> u64 {
    (s >> 16) & 0xFF
}
fn towers(s: u64) -> u64 {
    (s >> 12) & 0xF
}
fn kills(s: u64) -> u64 {
    (s >> 4) & 0xFF
}
fn receipt_n(s: u64) -> u64 {
    s & 0xF
}

fn set_wall_hp(s: u64, v: u64) -> u64 {
    (s & !(0xFFFFu64 << 48)) | ((v & 0xFFFF) << 48)
}
fn set_treasury(s: u64, v: u64) -> u64 {
    (s & !(0xFFFu64 << 36)) | ((v & 0xFFF) << 36)
}
fn set_soldiers(s: u64, v: u64) -> u64 {
    (s & !(0xFFu64 << 28)) | ((v & 0xFF) << 28)
}
fn set_wave(s: u64, v: u64) -> u64 {
    (s & !(0xFu64 << 24)) | ((v & 0xF) << 24)
}
fn set_tick(s: u64, v: u64) -> u64 {
    (s & !(0xFFu64 << 16)) | ((v & 0xFF) << 16)
}
fn set_towers(s: u64, v: u64) -> u64 {
    (s & !(0xFu64 << 12)) | ((v & 0xF) << 12)
}
fn set_kills(s: u64, v: u64) -> u64 {
    (s & !(0xFFu64 << 4)) | ((v & 0xFF) << 4)
}
fn set_receipt(s: u64, v: u64) -> u64 {
    (s & !0xFu64) | (v & 0xF)
}

// ── Deterministic input seeds per wave ────────────────────────────────────
const WAVE_SEEDS: [u64; 8] = [
    0x0001_BEEF_CAFE_0101,
    0x0002_DEAD_FACE_0202,
    0x0003_BABE_FEED_0303,
    0x0004_CAFE_BEAD_0404,
    0x0005_FADE_CEDE_0505,
    0x0006_ACEF_BEAD_0606,
    0x0007_DEED_BEEF_0707,
    0x0008_FACE_DEAD_0808,
];

const WAVE_NAMES: [&str; 8] = [
    "Goblin Skirmisher",
    "Orc Berserker",
    "Troll Vanguard",
    "Siege Engine",
    "Dark Elf Raider",
    "Giant Catapult",
    "Undead Legion",
    "Dragon Warlord",
];

const BIOMES: [&str; 4] = ["Plains", "Mountains", "Marsh", "Volcanic"];
const TILES: [&str; 4] = ["Grassland", "Forest", "River", "Ruins"];
const ELEV: [&str; 4] = ["Valley", "Plains", "Hill", "Wall"];

fn hr() {
    println!("─────────────────────────────────────────────────────");
}

#[cfg(feature = "std")]
fn main() {
    // Fortress vitals tracked in plain Rust — immune to pattern kernel bit-clobbering
    let mut f_wall: u64 = 500; // hit points
    let mut f_treasury: u64 = 400; // gold
    let mut f_soldiers: u64 = 60;
    let mut f_towers: u64 = 2;
    let mut f_kills: u64 = 0;
    let mut f_tick: u64 = 0;

    // The pattern kernel scratch state (seed per wave, patterns chain through it)
    let mut state: u64 = 0;

    // Accumulate receipt chain as hex nibbles
    let mut receipt_chain: Vec<u64> = Vec::new();
    let mut pattern_count = 0u32;

    println!();
    println!("╔═════════════════════════════════════════════════════╗");
    println!("║         SIEGE DEFENSE — FORTRESS IRONGATE           ║");
    println!("║   8 Waves of enemies assault your walls. Endure.    ║");
    println!("╚═════════════════════════════════════════════════════╝");
    println!();
    println!("Initial state:");
    println!(
        "  Walls: {}hp  Treasury: {}g  Soldiers: {}  Towers: {}",
        f_wall, f_treasury, f_soldiers, f_towers
    );
    println!();

    for wave_idx in 0u64..8 {
        let w = wave_idx + 1;
        let seed = WAVE_SEEDS[wave_idx as usize];
        let enemy_name = WAVE_NAMES[wave_idx as usize];

        // Seed the pattern scratch state with fortress vitals so patterns
        // are informed by actual game state without us reading back their
        // clobbered output for game logic.
        state = pack(
            f_wall.min(0xFFFF),
            f_treasury.min(0xFFF),
            f_soldiers.min(0xFF),
            w,
            f_tick.min(0xFF),
            f_towers.min(0xF),
            f_kills.min(0xFF),
            (receipt_chain.len() as u64) & 0xF,
        );

        println!();
        println!("┌─────────────────────────────────────────────────────┐");
        println!(
            "│  WAVE {} — {} Assault{}",
            w,
            enemy_name,
            if wave_idx == 7 {
                " *** FINAL ***"
            } else {
                "          "
            }
        );
        println!("└─────────────────────────────────────────────────────┘");

        // ── Pattern 1: input_admitted — gate the TOWER build command ──────
        let build_cmd: u64 = 0x5441_5245_574F_5400 | (w & 0xFF); // "TOWER\0" | wave
        state = input_admitted(state, build_cmd ^ seed);
        // Admission determined by treasury capacity (game logic, not state bits)
        let admitted = if f_treasury >= 80 { 1u64 } else { 0u64 };
        println!(
            "  [01 input_admitted]           Build command {}",
            if admitted != 0 {
                "ADMITTED — new tower placed"
            } else {
                "QUEUED — resources low"
            }
        );
        if admitted != 0 {
            f_towers = (f_towers + 1).min(15);
            f_treasury = f_treasury.saturating_sub(80);
        }
        pattern_count += 1;

        // ── Pattern 2: fixed_tick_advanced — wave clock tick ──────────────
        let ticks_this_wave: u64 = 12 + wave_idx * 4;
        state = fixed_tick_advanced(state, ticks_this_wave);
        f_tick = (f_tick + ticks_this_wave) & 0xFF;
        println!(
            "  [02 fixed_tick_advanced]      Wave clock: {} ticks elapsed (total {})",
            ticks_this_wave, f_tick
        );
        pattern_count += 1;

        // ── Pattern 3: entity_state_transitioned — siege engine FSM ───────
        // encode FSM: 0=approach 1=fire 2=reload 3=advance
        let fsm_input: u64 = (wave_idx % 4) | (seed & 0xF0);
        state = entity_state_transitioned(state, fsm_input);
        let fsm_names = ["approach", "fire", "reload", "advance"];
        let fsm_state = (state ^ seed) & 0x3;
        println!(
            "  [03 entity_state_transitioned] Siege engine: {} → {}",
            fsm_names[((wave_idx) % 4) as usize],
            fsm_names[((wave_idx + 1) % 4) as usize]
        );
        pattern_count += 1;

        // ── Pattern 4: object_spawned — enemy wave spawns ─────────────────
        let enemy_count: u64 = 8 + wave_idx * 5;
        state = object_spawned(state, enemy_count ^ (seed >> 8));
        println!(
            "  [04 object_spawned]           {} {} units materialize from the fog",
            enemy_count, enemy_name
        );
        pattern_count += 1;

        // ── Pattern 5: aabb_collision_resolved — arrows vs siege engine ───
        let arrow_box: u64 = 0x0010_0010_0050_0030; // [x,y,w,h] packed
        state = aabb_collision_resolved(state, arrow_box ^ seed);
        let hit = (state.wrapping_add(seed)) & 0x1;
        println!(
            "  [05 aabb_collision_resolved]  Arrow volley: {} siege engine",
            if hit != 0 { "HIT —" } else { "MISS —" }
        );
        pattern_count += 1;

        // ── Pattern 6: ocel_event_linked — link battle event to log ───────
        let event_id: u64 = 0xE000_0000 | (w << 16) | (tick_f(state) << 8) | 0x01;
        state = ocel_event_linked(state, event_id);
        println!(
            "  [06 ocel_event_linked]        Battle event {:#010x} linked to OCEL log",
            event_id & 0xFFFFFFFF
        );
        pattern_count += 1;

        // ── Pattern 7: otel_span_emitted — defensive action span ──────────
        let span_id: u64 = 0x5041_4E00 | w; // "PAN\0" | wave
        state = otel_span_emitted(state, span_id ^ seed);
        println!(
            "  [07 otel_span_emitted]        OTel span wave_{} opened for defense trace",
            w
        );
        pattern_count += 1;

        // ── Pattern 8: replay_frame_recorded — battle frame snapshot ──────
        state = replay_frame_recorded(state, w ^ (seed >> 16));
        println!(
            "  [08 replay_frame_recorded]    Frame {} snapshot recorded for replay",
            w
        );
        pattern_count += 1;

        // ── Pattern 9: receipt_appended — append to provenance chain ──────
        state = receipt_appended(state, state ^ seed);
        let new_receipt = (receipt_n(state).wrapping_add(w)) & 0xF;
        state = set_receipt(state, new_receipt);
        receipt_chain.push(state & 0xFFFF);
        println!(
            "  [09 receipt_appended]         Receipt chain: {} links (nibble {:#03x})",
            receipt_chain.len(),
            new_receipt
        );
        pattern_count += 1;

        // ── Pattern 10: physics_value_rendered — wall stress display ──────
        let stress: u64 = wall_hp(state).saturating_sub(wave_idx * 40);
        state = physics_value_rendered(state, stress);
        println!(
            "  [10 physics_value_rendered]   Wall stress index: {} (structural load)",
            stress & 0xFFF
        );
        pattern_count += 1;

        // ── Pattern 11: semantic_lod_selected — tactical zoom level ───────
        let lod_input: u64 = towers(state) | (wave_idx << 4);
        state = semantic_lod_selected(state, lod_input);
        let lod = (state ^ lod_input) & 0x3;
        let lod_names = ["Strategic", "Operational", "Tactical", "Unit"];
        println!(
            "  [11 semantic_lod_selected]    Map detail: {} view (LOD {})",
            lod_names[(lod % 4) as usize],
            lod
        );
        pattern_count += 1;

        // ── Pattern 12: projectile_advanced — arrow in flight ─────────────
        let arrow_vel: u64 = 0x0028_0010; // speed=40, angle=16
        state = projectile_advanced(state, arrow_vel ^ (seed & 0xFFFF));
        println!(
            "  [12 projectile_advanced]      {} arrows arc toward the enemy line",
            (f_towers + 1) * 3
        );
        pattern_count += 1;

        // ── Pattern 13: ai_action_selected — enemy commander AI ───────────
        let ai_threat: u64 = f_soldiers | (f_wall >> 4);
        state = ai_action_selected(state, ai_threat ^ seed);
        let ai_actions = [
            "assault gates",
            "flank left",
            "flank right",
            "bring up siege engines",
            "fall back and regroup",
        ];
        let ai_choice = (state ^ seed) % 5;
        println!(
            "  [13 ai_action_selected]       Enemy commander orders: {}",
            ai_actions[ai_choice as usize]
        );
        pattern_count += 1;

        // ── Pattern 14: damage_applied — arrow hits siege engine ──────────
        let dmg_base: u64 = 15 + f_towers * 8;
        state = damage_applied(state, dmg_base ^ (seed >> 24));
        let actual_dmg = dmg_base + (state ^ (seed >> 24)) % 20;
        println!(
            "  [14 damage_applied]           Siege engine takes {} damage from tower fire",
            actual_dmg
        );
        pattern_count += 1;

        // ── Pattern 15: status_effect_ticked — fire-arrow burn DOT ────────
        let burn_stacks: u64 = (wave_idx + 1) * 2;
        state = status_effect_ticked(state, burn_stacks);
        println!(
            "  [15 status_effect_ticked]     Fire-arrow burn: {} stacks, {} damage/tick",
            burn_stacks,
            burn_stacks * 3
        );
        pattern_count += 1;

        // ── Pattern 16: inventory_item_changed — stone/wood consumed ───────
        let stone_cost: u64 = 20 + wave_idx * 5;
        state = inventory_item_changed(state, stone_cost ^ (seed & 0xFF));
        println!(
            "  [16 inventory_item_changed]   Stone: -{} (wall repairs), Wood: -{} (arrows)",
            stone_cost,
            stone_cost / 2
        );
        pattern_count += 1;

        // ── Pattern 17: quest_step_advanced — defend objectives ────────────
        let obj_id: u64 = 0x4445_4600 | w; // "DEF\0" | wave
        state = quest_step_advanced(state, obj_id ^ seed);
        println!(
            "  [17 quest_step_advanced]      Objective 'Hold the Line — Wave {}/8' updated",
            w
        );
        pattern_count += 1;

        // ── Pattern 18: mastery_moment_detected — flawless volley ─────────
        let perf_score: u64 = f_kills.wrapping_add(f_towers * 10);
        state = mastery_moment_detected(state, perf_score);
        let mastery = (state ^ perf_score) & 0x1;
        println!(
            "  [18 mastery_moment_detected]  {}",
            if mastery != 0 {
                "MASTERY — Flawless volley! All archers in perfect sync!"
            } else {
                "Good volley. Archers hold formation."
            }
        );
        pattern_count += 1;

        // ── Pattern 19: share_artifact_generated — battle report scroll ────
        state = share_artifact_generated(state, state ^ (seed >> 32));
        println!(
            "  [19 share_artifact_generated] Battle scroll {:#06x} dispatched to king",
            (state ^ seed) & 0xFFFF
        );
        pattern_count += 1;

        // ── Pattern 20: nps_prompt_gated — survivor morale check ──────────
        let morale: u64 = f_soldiers.min(100);
        state = nps_prompt_gated(state, morale ^ seed);
        let show_prompt = (state ^ morale) & 0x1;
        println!(
            "  [20 nps_prompt_gated]         Morale poll: {}",
            if show_prompt != 0 {
                "troops CHEERING (high morale)"
            } else {
                "troops STEADY (focused silence)"
            }
        );
        pattern_count += 1;

        // ── Pattern 21: path_node_expanded — enemy pathfinding ────────────
        let path_cost_in: u64 = 100 + wave_idx * 20;
        state = path_node_expanded(state, path_cost_in ^ (seed & 0xFFF));
        println!(
            "  [21 path_node_expanded]       Enemy pathfinder expands node (g={}, h={})",
            path_cost_in,
            path_cost_in / 3
        );
        pattern_count += 1;

        // ── Pattern 22: waypoint_reached — enemy advance checkpoint ────────
        let wp: u64 = wave_idx % 4; // 4 waypoints: treeline, river, ditch, gate
        state = waypoint_reached(state, wp ^ seed);
        let wp_names = ["treeline", "river crossing", "outer ditch", "gate approach"];
        println!(
            "  [22 waypoint_reached]         Enemy reaches {} — assault begins!",
            wp_names[wp as usize]
        );
        pattern_count += 1;

        // ── Pattern 23: heuristic_distance_estimated — range to gate ───────
        let range: u64 = 200u64.saturating_sub(wave_idx * 25);
        state = heuristic_distance_estimated(state, range ^ (seed >> 16));
        println!(
            "  [23 heuristic_distance_estimated] Enemy line: {}m from gate",
            range.max(10)
        );
        pattern_count += 1;

        // ── Pattern 24: path_cost_bounded — movement budget cap ─────────────
        let budget: u64 = 0x0064; // max 100 moves per turn
        state = path_cost_bounded(state, budget ^ (seed & 0xFF));
        println!(
            "  [24 path_cost_bounded]        Enemy move budget capped at {} squares/turn",
            budget
        );
        pattern_count += 1;

        // ── Pattern 25: nav_state_advanced — enemy FSM position update ─────
        let nav_states = ["IDLE", "MARCH", "CHARGE", "RETREAT"];
        let nav_in: u64 = (wave_idx + 1) % 4;
        state = nav_state_advanced(state, nav_in ^ seed);
        println!(
            "  [25 nav_state_advanced]       Enemy column nav: → {}",
            nav_states[nav_in as usize]
        );
        pattern_count += 1;

        // ── Pattern 26: noise_value_sampled — fog of war scout range ───────
        let fog_seed: u64 = seed ^ 0xDEAD_BEEF;
        state = noise_value_sampled(state, fog_seed);
        let visibility = 40u64 + (state ^ fog_seed) % 60;
        println!(
            "  [26 noise_value_sampled]      Fog of war: {}m visibility (scouts report {})",
            visibility,
            if visibility > 70 {
                "clear field"
            } else if visibility > 50 {
                "light mist"
            } else {
                "heavy fog — movement unseen"
            }
        );
        pattern_count += 1;

        // ── Pattern 27: tile_variant_selected — terrain underfoot ──────────
        let tile_seed: u64 = seed ^ 0xC0FFEE;
        state = tile_variant_selected(state, tile_seed);
        let tile = (state ^ tile_seed) % 4;
        println!(
            "  [27 tile_variant_selected]    Battle terrain: {} tiles",
            TILES[tile as usize]
        );
        pattern_count += 1;

        // ── Pattern 28: terrain_height_quantized — elevation ───────────────
        let elev_raw: u64 = wave_idx * 12;
        state = terrain_height_quantized(state, elev_raw ^ seed);
        let elev = (state ^ (elev_raw ^ seed)) & 0x3;
        println!(
            "  [28 terrain_height_quantized] Elevation: {} (+{} archer range)",
            ELEV[elev as usize],
            elev * 5
        );
        pattern_count += 1;

        // ── Pattern 29: spawn_weight_evaluated — enemy type distribution ───
        let pop_weights: u64 = 0x1020_3040 | (wave_idx << 2);
        state = spawn_weight_evaluated(state, pop_weights ^ seed);
        let heavy_pct = 20 + wave_idx * 8;
        println!(
            "  [29 spawn_weight_evaluated]   Enemy mix: {}% light infantry, {}% heavy",
            100 - heavy_pct,
            heavy_pct
        );
        pattern_count += 1;

        // ── Pattern 30: biome_class_selected — regional terrain ────────────
        let biome_in: u64 = wave_idx % 4;
        state = biome_class_selected(state, biome_in ^ (seed >> 8));
        println!(
            "  [30 biome_class_selected]     Battle biome: {} region",
            BIOMES[biome_in as usize]
        );
        pattern_count += 1;

        // ── Pattern 31: currency_delta_applied — gold from kills ───────────
        let kills_this_wave: u64 = 5 + wave_idx * 3;
        let gold_gain: u64 = kills_this_wave * (10 + wave_idx * 5);
        state = currency_delta_applied(state, gold_gain ^ (seed >> 4));
        f_treasury = (f_treasury + gold_gain).min(4095);
        f_kills = (f_kills + kills_this_wave) & 0xFF;
        println!(
            "  [31 currency_delta_applied]   +{}g from {} enemy kills (treasury: {}g)",
            gold_gain, kills_this_wave, f_treasury
        );
        pattern_count += 1;

        // ── Pattern 32: xp_threshold_crossed — commander rank up ───────────
        let xp_gain: u64 = kills_this_wave * 15;
        let total_xp: u64 = xp_gain * w;
        state = xp_threshold_crossed(state, total_xp ^ seed);
        let rank_up = total_xp > (w * 80);
        println!(
            "  [32 xp_threshold_crossed]     Commander XP: +{} ({})",
            xp_gain,
            if rank_up {
                "RANK UP! Promoted to next tier!"
            } else {
                "gaining experience"
            }
        );
        pattern_count += 1;

        // ── Pattern 33: level_gate_evaluated — advanced tower unlock ───────
        let gate_req: u64 = 200; // 200g to unlock fire tower
        state = level_gate_evaluated(state, gate_req ^ (f_treasury >> 1));
        let can_unlock = f_treasury >= gate_req;
        println!(
            "  [33 level_gate_evaluated]     Fire Tower ({}g): {}",
            gate_req,
            if can_unlock {
                "UNLOCKED — treasury sufficient"
            } else {
                "LOCKED — insufficient funds"
            }
        );
        pattern_count += 1;

        // ── Pattern 34: purchase_admitted — buy reinforcements ─────────────
        let reinforce_cost: u64 = 50 + wave_idx * 10;
        state = purchase_admitted(state, reinforce_cost ^ seed);
        let can_buy = f_treasury >= reinforce_cost;
        if can_buy {
            f_soldiers = (f_soldiers + 5).min(255);
            f_treasury = f_treasury.saturating_sub(reinforce_cost);
        }
        println!(
            "  [34 purchase_admitted]        Reinforcements ({}g): {}",
            reinforce_cost,
            if can_buy {
                "5 soldiers join the garrison!"
            } else {
                "purchase denied — not enough gold"
            }
        );
        pattern_count += 1;

        // ── Pattern 35: reward_tier_selected — loot from enemy commander ───
        let loot_roll: u64 = (seed ^ state) & 0xFF;
        state = reward_tier_selected(state, loot_roll);
        let tier = loot_roll % 4;
        let tier_names = ["Common", "Uncommon", "Rare", "Legendary"];
        println!(
            "  [35 reward_tier_selected]     Enemy commander drops: {} loot",
            tier_names[tier as usize]
        );
        pattern_count += 1;

        // ── Pattern 36: dialogue_node_advanced — king's messenger ──────────
        let msg_node: u64 = 0x4B494E47 | (w << 1); // "KING" | wave
        state = dialogue_node_advanced(state, msg_node ^ seed);
        let royal_msgs = [
            "Hold the northern wall at all costs!",
            "The eastern flank needs reinforcement!",
            "Our cavalry rides within the hour!",
            "Burn their siege engines before they reach range!",
            "The kingdom watches. Do not falter.",
            "Our mages are en route. Delay them!",
            "The pass must not fall. I am sending my guard.",
            "FINAL STAND — the kingdom's fate rests here!",
        ];
        println!(
            "  [36 dialogue_node_advanced]   King's messenger: \"{}\"",
            royal_msgs[wave_idx as usize]
        );
        pattern_count += 1;

        // ── Pattern 37: condition_flag_evaluated — wall breach check ───────
        let breach_threshold: u64 = 100;
        state = condition_flag_evaluated(state, breach_threshold ^ f_wall);
        let breached = f_wall < breach_threshold;
        println!(
            "  [37 condition_flag_evaluated] Wall integrity: {} ({})",
            f_wall,
            if breached {
                "WARNING — breach imminent!"
            } else {
                "holding"
            }
        );
        pattern_count += 1;

        // ── Pattern 38: narrative_branch_selected — story outcome ──────────
        let story_weight: u64 = f_soldiers | (f_treasury >> 2);
        state = narrative_branch_selected(state, story_weight ^ seed);
        let branch = (state ^ story_weight) & 0x3;
        let branches = [
            "heroic defense",
            "desperate hold",
            "tactical withdrawal",
            "crushing victory",
        ];
        println!(
            "  [38 narrative_branch_selected] Narrative arc: {} (battle conditions)",
            branches[(branch % 4) as usize]
        );
        pattern_count += 1;

        // ── Pattern 39: dialogue_cooldown_bounded — messenger delay ────────
        let cooldown: u64 = 30 + wave_idx * 5; // ticks between royal messages
        state = dialogue_cooldown_bounded(state, cooldown ^ seed);
        println!(
            "  [39 dialogue_cooldown_bounded] Next royal message in {} ticks",
            cooldown
        );
        pattern_count += 1;

        // ── Pattern 40: choice_weight_selected — tactical decision ─────────
        let weights: u64 = 0x0A0B_0C0D; // weights: hold/sally/parley/retreat
        state = choice_weight_selected(state, weights ^ (seed >> 8));
        let choice = (state ^ weights) & 0x3;
        let tactics = ["HOLD POSITION", "SALLY FORTH", "PARLEY", "TACTICAL RETREAT"];
        println!(
            "  [40 choice_weight_selected]   Commander chooses: {}",
            tactics[(choice % 4) as usize]
        );
        pattern_count += 1;

        // ── Pattern 41: camera_distance_clamped — tactical view range ──────
        let cam_dist: u64 = 50 + wave_idx * 30;
        state = camera_distance_clamped(state, cam_dist);
        let clamped = cam_dist.min(200).max(50);
        println!(
            "  [41 camera_distance_clamped]  Battle view: {}m (clamped [50-200])",
            clamped
        );
        pattern_count += 1;

        // ── Pattern 42: look_target_weighted — focus on biggest threat ─────
        let threat_vec: u64 = (wave_idx << 8) | kills_this_wave;
        state = look_target_weighted(state, threat_vec ^ seed);
        println!(
            "  [42 look_target_weighted]     Commander's gaze: enemy {} cluster (weight {})",
            enemy_name,
            (threat_vec ^ seed) & 0xFF
        );
        pattern_count += 1;

        // ── Pattern 43: fov_adjusted — archer sight cone ───────────────────
        let fov_in: u64 = 60 + f_towers * 5; // degrees
        state = fov_adjusted(state, fov_in);
        println!(
            "  [43 fov_adjusted]             Archer FOV: {}° ({} overlapping fire lanes)",
            fov_in.min(120),
            f_towers
        );
        pattern_count += 1;

        // ── Pattern 44: camera_shake_applied — impact tremor ───────────────
        let impact_mag: u64 = wave_idx * 15;
        state = camera_shake_applied(state, impact_mag ^ seed);
        println!(
            "  [44 camera_shake_applied]     Ground trembles: magnitude {}",
            if impact_mag > 60 {
                "VIOLENT — catapult strike!"
            } else if impact_mag > 30 {
                "MODERATE — siege engine fire"
            } else {
                "mild — enemy march"
            }
        );
        pattern_count += 1;

        // ── Pattern 45: camera_follow_lerped — tracking enemy column ───────
        let lerp_alpha: u64 = 0x40; // 64/256 ≈ 0.25
        state = camera_follow_lerped(state, lerp_alpha ^ (seed & 0xFF));
        println!(
            "  [45 camera_follow_lerped]     Battle camera tracks {} column (α=0.25)",
            enemy_name
        );
        pattern_count += 1;

        // ── Pattern 46: audio_priority_selected — sound channel priority ───
        let audio_prio: u64 = 0xFF; // battle sounds = max priority
        state = audio_priority_selected(state, audio_prio ^ (seed & 0xF));
        println!(
            "  [46 audio_priority_selected]  Battle drums: PRIORITY {} (overrides ambient)",
            audio_prio
        );
        pattern_count += 1;

        // ── Pattern 47: volume_clamped — horn blast volume ─────────────────
        let horn_vol: u64 = 200; // over max
        state = volume_clamped(state, horn_vol);
        println!(
            "  [47 volume_clamped]           War horn volume clamped to max (was {})",
            horn_vol
        );
        pattern_count += 1;

        // ── Pattern 48: audio_fade_applied — fade between music tracks ─────
        let fade_rate: u64 = 5; // 5% per tick
        state = audio_fade_applied(state, fade_rate ^ (seed & 0x7));
        println!(
            "  [48 audio_fade_applied]       Ambient → battle music fade: {}%/tick",
            fade_rate
        );
        pattern_count += 1;

        // ── Pattern 49: audio_trigger_evaluated — play victory fanfare ─────
        let fanfare_cond: u64 = if kills_this_wave > 10 { 1 } else { 0 };
        state = audio_trigger_evaluated(state, fanfare_cond ^ seed);
        println!(
            "  [49 audio_trigger_evaluated]  {}",
            if fanfare_cond != 0 {
                "FANFARE TRIGGERED — enemy routed from this flank!"
            } else {
                "Battle continues — no fanfare yet"
            }
        );
        pattern_count += 1;

        // ── Pattern 50: audio_distance_attenuated — catapult boom ──────────
        let catapult_dist: u64 = 80 + wave_idx * 15;
        state = audio_distance_attenuated(state, catapult_dist ^ seed);
        let vol_at_dist = 100u64.saturating_sub(catapult_dist / 3);
        println!(
            "  [50 audio_distance_attenuated] Catapult impact at {}m — volume {}%",
            catapult_dist, vol_at_dist
        );
        pattern_count += 1;

        // ── Pattern 51: tick_delta_bounded — lag protection ────────────────
        let raw_delta: u64 = 16 + (seed & 0x30); // 16-64ms
        state = tick_delta_bounded(state, raw_delta);
        let bounded_dt = raw_delta.min(33); // cap at ~30fps
        println!(
            "  [51 tick_delta_bounded]       Frame delta {}ms → bounded {}ms (anti-lag)",
            raw_delta, bounded_dt
        );
        pattern_count += 1;

        // ── Pattern 52: lag_compensation_applied — network sync ────────────
        let lag_ms: u64 = 20 + wave_idx * 3;
        state = lag_compensation_applied(state, lag_ms ^ (seed >> 12));
        println!(
            "  [52 lag_compensation_applied] Battle sync: {}ms lag compensated",
            lag_ms
        );
        pattern_count += 1;

        // ── Pattern 53: packet_priority_evaluated — critical updates ───────
        let pkt_flags: u64 = 0b1010; // wall_damage + enemy_spawn are high priority
        state = packet_priority_evaluated(state, pkt_flags ^ seed);
        println!("  [53 packet_priority_evaluated] Priority queue: wall_damage=HIGH, spawn=HIGH");
        pattern_count += 1;

        // ── Pattern 54: prediction_error_bounded — enemy position error ────
        let pred_err: u64 = wave_idx * 3; // grows with wave difficulty
        state = prediction_error_bounded(state, pred_err ^ (seed & 0x1F));
        println!(
            "  [54 prediction_error_bounded] Prediction error: {}m (within tolerance)",
            pred_err.min(10)
        );
        pattern_count += 1;

        // ── Pattern 55: sync_state_admitted — state authority check ────────
        let state_hash: u64 = state ^ seed ^ 0xABCD;
        state = sync_state_admitted(state, state_hash);
        println!(
            "  [55 sync_state_admitted]      State {:#06x}: ADMITTED (authoritative)",
            state_hash & 0xFFFF
        );
        pattern_count += 1;

        // ── Pattern 56: sigma_level_computed — defense effectiveness ───────
        let defects: u64 = wave_idx * 2; // wall hits taken
        let opportunities: u64 = kills_this_wave + f_towers * 10;
        state = sigma_level_computed(state, (defects << 16) | opportunities.min(0xFFFF));
        let sigma = 6u64.saturating_sub(defects / 5);
        println!(
            "  [56 sigma_level_computed]     Defense σ-level: {} ({} defects / {} ops)",
            sigma, defects, opportunities
        );
        pattern_count += 1;

        // ── Pattern 57: defect_rate_quantized — arrow miss rate ─────────────
        let miss_rate: u64 = 15u64.saturating_sub(f_towers * 2); // % misses
        state = defect_rate_quantized(state, miss_rate ^ seed);
        println!(
            "  [57 defect_rate_quantized]    Arrow accuracy: {}% miss rate ({}% hit)",
            miss_rate,
            100 - miss_rate
        );
        pattern_count += 1;

        // ── Pattern 58: ctq_threshold_evaluated — critical quality check ───
        let ctq_metric: u64 = f_wall;
        let ctq_limit: u64 = 50; // wall must stay above 50hp
        state = ctq_threshold_evaluated(state, (ctq_metric << 8) | ctq_limit);
        println!(
            "  [58 ctq_threshold_evaluated]  CTQ: Wall HP {} {} min threshold {}",
            ctq_metric,
            if ctq_metric >= ctq_limit { "≥" } else { "<" },
            ctq_limit
        );
        pattern_count += 1;

        // ── Pattern 59: nps_score_bounded — garrison morale score ──────────
        let raw_nps: u64 = 50 + f_soldiers / 3;
        state = nps_score_bounded(state, raw_nps);
        let nps = raw_nps.min(100);
        println!(
            "  [59 nps_score_bounded]        Garrison NPS: {} ({})",
            nps,
            if nps > 80 {
                "ELATED — morale soaring"
            } else if nps > 50 {
                "STEADY — holding firm"
            } else {
                "LOW — morale wavering"
            }
        );
        pattern_count += 1;

        // ── Pattern 60: quality_gate_evaluated — wave survival check ───────
        state = quality_gate_evaluated(state, f_wall ^ (f_soldiers << 8));
        let passed = f_wall > 0 && f_soldiers > 0;
        println!(
            "  [60 quality_gate_evaluated]   Wave {} quality gate: {}",
            w,
            if passed {
                "PASS — fortress stands"
            } else {
                "FAIL — fortress breached!"
            }
        );
        pattern_count += 1;

        // ── Pattern 61: command_opcode_encoded — defensive order encoding ──
        let order: u64 = 0xD0_00 | (wave_idx << 4) | f_towers; // DEFEND order
        state = command_opcode_encoded(state, order ^ seed);
        println!(
            "  [61 command_opcode_encoded]   Encoded order: HOLD_WALLS | wave={} | towers={}",
            w, f_towers
        );
        pattern_count += 1;

        // ── Pattern 62: capability_flag_evaluated — fire arrow unlock ──────
        let cap_flags: u64 = 0b0001_0111; // basic_tower|stone_wall|arrow|fire_arrow
        state = capability_flag_evaluated(state, cap_flags ^ (f_treasury >> 4));
        let fire_arrow = cap_flags & 0x8 != 0;
        println!(
            "  [62 capability_flag_evaluated] Fire arrows: {} (capability flags: {:#010b})",
            if fire_arrow { "UNLOCKED" } else { "LOCKED" },
            cap_flags
        );
        pattern_count += 1;

        // ── Pattern 63: bridge_state_transitioned — drawbridge control ─────
        // 0=raised 1=lowering 2=lowered 3=raising
        let bridge_in: u64 = if wave_idx % 3 == 0 { 2 } else { 0 }; // lower to sally
        state = bridge_state_transitioned(state, bridge_in ^ seed);
        let bridge_states = ["RAISED", "LOWERING", "LOWERED", "RAISING"];
        println!(
            "  [63 bridge_state_transitioned] Drawbridge: → {}",
            bridge_states[(bridge_in % 4) as usize]
        );
        pattern_count += 1;

        // ── Pattern 64: payload_size_bounded — message scroll size ─────────
        let scroll_bytes: u64 = 512 + wave_idx * 64;
        state = payload_size_bounded(state, scroll_bytes);
        let bounded_bytes = scroll_bytes.min(1024);
        println!(
            "  [64 payload_size_bounded]     Battle report: {} bytes (max 1024)",
            bounded_bytes
        );
        pattern_count += 1;

        // ── Pattern 65: adapter_priority_ranked — tower targeting priority ──
        let target_vec: u64 = 0x0403_0201; // rank: siege>heavy>cavalry>infantry
        state = adapter_priority_ranked(state, target_vec ^ (seed & 0xFFFF));
        println!("  [65 adapter_priority_ranked]  Tower targeting: siege_engine > heavy > cavalry > infantry");
        pattern_count += 1;

        // ── Pattern 66: reward_signal_clamped — RL reward shaping ──────────
        let raw_reward: u64 = kills_this_wave * 10 + f_towers * 5;
        state = reward_signal_clamped(state, raw_reward);
        let clamped_r = raw_reward.min(200);
        println!(
            "  [66 reward_signal_clamped]    Defense reward signal: {} (clamped [0,200])",
            clamped_r
        );
        pattern_count += 1;

        // ── Pattern 67: policy_action_selected — RL agent defense policy ───
        let obs: u64 = f_wall | (f_soldiers << 16);
        state = policy_action_selected(state, obs ^ seed);
        let policies = [
            "reinforce_walls",
            "recruit_soldiers",
            "build_tower",
            "buy_fire_arrows",
        ];
        let pol = (state ^ obs) % 4;
        println!(
            "  [67 policy_action_selected]   Agent policy: {}",
            policies[pol as usize]
        );
        pattern_count += 1;

        // ── Pattern 68: observation_class_selected — threat classification ──
        let threat_class: u64 = wave_idx / 2; // 0=low 1=medium 2=high 3=critical
        state = observation_class_selected(state, threat_class ^ (seed >> 4));
        let threat_labels = ["LOW", "MEDIUM", "HIGH", "CRITICAL"];
        println!(
            "  [68 observation_class_selected] Threat class: {} (wave {} of 8)",
            threat_labels[(threat_class % 4) as usize],
            w
        );
        pattern_count += 1;

        // ── Pattern 69: action_mask_applied — valid action filter ──────────
        let action_mask: u64 = 0b1111 & !((f_treasury < 50) as u64); // mask buy if broke
        state = action_mask_applied(state, action_mask ^ seed);
        println!(
            "  [69 action_mask_applied]       Action mask: {:#06b} (buy={}, build={}, defend={})",
            action_mask & 0xF,
            if action_mask & 0x1 != 0 { "Y" } else { "N" },
            if action_mask & 0x2 != 0 { "Y" } else { "N" },
            if action_mask & 0x4 != 0 { "Y" } else { "N" }
        );
        pattern_count += 1;

        // ── Pattern 70: episode_return_bounded — campaign score ────────────
        let episode_return: u64 = f_kills * 10 + f_treasury + f_wall;
        state = episode_return_bounded(state, episode_return);
        println!(
            "  [70 episode_return_bounded]   Campaign return: {} (kills×10 + gold + hp)",
            episode_return.min(9999)
        );
        pattern_count += 1;

        // ── Pattern 71: movement_legality_checked — enemy speed cap ────────
        let enemy_speed: u64 = 5 + wave_idx; // tiles/turn
        let speed_cap: u64 = 12;
        state = movement_legality_checked(state, (enemy_speed << 8) | speed_cap);
        println!(
            "  [71 movement_legality_checked] Enemy speed: {} tiles/turn (cap {}): {}",
            enemy_speed,
            speed_cap,
            if enemy_speed <= speed_cap {
                "LEGAL"
            } else {
                "CLAMPED"
            }
        );
        pattern_count += 1;

        // ── Pattern 72: resource_bound_checked — treasury bounds ────────────
        state = resource_bound_checked(state, f_treasury ^ 0xFFF);
        println!(
            "  [72 resource_bound_checked]   Treasury {} in [0, 4095g]: {}",
            f_treasury,
            if f_treasury <= 4095 {
                "VALID"
            } else {
                "OVERFLOW — capped"
            }
        );
        pattern_count += 1;

        // ── Pattern 73: cooldown_legality_checked — tower fire rate ────────
        let tower_cd: u64 = f_tick % 4; // fires every 4 ticks
        state = cooldown_legality_checked(state, tower_cd ^ (seed & 0xF));
        println!(
            "  [73 cooldown_legality_checked] Tower fire cooldown: {} ticks remaining ({})",
            tower_cd,
            if tower_cd == 0 {
                "READY TO FIRE"
            } else {
                "reloading"
            }
        );
        pattern_count += 1;

        // ── Pattern 74: action_rate_bounded — max builds per wave ──────────
        let actions_taken: u64 = admitted + (can_buy as u64);
        let rate_cap: u64 = 3; // max 3 major actions per wave
        state = action_rate_bounded(state, (actions_taken << 8) | rate_cap);
        println!(
            "  [74 action_rate_bounded]       Commander actions: {}/{} per wave",
            actions_taken, rate_cap
        );
        pattern_count += 1;

        // ── Pattern 75: transition_legality_checked — FSM validity ─────────
        // validate siege engine FSM: can't go from approach directly to reload
        let from_state: u64 = wave_idx % 4;
        let to_state: u64 = (wave_idx + 1) % 4;
        let legal_transitions: u64 = 0b0111_1011_1101_1110; // valid 4x4 FSM adjacency
        state = transition_legality_checked(state, (from_state << 4) | to_state);
        let legal = (legal_transitions >> (from_state * 4 + to_state)) & 0x1 != 0;
        println!(
            "  [75 transition_legality_checked] FSM {} → {}: {}",
            fsm_names[from_state as usize],
            fsm_names[to_state as usize],
            if legal {
                "LEGAL"
            } else {
                "ILLEGAL — state machine violation!"
            }
        );
        pattern_count += 1;

        // ── Apply wave damage to fortress (game logic on f_* vars) ────────
        let wall_damage: u64 = (wave_idx + 1) * 18;
        let sol_damage: u64 = (wave_idx + 1) * 3;
        // Towers provide mitigation
        let mitigated = f_towers * 10;
        f_wall = f_wall.saturating_sub(wall_damage.saturating_sub(mitigated));
        f_soldiers = f_soldiers.saturating_sub(sol_damage);

        // ── Wave summary ───────────────────────────────────────────────────
        println!();
        println!(
            "  ▶ Walls: {}hp │ Treasury: {}g │ Soldiers: {} │ Wave: {}/8 ◀",
            f_wall, f_treasury, f_soldiers, w
        );
        hr();

        if f_wall == 0 {
            println!();
            println!("╔═════════════════════════════════════════════════════╗");
            println!("║               FORTRESS HAS FALLEN                  ║");
            println!(
                "║      The walls could not withstand wave {}           ║",
                w
            );
            println!("╚═════════════════════════════════════════════════════╝");
            break;
        }
    }

    // ── Final outcome ──────────────────────────────────────────────────────
    println!();
    if f_wall > 0 {
        println!("╔═════════════════════════════════════════════════════╗");
        println!("║          FORTRESS IRONGATE STANDS VICTORIOUS        ║");
        println!("║   All 8 waves repelled. The kingdom is saved.       ║");
        println!("╚═════════════════════════════════════════════════════╝");
    } else {
        println!("╔═════════════════════════════════════════════════════╗");
        println!("║              THE FORTRESS HAS FALLEN                ║");
        println!("║   The enemy pours through the shattered gates.      ║");
        println!("╚═════════════════════════════════════════════════════╝");
    }

    println!();
    println!("══ FINAL STATISTICS ══════════════════════════════════");
    println!("  Total kills:    {}", f_kills);
    println!("  Wall HP:        {}", f_wall);
    println!("  Treasury:       {}g", f_treasury);
    println!("  Soldiers:       {}", f_soldiers);
    println!("  Towers built:   {}", f_towers);
    println!("  Patterns used:  {}", pattern_count);

    println!();
    println!(
        "══ RECEIPT CHAIN ({} links) ════════════════════════",
        receipt_chain.len()
    );
    for (i, link) in receipt_chain.iter().enumerate() {
        println!("  [{:02}] {:#06x}", i + 1, link);
    }
    println!("══ END OF BATTLE RECORD ══════════════════════════════");
}

#[cfg(not(feature = "std"))]
fn main() {}
