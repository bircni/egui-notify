use eframe::{
    egui::{Context, Window},
    App, Frame, NativeOptions,
};
use egui::{Style, Visuals};

struct ExampleApp {
    dark: bool,
}

impl App for ExampleApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        Window::new("Controls").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Success").clicked() {
                    egui_notify::toast!(Success, "Success");
                }

                if ui.button("Info").clicked() {
                    egui_notify::toast!(Info, "Info");
                }

                if ui.button("Warning").clicked() {
                    egui_notify::toast!(Warning, "Warning");
                }

                if ui.button("Error").clicked() {
                    egui_notify::toast!(Error, "Error");
                }

                if ui.button("Basic").clicked() {
                    egui_notify::toast!(Basic, "Basic");
                }
            });

            ui.separator();

            if ui.radio(self.dark, "Toggle dark theme").clicked() {
                self.dark = !self.dark;

                let mut style = ctx.style().as_ref().clone();
                if self.dark {
                    style.visuals = Visuals::dark();
                } else {
                    style.visuals = Visuals::light();
                }
                ctx.set_style(style);
            }
        });

        egui_notify::r#macro::TOASTS.write().show(ctx, None);
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "macros-example",
        NativeOptions::default(),
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style::default());

            Ok(Box::new(ExampleApp { dark: true }))
        }),
    )
}
