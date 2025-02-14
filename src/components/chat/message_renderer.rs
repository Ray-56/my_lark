use bevy_egui::{
    self,
    egui::{Button, CollapsingHeader, RichText, Ui},
};

use crate::resources::NotificationTheme;

use super::{ChatMainStyle, ChatMessage, MessageRenderer};

pub struct TextMessageRenderer;
pub struct FileMessageRenderer;
pub struct CodeMessageRenderer;
pub struct ImageMessageRenderer;

impl MessageRenderer for TextMessageRenderer {
    fn render(
        &self,
        ui: &mut Ui,
        message: &ChatMessage,
        style: &ChatMainStyle,
        _theme: &NotificationTheme,
    ) {
        ui.label(RichText::new(&message.content).color(style.colors.text));
    }
}

impl MessageRenderer for CodeMessageRenderer {
    fn render(
        &self,
        ui: &mut Ui,
        message: &ChatMessage,
        _style: &ChatMainStyle,
        _theme: &NotificationTheme,
    ) {
        CollapsingHeader::new("code")
            .id_salt(&message.id)
            .default_open(true)
            .show(ui, |ui| {
                ui.code(&message.content);
            });
    }
}

impl MessageRenderer for FileMessageRenderer {
    fn render(
        &self,
        ui: &mut Ui,
        message: &ChatMessage,
        _style: &ChatMainStyle,
        _theme: &NotificationTheme,
    ) {
        ui.add(Button::new(format!("📎 {}", &message.content)).frame(false));
    }
}

impl MessageRenderer for ImageMessageRenderer {
    fn render(
        &self,
        ui: &mut Ui,
        _message: &ChatMessage,
        _style: &ChatMainStyle,
        _theme: &NotificationTheme,
    ) {
        ui.label("ToDo images");
    }
}
