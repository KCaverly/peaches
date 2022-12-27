use std::process;

pub fn get_container_names() {
    let ls = process::Command::new("docker")
        .args(vec!["ps", "-a"])
        .output()
        .unwrap();

    let output = String::from_utf8(ls.stdout);
    let container_names = Vec<String>;
    for line in output.expect("IS DOCKER INSTALLED?").split("\n") {
        println!("LINE: {:#?}", line);
        let name = line.split("   ").last().unwrap().trim();
        if (name.len() > 0) & (name != "NAMES") {
            println!("NAME: {:#?}", name)
        }
    }
}
