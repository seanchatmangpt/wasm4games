use wasm4games::patterns::{
    heuristic_distance_estimated, inventory_item_changed, mastery_moment_detected,
    nav_state_advanced, nps_prompt_gated, path_cost_bounded, path_node_expanded,
    quest_step_advanced, share_artifact_generated, status_effect_ticked, waypoint_reached,
};

fn main() {
    let mut state = 0x0000_0001u64;
    let input = 0x0000_0010u64;

    // Pattern 15: status_effect_ticked — player has a buff ticking
    state = status_effect_ticked(state, input);
    println!("status_effect_ticked:       state = {:#018x}", state);

    // Pattern 16: inventory_item_changed — player picks up an item on the path
    state = inventory_item_changed(state, input);
    println!("inventory_item_changed:     state = {:#018x}", state);

    // Pattern 17: quest_step_advanced — player completes a quest objective
    state = quest_step_advanced(state, input);
    println!("quest_step_advanced:        state = {:#018x}", state);

    // Pattern 18: mastery_moment_detected — player demonstrates skill mastery
    state = mastery_moment_detected(state, input);
    println!("mastery_moment_detected:    state = {:#018x}", state);

    // Pattern 19: share_artifact_generated — achievement artifact created for sharing
    state = share_artifact_generated(state, input);
    println!("share_artifact_generated:   state = {:#018x}", state);

    // Pattern 20: nps_prompt_gated — check whether to show NPS survey prompt
    state = nps_prompt_gated(state, input);
    println!("nps_prompt_gated:           state = {:#018x}", state);

    // Pattern 21: path_node_expanded — A* expands a node along the quest path
    state = path_node_expanded(state, input);
    println!("path_node_expanded:         state = {:#018x}", state);

    // Pattern 22: waypoint_reached — player arrives at a waypoint
    state = waypoint_reached(state, input);
    println!("waypoint_reached:           state = {:#018x}", state);

    // Pattern 23: heuristic_distance_estimated — estimate remaining path cost
    state = heuristic_distance_estimated(state, input);
    println!("heuristic_distance_estimated: state = {:#018x}", state);

    // Pattern 24: path_cost_bounded — verify path cost stays within budget
    state = path_cost_bounded(state, input);
    println!("path_cost_bounded:          state = {:#018x}", state);

    // Pattern 25: nav_state_advanced — navigation state machine steps forward
    state = nav_state_advanced(state, input);
    println!("nav_state_advanced:         state = {:#018x}", state);

    println!("world_sim: 11 patterns | final_state: {:#018x}", state);
}
