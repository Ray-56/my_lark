use bevy_egui::{
    egui::{self, epaint::Shadow, style::ScrollStyle, Color32, Rounding, Stroke, Vec2, Visuals},
    EguiContexts,
};

pub fn setup_ui(mut context: EguiContexts) {
    let ctx = context.ctx_mut();
    setup_fonts(ctx);
    setup_style(ctx);
    ctx.system_theme();
    setup_common_visuals(ctx);
}

pub fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.extend([
        (
            "cjk".to_owned(), // （Chinese, Japanese, Korean）的缩写
            egui::FontData::from_static(include_bytes!(
                "../../assets/fonts/notosanssc-regular.otf"
            )),
        ),
        (
            "iconfong".to_owned(),
            egui::FontData::from_static(include_bytes!("../../assets/fonts/iconfont.ttf")),
        ),
    ]);

    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .extend(["cjk".to_owned(), "iconfong".to_owned()]);

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("cjk".to_owned());

    ctx.set_fonts(fonts);
}

fn setup_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    ctx.set_pixels_per_point(1.2);

    style.text_styles = [
        (egui::TextStyle::Heading, egui::FontId::proportional(20.0)),
        (egui::TextStyle::Body, egui::FontId::proportional(16.0)),
        (egui::TextStyle::Monospace, egui::FontId::monospace(14.0)),
        (egui::TextStyle::Button, egui::FontId::proportional(16.0)),
        (egui::TextStyle::Small, egui::FontId::proportional(12.0)),
    ]
    .into();

    ctx.set_style(style);
}

fn setup_common_visuals(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    let rounding = 12.0;
    // 设置基础视觉样式
    style.visuals = Visuals {
        window_rounding: Rounding::same(rounding),
        menu_rounding: Rounding::same(rounding),
        popup_shadow: Shadow {
            offset: Vec2::new(0.0, 3.0),
            blur: 1.0,
            spread: 10.0,
            ..Default::default()
        },
        window_shadow: Shadow {
            offset: Vec2::new(0.0, 3.0),
            blur: 1.0,
            spread: 10.0,
            ..Default::default()
        },
        panel_fill: Color32::TRANSPARENT,
        widgets: {
            let mut widgets = style.visuals.widgets.clone();
            widgets.noninteractive.rounding = Rounding::same(rounding);
            widgets.inactive.rounding = Rounding::same(rounding);
            widgets.hovered.rounding = Rounding::same(rounding);
            widgets.active.rounding = Rounding::same(rounding);
            widgets.open.rounding = Rounding::same(rounding);
            widgets
        },
        ..style.visuals
    };

    style.visuals.window_stroke =
        Stroke::new(1.0, Color32::from_rgba_premultiplied(255, 255, 255, 12));
    
    // 未悬停状态的滚动条
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgba_premultiplied(128, 128, 128, 60);
    style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;

    // 悬停状态的滚动条
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgba_premultiplied(128, 128, 128, 80);
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;

    // 激活状态的滚动条
    style.visuals.widgets.active.bg_fill = Color32::from_rgba_premultiplied(128, 128, 128, 100);
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;

    // 滚动条的大小和圆角
    style.spacing.scroll = ScrollStyle { bar_width: 1.5, ..Default::default() }; 
    style.visuals.widgets.inactive.rounding = Rounding::same(3.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(3.0);
    style.visuals.widgets.active.rounding = Rounding::same(3.0);

    ctx.set_style(style);
}
