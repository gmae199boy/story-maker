#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct StoryWindow {
  pub title: String,
  pub description: String,
  pub content: String,
  pub is_open: bool,

  changed_title: String,
  changed_description: String,
  changed_content: String,
}

impl Default for StoryWindow {
  fn default() -> Self {
    Self {
      is_open: false,
      title: "".to_string(),
      description: "".to_string(),
      content: "".to_string(),

      changed_title: "".to_string(),
      changed_description: "".to_string(),
      changed_content: "".to_string(),
    }
  }
}

impl StoryWindow {
  pub fn init(title: String, desc: String, content: String) -> Self {
    Self {
      is_open: false,
      title: title.clone(),
      description: desc.clone(),
      content: content.clone(),
      changed_title: title,
      changed_description: desc,
      changed_content: content,
    }
  }
  pub fn show(&mut self, ui: &mut egui::Ui) {
    ui.checkbox(&mut self.is_open, &self.title);
  }

  pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::Window::new(&self.title)
      .open(&mut self.is_open)
      .vscroll(true)
      .resizable([true, true])
      .show(ctx, |ui| {
        ui.horizontal(|ui| {
          ui.label("제목:식별자");
          ui.text_edit_singleline(&mut self.changed_title);
        });
        ui.horizontal(|ui| {
          ui.label("설명");
          ui.text_edit_singleline(&mut self.changed_description);
        });
        ui.horizontal(|ui| {
          ui.label("내용");
          ui.text_edit_multiline(&mut self.changed_content);
        });

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
          if ui.button("변경").clicked() {
            self.title = self.changed_title.clone();
            self.description = self.changed_description.clone();
            self.content = self.changed_content.clone();
          }
        });
      });
  }
}

#[derive(serde::Deserialize, serde::Serialize, Clone)]
#[serde(default)]
struct Metadata {}

impl Default for Metadata {
  fn default() -> Self {
    Self {}
  }
}

// impl serde::Serialize for Metadata {
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//   where
//     S: serde::Serializer,
//   {
//     let a = self.attributes.clone();
//     let attributes = a
//       .into_iter()
//       .filter(|v| !v.trait_type.is_empty())
//       .collect::<Vec<Attribute>>();
//     let mut metadata = self.clone();
//     metadata.attributes = attributes;

//     metadata.serialize(serializer)
//   }
// }

// let mut headers = BTreeMap::new();
//   headers.insert(
//     "Content-Type".to_string(),
//     "application/json".to_string(),
// );

// let request = ehttp::Request {
//   method: "POST".to_string(),
//   url: "http://localhost:3001/json".to_string(),
//   headers,
//   body: metadata_json.as_bytes().to_vec(),
// };

// let bytes = metadata_json.as_bytes();

// let uint8arr = web_sys::js_sys::Uint8Array::new(
//   &unsafe { web_sys::js_sys::Uint8Array::view(&bytes) }.into(),
// );
// let array = web_sys::js_sys::Array::new();
// array.push(&uint8arr.buffer());

// let blob = Blob::new_with_str_sequence(&array).unwrap();

// let blob_url = Url::create_object_url_with_blob(&blob).unwrap();
// ui.ctx().open_url(egui::OpenUrl {
//   url: blob_url,
//   new_tab: true,
// });
// ehttp::fetch(request, move |_| {});
