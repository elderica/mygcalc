use evalexpr::eval;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    display: String,
    // accumulator: f64,
    // radio: bool,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            display: "0".to_owned(),
            // accumulator: 0.,
            // radio: true,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {
        // eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self {
            display,
            // accumulator,
            // radio,
        } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.vertical_centered_justified(|ui| {
                ui.text_edit_singleline(display);
                ui.columns(4, |ui| {
                    ui[0].vertical_centered_justified(|ui| {
                        // if ui.add(egui::Button::new("1").frame(true)).clicked() {
                        if ui.button("1").clicked() {
                            if *display == "0" {
                                *display = "1".to_string();
                            } else {
                                *display += "1";
                            }
                        }
                        if ui.button("4").clicked() {
                            if *display == "0" {
                                *display = "4".to_string();
                            } else {
                                *display += "4";
                            }
                        }
                        if ui.button("7").clicked() {
                            if *display == "0" {
                                *display = "7".to_string();
                            } else {
                                *display += "7";
                            }
                        }
                        if ui.button("0").clicked() {
                            if *display == "0" {
                                *display = "0".to_string();
                            } else {
                                *display += "0";
                            }
                        }
                        if ui.button("<-").clicked() {
                            display.pop();
                            if display.is_empty() {
                                *display = "0".to_string();
                            }
                        }
                    });
                    ui[1].vertical_centered_justified(|ui| {
                        if ui.button("2").clicked() {
                            if *display == "0" {
                                *display = "2".to_string();
                            } else {
                                *display += "2";
                            }
                        }
                        if ui.button("5").clicked() {
                            if *display == "0" {
                                *display = "5".to_string();
                            } else {
                                *display += "5";
                            }
                        }
                        if ui.button("8").clicked() {
                            if *display == "0" {
                                *display = "8".to_string();
                            } else {
                                *display += "8";
                            }
                        }
                        if ui.button(".").clicked() && !display.contains('.') {
                            *display += ".";
                        }
                        if ui.button("C").clicked() {
                            *display = "0".to_string();
                        }
                    });
                    ui[2].vertical_centered_justified(|ui| {
                        if ui.button("3").clicked() {
                            if *display == "0" {
                                *display = "3".to_string();
                            } else {
                                *display += "3";
                            }
                        }
                        if ui.button("6").clicked() {
                            if *display == "0" {
                                *display = "6".to_string();
                            } else {
                                *display += "6";
                            }
                        }
                        if ui.button("9").clicked() {
                            if *display == "0" {
                                *display = "9".to_string();
                            } else {
                                *display += "9";
                            }
                        }
                        if ui.button("(").clicked() {
                            if *display == "0" {
                                *display = "(".to_string();
                            } else {
                                *display += "(";
                            }
                        }
                        if ui.button(")").clicked() {
                            if *display == "0" {
                                *display = ")".to_string();
                            } else {
                                *display += ")";
                            }
                        }
                    });
                    ui[3].vertical_centered_justified(|ui| {
                        if ui.button("+").clicked() {
                            if *display == "0" {
                                *display = "+".to_string();
                            } else {
                                *display += "+";
                            }
                        }
                        if ui.button("-").clicked() {
                            if *display == "0" {
                                *display = "-".to_string();
                            } else {
                                *display += "-";
                            }
                        }
                        if ui.button("*").clicked() {
                            if *display == "0" {
                                *display = "*".to_string();
                            } else {
                                *display += "*";
                            }
                        }
                        if ui.button("/").clicked() {
                            if *display == "0" {
                                *display = "/".to_string();
                            } else {
                                *display += "/";
                            }
                        }
                        if ui.button("=").clicked() {
                            if let Ok(v) = eval(display) {
                                *display = v.to_string();
                            } else {
                                *display = "ERROR".to_string();
                            }
                        }
                    });
                });

                // egui::ComboBox::from_label("")
                //     .selected_text(format!("{:?}", radio))
                //     .width(320.)
                //     .show_ui(ui, |ui| {
                //         ui.selectable_value(radio, true, "true");
                //         ui.selectable_value(radio, false, "false");
                //     });
            });
        });
    }
}
