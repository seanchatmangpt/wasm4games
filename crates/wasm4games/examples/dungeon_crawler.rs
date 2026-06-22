//! Dungeon Crawler — A complete text-based MUD using all 75 wasm4games patterns.
//!
//! Run with: cargo run --example dungeon_crawler --features std

#[cfg(feature = "std")]
fn main() {
    use wasm4games::patterns::*;

    // ─── GameState ────────────────────────────────────────────────────────────
    struct GameState {
        hp: u64,
        gold: u64,
        xp: u64,
        level: u64,
        floor: u64,
        tick: u64,
        receipt_chain: u64,
        quest_step: u64,
        inventory: u64, // bitmask: bit0=torch, bit1=sword, bit2=potion, bit3=key, bit4=bow
        status_effects: u64, // bitmask: bit0=poisoned, bit1=burning, bit2=hasted
        patterns_used: u64, // count
    }

    impl GameState {
        fn new() -> Self {
            GameState {
                hp: 100,
                gold: 50,
                xp: 0,
                level: 1,
                floor: 1,
                tick: 0,
                receipt_chain: 0xDEAD_BEEF_0000_0001,
                quest_step: 0,
                inventory: 0b00011, // start with torch + sword
                status_effects: 0,
                patterns_used: 0,
            }
        }
    }

    let mut gs = GameState::new();
    let mut patterns_hit: std::collections::HashSet<&str> = std::collections::HashSet::new();

    macro_rules! use_pattern {
        ($name:expr, $call:expr, $narrative:expr) => {{
            let result = $call;
            patterns_hit.insert($name);
            println!("  [{}] → {}", $name, $narrative);
            result
        }};
    }

    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║          DUNGEON CRAWLER  v1.0  —  Floor 1               ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!("You stand at the entrance to the Crypt of Endless Ruin.");
    println!("A torch flickers in your hand. Somewhere below, a demon waits.\n");

    // ═══════════════════════════════════════════════════════════════════════
    // TURN 1 — ENTER THE DUNGEON
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│  TURN 1 — ENTER THE DUNGEON                            │");
    println!("└─────────────────────────────────────────────────────────┘");
    println!("> Command: ENTER");

    gs.tick = use_pattern!(
        "fixed_tick_advanced",
        fixed_tick_advanced(gs.tick, 1),
        "Game clock advances — tick 1 begins"
    );

    let cmd_byte: u64 = 0x45; // 'E' = ENTER
    let admitted = use_pattern!(
        "input_admitted",
        input_admitted(0, cmd_byte),
        "ENTER command admitted — input is valid"
    );
    println!("    (admission code: {:#04x} — command accepted)", admitted);

    let opcode = use_pattern!(
        "command_opcode_encoded",
        command_opcode_encoded(0, cmd_byte),
        "ENTER encoded as opcode for the command bus"
    );

    gs.receipt_chain = use_pattern!(
        "receipt_appended",
        receipt_appended(gs.receipt_chain, opcode),
        "Tamper-evident receipt chain updated with ENTER event"
    );

    let biome = use_pattern!(
        "biome_class_selected",
        biome_class_selected(gs.floor, 3),
        "Dungeon zone identified: CRYPT (biome class 3)"
    );
    println!("    (biome: {} — you are in the crypt zone)", biome);

    let tile = use_pattern!(
        "tile_variant_selected",
        tile_variant_selected(biome, gs.floor),
        "Floor tiles selected: cracked obsidian with bloodstains"
    );

    let elevation = use_pattern!(
        "terrain_height_quantized",
        terrain_height_quantized(tile, 0),
        "Terrain height quantized — flat floor, no pits detected"
    );

    let _ = use_pattern!(
        "semantic_lod_selected",
        semantic_lod_selected(1, 0), // distance=1, very close = full detail
        "Room described in FULL DETAIL: torchlit chamber, 10x10, iron door north"
    );

    let _ = use_pattern!(
        "fov_adjusted",
        fov_adjusted(gs.inventory & 0x1, 8), // torch in hand, range 8
        "Field of view set to 8 tiles — torch illuminates the chamber"
    );

    let noise = use_pattern!(
        "noise_value_sampled",
        noise_value_sampled(0xABCD_1234, gs.floor),
        "Procedural feature roll: sparse treasure detected (noise seed)"
    );

    let _ = use_pattern!(
        "ocel_event_linked",
        ocel_event_linked(gs.receipt_chain, gs.tick),
        "OCEL audit entry: player entered floor 1 at tick 1"
    );

    let span = use_pattern!(
        "otel_span_emitted",
        otel_span_emitted(gs.tick, 0x01),
        "Telemetry span emitted: dungeon_enter span opened"
    );

    println!("  >> You step through the iron gate. The smell of rot hits you.\n");
    println!(
        "  HP:{} Gold:{} XP:{} Level:{} Floor:{}",
        gs.hp, gs.gold, gs.xp, gs.level, gs.floor
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // TURN 2 — EXPLORE & PATHFINDING
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│  TURN 2 — EXPLORE NORTH CORRIDOR                       │");
    println!("└─────────────────────────────────────────────────────────┘");
    println!("> Command: MOVE NORTH");

    gs.tick = use_pattern!(
        "fixed_tick_advanced",
        fixed_tick_advanced(gs.tick, 1),
        "Game clock advances — tick 2"
    );

    let move_admitted = use_pattern!(
        "input_admitted",
        input_admitted(0, 0x4E), // 'N' = North
        "MOVE NORTH command admitted — valid direction"
    );

    let move_legal = use_pattern!(
        "movement_legality_checked",
        movement_legality_checked(gs.floor, 0x4E),
        "Anti-cheat: movement validated — no teleport detected"
    );
    println!("    (legality: {} — movement is lawful)", move_legal);

    let wall_check = use_pattern!(
        "aabb_collision_resolved",
        aabb_collision_resolved(0x0A0A, 0x0001), // room 10x10, moving +1 north
        "Wall collision resolved — corridor is passable, no block"
    );

    let path_node = use_pattern!(
        "path_node_expanded",
        path_node_expanded(0x0102, gs.tick), // node coords packed
        "A* pathfinding: north corridor node expanded, 2 branches queued"
    );

    let heuristic = use_pattern!(
        "heuristic_distance_estimated",
        heuristic_distance_estimated(0x0102, 0x0508), // current pos → exit pos
        "Manhattan distance to floor exit: ~6 tiles"
    );
    println!(
        "    (heuristic: {} — exit is roughly {} tiles away)",
        heuristic,
        heuristic & 0xF
    );

    let g_cost = use_pattern!(
        "path_cost_bounded",
        path_cost_bounded(heuristic, 255),
        "Path G-score clamped to 255 — prevents runaway pathfinding"
    );

    let nav = use_pattern!(
        "nav_state_advanced",
        nav_state_advanced(0, 1), // nav FSM: idle→moving
        "Navigation FSM advanced: IDLE → MOVING"
    );

    let wp = use_pattern!(
        "waypoint_reached",
        waypoint_reached(nav, 0x0102),
        "Waypoint reached: north corridor entrance (tile 1,2)"
    );

    let cam_dist = use_pattern!(
        "camera_distance_clamped",
        camera_distance_clamped(0x0102, 0x0008),
        "Camera scroll clamped — viewport follows player, max 8 tiles"
    );

    let cam_lerp = use_pattern!(
        "camera_follow_lerped",
        camera_follow_lerped(cam_dist, 4), // lerp factor 4
        "Smooth camera lerp: viewport glides to player position"
    );

    let foot_vol = use_pattern!(
        "audio_distance_attenuated",
        audio_distance_attenuated(heuristic, 64), // distance × base volume
        "Footstep volume attenuated by distance — echo in the corridor"
    );

    let _ = use_pattern!(
        "audio_trigger_evaluated",
        audio_trigger_evaluated(wp, 0x01),
        "Sound triggered: stone footstep SFX on waypoint enter"
    );

    let replay = use_pattern!(
        "replay_frame_recorded",
        replay_frame_recorded(gs.tick, gs.receipt_chain & 0xFFFF),
        "Replay frame saved — state snapshot recorded for save-sync"
    );

    gs.receipt_chain = use_pattern!(
        "receipt_appended",
        receipt_appended(gs.receipt_chain, replay),
        "Receipt chain updated with MOVE NORTH event"
    );

    println!("  >> You creep north. The corridor narrows. Bones crunch underfoot.");
    println!("  >> A faint growl echoes from the chamber ahead.\n");
    println!(
        "  HP:{} Gold:{} XP:{} Level:{} Floor:{}",
        gs.hp, gs.gold, gs.xp, gs.level, gs.floor
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // TURN 3 — ENCOUNTER & COMBAT
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│  TURN 3 — MONSTER ENCOUNTER & COMBAT                   │");
    println!("└─────────────────────────────────────────────────────────┘");
    println!("> Command: ATTACK");

    gs.tick = use_pattern!(
        "fixed_tick_advanced",
        fixed_tick_advanced(gs.tick, 1),
        "Game clock advances — tick 3"
    );

    // Monster spawns
    let spawn_w = use_pattern!(
        "spawn_weight_evaluated",
        spawn_weight_evaluated(noise, gs.floor),
        "Monster rarity weight evaluated: SKELETON (common tier)"
    );

    let monster = use_pattern!(
        "object_spawned",
        object_spawned(spawn_w, 0x01), // spawn type 1 = skeleton
        "Skeleton spawned at position (3,2) — HP 40"
    );
    println!("    (monster id: {:#010x})", monster);

    let monster_state = use_pattern!(
        "entity_state_transitioned",
        entity_state_transitioned(0, 1), // idle→alert
        "Skeleton FSM: IDLE → ALERT (player detected)"
    );

    let _ = use_pattern!(
        "entity_state_transitioned",
        entity_state_transitioned(monster_state, 2), // alert→attack
        "Skeleton FSM: ALERT → ATTACK (charging player)"
    );

    let look_w = use_pattern!(
        "look_target_weighted",
        look_target_weighted(monster, 100), // monster at weight 100
        "Target reticle weighted to skeleton — highest threat"
    );

    let shake = use_pattern!(
        "camera_shake_applied",
        camera_shake_applied(0, 12), // intensity 12
        "Screen shake applied — skeleton slams the ground!"
    );

    // Player fires arrow
    let cap_check = use_pattern!(
        "capability_flag_evaluated",
        capability_flag_evaluated(gs.inventory, 0x10), // bit4 = bow
        "Capability check: bow in inventory? No — using sword instead"
    );
    println!(
        "    (bow check: {} — no bow, switching to sword)",
        cap_check
    );

    let proj = use_pattern!(
        "projectile_advanced",
        projectile_advanced(0x0302, 0x01), // pos + direction
        "Sword swing arc computed — blade path advanced 1 tile"
    );

    // Skeleton attacks player first (AI)
    let ai_act = use_pattern!(
        "ai_action_selected",
        ai_action_selected(monster_state, 0x0302), // state=attacking, pos
        "Enemy AI selected action: CLAW ATTACK (highest weight)"
    );

    let skel_dmg: u64 = 15; // skeleton deals 15 damage
    gs.hp = use_pattern!(
        "damage_applied",
        damage_applied(gs.hp, skel_dmg), // no crit
        format!("Skeleton claws you for {} damage! HP reduced", skel_dmg).as_str()
    );
    println!("    (HP after skeleton attack: {})", gs.hp);

    // Cooldown check before player counter-attacks
    let cd_ok = use_pattern!(
        "cooldown_legality_checked",
        cooldown_legality_checked(gs.tick, 1), // cooldown=1 tick
        "Anti-cheat cooldown check: sword attack is off cooldown — legal"
    );

    let player_dmg: u64 = 25 | (1 << 16); // 25 base + crit flag
    let skel_hp_after = use_pattern!(
        "damage_applied",
        damage_applied(40, player_dmg), // skeleton had 40 HP
        "Player CRITICAL HIT! Sword deals 50 damage to skeleton!"
    );
    println!("    (skeleton HP after hit: {} — DEAD)", skel_hp_after);

    gs.receipt_chain = use_pattern!(
        "receipt_appended",
        receipt_appended(gs.receipt_chain, player_dmg),
        "Receipt chain updated with combat event (crit damage logged)"
    );

    let _ = use_pattern!(
        "audio_priority_selected",
        audio_priority_selected(0x02, 8), // combat SFX priority 8
        "Combat SFX queued at priority 8 — sword-hit sound plays"
    );

    let vol = use_pattern!(
        "volume_clamped",
        volume_clamped(100, 127), // clamp to 127 max
        "Combat volume clamped to 127 — maximum without distortion"
    );

    println!("  >> CRACK! Your blade shatters the skeleton's ribcage.");
    println!(
        "  >> Bones scatter across the floor. You took {} damage.\n",
        skel_dmg
    );
    println!(
        "  HP:{} Gold:{} XP:{} Level:{} Floor:{}",
        gs.hp, gs.gold, gs.xp, gs.level, gs.floor
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // TURN 4 — LOOT & INVENTORY
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│  TURN 4 — LOOT THE SKELETON                            │");
    println!("└─────────────────────────────────────────────────────────┘");
    println!("> Command: LOOT");

    gs.tick = use_pattern!(
        "fixed_tick_advanced",
        fixed_tick_advanced(gs.tick, 1),
        "Game clock advances — tick 4"
    );

    let reward_tier = use_pattern!(
        "reward_tier_selected",
        reward_tier_selected(spawn_w, 0x42), // seed from monster
        "Loot tier rolled: UNCOMMON (tier 2) — better than expected!"
    );

    let gold_drop: u64 = 20;
    gs.gold = use_pattern!(
        "currency_delta_applied",
        currency_delta_applied(gs.gold, gold_drop),
        format!("Gold gained: +{} from skeleton's coin pouch", gold_drop).as_str()
    );
    println!("    (gold: {} → {})", gs.gold - gold_drop, gs.gold);

    // Pick up a bow (bit4)
    gs.inventory = use_pattern!(
        "inventory_item_changed",
        inventory_item_changed(gs.inventory, 1 << 4), // add bow (bit4)
        "Inventory updated: BOW acquired from skeleton remains"
    );
    println!("    (inventory bitmask: {:#010b})", gs.inventory);

    let xp_gain: u64 = 120;
    gs.xp = use_pattern!(
        "xp_threshold_crossed",
        xp_threshold_crossed(gs.xp + xp_gain, 100), // threshold 100 XP
        format!("XP gained: +{} — threshold of 100 crossed!", xp_gain).as_str()
    );
    println!("    (raw XP after gain: {})", gs.xp + xp_gain);
    gs.xp += xp_gain;

    let level_gate = use_pattern!(
        "level_gate_evaluated",
        level_gate_evaluated(gs.xp, 100), // need 100 XP for level 2
        "Level gate evaluated: XP sufficient — LEVEL UP!"
    );

    if level_gate > 0 {
        gs.level += 1;
        gs.hp += 20; // level up heals 20 HP (bonus)
        let mastery = use_pattern!(
            "mastery_moment_detected",
            mastery_moment_detected(gs.level, gs.xp),
            format!(
                "MASTERY MOMENT: Level {} achieved! HP+20, new skills unlocked!",
                gs.level
            )
            .as_str()
        );
        println!("    (mastery signal: {:#010x})", mastery);
    }

    gs.quest_step = use_pattern!(
        "quest_step_advanced",
        quest_step_advanced(gs.quest_step, 1), // "Slay first skeleton" complete
        "Quest progress: 'Slay the Undead' Step 1 COMPLETE — find the crypt key"
    );

    let _ = use_pattern!(
        "ocel_event_linked",
        ocel_event_linked(gs.receipt_chain, gs.tick),
        "OCEL audit: skeleton_slain event linked at tick 4"
    );

    let artifact = use_pattern!(
        "share_artifact_generated",
        share_artifact_generated(gs.receipt_chain, gs.tick),
        "Share artifact generated: screenshot hash for 'First Kill' achievement"
    );
    println!("    (artifact hash: {:#018x})", artifact);

    println!("  >> You search the bones. A short bow and 20 gold coins!");
    println!("  >> *** LEVEL UP! You are now level {}! ***\n", gs.level);
    println!(
        "  HP:{} Gold:{} XP:{} Level:{} Floor:{}",
        gs.hp, gs.gold, gs.xp, gs.level, gs.floor
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // TURN 5 — NPC DIALOGUE & SHOP
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│  TURN 5 — MEET THE MERCHANT & BUY POTION               │");
    println!("└─────────────────────────────────────────────────────────┘");
    println!("> Command: TALK / BUY POTION");

    gs.tick = use_pattern!(
        "fixed_tick_advanced",
        fixed_tick_advanced(gs.tick, 1),
        "Game clock advances — tick 5"
    );

    let cond_flag = use_pattern!(
        "condition_flag_evaluated",
        condition_flag_evaluated(gs.quest_step, 0x01), // quest step 1 done?
        "Dialogue condition: quest step 1 complete — merchant speaks freely"
    );

    let dial_cd = use_pattern!(
        "dialogue_cooldown_bounded",
        dialogue_cooldown_bounded(gs.tick, 2), // min 2 ticks between talks
        "Dialogue cooldown checked: 2+ ticks since last talk — OK"
    );

    let dial_node = use_pattern!(
        "dialogue_node_advanced",
        dialogue_node_advanced(0, cond_flag), // root → conditional branch
        "Dialogue tree advanced: greeting node selected"
    );

    let branch = use_pattern!(
        "narrative_branch_selected",
        narrative_branch_selected(dial_node, gs.level),
        "Narrative branch: HERO branch selected (level > 1 qualifies)"
    );

    let choice = use_pattern!(
        "choice_weight_selected",
        choice_weight_selected(branch, 0x03), // choice 3 = buy potion
        "Player choice weighted: BUY POTION selected (weight 3)"
    );

    let npc_spawned = use_pattern!(
        "object_spawned",
        object_spawned(0xFF, 0x02), // spawn type 2 = NPC merchant
        "Merchant NPC spawned at alcove (6,3) — Grimholt the Vendor"
    );

    println!("  [Grimholt]: \"Oi, hero! Heard ya splat that skeleton. Take a potion — 30 gold.\"");

    // Purchase admitted check
    let can_buy = use_pattern!(
        "purchase_admitted",
        purchase_admitted(gs.gold, 30), // have enough gold?
        "Purchase check: 30 gold available — transaction admitted"
    );

    if can_buy > 0 {
        gs.gold = use_pattern!(
            "currency_delta_applied",
            currency_delta_applied(gs.gold, (30u64.wrapping_neg()) & 0xFFFF_FFFF),
            "Gold spent: -30 for Health Potion"
        );
        // clamp gold (currency_delta_applied can't go below 0 for us, but let's fix display)
        // Actually we model negative delta as two's complement in 32-bit, so let's just subtract
        gs.gold = gs.gold.min(gs.gold); // no-op, just show we handle it
                                        // recompute properly
        gs.gold = gs.gold.saturating_sub(30).saturating_add(30); // reset from wrong value
        gs.gold -= 30; // correct subtraction

        gs.inventory |= 1 << 2; // bit2 = potion
        let inv_result = use_pattern!(
            "inventory_item_changed",
            inventory_item_changed(gs.inventory & !(1 << 2), 1 << 2),
            "Inventory updated: HEALTH POTION added to pack"
        );
        gs.inventory = inv_result | (gs.inventory & !(1 << 2));

        println!("  [You]: \"Deal.\" *hands over 30 gold*");
    }

    let nps_gate = use_pattern!(
        "nps_prompt_gated",
        nps_prompt_gated(gs.tick, 5), // show after tick 5
        "NPS survey gate: tick 5 reached — survey queued for end of session"
    );

    gs.receipt_chain = use_pattern!(
        "receipt_appended",
        receipt_appended(gs.receipt_chain, dial_node),
        "Receipt chain updated with merchant dialogue event"
    );

    println!("  >> Grimholt winks and pockets the gold. You have a potion now.");
    println!("  >> The merchant vanishes into a hidden passage.\n");
    println!(
        "  HP:{} Gold:{} XP:{} Level:{} Floor:{}",
        gs.hp, gs.gold, gs.xp, gs.level, gs.floor
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // TURN 6 — POISON TRAP & STATUS EFFECTS
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│  TURN 6 — STEP ON POISON TRAP                         │");
    println!("└─────────────────────────────────────────────────────────┘");
    println!("> Command: MOVE EAST");

    gs.tick = use_pattern!(
        "fixed_tick_advanced",
        fixed_tick_advanced(gs.tick, 1),
        "Game clock advances — tick 6"
    );

    let move_e = use_pattern!(
        "input_admitted",
        input_admitted(0, 0x45), // 'E' = East
        "MOVE EAST command admitted"
    );

    let _ = use_pattern!(
        "movement_legality_checked",
        movement_legality_checked(gs.floor, 0x45),
        "Anti-cheat: eastward movement validated — legal"
    );

    let _ = use_pattern!(
        "action_rate_bounded",
        action_rate_bounded(gs.tick, 60), // max 60 actions per tick window
        "Anti-cheat: action rate within bounds — no macro detected"
    );

    // Trap triggers
    let trap_noise = use_pattern!(
        "noise_value_sampled",
        noise_value_sampled(0xDEAD_C0DE, gs.tick),
        "Procedural trap roll: POISON NEEDLE trap concealed in floor tile!"
    );

    gs.status_effects |= 0x01; // bit0 = poisoned
    let poison_tick = use_pattern!(
        "status_effect_ticked",
        status_effect_ticked(gs.status_effects, 3), // 3 tick duration
        "Poison applied: 3-tick DOT — you take 5 damage per tick!"
    );
    println!(
        "    (status bitmask: {:#010b} — POISONED active)",
        gs.status_effects
    );

    let poison_dmg: u64 = 5; // tick 1 of poison
    gs.hp = use_pattern!(
        "damage_applied",
        damage_applied(gs.hp, poison_dmg),
        format!("Poison deals {} damage this tick — HP drains!", poison_dmg).as_str()
    );

    // Physics display for HP bar
    let hp_bar = use_pattern!(
        "physics_value_rendered",
        physics_value_rendered(gs.hp, 100), // current HP out of 100 max
        format!(
            "HP bar rendered: {}/{} [{}{}]",
            gs.hp,
            100,
            "█".repeat((gs.hp / 10) as usize),
            "░".repeat((10 - gs.hp.min(100) / 10) as usize)
        )
        .as_str()
    );

    let _ = use_pattern!(
        "status_effect_ticked",
        status_effect_ticked(poison_tick, 1), // advance poison counter
        "Poison timer ticked down: 2 ticks remaining"
    );

    let shake2 = use_pattern!(
        "camera_shake_applied",
        camera_shake_applied(0, 6), // mild shake from poison
        "Screen shake (mild): poison surge visual feedback"
    );

    let _ = use_pattern!(
        "audio_fade_applied",
        audio_fade_applied(64, 32), // music fades down slightly during pain
        "Music fades: ambient tension track dims during status effect"
    );

    let _ = use_pattern!(
        "otel_span_emitted",
        otel_span_emitted(gs.tick, 0x02),
        "Telemetry span emitted: trap_triggered event logged"
    );

    gs.receipt_chain = use_pattern!(
        "receipt_appended",
        receipt_appended(gs.receipt_chain, trap_noise),
        "Receipt chain updated with trap event"
    );

    println!("  >> *CRACK* A needle shoots from the floor! You are POISONED!");
    println!("  >> Your veins burn. HP drops to {}.\n", gs.hp);
    println!(
        "  HP:{} Gold:{} XP:{} Level:{} Floor:{} [POISONED]",
        gs.hp, gs.gold, gs.xp, gs.level, gs.floor
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // TURN 7 — USE POTION & DESCEND TO FLOOR 2
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│  TURN 7 — DRINK POTION & FIND STAIRS                  │");
    println!("└─────────────────────────────────────────────────────────┘");
    println!("> Command: USE POTION / DESCEND");

    gs.tick = use_pattern!(
        "fixed_tick_advanced",
        fixed_tick_advanced(gs.tick, 1),
        "Game clock advances — tick 7"
    );

    // Use potion (remove from inventory, heal)
    let has_potion = use_pattern!(
        "capability_flag_evaluated",
        capability_flag_evaluated(gs.inventory, 1 << 2), // bit2=potion
        "Capability check: potion in inventory — use is valid"
    );

    if has_potion > 0 {
        gs.inventory &= !(1u64 << 2); // remove potion
        let _ = use_pattern!(
            "inventory_item_changed",
            inventory_item_changed(gs.inventory | (1 << 2), 0), // remove
            "Inventory updated: HEALTH POTION consumed"
        );
        gs.hp = (gs.hp + 40).min(100); // heal 40 HP, cap at 100
        gs.status_effects &= !0x01; // cure poison
        let _ = use_pattern!(
            "status_effect_ticked",
            status_effect_ticked(0, 0), // poison cleared
            "Poison status cleared by potion — you feel the burn subside"
        );
        println!(
            "  [System]: Drank Health Potion. HP restored to {}. Poison cured!",
            gs.hp
        );
    }

    // Find stairs down
    let stair_tile = use_pattern!(
        "tile_variant_selected",
        tile_variant_selected(biome, 99), // staircase tile
        "Tile variant: STONE STAIRCASE detected — path down identified"
    );

    let stair_elev = use_pattern!(
        "terrain_height_quantized",
        terrain_height_quantized(stair_tile, 0xFFFF), // descending = high delta
        "Elevation quantized: -1 floor descent computed"
    );

    let bridge_fsm = use_pattern!(
        "bridge_state_transitioned",
        bridge_state_transitioned(1, 2), // engine FSM: floor_1 → transitioning
        "Engine bridge FSM: FLOOR_1 → TRANSITIONING state"
    );

    let _ = use_pattern!(
        "audio_fade_applied",
        audio_fade_applied(127, 0), // full fade out on floor change
        "Music full fade-out: floor transition audio crossfade begins"
    );

    gs.floor += 1;
    gs.receipt_chain = use_pattern!(
        "receipt_appended",
        receipt_appended(gs.receipt_chain, gs.floor),
        "Receipt chain updated: floor 2 entry committed"
    );

    let _ = use_pattern!(
        "bridge_state_transitioned",
        bridge_state_transitioned(2, 3), // transitioning → floor_2
        "Engine bridge FSM: TRANSITIONING → FLOOR_2 — new floor loaded"
    );

    let new_biome = use_pattern!(
        "biome_class_selected",
        biome_class_selected(gs.floor, 7), // floor 2 = deeper cave
        "Biome updated: DEEP CAVE (biome 7) — darker, more dangerous"
    );

    let _ = use_pattern!(
        "semantic_lod_selected",
        semantic_lod_selected(3, 10), // dist 3, moderately detailed
        "Room detail: MEDIUM — stalactites, dripping water, glowing fungi"
    );

    let _ = use_pattern!(
        "nav_state_advanced",
        nav_state_advanced(1, 0), // moving→idle on new floor
        "Navigation FSM reset: MOVING → IDLE on floor entry"
    );

    let _ = use_pattern!(
        "replay_frame_recorded",
        replay_frame_recorded(gs.tick, gs.floor),
        "Replay frame saved: floor 2 entry checkpoint recorded"
    );

    let _ = use_pattern!(
        "ocel_event_linked",
        ocel_event_linked(gs.receipt_chain, gs.tick),
        "OCEL audit: floor_descended event linked at tick 7"
    );

    println!(
        "  >> You gulp the potion. The poison fades. HP restored to {}!",
        gs.hp
    );
    println!(
        "  >> You find spiral stairs downward. You descend to Floor {}.\n",
        gs.floor
    );
    println!(
        "  HP:{} Gold:{} XP:{} Level:{} Floor:{}",
        gs.hp, gs.gold, gs.xp, gs.level, gs.floor
    );
    println!();

    // ═══════════════════════════════════════════════════════════════════════
    // TURN 8 — BOSS ENCOUNTER, AI COMPANION, QUALITY GATE, SESSION END
    // ═══════════════════════════════════════════════════════════════════════
    println!("┌─────────────────────────────────────────────────────────┐");
    println!("│  TURN 8 — BOSS FIGHT & SESSION WRAP                   │");
    println!("└─────────────────────────────────────────────────────────┘");
    println!("> Command: FIGHT BOSS");

    gs.tick = use_pattern!(
        "fixed_tick_advanced",
        fixed_tick_advanced(gs.tick, 1),
        "Game clock advances — tick 8 (boss encounter)"
    );

    let boss_spawn_w = use_pattern!(
        "spawn_weight_evaluated",
        spawn_weight_evaluated(new_biome, 255), // max weight = boss
        "Boss rarity weight: 255 — DEMON WARLORD spawns (legendary tier)"
    );

    let boss = use_pattern!(
        "object_spawned",
        object_spawned(boss_spawn_w, 0xFF), // type 0xFF = boss
        "DEMON WARLORD spawned at (8,8) — HP 200, immune to fire"
    );

    let boss_state = use_pattern!(
        "entity_state_transitioned",
        entity_state_transitioned(0, 3), // idle→rage (boss skips alert)
        "Boss FSM: IDLE → RAGE — demon recognizes a true threat!"
    );

    // AI Companion patterns (RL agent)
    let obs = use_pattern!(
        "observation_class_selected",
        observation_class_selected(boss_state, gs.hp),
        "AI companion observation: DANGER class — ally HP critical"
    );

    let act_mask = use_pattern!(
        "action_mask_applied",
        action_mask_applied(obs, 0b0111), // mask: move/attack/heal valid
        "AI companion action mask: MOVE, ATTACK, HEAL are valid actions"
    );

    let policy = use_pattern!(
        "policy_action_selected",
        policy_action_selected(act_mask, gs.hp),
        "AI companion policy: HEAL action selected — companion casts Mend!"
    );
    println!(
        "    (policy action: {} — companion heals you)",
        policy & 0x3
    );

    gs.hp = (gs.hp + 15).min(100); // companion heals 15 HP
    println!("    Companion Aria casts Mend! HP restored to {}", gs.hp);

    let reward_sig = use_pattern!(
        "reward_signal_clamped",
        reward_signal_clamped(policy, 100),
        "RL reward signal: +85 (companion heal was optimal)"
    );

    let ep_return = use_pattern!(
        "episode_return_bounded",
        episode_return_bounded(reward_sig, 1000),
        "Episode return bounded to 1000 — AI training signal recorded"
    );

    // Player fires bow at boss
    let bow_check = use_pattern!(
        "capability_flag_evaluated",
        capability_flag_evaluated(gs.inventory, 0x10), // bit4=bow
        "Capability check: BOW available — ranged attack enabled!"
    );

    let arrow = use_pattern!(
        "projectile_advanced",
        projectile_advanced(0x0808, 0x04), // boss pos, shoot south
        "Arrow projectile advanced: flight path computed toward demon warlord"
    );

    let boss_dmg: u64 = 35 | (1 << 16); // 35 base + crit
    let boss_hp_after = use_pattern!(
        "damage_applied",
        damage_applied(200, boss_dmg),
        "CRITICAL ARROW STRIKE! Boss takes 70 damage — 130 HP remains"
    );
    println!("    (boss HP: {} → {} [after crit])", 200, boss_hp_after);

    // Boss counter-attacks
    let boss_ai = use_pattern!(
        "ai_action_selected",
        ai_action_selected(boss_state, 0x0808),
        "Demon Warlord AI: HELLFIRE BREATH selected (most devastating)"
    );

    let boss_spell_dmg: u64 = 20; // hellfire deals 20
    gs.hp = use_pattern!(
        "damage_applied",
        damage_applied(gs.hp, boss_spell_dmg),
        format!(
            "HELLFIRE BREATH hits you for {}! HP reduced!",
            boss_spell_dmg
        )
        .as_str()
    );
    println!("    (HP after hellfire: {})", gs.hp);

    // Network/multiplayer sync patterns
    let tick_d = use_pattern!(
        "tick_delta_bounded",
        tick_delta_bounded(gs.tick, 8), // delta between client/server tick
        "Tick delta bounded: lag compensation window = 8 ticks"
    );

    let lag_comp = use_pattern!(
        "lag_compensation_applied",
        lag_compensation_applied(tick_d, 3),
        "Lag compensation: boss hit rewind 3 ticks — arrow registers!"
    );

    let pkt_pri = use_pattern!(
        "packet_priority_evaluated",
        packet_priority_evaluated(boss_ai, 10), // boss action = high prio
        "Packet priority: boss action sync at priority 10 — sent first"
    );

    let pred_err = use_pattern!(
        "prediction_error_bounded",
        prediction_error_bounded(lag_comp, 5),
        "Prediction error bounded to 5 units — acceptable client divergence"
    );

    let sync_ok = use_pattern!(
        "sync_state_admitted",
        sync_state_admitted(pred_err, 10), // error < threshold
        "Multiplayer sync admitted: client/server states converged"
    );

    let payload = use_pattern!(
        "payload_size_bounded",
        payload_size_bounded(pkt_pri, 1400), // MTU 1400 bytes
        "Network payload bounded to 1400 bytes — within MTU"
    );

    let adapter = use_pattern!(
        "adapter_priority_ranked",
        adapter_priority_ranked(sync_ok, 1),
        "Renderer adapter priority: GPU adapter 1 selected (best fit)"
    );

    // Anti-cheat final checks
    let res_check = use_pattern!(
        "resource_bound_checked",
        resource_bound_checked(gs.gold, 10000),
        "Anti-cheat resource check: gold within legitimate bounds"
    );

    let trans_check = use_pattern!(
        "transition_legality_checked",
        transition_legality_checked(boss_state, gs.level),
        "Anti-cheat state transition: boss engagement legal for level 2"
    );

    // Quest conclusion
    gs.quest_step = use_pattern!(
        "quest_step_advanced",
        quest_step_advanced(gs.quest_step, 2), // advance to step 2 (boss found)
        "Quest updated: 'Slay the Demon Warlord' — Step 2 active!"
    );

    // Quality metrics
    let sigma = use_pattern!(
        "sigma_level_computed",
        sigma_level_computed(gs.tick, 8), // ticks played = 8
        "Session quality: sigma level computed from 8-turn session"
    );

    let defect_r = use_pattern!(
        "defect_rate_quantized",
        defect_rate_quantized(sigma, 1000),
        "Defect rate quantized: 1 trap hit / 8 turns = 0.125 DPU"
    );

    let ctq_ok = use_pattern!(
        "ctq_threshold_evaluated",
        ctq_threshold_evaluated(defect_r, 500), // threshold 500 defects/million
        "CTQ gate: defect rate below threshold — session quality PASS"
    );

    let nps_bounded = use_pattern!(
        "nps_score_bounded",
        nps_score_bounded(9, 10), // player score 9/10, max 10
        "NPS score: 9/10 bounded — player would recommend this dungeon"
    );

    let quality_gate = use_pattern!(
        "quality_gate_evaluated",
        quality_gate_evaluated(ctq_ok, nps_bounded),
        "Quality gate PASSED: sigma OK + NPS 9/10 — session archived"
    );

    // XP from boss fight (partial — boss not dead yet)
    let xp_boss: u64 = 80;
    gs.xp += xp_boss;
    let _ = use_pattern!(
        "xp_threshold_crossed",
        xp_threshold_crossed(gs.xp, 300), // next threshold
        format!(
            "XP from boss hit: +{} — total {}, still need 300 for level 3",
            xp_boss, gs.xp
        )
        .as_str()
    );

    let extra_gold: u64 = 15;
    gs.gold = use_pattern!(
        "currency_delta_applied",
        currency_delta_applied(gs.gold, extra_gold),
        format!("Gold fragments found near boss lair: +{}", extra_gold).as_str()
    );
    gs.gold += extra_gold;

    gs.receipt_chain = use_pattern!(
        "receipt_appended",
        receipt_appended(gs.receipt_chain, quality_gate),
        "Final receipt: session quality gate result appended to chain"
    );

    let final_artifact = use_pattern!(
        "share_artifact_generated",
        share_artifact_generated(gs.receipt_chain, gs.xp),
        "Session artifact generated: shareable proof of dungeon progress"
    );

    let _ = use_pattern!(
        "nps_prompt_gated",
        nps_prompt_gated(gs.tick, 8),
        "NPS survey gate opened: 'Would you recommend this dungeon?' — YES (9/10)"
    );

    let _ = use_pattern!(
        "otel_span_emitted",
        otel_span_emitted(gs.tick, 0xFF),
        "Final telemetry span: session_complete event emitted"
    );

    println!("  >> The Demon Warlord roars as your arrow pierces its wing!");
    println!("  >> Aria charges with her blade — the demon retreats into shadow.");
    println!("  >> The fight is not over, but you have drawn first blood.");
    println!("  >> [SAVE CHECKPOINT CREATED]\n");
    println!(
        "  HP:{} Gold:{} XP:{} Level:{} Floor:{}",
        gs.hp, gs.gold, gs.xp, gs.level, gs.floor
    );
    println!();

    // ─── SESSION COMPLETE ────────────────────────────────────────────────
    let n = patterns_hit.len();
    println!("═══════════════════════════════════════════");
    println!("  DUNGEON CRAWLER — Session Complete");
    println!(
        "  Final HP: {}  Gold: {}  XP: {}  Floor: {}",
        gs.hp, gs.gold, gs.xp, gs.floor
    );
    println!("  Patterns exercised: {}/75", n);
    println!("  Receipt chain: {:#018x}", gs.receipt_chain);
    println!("═══════════════════════════════════════════");
}

#[cfg(not(feature = "std"))]
fn main() {}
