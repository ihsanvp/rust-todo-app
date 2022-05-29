use std::process::Command;

fn main() {
    let mut version = "";
    let lines: Vec<String> = std::fs::read_to_string("Cargo.toml")
        .expect("Failed to read input")
        .split("\n")
        .map(|s| s.to_string()) // Convert &str to String
        .collect();

    for line in lines.iter() {
        if line.contains("version") {
            version = line.split("\"").collect::<Vec<&str>>()[1];
        }
    }

    let mut create_tag = Command::new("git")
        .arg("tag")
        .arg(["v", version].concat())
        .spawn()
        .expect("Create tag failed");

    create_tag.wait().expect("failed to wait");

    let mut push = Command::new("git")
        .arg("push")
        .arg("--tags")
        .spawn()
        .expect("Create tag failed");

    push.wait().expect("failed to wait");
}
