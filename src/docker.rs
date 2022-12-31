use std::process;

pub fn get_container_names() -> Vec<String> {
    let ls = process::Command::new("docker")
        .args(vec!["ps"])
        .output()
        .unwrap();

    let output = String::from_utf8(ls.stdout);
    let mut container_names: Vec<String> = Vec::new();
    for line in output.expect("IS DOCKER INSTALLED?").split("\n") {
        let name = line.split("   ").last().unwrap().trim();
        if (name.len() > 0) & (name != "NAMES") {
            container_names.push(name.to_string());
        }
    }

    return container_names;
}
