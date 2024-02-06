use std::path::Path;
use std::env;
use rust_hasher::{FileHashes,ExtensionFilter,BashResults};

const PATH_TO_GAMEDEFS: &str = "vanilla_hashes.json";

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() < 4 {
		println!("Usage: GameContentPath Extensions FullModPath");
		println!("eg: C:/BeamNG.drive/content/vehicles dds;jbeam C:/mymod.zip");
		return
	}
	let game_path = &args[1];
	let extension_filter = ExtensionFilter::from(&args[2]);
	let mod_path = &args[3];
	
	let mut vanilla_hashes = FileHashes::new();
	if Path::new(PATH_TO_GAMEDEFS).exists() {
		println!("Loading known hashes");
		vanilla_hashes.load_from_file(PATH_TO_GAMEDEFS).unwrap();
	} else {
		println!("Indexing vanilla game content");
		index_vanilla_content(&mut vanilla_hashes, &extension_filter, game_path);
		vanilla_hashes.save_to_file(PATH_TO_GAMEDEFS).unwrap();
	}
	
	println!("Learning - {}", mod_path);
	let mut mod_hashes = FileHashes::new();
	mod_hashes.hash_from_zip(mod_path, &extension_filter).unwrap();
	println!("Bashing");
	let (results, amount) = BashResults::from(&vanilla_hashes, &mod_hashes);
	if amount == 0 {return}
	println!("{} Hits", amount);
	results.save_to_file("results.json");
}

fn index_vanilla_content(hashes: &mut FileHashes, filter: &ExtensionFilter, gamepath: &str) {
	let files = match std::fs::read_dir(gamepath) {
		Ok(v) => v,
		Err(_) => panic!("No files in the defined game content directory"),
	};
	
	// this may panic
	for file in files {
		let file = file.unwrap().path();
		if !file.is_file() {continue}
		if file.extension().unwrap().to_str().unwrap().to_lowercase() != "zip" {continue}
		
		println!("Indexing - {}", file.file_name().unwrap().to_str().unwrap());
		hashes.hash_from_zip(file.to_str().unwrap(), &filter).unwrap();
	}
}