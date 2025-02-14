use std::collections::HashMap;

use bevy::prelude::ResMut;
use bevy_egui::egui::{self, Frame, RichText};

use crate::{
    resources::{NotificationTheme, UiState},
    ChatEvent, ChatFilter, ChatListController, ChatListView,
};

pub fn left_chat_list_ui(
    ctx: &egui::Context,
    ui_state: &mut ResMut<UiState>,
    theme: &mut ResMut<NotificationTheme>,
) -> egui::InnerResponse<()> {
    let unread_counts: HashMap<String, i32> = ui_state
        .unread_counts
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect();

    let mut controller =
        ChatListController::new(&ui_state.chats, &ui_state.select_chat_id, &unread_counts);

    let chats = ui_state.chats.clone();
    let colors = theme.current_colors();
    let frame = Frame {
        fill: colors.background,
        rounding: theme.style.rounding,
        ..Default::default()
    };
    egui::SidePanel::left("chat_list_panel")
        .resizable(true)
        .default_width(250.0)
        .width_range(250.0..=1024.0)
        .frame(frame)
        .show(ctx, |ui| {
            ui.add_space(18.0);
            ui.horizontal(|ui| {
                ui.add_space(18.0);
                let button_text = RichText::new("\u{e609}")
                    .font(theme.fonts.icon.clone())
                    .color(if ui_state.show_siderbar {
                        theme.text_styles.sidebar_button.selected_color
                    } else {
                        theme.text_styles.sidebar_button.color
                    });

                let btn_response = ui.add(egui::Button::new(button_text).frame(false));

                btn_response
                    .clone()
                    .on_hover_text(if ui_state.show_siderbar {
                        "Hide sidebar"
                    } else {
                        "Show sidebar"
                    });

                if btn_response.clicked() {
                    ui_state.show_siderbar = !ui_state.show_siderbar;
                }

                ui.add_space(ui.available_width() - 45.0);

                // 标题
                ui.heading(
                    RichText::new("消息")
                        .font(theme.fonts.title.clone())
                        .color(theme.text_styles.title.color)
                        .strong(),
                );
            });

            let mut view = ChatListView::new(&mut controller);
            match view.render(ui, theme) {
                ChatEvent::Selected { id } => {
                    if let Some(_chat) = chats.iter().find(|c| c.id == id) {
                        ui_state.select_chat_id = id.clone();
                        let chat_messages = ui_state
                            .messages
                            .iter()
                            .filter(|msg| msg.chat_id == id)
                            .cloned()
                            .collect::<Vec<_>>();
                        ui_state.messages = chat_messages;
                        // 标记为已读
                        if let Some(count) = ui_state.unread_counts.get_mut(&id) {
                            *count = 0;
                        }
                    }
                }
                ChatEvent::None => {}
            }

            if controller.view().filter == ChatFilter::Pinned {
                ui_state.show_pin_message = true;
            } else {
                ui_state.show_pin_message = false;
            }

            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
}

pub fn left_sidebar_ui(
    ctx: &egui::Context,
    ui_state: &mut ResMut<UiState>,
    theme: &NotificationTheme,
) -> egui::InnerResponse<()> {
    let colors = theme.current_colors();
    egui::SidePanel::left("left_sidebar_ui")
        .resizable(true)
        .max_width(180.0)
        .default_width(150.0)
        .frame(Frame {
            fill: colors.background,
            rounding: theme.style.rounding,
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.add_space(10.0);
            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new("分组")
                        .font(theme.fonts.title.clone())
                        .color(theme.text_styles.title.color),
                );
                ui.add_space(10.0);
                let menu_items = [
                    ("标记", 2),
                    ("@我", 2),
                    ("标签", 2),
                    ("单聊", 2),
                    ("群组", 2),
                    ("云文档", 2),
                    ("话题", 2),
                    ("已完成", 2),
                ];
                for (label, count) in menu_items {
                    ui.horizontal(|ui| {
                        ui.add_space(10.0);
                        let is_selected = ui_state.selected_siderbar_button == label;
                        let response = ui.selectable_label(is_selected, label);
                        ui.add_space(ui.available_width() - 45.0);
                        ui.label(count.to_string());

                        if response.clicked() {
                            ui_state.selected_siderbar_button = label.to_string();
                        }
                        ui.add_space(10.0);
                    });
                }
            });
        })
}
