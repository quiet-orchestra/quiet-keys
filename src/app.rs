use std::fs;

use egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Vec2, epaint::CornerRadiusF32, vec2};
use kle_serial::{self, Keyboard};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct QuietKeysApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

}

impl Default for QuietKeysApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl QuietKeysApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for QuietKeysApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);

                
            });

        });

        egui::Panel::bottom("bottom_panel").show_inside(ui, |ui|{
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {

            draw_keyboard(ui);

        });


    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

fn draw_keyboard(ui: &mut egui::Ui){
    let painter = ui.painter();
    let window = ui.max_rect();

    if window.width() > 120.0 && window.height() > 120.0 {
        let origin = window.left_top();

        let kle_json = match fs::read_to_string("src/ansi_104.json") {
            Ok(text) => text,
            Err(_) => {
                ui.label("Couldn't find json file");
                return;
            }
        };


        let keyboard: Keyboard = match serde_json::from_str(&kle_json) {
            Ok(parsed_text) => parsed_text,
            Err(_) => {
                ui.label("Failed to parse json text");
                return;
            },
        };

        let keyboard_rect = get_rect_from_keyboard(keyboard.clone());
        let unit_width = Pos2::new(window.width() / keyboard_rect.width(), 0.).to_vec2();
        let unit_height = Pos2::new(0., window.height() / keyboard_rect.height()).to_vec2();

        for key in keyboard.keys{

            let (
                rect,
                corner_radius, 
                stroke, 
                stroke_kind
            ) = (
                Rect::from_min_size(
                    origin + unit_width * key.x as f32  + unit_height * key.y as f32, 
                    Vec2::new(unit_width.x * 0.95 * key.width as f32, unit_height.y * 0.95 * key.height as f32)
                ),
                0.5,
                Stroke::new(2., Color32::WHITE.blend(Color32::GRAY)),
                StrokeKind::Middle,
            );
            painter.rect_stroke(rect, corner_radius, stroke, stroke_kind);
    
        }
        
    }
    else {
        ui.label("Window too small");
    }
    
}

fn get_rect_from_keyboard(keyboard: Keyboard) -> Rect{
    let (mut x, mut y) = (0.,0.);
    for key in keyboard.keys {
        let key_right = (key.x + key.width) as f32;
        let key_bottom = (key.y + key.height) as f32;

        if key_right > x {
            x = key_right;
        }
        if key_bottom > y {
            y = key_bottom;
        }
    }

    return Rect { min: Pos2::ZERO, max: Pos2 { x, y } };
}