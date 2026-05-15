mod dr3_equation;

use eframe::egui::{self};

use dr3_equation::equation as Dr3_equation;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_inner_size([356.0, 650.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Calculator",
        options,
        Box::new(|_cc| Ok(Box::new(Dr3CalculatorApp::default()))),
    )
}

struct Dr3CalculatorApp {
    display: String,
    result: String,
}

impl Default for Dr3CalculatorApp {
    fn default() -> Self {
        Dr3CalculatorApp {
            display: String::from(""),
            result: String::from(""),
        }
    }
}

impl eframe::App for Dr3CalculatorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let bg = egui::Color32::from_rgb(26, 26, 26);
        let display_bg = egui::Color32::from_rgb(17, 17, 17);
        let num_bg = egui::Color32::from_rgb(46, 46, 46);
        let op_bg = egui::Color32::from_rgb(58, 48, 32);
        let op_fg = egui::Color32::from_rgb(240, 180, 41);
        let fn_bg = egui::Color32::from_rgb(34, 34, 34);
        let fn_fg = egui::Color32::from_rgb(170, 170, 170);
        let eq_bg = egui::Color32::from_rgb(240, 180, 41);
        let eq_fg = egui::Color32::from_rgb(26, 26, 10);
        let clr_bg = egui::Color32::from_rgb(58, 26, 26);
        let clr_fg = egui::Color32::from_rgb(224, 85, 85);
        let text_primary = egui::Color32::from_rgb(240, 240, 240);
        let text_muted = egui::Color32::from_rgb(102, 102, 102);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(bg)
                    .inner_margin(egui::Margin::symmetric(20.0, 20.0)),
            )
            .show(ctx, |ui| {
                let gap = 8.0;
                let available = ui.available_width();
                let cell_size = ((available - gap * 3.0) / 4.0).floor();
                let cell = [cell_size, cell_size];
                let rounding = 10.0;

                // Display
                egui::Frame::none()
                    .fill(display_bg)
                    .rounding(12.0)
                    .inner_margin(egui::Margin::symmetric(20.0, 16.0))
                    .show(ui, |ui| {
                        ui.set_min_width(ui.available_width());
                        ui.set_min_height(90.0);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                            ui.colored_label(text_muted, &self.display);
                            ui.add_space(4.0);
                            ui.label(
                                egui::RichText::new(if self.result.is_empty() {
                                    "0"
                                } else {
                                    &self.result
                                })
                                .size(32.0)
                                .color(text_primary),
                            );
                        });
                    });

                ui.add_space(12.0);

                let mut styled_btn = |ui: &mut egui::Ui,
                                      label: &str,
                                      bg: egui::Color32,
                                      fg: egui::Color32|
                 -> bool {
                    let btn = egui::Button::new(egui::RichText::new(label).size(20.0).color(fg))
                        .fill(bg)
                        .rounding(rounding);
                    ui.add_sized(cell, btn).clicked()
                };

                egui::Grid::new("calc_grid")
                    .spacing([gap, gap])
                    .show(ui, |ui| {
                        if styled_btn(ui, "^", fn_bg, fn_fg) {
                            self.display += "^";
                        }
                        if styled_btn(ui, "√", fn_bg, fn_fg) {
                            self.display += "√";
                        }
                        ui.end_row();

                        if styled_btn(ui, "C", clr_bg, clr_fg) {
                            self.display.clear();
                            self.result.clear();
                        }
                        if styled_btn(ui, "(", fn_bg, fn_fg) {
                            self.display += "(";
                        }
                        if styled_btn(ui, ")", fn_bg, fn_fg) {
                            self.display += ")";
                        }
                        if styled_btn(ui, "÷", op_bg, op_fg) {
                            self.display += "/";
                        }
                        ui.end_row();

                        for (label, val) in [("7", "7"), ("8", "8"), ("9", "9")] {
                            if styled_btn(ui, label, num_bg, text_primary) {
                                self.display += val;
                            }
                        }
                        if styled_btn(ui, "×", op_bg, op_fg) {
                            self.display += "*";
                        }
                        ui.end_row();

                        for (label, val) in [("4", "4"), ("5", "5"), ("6", "6")] {
                            if styled_btn(ui, label, num_bg, text_primary) {
                                self.display += val;
                            }
                        }
                        if styled_btn(ui, "+", op_bg, op_fg) {
                            self.display += "+";
                        }
                        ui.end_row();

                        for (label, val) in [("1", "1"), ("2", "2"), ("3", "3")] {
                            if styled_btn(ui, label, num_bg, text_primary) {
                                self.display += val;
                            }
                        }
                        if styled_btn(ui, "−", op_bg, op_fg) {
                            self.display += "-";
                        }

                        ui.end_row();

                        if styled_btn(ui, "0", num_bg, text_primary) {
                            self.display += "0";
                        }
                        if styled_btn(ui, ".", num_bg, text_primary) {
                            self.display += ".";
                        }
                        if styled_btn(ui, "<-", fn_bg, fn_fg) {
                            self.display.pop();
                        }
                        if ui
                            .add_sized(
                                cell,
                                egui::Button::new(egui::RichText::new("=").size(26.0).color(eq_fg))
                                    .fill(eq_bg)
                                    .rounding(rounding),
                            )
                            .clicked()
                        {
                            self.result = Dr3_equation::resolve_evaluation_result_to_string(
                                Dr3_equation::evaluate(&self.display),
                            );
                        }
                        ui.end_row();
                    });
            });
    }
}
