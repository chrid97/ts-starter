use std::fs::{File, self};
use std::io::{self, prelude::*};
use std::path::Path;
use std::process::Command;
use serde::{Serialize, Deserialize};

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, Debug)]
struct TsConfig {
    compilerOptions: Vec<String>,
}

fn main() {
    update_tsconfig();
    // let _ = install_or_update_prettier();
}

fn update_tsconfig() {
    const TSCONFIG_STRICT_RULES: &str = r#"{
    "strict": true,
    "strictNullChecks": true,
    "noImplicitAny": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
}"#;
    let tsconfig_json = fs::read_to_string("tsconfig.json").expect("unable to read json");
    let json = serde_json::from_str::<TsConfig>(&tsconfig_json).expect("wrong");
    println!("{:#?}", json);
    // let mut file = File::create("tsconfig.json").expect("uh oh");
    // file.write_all(TSCONFIG_STRICT_RULES.as_bytes())
    //     .expect("bzzt");

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
    // println!("{}", Path::new(".prettierrc").exists());
    let mut file = File::create(".prettierrc").expect("uh oh");
    file.write_all(PRETTIER_FILE_CONFIG.as_bytes())
        .expect("bzzt");
    Ok(())
}

// update/generate tsconfig and eslint
// create next projects from here
// when generating a next project remove the boilerplate
// and ammend the original commit to say Initial Commit
