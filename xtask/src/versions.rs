use std::path::Path;

use regex::Regex;

use crate::{
    ci,
    errors::{Error, Result},
    out,
};

fn bump_version_in_file(
    file_path: &str,
    version_regex: &str,
    replacement_version: &str,
    once: bool,
) {
    let path = Path::new(file_path);

    // Read the content of the file
    if path.exists() {
        println!("bumping in {file_path}");
        let file_content = std::fs::read_to_string(file_path).expect("read file");

        // Apply regex replacement
        let re = Regex::new(version_regex).expect("Invalid regex");
        if !re.is_match(&file_content) {
            println!("cannot match on {file_path}");
            return;
        }
        let new_content = if once {
            re.replace(&file_content, replacement_version)
        } else {
            re.replace_all(&file_content, replacement_version)
        };

        std::fs::write(path, new_content.to_string()).expect("write file");
    }
}

pub fn bump_version(version: &str) -> Result<()> {
    // XXX run tests with local loco:
    // set LOCO_DEV_MODE_PATH=/<path-to>/projects/loco/
    // and run the loco-new test suite
    //

    // replace main versions
    let version_replacement = format!(r#"version = "{version}""#);
    bump_version_in_file("Cargo.toml", r"(?m)^version.*$", &version_replacement, true);

    bump_version_in_file(
        "loco-gen/Cargo.toml",
        r"(?m)^version.*$",
        &version_replacement,
        true,
    );

    // sync new version to subcrates in main Cargo.toml
    let loco_gen_dep = format!(r#"loco-gen = {{ version = "{version}","#);
    bump_version_in_file("Cargo.toml", r"(?m)^loco-gen [^,]*,", &loco_gen_dep, false);

    // replace the loco new version pointer
    // pub const LOCO_VERSION: &str = "0.13";
    let const_version_replacement = format!(r#"pub const LOCO_VERSION: &str = "{version}";"#);
    bump_version_in_file(
        "loco-new/src/lib.rs",
        r#"(?m)^pub const LOCO_VERSION: &str = "0.13";$"#,
        &const_version_replacement,
        true,
    );

    Ok(())
}
