# BeamNG-JbeamToJson

## Motivation
JBeam files are leniant json. So much leniant that i could not find any common strict or leniant json decoder that could successfully decode jbeam's without running into dozens of issues. It turns out that the jbeam json format is of the sjson format. This is a json format that has barely any coverage through libaries. This proposed the issue that it was simply not possible to work with jbeam files outside of the game in a trusted manner.

## Default Usage
Command line tool.
- `jbeam_to_json.exe "C:/BeamNG.drive/content/vehicles"`
Will drop all converted jbeams to the `./extracted` directory in the following tree hirachy.
```
zipname.zip/vehicles/jbmname/*
eg. atv.zip/vehicles/atv/8x8/atv_fenders.jbeam
```

## Script wise
- lib.rs features a jbeam to json decoder which makes uses of the game own sjson.lua.
```rust
let converter = JBeamToJson::new()?;
```
- lib.rs also features a extractor which extracts jbeams from zip files
```rust
let extractor = JBeamExtractor::new(PathToZipFile, PathToExtractTo);
```
- Combined you can extract and automatically have this method convert the jbeams
```rust
exctractor.extract_and_convert(&converter)?;
```
- Or convert sjson on your own
```rust
let jbeam = r#"{"test":123}"#;
let json = converter.convert(&jbeam);
```
