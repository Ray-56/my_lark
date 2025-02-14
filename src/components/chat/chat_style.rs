use super::{
    chat_model::{ChatColors, ChatFoots, ChatMainStyle},
    AVATAR_COLORS,
};
use bevy_egui::egui::{self, FontFamily, FontId, Margin, Rounding};
impl Default for ChatMainStyle {
    fn default() -> Self {
        Self {
            colors: ChatColors::default(),
        }
    }
}

impl Default for ChatColors {
    fn default() -> Self {
        Self {
            text: egui::Color32::from_rgb(255, 255, 255),
            avatar_colors: AVATAR_COLORS.to_vec(),
        }
    }
}

impl Default for ChatFoots {
    fn default() -> Self {
        Self {
            title: FontId::new(16., FontFamily::default()),
            content: FontId::new(14., FontFamily::default()),
            timestamp: FontId::new(12., FontFamily::default()),
        }
    }
}
