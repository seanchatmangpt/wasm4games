use wasm4games::patterns::{
    lag_compensation_applied, packet_priority_evaluated, prediction_error_bounded,
    sync_state_admitted, tick_delta_bounded,
};

fn main() {
    let mut state = 0x0000_000Au64; // 10ms tick delta
    let input = 0x0000_0003u64;

    state = tick_delta_bounded(state, input);
    println!("tick_delta_bounded:       state = {:#018x}", state);

    state = lag_compensation_applied(state, input);
    println!("lag_compensation_applied: state = {:#018x}", state);

    state = packet_priority_evaluated(state, input);
    println!("packet_priority_evaluated: state = {:#018x}", state);

    state = prediction_error_bounded(state, input);
    println!("prediction_error_bounded: state = {:#018x}", state);

    state = sync_state_admitted(state, input);
    println!("sync_state_admitted:      state = {:#018x}", state);

    println!("multiplayer: 5 patterns | final_state: {:#018x}", state);
}
