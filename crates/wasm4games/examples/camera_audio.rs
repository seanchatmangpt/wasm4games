use wasm4games::patterns::{
    audio_distance_attenuated, audio_fade_applied, audio_priority_selected,
    audio_trigger_evaluated, camera_distance_clamped, camera_follow_lerped, camera_shake_applied,
    fov_adjusted, look_target_weighted, volume_clamped,
};

fn main() {
    let mut state = 0x0000_0080u64;
    let input = 0x0000_0020u64;

    // Camera tracks an explosion, audio fades out

    // Pattern 41: camera_distance_clamped
    state = camera_distance_clamped(state, input);
    println!("camera_distance_clamped:    state = {:#018x}", state);

    // Pattern 42: look_target_weighted
    state = look_target_weighted(state, input);
    println!("look_target_weighted:       state = {:#018x}", state);

    // Pattern 43: fov_adjusted
    state = fov_adjusted(state, input);
    println!("fov_adjusted:               state = {:#018x}", state);

    // Pattern 44: camera_shake_applied
    state = camera_shake_applied(state, input);
    println!("camera_shake_applied:       state = {:#018x}", state);

    // Pattern 45: camera_follow_lerped
    state = camera_follow_lerped(state, input);
    println!("camera_follow_lerped:       state = {:#018x}", state);

    // Pattern 46: audio_priority_selected
    state = audio_priority_selected(state, input);
    println!("audio_priority_selected:    state = {:#018x}", state);

    // Pattern 47: volume_clamped
    state = volume_clamped(state, input);
    println!("volume_clamped:             state = {:#018x}", state);

    // Pattern 48: audio_fade_applied
    state = audio_fade_applied(state, input);
    println!("audio_fade_applied:         state = {:#018x}", state);

    // Pattern 49: audio_trigger_evaluated
    state = audio_trigger_evaluated(state, input);
    println!("audio_trigger_evaluated:    state = {:#018x}", state);

    // Pattern 50: audio_distance_attenuated
    state = audio_distance_attenuated(state, input);
    println!("audio_distance_attenuated:  state = {:#018x}", state);

    println!("camera_audio: 10 patterns | final_state: {:#018x}", state);
}
