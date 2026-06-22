use wasm4games::patterns::{
    biome_class_selected, choice_weight_selected, condition_flag_evaluated, currency_delta_applied,
    dialogue_cooldown_bounded, dialogue_node_advanced, level_gate_evaluated,
    narrative_branch_selected, noise_value_sampled, purchase_admitted, reward_tier_selected,
    spawn_weight_evaluated, terrain_height_quantized, tile_variant_selected, xp_threshold_crossed,
};

fn main() {
    let mut state: u64 = 0x0000_0032u64;
    let input: u64 = 0x0000_0001u64;

    // Player enters a new biome — sample noise, select tile, quantize terrain, evaluate spawns, classify biome
    state = noise_value_sampled(state, input);
    println!("noise_value_sampled:       state = {:#018x}", state);

    state = tile_variant_selected(state, input);
    println!("tile_variant_selected:     state = {:#018x}", state);

    state = terrain_height_quantized(state, input);
    println!("terrain_height_quantized:  state = {:#018x}", state);

    state = spawn_weight_evaluated(state, input);
    println!("spawn_weight_evaluated:    state = {:#018x}", state);

    state = biome_class_selected(state, input);
    println!("biome_class_selected:      state = {:#018x}", state);

    // Player purchases an item — apply currency delta, check XP threshold, evaluate level gate, admit purchase, select reward tier
    state = currency_delta_applied(state, input);
    println!("currency_delta_applied:    state = {:#018x}", state);

    state = xp_threshold_crossed(state, input);
    println!("xp_threshold_crossed:      state = {:#018x}", state);

    state = level_gate_evaluated(state, input);
    println!("level_gate_evaluated:      state = {:#018x}", state);

    state = purchase_admitted(state, input);
    println!("purchase_admitted:         state = {:#018x}", state);

    state = reward_tier_selected(state, input);
    println!("reward_tier_selected:      state = {:#018x}", state);

    // Player triggers dialogue — advance node, evaluate condition flag, select narrative branch, bound cooldown, select choice weight
    state = dialogue_node_advanced(state, input);
    println!("dialogue_node_advanced:    state = {:#018x}", state);

    state = condition_flag_evaluated(state, input);
    println!("condition_flag_evaluated:  state = {:#018x}", state);

    state = narrative_branch_selected(state, input);
    println!("narrative_branch_selected: state = {:#018x}", state);

    state = dialogue_cooldown_bounded(state, input);
    println!("dialogue_cooldown_bounded: state = {:#018x}", state);

    state = choice_weight_selected(state, input);
    println!("choice_weight_selected:    state = {:#018x}", state);

    println!(
        "narrative_camera: 15 patterns | final_state: {:#018x}",
        state
    );
}
