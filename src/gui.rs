use crate::cli::Cli;
use crate::generator;
use eframe::egui;

#[derive(Default)]
pub struct GuiApp {
    length: usize,
    use_digits: bool,
    use_symbols: bool,
    use_uppercase: bool,
    no_similar: bool,
    exclude: String,
    generated_password: String,
}

impl eframe::App for GuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Password Generator");

            ui.add(egui::Slider::new(&mut self.length, 4..=64).text("Length"));
            ui.checkbox(&mut self.use_digits, "Include digits");
            ui.checkbox(&mut self.use_symbols, "Include symbols");
            ui.checkbox(&mut self.use_uppercase, "Include uppercase");
            ui.checkbox(&mut self.no_similar, "Exclude similar characters");
            ui.add(
                egui::TextEdit::singleline(&mut self.exclude)
                    .hint_text("Exclude characters (e.g. abc123)"),
            );

            if ui.button("Generate password").clicked() {
                let cli_config = Cli {
                    length: self.length,
                    digits: self.use_digits,
                    symbols: self.use_symbols,
                    uppercase: self.use_uppercase,
                    no_similar: self.no_similar,
                    exclude: if self.exclude.is_empty() {
                        None
                    } else {
                        Some(self.exclude.clone())
                    },
                };
                self.generated_password = generator::generate(&cli_config);
            }

            if !self.generated_password.is_empty() {
                ui.separator();
                ui.label("Generated password:");
                ui.text_edit_singleline(&mut self.generated_password);
            }
        });
    }
}
