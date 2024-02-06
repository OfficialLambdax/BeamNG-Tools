# Content Basher

Bashes mod contents against the game content to find duplicates of files by hashing select files with sha256.

Tool as is, automatically parses the game defintions and on completion writes those to the disk, which are then reused on next use.

Example Usage of the binary
`rust_hasher GameContentPath Extensions FullModPath`
`eg. rust_hasher C:/BeamNG.drive/content/vehicles dds;jbeam C:/mymod.zip`

#### Internal functionality
```rust
// declare the extensions to extract and hash
let extension_filter = ExtensionFilter::from("dds;json;whatever");

// add hashes to the hashstruct.. 
let mut mod_hashes = FileHashes::new();
mod_hashes.hash_from_zip(path_to_mod_1, &extension_filter).unwrap();
mod_hashes.hash_from_zip(path_to_mod_2, &extension_filter).unwrap();
// ..

let mut vanilla_hashes = FileHashes::new();
index_vanilla_content(&mut vanilla_hashes, &extension_filter, game_path);

// bash
let (results, amount) = BashResults::from(&vanilla_hashes, &mod_hashes);
if amount > 0 {results.save_to_file("results.json")}
```

Note: Im pretty new to rust, pls bare with me (: