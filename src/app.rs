use std::fmt::Display;

use egui::{text::LayoutJob, vec2, Align, Align2, FontSelection, Frame, InnerResponse, Margin, RichText, Stroke, Style};
use egui_flex::{item, Flex, FlexAlign, FlexAlignContent, FlexJustify};
use egui_material_icons::{icon_button, icons::{ICON_ADD, ICON_CIRCLE, ICON_DELETE, ICON_JOIN, ICON_POWER, ICON_POWER_OFF, ICON_POWER_SETTINGS_NEW, ICON_UNDO, ICON_VOLUME_OFF, ICON_VOLUME_UP}};
use strum::VariantArray;

use crate::theme;

#[derive(serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::VariantArray, PartialEq, Copy, Clone)]
enum AudioSource {
    #[strum(to_string = "Analog 1")]
    Analog1,
    #[strum(to_string = "Analog 2")]
    Analog2,
    #[strum(to_string = "Analog 3")]
    Analog3,
    #[strum(to_string = "Analog 4")]
    Analog4,
    #[strum(to_string = "Analog 5")]
    Analog5,
    #[strum(to_string = "Analog 6")]
    Analog6,
    #[strum(to_string = "Analog 7")]
    Analog7,
    #[strum(to_string = "Analog 8")]
    Analog8,
    #[strum(to_string = "PCM 1")]
    PCM1,
    #[strum(to_string = "PCM 2")]
    PCM2,
    #[strum(to_string = "PCM 3")]
    PCM3,
    #[strum(to_string = "PCM 4")]
    PCM4,
    #[strum(to_string = "PCM 5")]
    PCM5,
    #[strum(to_string = "PCM 6")]
    PCM6,
    #[strum(to_string = "SPDIF 1")]
    SPDIF1,
    #[strum(to_string = "SPDIF 2")]
    SPDIF2,
    #[strum(to_string = "ADAT 1")]
    ADAT1,
    #[strum(to_string = "ADAT 2")]
    ADAT2,
    #[strum(to_string = "ADAT 3")]
    ADAT3,
    #[strum(to_string = "ADAT 4")]
    ADAT4,
    #[strum(to_string = "ADAT 5")]
    ADAT5,
    #[strum(to_string = "ADAT 6")]
    ADAT6,
    #[strum(to_string = "ADAT 7")]
    ADAT7,
    #[strum(to_string = "ADAT 8")]
    ADAT8,
    #[strum(to_string = "Mix A")]
    MixA,
    #[strum(to_string = "Mix B")]
    MixB,
    #[strum(to_string = "Mix C")]
    MixC,
    #[strum(to_string = "Mix D")]
    MixD,
    #[strum(to_string = "Mix E")]
    MixE,
    #[strum(to_string = "Mix F")]
    MixF
}

#[derive(serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::VariantArray, PartialEq, Copy, Clone)]
enum AudioDestination {
    #[strum(to_string = "Mix A")]
    MixA,
    #[strum(to_string = "Mix B")]
    MixB,
    #[strum(to_string = "Mix C")]
    MixC,
    #[strum(to_string = "Mix D")]
    MixD,
    #[strum(to_string = "Mix E")]
    MixE,
    #[strum(to_string = "Mix F")]
    MixF
}

#[derive(serde::Deserialize, serde::Serialize)]
struct MixerDestination {
    stereo: bool,
    dest: AudioSource,
    dest_r: AudioSource,
    gain: f32
}

#[derive(serde::Deserialize, serde::Serialize)]
struct MixerEntry {
    name: String,
    enabled: bool,
    stereo: bool,
    split: bool,
    source: AudioSource,
    source_r: AudioSource,
    dests: Vec<MixerDestination>
}

impl MixerEntry {
    fn new() -> Self {
        Self {
            name: "Unnamed".to_owned(),
            enabled: true,
            stereo: false,
            split: false,
            source: AudioSource::Analog1,
            source_r: AudioSource::Analog2,
            dests: Vec::new()
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
struct MixerOutput {
    name: String,
    gain: f32,
    mute: bool,
    source: (AudioSource, AudioSource),
    split: bool
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct ScarlettControlApp {
    // test
    test: String,
    // selected: Option<AudioSource>
    capture: [Option<AudioSource>; 18],
    // TODO make this fixed size array to manage IDs better
    mixer_entries: Vec<MixerEntry>,
    global_gain: f32,
    global_mute: bool,
    hi_z_1: bool,
    hi_z_2: bool,
    outputs: [MixerOutput; 3]
}

impl Default for ScarlettControlApp {
    fn default() -> Self {
        let output = |name: &str| MixerOutput {
            name: name.to_owned(),
            gain: 0.0,
            mute: false,
            source: (AudioSource::MixA, AudioSource::MixB),
            split: false
        };

        Self {
            test: "wahoo!".to_owned(),
            capture: std::array::from_fn(|i| Some(AudioSource::VARIANTS[i])),
            mixer_entries: Vec::new(),
            global_gain: 0.0,
            global_mute: false,
            hi_z_1: false,
            hi_z_2: false,
            outputs: [
                output("Monitor"),
                output("Headphone"),
                output("SPDIF")
            ]
        }
    }
}

impl ScarlettControlApp {
    // called once before first frame
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // can customize look and feel !
        // cc.egui_ctx.set_fonts();

        egui_material_icons::initialize(&cc.egui_ctx);
        cc.egui_ctx.add_font(theme::font());
        cc.egui_ctx.set_visuals(theme::visuals(cc.egui_ctx.style().visuals.clone()));


        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else { Default::default() }
    }
}

fn set_menu_style(style: &mut Style) {
    // style.spacing.button_padding = vec2(2.0, 0.0);
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    style.visuals.widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
    style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
}

impl eframe::App for ScarlettControlApp {
    // save state before shutdown
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    // repaint
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            Flex::horizontal().w_full().justify(FlexJustify::SpaceBetween).align_items(FlexAlign::Center).show(ui, |flex| {
                flex.add_ui(item(), |ui| {
                    set_menu_style(ui.style_mut());
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });    
                });

                flex.add_flex(item(), Flex::horizontal().gap(vec2(8.0, 8.0)).justify(FlexJustify::Center), |flex| {
                    flex.add_ui(item(), |ui| {
                        ui.label(RichText::new("Global").weak());
                    });
                    flex.add_ui(item(), |ui| {
                        mute_gain(ui, &mut self.global_mute, &mut self.global_gain);
                    });
                    flex.add_ui(item(), |ui| {
                        ui.horizontal(|ui| {
                            for (i, l) in [&mut self.hi_z_1, &mut self.hi_z_2].iter_mut().enumerate() {
                                if ui.selectable_label(**l, format!("HiZ {}", i + 1)).clicked() {
                                    **l = !**l;
                                }
                            }
                        })
                    });
                });
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.add_space(2.0);
            egui::Grid::new("bottom_g").num_columns(2).start_row(1).striped(true).show(ui, |ui| {
                for o in self.outputs.as_mut() {
                    ui.label(o.name.clone());
                    Flex::horizontal().w_full().align_items(FlexAlign::Center).gap(vec2(12.0, 12.0)).show(ui, |flex| {
                        flex.add_ui(item(), |ui| {
                            mute_gain(ui, &mut o.mute, &mut o.gain);
                        });
                        flex.add_ui(item(), |ui| {
                            mono_stereo_combobox(ui, o.name.clone(), &mut o.source.0, &mut o.source.1, &mut o.split);
                        })
                    });
                    ui.end_row()
                }
            })    
        });

        egui::SidePanel::left("capture")
            .resizable(false)
            .frame(Frame {
                inner_margin: Margin::symmetric(8.0, 4.0),
                fill: ctx.style().visuals.panel_fill,
                ..Default::default()
            })
            .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                Flex::horizontal().w_full().align_items_content(Align2::LEFT_TOP).show(ui, |flex| {
                    flex.add_ui(item().grow(1.0), |ui| ui.heading("Capture"));
                    flex.add_ui(item(), |ui| {
                        if icon_button(ui, ICON_UNDO).clicked() {
                            self.capture = Self::default().capture;
                        }
                    });
                });
                ui.add_space(4.0);
                egui::Grid::new("capture_g")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        for (i, selected) in self.capture.as_mut_slice().iter_mut().enumerate() {
                            let label = (i + 1).to_string();
                            ui.label(label.clone());
                            egui::ComboBox::from_id_salt(label)
                                .selected_text(selected.map_or("Off".to_owned(), |s| s.to_string()))
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(selected, None, "Off");
                                    for e in AudioSource::VARIANTS {
                                        ui.selectable_value( selected, Some(*e), e.to_string());
                                    }
                                });    
                            ui.end_row();    
                        }
                    });
            });
        });

        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::symmetric(8.0, 4.0),
                fill: ctx.style().visuals.panel_fill,
                ..Default::default()
            })
            .show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                Flex::vertical()
                    .w_full()
                    .align_items_content(Align2::LEFT_TOP)
                    .gap(vec2(8.0, 8.0))
                    .show(ui, |flex| {
                        flex.add_ui(item(), |ui| ui.heading("Mixer"));

                        let remaining_channels = 18 - self.mixer_entries.iter().filter(|e| e.enabled).fold(0, |a, e| a + (if e.stereo {2} else {1}));

                        let mut to_remove: Vec<usize> = Vec::new();
                        for (i, m) in self.mixer_entries.iter_mut().enumerate()/*.filter_map(|(i, m)| m.as_mut().map(|e| (i, e)))*/ {
                            flex.add_ui(item(), |ui| {
                                if !m.enabled {
                                    ui.style_mut().visuals.override_text_color = Some(theme::colors::TEXT_DISABLED);
                                }
                                card_frame(m.enabled).show(ui, |ui| {
                                    Flex::horizontal().w_full().align_items(FlexAlign::Center).show(ui, |flex| {
                                        flex.add_ui(item(), |ui| {
                                            ui.add_enabled_ui(m.enabled || remaining_channels > if m.stereo { 1 } else { 0 }, |ui| {
                                                if egui_material_icons::icon_button(ui, if m.enabled { ICON_POWER } else { ICON_POWER_OFF }).clicked() {
                                                    m.enabled = !m.enabled;
                                                }
                                            })
                                        });
                                        // flex.add(item(), egui::Label::new(format!("id: {}", i)));
                                        flex.add(item().grow(1.0), egui::TextEdit::singleline(&mut m.name).background_color(egui::Color32::TRANSPARENT).desired_width(0.0).margin(Margin::symmetric(4.0, 0.0)));
                                        flex.add_ui(item(), |ui|
                                            ui.add_enabled(m.stereo || remaining_channels > 0, egui::Checkbox::new(&mut m.stereo, "Stereo"))
                                        );
                                        flex.add_ui(item(), |ui| {
                                            if egui_material_icons::icon_button(ui, ICON_DELETE).clicked() {
                                                // self.mixer_entries.remove(i);
                                                to_remove.push(i);
                                                // self.mixer_entries[i] = None;
                                            }
                                        });
                                    });
                                    ui.horizontal(|ui| {
                                        ui.label("Source");
                                        if m.stereo {
                                            mono_stereo_combobox(ui, format!("mc-{}", i), &mut m.source, &mut m.source_r, &mut m.split);
                                        } else {
                                            variant_combobox(ui, format!("m-{}", i), &mut m.source);
                                        }    
                                    });
                                    ui.add_space(8.0);
                                    ui.label("Destinations");
                                });
                            });    
                        }
                        for i in to_remove {
                            self.mixer_entries.remove(i);
                        }

                        // let next_empty = self.mixer_entries.iter().position(|v| v.is_none());
                        flex.add_flex(item(), Flex::horizontal().align_content(FlexAlignContent::Stretch).w_full(), |flex| {
                            flex.add_ui(item().grow(1.0), |ui| {
                                ui.scope(|ui| {
                                    ui.spacing_mut().button_padding = vec2(8.0, 8.0);
                                    if ui.add_enabled(
                                        remaining_channels > 0, 
                                        egui::Button::new(rich_text_add(
                                            egui_material_icons::icon_text(ICON_ADD), 
                                            RichText::new("Add")))
                                    ).clicked() {
                                        self.mixer_entries.push(MixerEntry::new());
                                    }
                                });    
                            });
                        });
                    })
            });
        });
    }
}

