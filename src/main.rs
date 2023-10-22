use std::fs::File;
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::Command;

fn main() {
    let _ = install_or_update_prettier();
}

fn install_or_update_prettier() -> std::io::Result<()> {
    const PRETTIER_FILE_CONFIG: &str = r#"{
    "semi": true,
    "singleQuote": true,
    "trailingComma": "all",
    "arrowParens": "always",
    "tabWidth": 4
}"#;
    let output = Command::new("npm")
        .arg("install")
        .arg("--save-dev")
        .arg("--save-exact")
        .arg("prettier")
        .output()
        .expect("Didn't work");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
    println!("{}", Path::new(".prettierrc").exists());
    let mut file = File::create(".prettierrc").expect("uh oh");
    file.write_all(PRETTIER_FILE_CONFIG.as_bytes())
        .expect("bzzt");
    Ok(())
}

// update/generate tsconfig and eslint
// create next projects from here
// when generating a next project remove the boilerplate
// and ammend the original commit to say Initial Commit
