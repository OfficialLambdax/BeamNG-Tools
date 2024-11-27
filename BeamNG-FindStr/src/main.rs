use std::collections::HashMap;
use std::env;
use winreg::enums::*;
use winreg::RegKey;
use std::fs::File;
use zip::ZipArchive;
use std::io::Read;

use find_str::*;

fn main() {
	colored::control::set_virtual_terminal(true).unwrap(); // windows my ass
	
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {println!("Usage: find_str.exe SearchString extension1,extension2(Optional)"); return}
	
	// specify the string to search for
	let search_for = &args[1].to_lowercase();
	if search_for.len() == 0 {println!("Search string is to short"); return}
	
	// specify the valid extensions
	let extensions_str;
	if args.len() < 3 {
		extensions_str = "lua,js,jbeam";
	} else {
		extensions_str = &args[2];
	}
	let extensions_str = extensions_str.to_lowercase();
	
	// specify path to the games main directory
	let hkcu = RegKey::predef(HKEY_CURRENT_USER);
	let beamng = hkcu.open_subkey("SOFTWARE\\BeamNG\\BeamNG.drive").expect("Cannot find HKCU/Software/BeamNG/BeamNG.drive. Game not installed?");
	let game_path: String = beamng.get_value("rootpath").expect("Cannot find rootpath within HKCU/Software/BeamNG/BeamNG.drive. Game not installed?");
	let game_path = &game_path;
	
	
	// program start
	println!("Only parsing files with these extensions - {}", extensions_str);
	let extensions_vec: Vec<&str> = extensions_str.split(",").collect();
	let mut extensions_map: HashMap<&str, bool> = HashMap::new();
	for extension in extensions_vec {
		let _ = extensions_map.insert(extension, true);
	}
	
	let game_files = recurse_files(game_path).unwrap();
	println!("Searching through loose files");
	for file in &game_files {
		let file_extension = match file.extension() {
			Some(v) => v.to_str().unwrap().to_lowercase(),
			None => continue,
		};
		
		if extensions_map.contains_key(&file_extension.as_str()) { // if typical file
			let file_contents = match std::fs::read_to_string(&file) {
				Ok(v) => v,
				Err(_) => continue,
			};
			
			let displayed_path = &file.to_str().unwrap()[game_path.len()..];
			FindStr::new(search_for, &file_contents, displayed_path).display_colored(15);
			
		}
	}
	
	// look up files inside zip archives last
	println!("Searching through files in zips");
	for file in game_files {
		let file_extension = match file.extension() {
			Some(v) => v.to_str().unwrap().to_lowercase(),
			None => continue,
		};
		
		if file_extension.to_lowercase() == "zip" { // if zip archive
			let archive = match File::open(file.clone()) {
				Ok(v) => v,
				Err(_) => continue,
			};
			let mut archive = match ZipArchive::new(archive) {
				Ok(v) => v,
				Err(_) => continue,
			};
			for idx in 0..archive.len() {
				let mut entry = match archive.by_index(idx) {
					Ok(v) => v,
					Err(_) => continue,
				};
				if !entry.is_file() {continue}
				let name = match entry.enclosed_name() {
					Some(v) => v.to_owned(),
					None => continue,
				};
				let complete_ext = match name.extension() {
					Some(v) => v.to_str().unwrap().to_lowercase(),
					None => continue,
				};
				if !extensions_map.contains_key(&complete_ext.as_str()) {continue}
				
				// read file from zip
				let mut content: Vec<u8> = Vec::new();
				let tmp = entry.read_to_end(&mut content);
				if tmp.is_err() {continue}
				
				let content = match std::str::from_utf8(&content) {
					Ok(v) => v,
					Err(_) => continue,
				};
				
				let displayed_path = file.to_str().unwrap()[game_path.len()..].to_string() + "/" + name.to_str().unwrap();
				FindStr::new(search_for, &content, &displayed_path).display_colored(15);
			}
		}
	}
}


fn recurse_files(path: impl AsRef<std::path::Path>) -> std::io::Result<Vec<std::path::PathBuf>> {
	let mut buf = vec![];
	let entries = std::fs::read_dir(path)?;

	for entry in entries {
		let entry = entry?;
		let meta = entry.metadata()?;

		if meta.is_dir() {
			let mut subdir = recurse_files(entry.path())?;
			buf.append(&mut subdir);
		}

		if meta.is_file() {
			buf.push(entry.path());
		}
	}

	Ok(buf)
}