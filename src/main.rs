use std::fs::File;
use std::io::prelude::*;

fn main() {
    let _ = install_or_update_prettier();
}

fn install_or_update_prettier() -> std::io::Result<()> {
    // check if prettier is installed in package.json
    // then check if prettier file exists
    // const INSTALL_PRETTIER_COMMAND: &str = "npm install --save-dev --save-exact prettier";
    const PRETTIER_FILE_CONFIG: &str = r#"{
    "semi": true,
    "singleQuote": true,
    "trailingComma": "all",
    "arrowParens": "always",
    "tabWidth": 4
}"#;
    let mut file = File::create(".prettierrc").expect("uh oh");
    file.write_all(PRETTIER_FILE_CONFIG.as_bytes()).expect("bzzt");
    Ok(())
}

// update/generate tsconfig and eslint
// create next projects from here
// when generating a next project remove the boilerplate
// and ammend the original commit to say Initial Commit
