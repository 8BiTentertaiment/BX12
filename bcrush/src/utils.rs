// Utility functions for Bcrush

pub fn calculate_depth(z: f32) -> u8 {
    // Depth calculation helper
    (z * 255.0) as u8
}
