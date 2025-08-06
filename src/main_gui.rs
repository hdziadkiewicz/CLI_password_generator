use eframe::NativeOptions;
use password_gen::gui::GuiApp;

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Password Generator GUI",
        options,
        Box::new(|_cc| Box::new(GuiApp::default())),
    )
}
