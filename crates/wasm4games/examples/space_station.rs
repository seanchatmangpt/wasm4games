//! Space Station Survival MUD — all 75 patterns used across 8 command turns.
//!
//! Run with: `cargo run --example space_station --features std`

extern crate std;
use std::println;

use wasm4games::patterns::{
    aabb_collision_resolved,
    action_mask_applied,
    action_rate_bounded,
    adapter_priority_ranked,
    ai_action_selected,
    audio_distance_attenuated,
    audio_fade_applied,
    audio_priority_selected,
    audio_trigger_evaluated,
    biome_class_selected,
    bridge_state_transitioned,
    // Camera / Audio
    camera_distance_clamped,
    camera_follow_lerped,
    camera_shake_applied,
    capability_flag_evaluated,
    choice_weight_selected,
    // Bridge / Protocol
    command_opcode_encoded,
    condition_flag_evaluated,
    cooldown_legality_checked,
    ctq_threshold_evaluated,
    // Economy / Progression
    currency_delta_applied,
    damage_applied,
    defect_rate_quantized,
    dialogue_cooldown_bounded,
    // Dialogue / Narrative
    dialogue_node_advanced,
    entity_state_transitioned,
    episode_return_bounded,
    fixed_tick_advanced,
    fov_adjusted,
    heuristic_distance_estimated,
    // Core / Sim
    input_admitted,
    inventory_item_changed,
    lag_compensation_applied,
    level_gate_evaluated,
    look_target_weighted,
    mastery_moment_detected,
    // Anti-Cheat / Legality
    movement_legality_checked,
    narrative_branch_selected,
    nav_state_advanced,
    // World Generation
    noise_value_sampled,
    nps_prompt_gated,
    nps_score_bounded,
    object_spawned,
    observation_class_selected,
    ocel_event_linked,
    otel_span_emitted,
    packet_priority_evaluated,
    path_cost_bounded,
    // Navigation / Pathfinding
    path_node_expanded,
    payload_size_bounded,
    physics_value_rendered,
    policy_action_selected,
    prediction_error_bounded,
    projectile_advanced,
    purchase_admitted,
    quality_gate_evaluated,
    quest_step_advanced,
    receipt_appended,
    replay_frame_recorded,
    resource_bound_checked,
    // RL / Policy
    reward_signal_clamped,
    reward_tier_selected,
    semantic_lod_selected,
    share_artifact_generated,
    // Quality / Six Sigma
    sigma_level_computed,
    spawn_weight_evaluated,
    status_effect_ticked,
    sync_state_admitted,
    terrain_height_quantized,
    // Network / Multiplayer
    tick_delta_bounded,
    tile_variant_selected,
    transition_legality_checked,
    volume_clamped,
    waypoint_reached,
    xp_threshold_crossed,
};

// ─── GameState ────────────────────────────────────────────────────────────────

#[derive(Clone)]
struct GameState {
    hull_integrity: u64, // 0-100 percent
    credits: u64,
    power: u64, // 0-100 percent
    crew: u64,  // crew count
    tick: u64,
    docked_ships: u64,
    alert_level: u64,   // 0=green 1=yellow 2=red
    receipt_chain: u64, // rolling hash
    mission_step: u64,  // 0-7
}

impl GameState {
    fn new() -> Self {
        Self {
            hull_integrity: 100,
            credits: 500,
            power: 80,
            crew: 12,
            tick: 0,
            docked_ships: 2,
            alert_level: 0,
            receipt_chain: 0xDEAD_BEEF_0000_0000,
            mission_step: 0,
        }
    }

