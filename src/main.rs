use json_comments::StripComments;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::process::Command;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all="camelCase")]
struct TsConfig {
    compiler_options: HashMap<String, CompilerOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    exclude: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum CompilerOptions {
    HashMap(HashMap<String, Vec<String>>),
    String(String),
    Boolean(bool),
    Vec(Vec<HashMapOrString>),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
enum HashMapOrString {
    String(String),
    HashMap(HashMap<String, String>),
}

fn main() {
    update_tsconfig();
    install_or_update_prettier();
}

fn update_tsconfig() {
    let tsconfig_json = fs::read_to_string("tsconfig.json").expect("Failed to read tsconfig.json");
    let mut buf = String::new();
    StripComments::new(tsconfig_json.as_bytes())
        .read_to_string(&mut buf)
        .expect("Unable to strip comments from tsconfig.json");
    let mut json =
        serde_json::from_str::<TsConfig>(&buf).expect("Failed to deserialize tsconfig.json");
    let _ = &json
        .compiler_options
        .insert(String::from("strict"), CompilerOptions::Boolean(true));
    let _ = &json.compiler_options.insert(
        String::from("strictNullChecks"),
        CompilerOptions::Boolean(true),
    );
    let _ = &json.compiler_options.insert(
        String::from("noUnusedLocals"),
        CompilerOptions::Boolean(true),
    );
    let _ = &json.compiler_options.insert(
        String::from("noUnusedParameters"),
        CompilerOptions::Boolean(true),
    );
    let _ = &json.compiler_options.insert(
        String::from("noFallthroughCasesInSwitch"),
        CompilerOptions::Boolean(true),
    );
    let _ = &json.compiler_options.insert(
        String::from("noImplicitAny"),
        CompilerOptions::Boolean(true),
    );
    let json = serde_json::to_string_pretty(&json).expect("Failed to serialize tsconfig.json");
    fs::write("tsconfig.json", json).expect("Failed to write to tsconfig.json");
}

fn install_or_update_prettier() {
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
    let mut file = File::create(".prettierrc").expect("uh oh");
    file.write_all(PRETTIER_FILE_CONFIG.as_bytes())
        .expect("bzzt");
}

// check if project already exists and if it does prompting if they want to update the
// configuration
// if not prompt the user to create a vite project or next project
// and eslint
// create next projects from here
// when generating a next project remove the boilerplate
// and ammend the original commit to say Initial Commit
