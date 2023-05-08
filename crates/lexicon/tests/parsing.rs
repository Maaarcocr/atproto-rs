use std::{fs::File, io::Read, path::Path};

use walkdir::WalkDir;

#[test]
fn can_parse_all_lexicons() {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let dir_path = Path::new(manifest_dir).join("../../atproto/lexicons");
    for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            if file_path.extension().unwrap() != "json" {
                continue;
            }
            let mut file = File::open(file_path).expect("Unable to open file");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Unable to read file");
            serde_json::from_str::<atproto_lexicon::LexiconDoc>(&contents)
                .expect("Unable to parse lexicon");
        }
    }
}
