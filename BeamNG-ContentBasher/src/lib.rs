use sha256::digest;
use std::io::{Error,ErrorKind};
use std::fs;
use std::io::{Read,Write};
use std::collections::HashMap;
use json::{object,JsonValue};
use zip::ZipArchive;


// Hash Creator
#[derive(Debug)]
pub struct FileHashes {
	data: HashMap<String, FoundIn>
}

#[derive(Debug)]
pub struct FoundIn {
	origin_zip: String, // eg. atv.zip
	hash_origin: String, // eg. etkc_main_r.data.DDS
}

#[derive(Debug)]
pub struct ExtensionFilter {
	extensions: HashMap<String, bool>,
}


impl FoundIn {
	pub fn zip_origin(&self) -> &str {
		&self.origin_zip
	}
	pub fn hash_origin(&self) -> &str {
		&self.hash_origin
	}
}

impl ExtensionFilter {
	pub fn from(extensions: &str) -> ExtensionFilter {
		let ext: Vec<&str> = extensions.split(";").collect();
		let mut extensions: HashMap<String, bool> = HashMap::new();
		for ext in ext {
			extensions.insert(ext.to_string().to_lowercase(), false);
		}
		ExtensionFilter {extensions: extensions}
	}
}

impl FileHashes {
	pub fn new() -> FileHashes {
		let data: HashMap<String, FoundIn> = HashMap::new();
		FileHashes {data: data}
	}
	pub fn exists(&self, key: &str) -> Option<&FoundIn> {
		if self.data.contains_key(key) {return Some(&self.data[key])}
		None
	}
	pub fn hash_from_zip(&mut self, pathtozip: &str, filter: &ExtensionFilter) -> Result<(), Error> { // adds hashes
		let archive = fs::File::open(&pathtozip)?;
		let mut archive = ZipArchive::new(archive)?;
		for idx in 0..archive.len() {
			let mut entry = archive.by_index(idx)?;
			if !entry.is_file() {continue}
			let name = match entry.enclosed_name() {
				Some(v) => v.to_owned(),
				None => continue,
			};
			let complete_name = match name.file_name() {
				Some(v) => v.to_str().unwrap(),
				None => continue,
			};
			let complete_ext = match name.extension() {
				Some(v) => v.to_str().unwrap().to_lowercase(),
				None => continue,
			};
			if !filter.extensions.contains_key(&complete_ext) {continue}
			
			let mut content: Vec<u8> = Vec::new();
			let tmp = entry.read_to_end(&mut content);
			if tmp.is_err() {continue}
			
			let hash = digest(&content);
			
			let _ = self.data.insert(hash, FoundIn {
				origin_zip: pathtozip.to_string(),
				hash_origin: complete_name.to_string(),
			});
		}
		
		Ok(())
	}
	pub fn load_from_file(&mut self, fromfile: &str) -> Result<(), Error> {
		let json = fs::read_to_string(fromfile)?;
		let json = match json::parse(&json) {
			Ok(v) => v,
			Err(_) => return Err(Error::new(ErrorKind::Other, "")), // todo
		};
		
		for (hash, foundin) in json.entries() {
			self.data.insert(hash.to_string(), FoundIn {
				origin_zip: foundin["origin_zip"].to_string(),
				hash_origin: foundin["hash_origin"].to_string(),
			});
		}
		Ok(())
	}
	pub fn save_to_file(&self, tofile: &str) -> Result<(), Error> {
		let mut json = JsonValue::new_object();
		for (hash, foundin) in &self.data {
			let _ = json.insert(hash, object!{
				origin_zip: foundin.origin_zip.clone(),
				hash_origin: foundin.hash_origin.clone(),
			});
		}
		let mut file = fs::File::create(tofile)?;
		file.write_all(json::stringify(json).as_bytes())?;
		Ok(())
	}
}


// Bashing
#[derive(Debug)]
pub struct BashResults {
	conflicts: Vec<BashConflict>
}

#[derive(Debug)]
pub struct BashConflict {
	base: String, // atv.zip
	base_file: String, // This.dds
	against: String, // SomeMod.zip
	against_file: String, // ThisDdsOfThatMod.dds
	hash: String,
}

impl BashResults {
	pub fn from(base: &FileHashes, against: &FileHashes) -> (BashResults, u32) {
		let mut conflicts: Vec<BashConflict> = Vec::new();
		for (hash, against_foundin) in &against.data {
			if !base.data.contains_key(hash) {continue}
			
			let base_foundin = base.data.get(hash).unwrap();
			conflicts.push(BashConflict {
				base: base_foundin.origin_zip.clone(),
				base_file: base_foundin.hash_origin.clone(),
				against: against_foundin.origin_zip.clone(),
				against_file: against_foundin.hash_origin.clone(),
				hash: hash.to_string()
			});
		}
		
		let size = conflicts.len() as u32;
		(BashResults {conflicts: conflicts}, size)
	}
	pub fn save_to_file(&self, pathtofile: &str) {
		let mut json = JsonValue::new_object();
		let mut index = 0;
		for conflict in &self.conflicts {
			let _ = json.insert(&index.to_string(), object!{
				base: conflict.base.clone(),
				base_file: conflict.base_file.clone(),
				against: conflict.against.clone(),
				against_file: conflict.against_file.clone(),
				hash: conflict.hash.clone(),
			});
			index += 1;
		}
		let mut file = fs::File::create(pathtofile).unwrap();
		file.write_all(json::stringify_pretty(json, 4).as_bytes()).unwrap();
	}
}