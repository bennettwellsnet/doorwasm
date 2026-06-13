// doorwasm - Rust source for the WebAssembly module
// This provides optimized calculations for door replacement projects.

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn calculate_optimal_materials(
    total_openings: u32,
    avg_width_in: f32,
    avg_height_in: f32,
    door_type: u32, // 0=prehung, 1=slab
) -> f32 {
    // Simplified optimization: calculate effective material "units"
    // In real version this would do bin-packing / cost optimization
    // across standard door sizes to minimize waste.
    let base_area = avg_width_in * avg_height_in / 144.0; // sq ft
    let waste_factor = if door_type == 0 { 1.08 } else { 1.12 }; // prehung vs slab
    let materials = (total_openings as f32) * base_area * waste_factor;
    materials
}

#[wasm_bindgen]
pub fn estimate_install_hours(
    total_openings: u32,
    complexity: f32, // 1.0 = simple, 1.5 = tricky frames
) -> f32 {
    // Productivity model: base 2.5 hours per door + complexity
    let base = total_openings as f32 * 2.5;
    base * complexity
}

#[wasm_bindgen]
pub fn optimize_bulk_purchase(
    num_doors: u32,
    target_size: f32, // e.g. 32.0 for 32" door
) -> f32 {
    // Demo of "WASM optimized" recommendation
    // Suggests standard sizes and quantity to buy to minimize custom orders
    if num_doors < 3 {
        return num_doors as f32 * target_size;
    }
    // Simple bulk logic: buy in groups of 6 for discount simulation
    let bulk = ((num_doors as f32 / 6.0).ceil() * 6.0) * target_size;
    bulk
}
