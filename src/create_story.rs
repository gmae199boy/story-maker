use std::collections::HashMap;

use crate::{story, StoryWindow};

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct CreateStoryWindow {
  pub is_open: bool,
  pub stories: HashMap<String, StoryWindow>,

  pub title: String,
  pub description: String,
  pub content: String,
}

impl Default for CreateStoryWindow {
  fn default() -> Self {
    Self {
      is_open: false,
      stories: HashMap::new(),
      title: "".to_string(),
      description: "".to_string(),
      content: "".to_string(),
    }
  }
}

impl CreateStoryWindow {
  pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.checkbox(&mut self.is_open, "스토리 생성");
  }

  pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::Window::new("스토리 생성")
      .open(&mut self.is_open)
      .vscroll(true)
      .resizable([true, true])
      .show(ctx, |ui| {
        ui.label("스토리를 생성해보세요");
        ui.horizontal(|ui| {
          ui.label("제목:식별자");
          ui.text_edit_singleline(&mut self.title);
        });
        ui.horizontal(|ui| {
          ui.label("설명");
          ui.text_edit_singleline(&mut self.description);
        });
        ui.horizontal(|ui| {
          ui.label("내용");
          ui.text_edit_multiline(&mut self.content);
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
          if ui.button("생성").clicked() {
            let story = StoryWindow::init(
              self.title.clone(),
              self.description.clone(),
              self.content.clone(),
            );
            // let story = StoryWindow {
            //   title: self.title.clone(),
            //   description: self.description.clone(),
            //   content: self.content.clone(),
            //   is_open: true,
            // };
            self.stories.insert(self.title.clone(), story);

            self.title.clear();
            self.description.clear();
            self.content.clear();
          }
        });
      });
  }
}