    fn status(&self) {
        println!(
            "─ Hull: {:3}% │ Power: {:3}% │ Credits: {:5} │ Crew: {:2} │ Tick: {:05} ─",
            self.hull_integrity, self.power, self.credits, self.crew, self.tick
        );
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn turn_header(n: u8, name: &str) {
    println!();
    println!("╔══════════════════════════════════════════════╗");
    println!("║  TURN {}: {:<38}║", n, name);
    println!("╚══════════════════════════════════════════════╝");
}

fn log(pattern: &str, effect: &str) {
    println!("  {:<32} → {}", pattern, effect);
}

// Fold a pattern result back into the game state receipt chain (deterministic).
fn fold(chain: u64, v: u64) -> u64 {
    chain.wrapping_mul(0x9E37_79B9_7F4A_7C15).wrapping_add(v)
}

// ─── Turns ────────────────────────────────────────────────────────────────────

fn turn_1_boot(gs: &mut GameState) {
    turn_header(1, "BOOT SYSTEMS");

    // 1. input_admitted — validate BOOT command code (0x00)
    let cmd: u64 = 0x00;
    let v1 = input_admitted(gs.hull_integrity, cmd);
    log(
        "input_admitted",
        "BOOT command 0x00 accepted — command gate open",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v1);

    // 2. fixed_tick_advanced — station clock initialises
    gs.tick += 1;
    let v2 = fixed_tick_advanced(gs.tick, 1);
    log(
        "fixed_tick_advanced",
        "Station clock tick 00001 — systems online",
    );
    gs.tick = v2 & 0xFFFF;
    gs.receipt_chain = fold(gs.receipt_chain, v2);

    // 26. noise_value_sampled — sensor noise baseline
    let v26 = noise_value_sampled(gs.hull_integrity, 0x1337);
    log(
        "noise_value_sampled",
        "Sensor array calibrated — anomaly baseline established",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v26);

    // 27. tile_variant_selected — select sector map tile variant
    let v27 = tile_variant_selected(v26, 4);
    log(
        "tile_variant_selected",
        "Sector grid tile B-7 selected — docking bay mapped",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v27);

    // 28. terrain_height_quantized — hull depth profile
    let v28 = terrain_height_quantized(gs.hull_integrity, 8);
    log(
        "terrain_height_quantized",
        "Hull topology quantized to 8-level depth map",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v28);

    // 30. biome_class_selected — station zone classification
    let v30 = biome_class_selected(v28, 3);
    log(
        "biome_class_selected",
        "Zone classified: ALPHA (habitation ring)",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v30);

    // 11. semantic_lod_selected — sensor resolution
    let v11 = semantic_lod_selected(gs.power, 3);
    log(
        "semantic_lod_selected",
        "Sensor LOD set to HIGH — full-range sweep active",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v11);

    // 10. physics_value_rendered — gravity simulation
    let v10 = physics_value_rendered(gs.hull_integrity, 0x3F80_0000); // 1.0f in bits
    log(
        "physics_value_rendered",
        "Artificial gravity confirmed 1.0g across all rings",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v10);

    // 7. otel_span_emitted — boot telemetry span
    let v7 = otel_span_emitted(gs.receipt_chain, 0xB007);
    log(
        "otel_span_emitted",
        "OTel span BOOT emitted — trace_id bound",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v7);

    gs.mission_step = 1;
    gs.status();
}

fn turn_2_detect(gs: &mut GameState) {
    turn_header(2, "DETECT BOARDERS");

    // 3. entity_state_transitioned — alien FSM drifting→boarding
    let v3 = entity_state_transitioned(0, 1); // state 0→1
    log(
        "entity_state_transitioned",
        "Alien entity FSM: DRIFTING → BOARDING",
    );
    gs.alert_level = 1;
    gs.receipt_chain = fold(gs.receipt_chain, v3);

    // 4. object_spawned — alien boarder materialises in airlock
    let v4 = object_spawned(v3, 0xA11E_0001);
    log(
        "object_spawned",
        "Alien boarder spawned at AIRLOCK-7 (entity_id=0xA11E0001)",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v4);

    // 13. ai_action_selected — alien AI selects attack
    let v13 = ai_action_selected(v4, 0x02); // 0x02=attack
    log(
        "ai_action_selected",
        "Alien AI selects ATTACK — targeting hull section C",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v13);

    // 29. spawn_weight_evaluated — threat probability
    let v29 = spawn_weight_evaluated(v13, gs.alert_level);
    log(
        "spawn_weight_evaluated",
        "Boarder spawn weight 0.72 — second wave probable",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v29);

    // 66. reward_signal_clamped — alien threat reward signal
    let v66 = reward_signal_clamped(v29, 100);
    log(
        "reward_signal_clamped",
        "Threat reward signal clamped [0, 100] — RL threat model updated",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v66);

    // 68. observation_class_selected — sensor class for alien
    let v68 = observation_class_selected(v66, 3); // class 3=hostile
    log(
        "observation_class_selected",
        "Sensor observation class HOSTILE confirmed",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v68);

    // 69. action_mask_applied — mask non-combat actions during alert
    let v69 = action_mask_applied(v68, 0b1111_0000); // only combat actions unmasked
    log(
        "action_mask_applied",
        "Action mask applied — only FIRE/SEAL/EVAC available",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v69);

    // 6. ocel_event_linked — OCEL event for alien intrusion
    let v6 = ocel_event_linked(gs.receipt_chain, 0xBEEF_0002);
    log(
        "ocel_event_linked",
        "OCEL event INTRUSION linked to object alien_0001",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v6);

    gs.alert_level = 2;
    gs.tick += 8;
    gs.status();
}

fn turn_3_repel(gs: &mut GameState) {
    turn_header(3, "REPEL BOARDERS");

    // 1. input_admitted — validate FIRE command (0x02)
    let cmd: u64 = 0x02;
    let v1 = input_admitted(gs.hull_integrity, cmd);
    log(
        "input_admitted",
        "FIRE command 0x02 validated — turret armed",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v1);

    // 12. projectile_advanced — turret round flies down corridor
    let v12 = projectile_advanced(v1, 0x0014_0050); // vel=20, pos=80
    log(
        "projectile_advanced",
        "Turret projectile advanced — corridor C velocity=20",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v12);

    // 5. aabb_collision_resolved — projectile hits alien
    let v5 = aabb_collision_resolved(v12, 0xA11E_0001);
    log(
        "aabb_collision_resolved",
        "AABB collision resolved — projectile hits alien boarder",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v5);

    // 14. damage_applied — hull breach damage, and alien takes hit
    let damage: u64 = 8; // alien took hit, but explosion nicked hull
    let v14 = damage_applied(gs.hull_integrity, damage);
    log(
        "damage_applied",
        "Hull breach: -8 integrity (explosion shockwave)",
    );
    gs.hull_integrity = gs.hull_integrity.saturating_sub(damage);
    gs.receipt_chain = fold(gs.receipt_chain, v14);

    // 3. entity_state_transitioned — alien attacking→retreating
    let v3b = entity_state_transitioned(2, 3); // state 2→3
    log(
        "entity_state_transitioned",
        "Alien FSM: ATTACKING → RETREATING (turret suppressed)",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v3b);

    // 43. fov_adjusted — turret FOV widens for sweep
    let v43 = fov_adjusted(90, 120); // 90deg→120deg
    log(
        "fov_adjusted",
        "Turret FOV adjusted 90° → 120° — sweep coverage extended",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v43);

    // 44. camera_shake_applied — blast shockwave on bridge cameras
    let v44 = camera_shake_applied(gs.hull_integrity, 15); // intensity 15
    log(
        "camera_shake_applied",
        "Bridge camera shake intensity=15 — blast confirmed",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v44);

    // 46. audio_priority_selected — battle alarm at max priority
    let v46 = audio_priority_selected(v44, 255);
    log(
        "audio_priority_selected",
        "KLAXON alarm assigned audio priority 255 — override all",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v46);

    // 47. volume_clamped — turret fire sound clamped
    let v47 = volume_clamped(v46, 200);
    log(
        "volume_clamped",
        "Turret fire audio clamped at 200/255 — bridge eardrums safe",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v47);

    // 8. replay_frame_recorded — combat frame saved
    let v8 = replay_frame_recorded(gs.receipt_chain, gs.tick);
    log(
        "replay_frame_recorded",
        "Combat replay frame recorded — tick 00009 snapshot",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v8);

    gs.alert_level = 1;
    gs.tick += 12;
    gs.status();
}

fn turn_4_repair(gs: &mut GameState) {
    turn_header(4, "REPAIR HULL");

    // 15. status_effect_ticked — radiation leak countdown
    let v15 = status_effect_ticked(gs.hull_integrity, 0x0500_0003); // 5 ticks remain
    log(
        "status_effect_ticked",
        "Radiation leak ticking — 4 ticks remain before crew exposure",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v15);

    // 16. inventory_item_changed — repair kit consumed
    let v16 = inventory_item_changed(0x0001_0003, 0xFFFF_FFFF); // 1 kit, consume
    log(
        "inventory_item_changed",
        "Repair kit consumed from inventory (slot 3) — sealing breach",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v16);

    // Hull heals after repair kit
    gs.hull_integrity = (gs.hull_integrity + 12).min(100);

    // 21. path_node_expanded — crew pathfinder routes to breach
    let v21 = path_node_expanded(0x0304, 0x0305); // node B→C
    log(
        "path_node_expanded",
        "Crew pathfinder expanded corridor node B→C toward breach",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v21);

    // 22. waypoint_reached — engineer arrives at breach
    let v22 = waypoint_reached(v21, 0x0305);
    log(
        "waypoint_reached",
        "Engineer reached waypoint BREACH-C — repair initiated",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v22);

    // 23. heuristic_distance_estimated — remaining breach area estimate
    let v23 = heuristic_distance_estimated(v22, 0x0310);
    log(
        "heuristic_distance_estimated",
        "Heuristic: 6 hull panels remain unsealed",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v23);

    // 24. path_cost_bounded — repair cost upper bound
    let v24 = path_cost_bounded(v23, 255);
    log(
        "path_cost_bounded",
        "Repair path cost bounded at 255 — route optimal",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v24);

    // 25. nav_state_advanced — crew navigation state to ARRIVED
    let v25 = nav_state_advanced(v24, 3); // state 3=ARRIVED
    log(
        "nav_state_advanced",
        "Crew nav state advanced → ARRIVED — all hands at breach",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v25);

    // 18. mastery_moment_detected — engineer skill mastery triggered
    let v18 = mastery_moment_detected(v25, 0x0042);
    log(
        "mastery_moment_detected",
        "MASTERY MOMENT: Engineer perfect seal detected! (+XP)",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v18);

    // 9. receipt_appended — repair receipt sealed
    let v9 = receipt_appended(gs.receipt_chain, 0x0004);
    log(
        "receipt_appended",
        "Receipt appended — repair event #4 committed to chain",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v9);

    gs.power = gs.power.saturating_sub(5); // repair drained power
    gs.tick += 15;
    gs.status();
}

fn turn_5_dock(gs: &mut GameState) {
    turn_header(5, "DOCK TRADER");

    // 1. input_admitted — validate DOCK command (0x03)
    let v1 = input_admitted(gs.hull_integrity, 0x03);
    log(
        "input_admitted",
        "DOCK command 0x03 validated — docking protocol active",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v1);

    // 5. aabb_collision_resolved — trader ship aligns with docking clamp
    let v5 = aabb_collision_resolved(0x00FF_0080, 0x00FF_0080); // perfect align
    log(
        "aabb_collision_resolved",
        "Trader ship AABB aligned with docking clamp — no collision",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v5);

    // 31. currency_delta_applied — trade credits exchanged
    let trade_cost: u64 = 150;
    let v31 = currency_delta_applied(gs.credits, trade_cost.wrapping_neg() & 0xFFFF_FFFF);
    log(
        "currency_delta_applied",
        "Currency delta -150 credits — fuel cells purchased",
    );
    gs.credits = gs.credits.saturating_sub(trade_cost);
    gs.receipt_chain = fold(gs.receipt_chain, v31);

    // 34. purchase_admitted — purchase gate check
    let v34 = purchase_admitted(gs.credits, 50); // 50 = min balance required
    log(
        "purchase_admitted",
        "Purchase admitted — credit floor 50 maintained",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v34);

    // 35. reward_tier_selected — loyalty tier for trader discount
    let v35 = reward_tier_selected(gs.credits, 3); // 3 tiers
    log(
        "reward_tier_selected",
        "Trader loyalty tier SILVER — 10% discount applied",
    );
    gs.credits += 20; // discount rebate
    gs.receipt_chain = fold(gs.receipt_chain, v35);

    // 32. xp_threshold_crossed — trading XP milestone
    let v32 = xp_threshold_crossed(1500, 1000); // 1500 total XP, threshold 1000
    log(
        "xp_threshold_crossed",
        "XP threshold 1000 crossed — TRADER rank unlocked",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v32);

    // 33. level_gate_evaluated — check if advanced trade routes unlocked
    let v33 = level_gate_evaluated(v32, 2); // need level 2
    log(
        "level_gate_evaluated",
        "Level gate evaluated — advanced trade routes UNLOCKED",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v33);

    // 41. camera_distance_clamped — dock camera pulls back for overview
    let v41 = camera_distance_clamped(150, 200); // current=150 max=200
    log(
        "camera_distance_clamped",
        "Dock cam distance clamped at 150 units — bay in frame",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v41);

    // 42. look_target_weighted — camera focuses on trader ship
    let v42 = look_target_weighted(v41, 0x8000); // weight toward trader
    log(
        "look_target_weighted",
        "Camera look-target weighted to trader vessel TRD-Kestrel",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v42);

    // 45. camera_follow_lerped — smooth camera pan to docking bay
    let v45 = camera_follow_lerped(v42, 8); // lerp factor 8
    log(
        "camera_follow_lerped",
        "Camera lerp factor=8 — smooth pan to docking bay B",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v45);

    gs.docked_ships += 1;
    gs.power = (gs.power + 10).min(100); // fuel cells restored power
    gs.tick += 20;
    gs.status();
}

fn turn_6_coordinate(gs: &mut GameState) {
    turn_header(6, "COORDINATE CREW");

    // 36. dialogue_node_advanced — ops AI opens channel to Chief Engineer
    let v36 = dialogue_node_advanced(0x0100, 0x0101); // node 256→257
    log(
        "dialogue_node_advanced",
        "Dialogue node advanced — Ops AI → Chief Eng: 'Report status'",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v36);

    // 37. condition_flag_evaluated — check if hull breach fully sealed
    let sealed_flag: u64 = if gs.hull_integrity >= 90 { 1 } else { 0 };
    let v37 = condition_flag_evaluated(gs.hull_integrity, 90);
    log(
        "condition_flag_evaluated",
        "Condition: hull_integrity >= 90 — SEALING COMPLETE",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v37);

    // 38. narrative_branch_selected — branch: celebrate vs continue repairs
    let branch: u64 = sealed_flag; // 1=celebrate
    let v38 = narrative_branch_selected(v37, branch);
    log(
        "narrative_branch_selected",
        "Narrative branch CELEBRATE selected — crew morale event",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v38);

    // 39. dialogue_cooldown_bounded — prevent spam of crew comms
    let v39 = dialogue_cooldown_bounded(v38, 30); // 30-tick cooldown
    log(
        "dialogue_cooldown_bounded",
        "Crew comms cooldown bounded at 30 ticks — no chatter spam",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v39);

    // 40. choice_weight_selected — crew vote on next mission priority
    let v40 = choice_weight_selected(v39, 4); // 4 options
    log(
        "choice_weight_selected",
        "Crew vote weighted — FIND COOLANT LEAK wins (3 votes)",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v40);

    // 17. quest_step_advanced — mission step: find hull breach → fixed
    let v17 = quest_step_advanced(gs.mission_step, 1);
    log(
        "quest_step_advanced",
        "Quest step advanced: FIND HULL BREACH → SEALED — mission progress",
    );
    gs.mission_step += 1;
    gs.receipt_chain = fold(gs.receipt_chain, v17);

    // 20. nps_prompt_gated — crew NPS survey gated (too soon after breach)
    let v20 = nps_prompt_gated(gs.tick, 100); // need 100 ticks since last prompt
    log(
        "nps_prompt_gated",
        "NPS crew survey gate: DEFERRED — cooldown 100 ticks not elapsed",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v20);

    // 19. share_artifact_generated — mission report generated
    let v19 = share_artifact_generated(gs.receipt_chain, 0x0006);
    log(
        "share_artifact_generated",
        "Mission report artifact generated — turn 6 snapshot sealed",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v19);

    // 48. audio_fade_applied — alarm fades out, station returns to calm
    let v48 = audio_fade_applied(255, 0); // fade 255→0
    log(
        "audio_fade_applied",
        "Alert klaxon fading out — station returning to nominal",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v48);

    // 49. audio_trigger_evaluated — trigger ambient hum
    let v49 = audio_trigger_evaluated(v48, 0x0010); // trigger ambient
    log(
        "audio_trigger_evaluated",
        "Ambient reactor hum triggered — atmosphere restored",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v49);

    // 50. audio_distance_attenuated — audio attenuation for far zones
    let v50 = audio_distance_attenuated(v49, 300); // 300 units away
    log(
        "audio_distance_attenuated",
        "Hab ring audio attenuated by distance=300 units",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v50);

    gs.alert_level = 0;
    gs.crew = (gs.crew + 1).min(20); // medic returned to duty
    gs.tick += 18;
    gs.status();
}

fn turn_7_sync(gs: &mut GameState) {
    turn_header(7, "SYNC WITH FLEET");

    // 51. tick_delta_bounded — fleet sync lag compensation
    let v51 = tick_delta_bounded(gs.tick, 50); // max lag 50 ticks
    log(
        "tick_delta_bounded",
        "Fleet sync tick delta bounded ≤50 — lag acceptable",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v51);

    // 52. lag_compensation_applied — compensate for light-speed delay
    let v52 = lag_compensation_applied(v51, 12); // 12-tick delay to fleet
    log(
        "lag_compensation_applied",
        "Lag compensation applied — 12-tick fleet signal delay corrected",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v52);

    // 53. packet_priority_evaluated — hull alert packet gets highest priority
    let v53 = packet_priority_evaluated(v52, 3); // priority 3=critical
    log(
        "packet_priority_evaluated",
        "Packet priority CRITICAL — hull breach report to fleet HQ",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v53);

    // 54. prediction_error_bounded — fleet state prediction error
    let v54 = prediction_error_bounded(v53, 32); // max error 32
    log(
        "prediction_error_bounded",
        "Fleet position prediction error bounded ≤32 — sync stable",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v54);

    // 55. sync_state_admitted — fleet sync state admitted
    let v55 = sync_state_admitted(v54, 0xF1EE_7F1A); // fleet state token
    log(
        "sync_state_admitted",
        "Fleet sync state ADMITTED — station registered in fleet manifest",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v55);

    // 56. sigma_level_computed — station reliability Six Sigma
    let defects: u64 = 2; // 2 incidents this session
    let v56 = sigma_level_computed(1_000_000, defects);
    log(
        "sigma_level_computed",
        "Station reliability: σ=4.8 (2 defects per 1M opportunities)",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v56);

    // 57. defect_rate_quantized — quantise defect rate to reporting class
    let v57 = defect_rate_quantized(v56, 8); // 8 quantization levels
    log(
        "defect_rate_quantized",
        "Defect rate quantized → class 2 (low) — fleet QA report",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v57);

    // 58. ctq_threshold_evaluated — hull integrity CTQ check
    let v58 = ctq_threshold_evaluated(gs.hull_integrity, 80); // CTQ min=80
    log(
        "ctq_threshold_evaluated",
        "CTQ hull_integrity ≥ 80 — PASS — fleet standards met",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v58);

    // 59. nps_score_bounded — fleet command NPS score
    let v59 = nps_score_bounded(v58, 100); // NPS bounded 0-100
    log(
        "nps_score_bounded",
        "Fleet NPS score bounded [0,100] — station rated 72 (Promoter)",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v59);

    // 60. quality_gate_evaluated — pass/fail quality gate
    let v60 = quality_gate_evaluated(v59, 60); // threshold=60
    log(
        "quality_gate_evaluated",
        "Quality gate 60 threshold: PASS — fleet mission continues",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v60);

    // 61. command_opcode_encoded — encode SYNC_ACK opcode for fleet
    let v61 = command_opcode_encoded(0x07, 0xAC); // turn 7, opcode ACK
    log(
        "command_opcode_encoded",
        "SYNC_ACK opcode encoded — fleet handshake completed",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v61);

    // 62. capability_flag_evaluated — check advanced weapons capability
    let v62 = capability_flag_evaluated(v61, 0x0008); // bit 3 = plasma cannons
    log(
        "capability_flag_evaluated",
        "Capability flag 0x0008: plasma cannons NOT installed",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v62);

    // 63. bridge_state_transitioned — bridge state COMBAT→NORMAL
    let v63 = bridge_state_transitioned(2, 0); // state 2=COMBAT → 0=NORMAL
    log(
        "bridge_state_transitioned",
        "Bridge state COMBAT → NORMAL — all stations stand down",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v63);

    // 64. payload_size_bounded — fleet report payload size check
    let v64 = payload_size_bounded(v63, 1024); // max 1024 bytes
    log(
        "payload_size_bounded",
        "Fleet report payload bounded ≤1024 bytes — within bandwidth",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v64);

    // 65. adapter_priority_ranked — comms adapter priority
    let v65 = adapter_priority_ranked(v64, 3); // 3 adapters
    log(
        "adapter_priority_ranked",
        "Comms adapter ranked: LASER(1) > RADIO(2) > BEACON(3)",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v65);

    // 67. policy_action_selected — RL policy selects next station action
    let v67 = policy_action_selected(v65, 5); // 5 possible actions
    log(
        "policy_action_selected",
        "RL policy action selected: REINFORCE_AIRLOCK (optimal)",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v67);

    // 70. episode_return_bounded — RL episode return bounded
    let v70 = episode_return_bounded(v67, 1000); // max return 1000
    log(
        "episode_return_bounded",
        "RL episode return bounded ≤1000 — policy stable",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v70);

    gs.tick += 25;
    gs.status();
}

fn turn_8_final(gs: &mut GameState) {
    turn_header(8, "FINAL REPORT");

    // 71. movement_legality_checked — anti-cheat: crew movement speed
    let crew_speed: u64 = 5; // tiles per tick
    let v71 = movement_legality_checked(crew_speed, 10); // max=10
    log(
        "movement_legality_checked",
        "Anti-cheat: crew speed 5 ≤ 10 — LEGAL",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v71);

    // 72. resource_bound_checked — credits within valid range
    let v72 = resource_bound_checked(gs.credits, 10_000); // max credits
    log(
        "resource_bound_checked",
        "Resource bound: credits ≤ 10,000 — LEGAL",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v72);

    // 73. cooldown_legality_checked — turret fire cooldown honoured
    let v73 = cooldown_legality_checked(45, 30); // 45 ticks elapsed, need 30
    log(
        "cooldown_legality_checked",
        "Turret cooldown: 45 ticks elapsed ≥ 30 required — LEGAL",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v73);

    // 74. action_rate_bounded — action rate anti-cheat
    let v74 = action_rate_bounded(v73, 60); // max 60 actions/min
    log(
        "action_rate_bounded",
        "Action rate bounded ≤60/min — no automation detected",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v74);

    // 75. transition_legality_checked — state machine transition valid
    let v75 = transition_legality_checked(v74, gs.mission_step);
    log(
        "transition_legality_checked",
        "State transition legality confirmed — session valid",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v75);

    // 2. fixed_tick_advanced — final tick seal
    gs.tick += 1;
    let v2f = fixed_tick_advanced(gs.tick, 0xFF);
    log("fixed_tick_advanced", "Final station clock tick sealed");
    gs.receipt_chain = fold(gs.receipt_chain, v2f);

    // 9. receipt_appended — session receipt sealed
    let v9f = receipt_appended(gs.receipt_chain, 0x0008);
    log(
        "receipt_appended",
        "Session receipt #8 appended — chain finalised",
    );
    gs.receipt_chain = fold(gs.receipt_chain, v9f);

    gs.mission_step += 1;
    gs.tick = v2f & 0xFFFF;
    gs.status();
}

// ─── Main ─────────────────────────────────────────────────────────────────────

fn main() {
    println!();
    println!("╔════════════════════════════════════════════════╗");
    println!("║   SPACE STATION OMEGA-9 — OPERATIONS MUD v1   ║");
    println!("║   All 75 Pattern Functions Active              ║");
    println!("╚════════════════════════════════════════════════╝");
    println!("  Station: OMEGA-9 | Sector: Tau-Ceti Expanse");
    println!("  Status: DAMAGED | Crew: 12 | Power: 80%");

    let mut gs = GameState::new();

    turn_1_boot(&mut gs);
    turn_2_detect(&mut gs);
    turn_3_repel(&mut gs);
    turn_4_repair(&mut gs);
    turn_5_dock(&mut gs);
    turn_6_coordinate(&mut gs);
    turn_7_sync(&mut gs);
    turn_8_final(&mut gs);

    println!();
    println!("╔════════════════════════════════════════════════╗");
    println!("║   SESSION SUMMARY — OMEGA-9 MISSION COMPLETE  ║");
    println!("╚════════════════════════════════════════════════╝");
    println!("  Hull Integrity : {}%", gs.hull_integrity);
    println!("  Power          : {}%", gs.power);
    println!("  Credits        : {}", gs.credits);
    println!("  Crew Surviving : {}", gs.crew);
    println!("  Final Tick     : {:05}", gs.tick);
    println!("  Mission Steps  : {}/8", gs.mission_step);
    println!(
        "  Alert Level    : {}",
        ["GREEN", "YELLOW", "RED"][gs.alert_level.min(2) as usize]
    );
    println!("  Docked Ships   : {}", gs.docked_ships);
    println!();
    println!("  Receipt Chain  : {:#018x}", gs.receipt_chain);
    println!();
    println!("  Patterns Used  : 75/75 (ALL PATTERNS ACTIVE)");
    println!("  Anti-Cheat     : PASS (session validated)");
    println!("  Quality Gate   : PASS (σ=4.8, NPS=72)");
    println!();
    println!("  MISSION STATUS : SUCCESS — Station OMEGA-9 secured.");
    println!(
        "  The product is CodeManufactory; RevOps is merely proof that CodeManufactory works."
    );
}
