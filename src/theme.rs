pub mod colors {
    // pub const OVERLAY1: egui::Color32 = egui::Color32::YELLOW;
    // pub const TEXT: egui::Color32 = egui::Color32::WHITE;
    pub const TEXT: egui::Color32 = egui::Color32::from_rgb(208, 208, 217);
    pub const TEXT_DISABLED: egui::Color32 = egui::Color32::from_rgb(146, 146, 153);
    pub const FRAME: egui::Color32 = egui::Color32::from_rgb(29, 29, 38);
    pub const FAINT: egui::Color32 = egui::Color32::from_rgb(34, 34, 45);
    pub const BG: egui::Color32 = egui::Color32::from_rgb(44, 44, 59);
    pub const BG2: egui::Color32 = egui::Color32::from_rgb(55, 55, 69);
    pub const ON: egui::Color32 = egui::Color32::from_rgb(178, 121, 242);
    pub const ACTIVE: egui::Color32 = egui::Color32::from_rgb(113, 58, 145);

}

fn make_widget_visual(old: egui::style::WidgetVisuals, bg_fill: egui::Color32) -> egui::style::WidgetVisuals {
    egui::style::WidgetVisuals {
        bg_fill,
        weak_bg_fill: bg_fill,
        bg_stroke: egui::Stroke {
            color: colors::BG,
            ..old.bg_stroke
        },
        fg_stroke: egui::Stroke {
            color: colors::TEXT,
            ..old.fg_stroke
        },
        ..old
    }
}

pub fn font() -> epaint::text::FontInsert {
    const MY_FONT: &str = "kosugi";

    epaint::text::FontInsert {
        name: MY_FONT.to_owned(),
        data: egui::FontData::from_static(include_bytes!("../KosugiMaruModded-Regular.ttf")).into(),
        families: vec![
            epaint::text::InsertFontFamily {
                family: egui::FontFamily::Proportional,
                priority: epaint::text::FontPriority::Highest
            },
            epaint::text::InsertFontFamily {
                family: egui::FontFamily::Monospace,
                priority: epaint::text::FontPriority::Lowest
            }
        ]
    }
}

pub fn visuals(old: egui::Visuals) -> egui::Visuals {
    egui::Visuals {
        override_text_color: Some(colors::TEXT),
        faint_bg_color: colors::FAINT,
        // extreme_bg_color: egui::Color32::BLUE,//colors::BG,
        // code_bg_color: egui::Color32::GREEN,//colors::FRAME,
        window_fill: colors::FRAME,
        panel_fill: colors::FRAME,
        window_stroke: egui::Stroke {
            color: colors::BG,
            ..old.window_stroke
        },
        widgets: egui::style::Widgets {
            noninteractive: make_widget_visual(old.widgets.noninteractive, colors::FRAME),
            inactive: make_widget_visual(old.widgets.inactive, colors::BG2),
            hovered: make_widget_visual(old.widgets.hovered, colors::BG),
            active: make_widget_visual(old.widgets.active, colors::ACTIVE),
            open: make_widget_visual(old.widgets.open, colors::ACTIVE),
        },
        selection: egui::style::Selection {
            bg_fill: colors::ACTIVE,
            stroke: egui::Stroke {
                color: colors::ON,
                ..old.selection.stroke
            }
        },
        dark_mode: true,
        ..old
    }
}