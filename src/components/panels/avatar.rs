use bevy::{prelude::ResMut, text};
use bevy_egui::egui::{self, menu};

use crate::resources::UiState;

struct WindowConf {
    width: f32,
    height: f32,
    position: egui::Pos2,
}

fn create_custom_window<'a>(
    ctx: &egui::Context,
    title: &'a str,
    config: WindowConf,
) -> egui::Window<'a> {
    egui::Window::new(title)
        .fixed_pos(config.position)
        .fixed_size(egui::vec2(config.width, config.height))
        .frame(
            egui::Frame::window(&ctx.style())
                .rounding(8.0)
                .shadow(egui::epaint::Shadow {
                    color: egui::Color32::from_black_alpha(60),
                    ..Default::default()
                })
                .fill(egui::Color32::from_rgb(32, 32, 32))
                .stroke(egui::Stroke::NONE)
                .outer_margin(0.0)
                .inner_margin(8.0),
        )
        .title_bar(false)
        .resizable(false)
}

fn create_avatar_button(text: &str, size: f32, button_size: f32) -> impl egui::Widget + '_ {
    egui::Button::new(
        egui::RichText::new(text)
            .color(egui::Color32::WHITE)
            .size(size),
    )
    .fill(egui::Color32::from_rgb(255, 148, 0))
    .rounding(25.0)
    .min_size(egui::vec2(button_size, button_size))
}

pub fn avatar(ui: &mut egui::Ui, ui_state: &mut ResMut<UiState>) {
    ui.add_space(5.0);
    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
        let avatar_response = ui.add(
            egui::Button::new(
                egui::RichText::new("R1")
                    .color(egui::Color32::WHITE)
                    .size(16.0),
            )
            .fill(egui::Color32::from_rgb(255, 148, 0))
            .rounding(30.0)
            .min_size(egui::vec2(32.0, 32.0)),
        );
        if avatar_response.clicked() {
            ui_state.show_avatar_menu = !ui_state.show_avatar_menu;
        }
        if ui_state.show_avatar_menu {
            show_avatar_menu(ui.ctx(), avatar_response.rect, ui_state);
        }
    });
}

struct MenuItem {
    text: String,
    show_red_hot: bool,
    is_separator_after: bool,
}
fn render_menu_item(ui: &mut egui::Ui, item: &MenuItem) -> egui::Response {
    let button = ui.add(
        egui::Button::new(&item.text)
            .fill(egui::Color32::TRANSPARENT)
            .min_size(egui::vec2(ui.available_width(), 36.0)),
    );
    if item.show_red_hot {
        ui.painter().circle_filled(
            button.rect.right_center() - egui::vec2(10.0, 10.0),
            4.0,
            egui::Color32::RED,
        );
    }
    if item.is_separator_after {
        ui.separator();
    }
    button
}

pub fn show_avatar_menu(
    ctx: &egui::Context,
    avatar_rect: egui::Rect,
    ui_state: &mut ResMut<UiState>,
) {
    let menu_width = 280.0;
    let config = WindowConf {
        width: 280.0,
        height: 400.0,
        position: egui::pos2(avatar_rect.right() + 10.0, avatar_rect.top()),
    };
    create_custom_window(ctx, "avatar_menu", config).show(ctx, |ui| {
        ui.set_min_width(menu_width);
        ui.add_space(16.0);
        ui.horizontal(|ui| {
            // 头像
            ui.add(create_avatar_button("R", 24.0, 48.0));
            ui.vertical(|ui| {
                ui.heading("R");
                ui.label("Lark personal account");
                ui.add_space(4.0);
                let status_button = ui.button("+ Status");
                if status_button.clicked() {
                    ui_state.show_status_menu = !ui_state.show_status_menu;
                }
                if ui_state.show_status_menu {
                    show_status_menu(ui.ctx(), status_button.rect, ui_state);
                }
            });
        });
        ui.add_space(8.0);
        let mut signature = String::new();
        ui.add(
            egui::TextEdit::singleline(&mut signature)
                .hint_text("Please enter your signature...")
                .margin(egui::vec2(8.0, 8.0)),
        );
        ui.add_space(8.0);
        ui.separator();

        let menu_items = vec![
            MenuItem {
                text: "Profile".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "My Link and QR Code".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "Add Account".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "Help and Customer Service".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "Settings".to_string(),
                show_red_hot: false,
                is_separator_after: true,
            },
            MenuItem {
                text: "Sign out".to_string(),
                show_red_hot: false,
                is_separator_after: false,
            },
            MenuItem {
                text: "Admin Background".to_string(),
                show_red_hot: false,
                is_separator_after: false,
            },
        ];
        for (index, item) in menu_items.iter().enumerate() {
            if render_menu_item(ui, item).clicked() {
                println!("menu item #: {} clicked: {}", index, item.text);
            }
        }
    });
}

