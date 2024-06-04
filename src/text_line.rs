pub struct TextLine {
  pub direction: egui::Direction,
  pub align: egui::Align,
  pub wrap: bool,
  pub justify: bool,
}

impl Default for TextLine {
  fn default() -> Self {
    Self {
      direction: egui::Direction::TopDown,
      align: egui::Align::Center,
      wrap: false,
      justify: true,
    }
  }
}

impl TextLine {
  pub fn single(self, ui: &mut egui::Ui, text: &mut String) {
    ui.with_layout(
      egui::Layout::from_main_dir_and_cross_align(self.direction, self.align)
        .with_main_wrap(self.wrap)
        .with_cross_justify(self.justify),
      |ui| {
        ui.text_edit_singleline(text);
      },
    );
  }

  pub fn multi(self, ui: &mut egui::Ui, text: &mut String) {
    ui.with_layout(
      egui::Layout::from_main_dir_and_cross_align(self.direction, self.align)
        .with_main_wrap(self.wrap)
        .with_cross_justify(self.justify),
      |ui| {
        ui.text_edit_multiline(text);
      },
    );
  }
}
// ui.with_layout(
//   egui::Layout::from_main_dir_and_cross_align(
//     egui::Direction::TopDown,
//     egui::Align::Center,
//   )
//   .with_main_wrap(false)
//   .with_cross_justify(true),
