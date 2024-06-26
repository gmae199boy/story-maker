#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod character_relations;
mod create_story;
mod events;
mod story;
mod text_line;
pub use app::StoryMakerApp;
pub use create_story::CreateStoryWindow;
pub use story::StoryWindow;
pub use text_line::TextLine;
