use std::path::PathBuf;

use homedir::my_home;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preference {
	width: u32,
	start_at_login: bool,
	hide_window_when_not_focus: bool,
	always_on_top: bool,
	show_short_cut: String,
	ai_url: String
}

impl Preference {
	pub fn new() -> Self {
		Self {
			width: 650,
			start_at_login: false,
			hide_window_when_not_focus: false,
			always_on_top: false,
			show_short_cut: String::from("Ctrl+Shift+P"),
			ai_url: String::from("https://copilot.microsoft.com/")
		}
	}

	fn get_save_path() -> PathBuf {
		let mut home = my_home().unwrap().unwrap();
		home.push(".config");
		home.push("com.iewnfod.copilot-desktop");
		home.push("preference.json");
		home
	}

	pub fn get_width(&self) -> u32 {
		self.width
	}

	pub fn get_start_at_login(&self) -> bool {
		self.start_at_login
	}

	pub fn get_hide_window_when_not_focus(&self) -> bool {
		self.hide_window_when_not_focus
	}

	pub fn get_show_short_cut(&self) -> String {
		self.show_short_cut.clone()
	}

	pub fn get_always_on_top(&self) -> bool {
		self.always_on_top
	}

	pub fn get_ai_url(&self) -> String {
		self.ai_url.clone()
	}

	pub fn set_width(&mut self, width: u32) {
		self.width = width;
	}

	pub fn from_saved() -> Option<Self> {
		let file_path = Self::get_save_path();
		if let Ok(content) = std::fs::read_to_string(file_path) {
			if let Ok(pre) = serde_json::from_str::<Self>(&content.as_str()) {
				Some(pre)
			} else {
				None
			}
		} else {
			None
		}
	}

	pub fn save(&self) {
		let file_path = Self::get_save_path();
		if let Ok(content) = serde_json::to_string(&self) {
			let parent = file_path.parent().unwrap();
			if !parent.exists() {
				std::fs::create_dir_all(parent).unwrap();
			}
			std::fs::write(file_path, content).unwrap();
		}
	}
}

impl Default for Preference {
	fn default() -> Self {
		if let Some(pre) = Self::from_saved() {
			pre
		} else {
			Self::new()
		}
	}
}
