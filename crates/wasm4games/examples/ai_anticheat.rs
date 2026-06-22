//! Patterns 66-75: AI agent action with anti-cheat validation.
//!
//! Demonstrates reward_signal_clamped, policy_action_selected,
//! observation_class_selected, action_mask_applied, episode_return_bounded,
//! movement_legality_checked, resource_bound_checked, cooldown_legality_checked,
//! action_rate_bounded, transition_legality_checked.

use wasm4games::patterns::{
    action_mask_applied, action_rate_bounded, cooldown_legality_checked, episode_return_bounded,
    movement_legality_checked, observation_class_selected, policy_action_selected,
    resource_bound_checked, reward_signal_clamped, transition_legality_checked,
};

fn main() {
    // state encodes max_resource=100 (0x64)
    let mut state: u64 = 0x0000_0064;
    // input encodes pos=80 (0x50) in low word, max_speed=5 in high word
    let input: u64 = 0x0050_0005;

    println!(
        "ai_anticheat: initial state={:#018x} input={:#018x}",
        state, input
    );

    // Pattern 66: clamp the incoming reward signal
    state = reward_signal_clamped(state, input);
    println!("  [66] reward_signal_clamped     -> {:#018x}", state);

    // Pattern 67: select a policy action from the clamped state
    state = policy_action_selected(state, input);
    println!("  [67] policy_action_selected    -> {:#018x}", state);

    // Pattern 68: classify the observation
    state = observation_class_selected(state, input);
    println!("  [68] observation_class_selected -> {:#018x}", state);

    // Pattern 69: apply action mask (filter illegal action bits)
    state = action_mask_applied(state, input);
    println!("  [69] action_mask_applied       -> {:#018x}", state);

    // Pattern 70: bound episode return
    state = episode_return_bounded(state, input);
    println!("  [70] episode_return_bounded    -> {:#018x}", state);

    // Pattern 71: anti-cheat — check movement legality
    state = movement_legality_checked(state, input);
    println!("  [71] movement_legality_checked -> {:#018x}", state);

    // Pattern 72: anti-cheat — check resource bounds
    state = resource_bound_checked(state, input);
    println!("  [72] resource_bound_checked    -> {:#018x}", state);

    // Pattern 73: anti-cheat — check cooldown legality
    state = cooldown_legality_checked(state, input);
    println!("  [73] cooldown_legality_checked -> {:#018x}", state);

    // Pattern 74: anti-cheat — bound action rate
    state = action_rate_bounded(state, input);
    println!("  [74] action_rate_bounded       -> {:#018x}", state);

    // Pattern 75: anti-cheat — verify transition legality
    state = transition_legality_checked(state, input);
    println!("  [75] transition_legality_checked -> {:#018x}", state);

    println!("ai_anticheat: 10 patterns | final_state: {:#018x}", state);
}
