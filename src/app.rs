use egui::{emath::TSTransform, Shape, Vec2};

use crate::{CreateStoryWindow, StoryWindow};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct StoryMakerApp {
  create_story_window: CreateStoryWindow,
  story_window: StoryWindow,
  settings: bool,
}

impl Default for StoryMakerApp {
  fn default() -> Self {
    Self {
      create_story_window: CreateStoryWindow::default(),
      story_window: StoryWindow::default(),
      settings: false,
    }
  }
}

impl StoryMakerApp {
  /// Called once before the first frame.
  pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
    // This is also where you can customize the look and feel of egui using
    // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

    setup_custom_fonts(&cc.egui_ctx);

    // Load previous app state (if any).
    // Note that you must enable the `persistence` feature for this to work.
    if let Some(storage) = cc.storage {
      return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
    }

    Default::default()
  }
}

impl eframe::App for StoryMakerApp {
  /// Called by the frame work to save state before shutdown.
  fn save(&mut self, storage: &mut dyn eframe::Storage) {
    eframe::set_value(storage, eframe::APP_KEY, self);
  }

  /// Called each time the UI needs repainting, which may be many times per second.
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
      // The top panel is often a good place for a menu bar:

      egui::menu::bar(ui, |ui| {
        // NOTE: no File->Quit on web pages!
        let is_web = cfg!(target_arch = "wasm32");
        if !is_web {
          ui.menu_button("File", |ui| {
            if ui.button("Quit").clicked() {
              ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
          });
          ui.add_space(16.0);
        }

        egui::widgets::global_dark_light_mode_buttons(ui);
      });
    });

    egui::SidePanel::left("item_list")
      .resizable(true)
      .default_width(100.0)
      .show(ctx, |ui| {
        ui.heading("Items");

        ui.separator();

        self.create_story_window.show(ui);
        if self.create_story_window.stories.len() > 0 {
          ui.separator();

          ui.collapsing("Stories", |ui| {
            for (_, story) in self.create_story_window.stories.iter_mut() {
              if story.is_open {
                story.update(ctx, _frame);
              }
              if ui.button(story.title.clone()).clicked() {
                story.is_open = !story.is_open;
              }
            }
          });
        }

        ui.add_space(300.0);

        ui.checkbox(&mut self.settings, "🔧 Settings");
      });

    egui::Shape::circle_filled(egui::Pos2 { x: 10., y: 10. }, 10., egui::Color32::RED).transform(
      TSTransform {
        scaling: 10.,
        translation: Vec2 { x: 10., y: 100. },
      },
    );
    self.create_story_window.update(ctx, _frame);

    egui::Window::new("🔧 Settings")
      .open(&mut self.settings)
      .vscroll(true)
      .show(ctx, |ui| {
        ctx.settings_ui(ui);
      });

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        egui::warn_if_debug_build(ui);
      });
    });
  }
}

fn setup_custom_fonts(ctx: &egui::Context) {
  let mut fonts = egui::FontDefinitions::default();

  fonts.font_data.insert(
    "font".to_owned(),
    egui::FontData::from_static(include_bytes!("../assets/font.ttf")),
  );

  fonts
    .families
    .entry(egui::FontFamily::Proportional)
    .or_default()
    .insert(0, "font".to_owned());

  fonts
    .families
    .entry(egui::FontFamily::Monospace)
    .or_default()
    .push("font".to_owned());

  ctx.set_fonts(fonts);
}
