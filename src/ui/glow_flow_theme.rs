// Implements a dynamic, UV-reactive color grading theme for the dashboard UI
pub struct GlowFlowTheme {
    pub current_uv_index: u8,
}

impl GlowFlowTheme {
    pub fn get_accent_color(&self) -> &'satic str {
        match self.current_uv_index {
            0..=2 => "#4CAF50", // Chill green
            3..=5 => "#FFEB3B", // Moderate yellow
            6..=8 => "#FF9800", // High orange
            _ => "#9C27B0",     // Full-send purple
        }
    }
}
