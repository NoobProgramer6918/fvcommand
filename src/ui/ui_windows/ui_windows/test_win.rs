use eframe::egui;
use egui::Ui;

pub fn test_win(ui: &mut egui::Ui) {
    ui.heading("Test");
    let _ = ui.button("Test");

    // Stable id derived from the parent ui, captured before the horizontal.
    let id = Ui::id(ui).with("test_text");
    ui.separator();
    ui.horizontal(|ui| {
        let mut test_text: String = ui.data(|d| d.get_temp(id)).unwrap_or_default();
        if ui.text_edit_singleline(&mut test_text).changed() {
            ui.data_mut(|d| d.insert_temp(id, test_text));
        }
        let _ = ui.button("Submit");
    });
}
