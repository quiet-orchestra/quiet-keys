use egui::{Color32, Pos2, Rect, Stroke, StrokeKind, Vec2, epaint::CornerRadiusF32, vec2};

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

    // fn logic(&mut self, ctx: &Context, frame: &mut Frame){
    //     let (x, y ) = ctx
    //     self.window_size = vec2(x, y)
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
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

            ui.separator();

            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

        });

        egui::CentralPanel::default().show_inside(ui, |ui| {

            draw_keyboard(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
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
        let standard_width = Pos2::new(window.width() / 16.0, 0.).to_vec2();
        let standard_height = Pos2::new(0., window.height() / 16.0).to_vec2();

        for row in 0..6 {
            for col in 0..16 {
                let (
                    rect,
                    corner_radius, 
                    stroke, 
                    stroke_kind
                ) = (
                    Rect::from_min_size(
                        origin + standard_width * col as f32  + standard_height * row as f32, 
                        Vec2::new(standard_width.x * 0.9, standard_height.y * 0.9)
                    ),
                    0.5,
                    Stroke::new(2., Color32::WHITE.blend(Color32::GRAY)),
                    StrokeKind::Middle,
                );
                painter.rect_stroke(rect, corner_radius, stroke, stroke_kind);
            }
        }
        
    }
    else {
        ui.label("Window too small");
    }
    
}
