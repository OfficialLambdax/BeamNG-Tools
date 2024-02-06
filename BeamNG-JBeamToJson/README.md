# BeamNG-JbeamToJson
This tool converts Jbeam files to strict json files using the game own sjson.lua decoder. That maybe usefull if you want to access jbeam files outside of the game, then when the decoder of your choice cannot decode jbeam files.

At default Extracts all JBeam file from the given game installation and puts them to `./extracted`

## Default Usage
`jbeam_to_json.exe "C:/BeamNG.drive/content/vehicles"`

## Extract Jbeams from Zip files and convert them
```rust
// Create converter
let converter = JBeamToJson::new()?;

// Create extractor
let extractor = JBeamExtractor::new(PathToZipFile, PathToExtractTo);

// Extract and convert
exctractor.extract_and_convert(&converter)?;
```

## Convert individual Jbeam data
```rust
// Create converter
let converter = JBeamToJson::new()?;

// Your JbeamData
let jbeam = r#"{"test":123}"#;

// convert
let json = converter.convert(&jbeam);
```

## Note
Im pretty new to Rust