struct StatusMenuItem<'a> {
    icon: &'a str,
    text: &'a str,
    duration: &'a str,
}
fn render_status_menu_item(ui: &mut egui::Ui, item: &StatusMenuItem) {
    ui.add_space(8.0);
    let resp = ui.add_sized(
        egui::vec2(ui.available_width(), 50.0),
        egui::Button::new("").fill(egui::Color32::TRANSPARENT),
    );
    if resp.clicked() {
        println!("status menu item clicked: {}", item.text);
    }
    status_style(ui, item.icon, item.text, item.duration, resp.rect);
}

pub fn show_status_menu(
    ctx: &egui::Context,
    button_rect: egui::Rect,
    ui_state: &mut ResMut<UiState>,
) {
    let _ = ui_state;
    let menu_width = 280.0;

    let menu_pos = egui::pos2(button_rect.right() + 10.0, button_rect.top());

    egui::Window::new("status_menu")
        .fixed_pos(menu_pos)
        .fixed_size(egui::vec2(menu_width, 400.0))
        .frame(
            egui::Frame::window(&ctx.style())
                .rounding(8.0)
                .shadow(egui::epaint::Shadow {
                    color: egui::Color32::from_black_alpha(60),
                    ..Default::default()
                })
                .fill(egui::Color32::from_rgb(32, 32, 32))
                .stroke(egui::Stroke::NONE)
                .outer_margin(0.0)
                .inner_margin(8.0),
        )
        .title_bar(false)
        .resizable(false)
        .title_bar(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.set_min_width(menu_width);
            ui.heading("My Status");
            ui.add_space(8.0);
            // TODO: replace icon
            let status_items = vec![
                StatusMenuItem {
                    icon: "🔕",
                    text: "Do Not Disturb",
                    duration: "Until 5:00 PM",
                },
                StatusMenuItem {
                    icon: "📅",
                    text: "In a meeting",
                    duration: "1 hour",
                },
                StatusMenuItem {
                    icon: "🏠",
                    text: "Working from home",
                    duration: "All day",
                },
            ];
            for item in status_items {
                render_status_menu_item(ui, &item);
            }
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui
                    .button(egui::RichText::new("+ Add status").size(14.0))
                    .clicked()
                {
                    println!("add status clicked");
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(egui::RichText::new("⚙").size(14.0)).clicked() {
                        println!("status settings clicked");
                    }
                })
            })
        });
}

fn status_style(ui: &mut egui::Ui, icon: &str, text: &str, duration: &str, resp: egui::Rect) {
	ui.allocate_ui_at_rect(resp, |ui| {
		ui.horizontal(|ui| {
			ui.label(egui::RichText::new(icon).size(17.0));
			ui.add_space(15.0);
			ui.vertical(|ui| {
				ui.label(egui::RichText::new(text).size(16.0));
				ui.label(egui::RichText::new(duration).size(14.0).color(egui::Color32::GRAY));
			});
			ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
				ui.label(egui::RichText::new("...").size(14.0).color(egui::Color32::GRAY));
			});
		});
	});
}