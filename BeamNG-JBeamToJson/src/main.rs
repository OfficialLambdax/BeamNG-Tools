use std::env;
use mlua::{Result};
use jbeam_to_json::*;

fn main() -> Result<()> {
	/*
		Create converter
		let converter = JBeamToJson::new()?;
		
		Create extractor
		let extractor = JBeamExtractor::new(PathToZipFile, PathToExtractTo);
		
		Exctract and convert
		exctractor.extract_and_convert(&converter)?;
		
	*/
	
	let args: Vec<String> = env::args().collect();
	if args.len() < 2 {
		println!("Usage: PathToContentVehiclesDirectory");
		println!("eg: C:/BeamNG.drive/content/vehicles");
		return Err(mlua::Error::external("Not enough arguments"))
	}
	
	let path_togame = &args[1];
	let path_toextraction = "extracted/";
	
	let converter = JBeamToJson::new()?;
	let gamepath = GamePath::new(path_togame)?;
	
	for file in gamepath.files {
		let extractor = JBeamExtractor::new(&file, path_toextraction);
		println!("Extracting and converting - {}", extractor.zip_name());
		
		extractor.extract_and_convert(&converter)?;
	}
	
	Ok(())
}

