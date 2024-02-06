# Content Basher

## Motivation
This tool was made out of the curiosity to see if modders tend to use game own files in their own. The focus was on texture files, as those are not under a permissive license.

## Default Usage
Command line tool.
- `rust_hasher GameContentPath Extensions FullModPath`
- `eg. rust_hasher C:/BeamNG.drive/content/vehicles dds;jbeam C:/mymod.zip`

1. Will automatically sha256 the game own files with the given extensions.
2. Will then sha256 the given mod files with the given extenstions.
3. Then Bash the generated hashes against each other
4. Write the results to `./results.json`
```json
{
	"0": {
		"base": "Path to the game own zip file",
		"base_file": "Name of the game own file within the zip file",
		"against": "Path to the mod zip file",
		"against_file": "Name of the mod file within the zip file",
		"hash": "sha256 hash"
	},
	"n": {}
}
```

## Innerworkings
Create a Extenstion filter
```rust
let extension_filter = ExtensionFilter::from("dds;json;whatever");
```
Index the game own files
```rust
let mut vanilla_hashes = FileHashes::new();
index_vanilla_content(&mut vanilla_hashes, &extension_filter, game_path);
```
Index the mod files
```rust
// add hashes to the hashstruct.. 
let mut mod_hashes = FileHashes::new();
mod_hashes.hash_from_zip(path_to_mod, &extension_filter).unwrap();
```
Bash and write the results to disk
```rust
// bash
let (results, amount) = BashResults::from(&vanilla_hashes, &mod_hashes);
if amount > 0 {results.save_to_file("results.json")}
```
