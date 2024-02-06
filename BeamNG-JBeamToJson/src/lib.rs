use mlua::{Error, Lua, Result};
use std::fs::{File};
use std::path::Path;
use zip::ZipArchive;
use std::io::{Read,Write};

#[derive(Debug)]
pub struct JBeamToJson {
	lua: Lua
}

impl JBeamToJson {
	pub fn new() -> Result<JBeamToJson> {
		let lua = Lua::new();
		lua.globals().set("SJSON", "")?;
		lua.globals().set("json_plain", "")?;
		lua.globals().set("json_decoded", "")?;
		lua.load(r#"SJSON = require("lua/sjson")"#).exec()?;
		Ok(JBeamToJson {lua: lua})
	}
	pub fn convert(&self, sjson: &str) -> Result<String> {
		self.lua.globals().set("json_plain", sjson)?;
		self.lua.globals().set("json_decoded", "")?;
		self.lua.load("json_decoded = SJSON.decode(json_plain)").exec()?;
		
		let json_decoded: mlua::Table = self.lua.globals().get("json_decoded")?;
		let json_str = serde_json::to_string_pretty(&json_decoded).map_err(Error::external)?;
		Ok(json_str)
	}
}

#[derive(Debug)]
pub struct GamePath {
	pub files: Vec<String>
}

impl GamePath {
	pub fn new(path: &str) -> Result<GamePath> {
		let files = std::fs::read_dir(&path)?;
		let mut files_vec: Vec<String> = Vec::new();
		for file in files {
			files_vec.push(file.unwrap().path().to_str().unwrap().to_string());
		}
		
		Ok(GamePath {files: files_vec})
	}
}

#[derive(Debug)]
pub struct JBeamExtractor {
	path: String,
	zip_name: String,
	extract_to: String
}

impl JBeamExtractor {
	pub fn new(path: &str, extract_to: &str) -> JBeamExtractor {
		let zip_name = Path::new(path);
		let zip_name = zip_name.file_name().unwrap().to_str().unwrap();
		let extract_to = extract_to.to_owned() + zip_name + "/";

		JBeamExtractor {
			path: path.to_string(),
			zip_name: zip_name.to_string(),
			extract_to: extract_to.to_string(),
		}
	}
	pub fn zip_name(&self) -> &str {
		&self.zip_name
	}
	pub fn extract_and_convert(&self, converter: &JBeamToJson) -> std::result::Result<(), std::io::Error> {
		let archive = File::open(&self.path)?;
		let mut archive = ZipArchive::new(archive)?;
		for idx in 0..archive.len() {
			let mut entry = archive.by_index(idx)?;
			if !entry.is_file() {continue}
			let name = match entry.enclosed_name() {
				Some(v) => v.to_owned(),
				None => continue,
			};
			let complete_ext = match name.extension() {
				Some(v) => v.to_str().unwrap().to_lowercase(),
				None => continue,
			};
			if &complete_ext != "jbeam" {continue}
			
			let new_path = match name.parent() {
				Some(v) => v.to_str().unwrap(),
				None => continue,
			};
			
			let mut content: Vec<u8> = Vec::new();
			let tmp = entry.read_to_end(&mut content);
			if tmp.is_err() {continue}
			
			let content = match std::str::from_utf8(&content) {
				Ok(v) => match converter.convert(&v) {
						Ok(v) => v,
						Err(_) => {
							eprintln!("Cannot convert - {}", name.display());
							continue;
						},
					},
				Err(_) => {
					eprintln!("Cannot read file from zip - {}", name.display());
					continue;
				},
			};
			
			if std::fs::create_dir_all(self.extract_to.to_owned() + new_path).is_err() {
				eprintln!("Cannot create parent folders - {}", self.extract_to.to_owned() + new_path);
				continue;
			}
			let new_filepath = self.extract_to.to_owned() + name.to_str().unwrap();
			let mut file = match File::create(&new_filepath) {
				Ok(v) => v,
				Err(_) => {
					eprintln!("Cannot open file - {}", new_filepath);
					continue;
				},
			};
			if file.write(content.as_bytes()).is_err() {
				eprintln!("Cannot write to file - {}", new_filepath);
			}
		}
		
		Ok(())
	}
}
