//! Combat simulation example — patterns 1-14 (Core Sim & Combat family).
//!
//! Run with: `cargo run --example combat_sim --features std`
//! (from the crates/wasm4games directory, or use -p wasm4games)

use wasm4games::patterns::{
    aabb_collision_resolved, ai_action_selected, damage_applied, entity_state_transitioned,
    fixed_tick_advanced, input_admitted, object_spawned, ocel_event_linked, otel_span_emitted,
    physics_value_rendered, projectile_advanced, receipt_appended, replay_frame_recorded,
    semantic_lod_selected,
};

fn main() {
    let mut state: u64 = 0x0000_0064; // 100 HP
    let input: u64 = 0x0000_0041;

    // 1. Gate input
    state = input_admitted(state, input);
    println!("01 input_admitted:             {:#018x}", state);

    // 2. Advance fixed tick
    state = fixed_tick_advanced(state, input);
    println!("02 fixed_tick_advanced:        {:#018x}", state);

    // 3. Transition entity state
    state = entity_state_transitioned(state, input);
    println!("03 entity_state_transitioned:  {:#018x}", state);

    // 4. Spawn object
    state = object_spawned(state, input);
    println!("04 object_spawned:             {:#018x}", state);

    // 5. Resolve collision
    state = aabb_collision_resolved(state, input);
    println!("05 aabb_collision_resolved:    {:#018x}", state);

    // 6. Link OCEL event
    state = ocel_event_linked(state, input);
    println!("06 ocel_event_linked:          {:#018x}", state);

    // 7. Emit OTEL span
    state = otel_span_emitted(state, input);
    println!("07 otel_span_emitted:          {:#018x}", state);

    // 8. Record replay frame
    state = replay_frame_recorded(state, input);
    println!("08 replay_frame_recorded:      {:#018x}", state);

    // 9. Append receipt
    state = receipt_appended(state, input);
    println!("09 receipt_appended:           {:#018x}", state);

    // 10. Render physics value
    state = physics_value_rendered(state, input);
    println!("10 physics_value_rendered:     {:#018x}", state);

    // 11. Select LOD
    state = semantic_lod_selected(state, input);
    println!("11 semantic_lod_selected:      {:#018x}", state);

    // 12. Advance projectile
    state = projectile_advanced(state, input);
    println!("12 projectile_advanced:        {:#018x}", state);

    // 13. Select AI action
    state = ai_action_selected(state, input);
    println!("13 ai_action_selected:         {:#018x}", state);

    // 14. Apply damage
    state = damage_applied(state, input);
    println!("14 damage_applied:             {:#018x}", state);

    println!("combat_sim: 14 patterns | final_state: {:#018x}", state);
}
