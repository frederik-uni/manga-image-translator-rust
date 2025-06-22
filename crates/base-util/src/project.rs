use std::{env, fs, path::PathBuf};

pub fn root_path() -> PathBuf {
    get_workspace_root()
        .or(std::env::current_dir().ok())
        .unwrap_or(PathBuf::from("./"))
}

fn get_workspace_root() -> Option<PathBuf> {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").ok()?);

    loop {
        let manifest_path = dir.join("Cargo.toml");
        if manifest_path.exists() {
            if let Ok(contents) = fs::read_to_string(&manifest_path) {
                if contents.contains("[workspace]") {
                    return Some(dir);
                }
            }
        }

        if !dir.pop() {
            break;
        }
    }

    None
}