fn gain_drag_value(v: &mut f32) -> egui::DragValue {
    let gain = *v;
    egui::DragValue::new(v).speed(0.1).range(-128.0..=6.0).prefix(if gain < 0.0 { "" } else { "+" }).suffix("dB")
}

fn mute_gain(ui: &mut egui::Ui, mute: &mut bool, gain: &mut f32) {
    ui.horizontal(|ui| {
        ui.scope(|ui| {
            if *mute {
                ui.style_mut().visuals.override_text_color = Some(egui::Color32::BROWN /* actually red??? */);
            }
            if egui_material_icons::icon_button(ui, if *mute { ICON_VOLUME_OFF } else { ICON_VOLUME_UP }).clicked() {
                *mute = !*mute;
            }
        });
        ui.add(gain_drag_value(gain));
    });
}

fn rich_text_add(a: RichText, b: RichText) -> LayoutJob {
    let mut layout_job = LayoutJob::default();
    a.append_to(&mut layout_job, &Style::default(), FontSelection::Default, Align::Center);
    RichText::new(" ").append_to(&mut layout_job, &Style::default(), FontSelection::Default, Align::Center);
    b.append_to(&mut layout_job, &Style::default(), FontSelection::Default, Align::Center);
    layout_job
}

fn card_frame(enabled: bool) -> egui::Frame {
    egui::Frame::none()
        .fill(theme::colors::BG)
        .stroke(Stroke::new(1.0, if enabled { theme::colors::ACTIVE } else { theme::colors::BG2 }))
        .inner_margin(8.0)
        .rounding(4.0)
}

