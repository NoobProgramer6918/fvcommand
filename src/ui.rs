mod ui_windows;

use eframe::egui;
use egui::Style;
use log::info;
use ui_windows::ui_windows as ui_win;

const WINDOW_WIDTH: f32 = 420.0;
const WINDOW_HEIGHT: f32 = 520.0;
const CARD_MARGIN: f32 = 6.0;
const CARD_RADIUS: f32 = 12.0;
const CLOSE_BUTTON_SIZE: f32 = 22.0;

pub fn main() {
    info!("Ran UI!");
    let viewport = egui::ViewportBuilder::default()
        .with_decorations(false)
        .with_transparent(true)
        .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
        .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
        .with_max_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
        .with_resizable(false);

    let native_options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native(
        "fvcommand",
        native_options,
        Box::new(|cc| Ok(Box::new(EguiApp::new(cc)))),
    )
    .unwrap_or_else(|err| log::error!("Failed to open the window: {err}"));
}

#[derive(Default)]
pub struct EguiApp {}

impl EguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for EguiApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // Fully transparent so only the rounded card is drawn.
        [0.0, 0.0, 0.0, 0.0]
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ui, |ui| {
                let card = ui.max_rect().shrink(CARD_MARGIN);
                ui.painter()
                    .rect_filled(card, CARD_RADIUS, egui::Color32::from_rgb(32, 32, 36));

                // Dragging anywhere on the card moves the window.
                // The close button is added afterwards, so it stays on top.
                let drag = ui
                    .interact(
                        card,
                        ui.scope_id().with("window_drag"),
                        egui::Sense::click_and_drag(),
                    )
                    .on_hover_cursor(egui::CursorIcon::Grab);
                if drag.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }

                let close_rect = egui::Rect::from_min_size(
                    egui::pos2(card.right() - 10.0 - CLOSE_BUTTON_SIZE, card.top() + 10.0),
                    egui::Vec2::splat(CLOSE_BUTTON_SIZE),
                );
                if ui
                    .put(close_rect, egui::Button::new("X").small())
                    .on_hover_text("Close")
                    .clicked()
                {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }

                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    ui.add_space(15.0);
                    ui.vertical(|ui| {
                        ui.add_space(3.0);
                        ui_win::test_win::test_win(ui);
                    })
                })
            });
    }
}