fn variant_combobox<T>(ui: &mut egui::Ui, id_salt: impl std::hash::Hash, selected: &mut T) -> InnerResponse<std::option::Option<()>>
    where T: VariantArray + ToString + PartialEq + Copy
{
    egui::ComboBox::from_id_salt(id_salt)
        .width(32.0)
        .selected_text(selected.to_string())
        .show_ui(ui, |ui| {
            for e in T::VARIANTS {
                ui.selectable_value( selected, *e, e.to_string());
            }
        })
}

fn mono_stereo_combobox<T>(
    ui: &mut egui::Ui,
    id_salt: String,
    left: &mut T,
    right: &mut T,
    split: &mut bool
) -> egui::InnerResponse<()>
    where T: VariantArray + PartialEq + Copy + Display
{
    Flex::horizontal()
        .show(ui, |flex| {
            flex.add_ui(item(), |ui| {
                if egui_material_icons::icon_button(ui, if *split { ICON_JOIN } else { ICON_CIRCLE }).clicked() {
                    *split = !*split;
                }
            });

            if *split {
                flex.add_ui(item().grow(1.0), |ui| variant_combobox(ui, id_salt.clone() + "_l", left));
                flex.add_ui(item().grow(1.0), |ui| variant_combobox(ui, id_salt + "_r", right));
            } else {
                if *left == *T::VARIANTS.last().unwrap() {
                    // make sure that we can still assign the next value to the right channel
                    *left = T::VARIANTS[T::VARIANTS.len() - 2];
                }
                *right = T::VARIANTS[T::VARIANTS.iter().position(|&x| x == *left).unwrap() + 1];
                flex.add_ui(item().grow(1.0), |ui| {
                    egui::ComboBox::from_id_salt(id_salt)
                        .selected_text(format!("{} / {}", left, right))
                        .show_ui(ui, |ui| {
                            for p in T::VARIANTS.chunks_exact(2) {
                                if let &[l, r] = p {
                                    ui.selectable_value(left, l, format!("{} / {}", l, r));
                                }
                            }
                        })
                });
            }
        })
}