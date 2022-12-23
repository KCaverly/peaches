use std::process;

pub fn git_pull_dotfiles(location: &str, command: &str) {
    println!("{}", location);

    let mv = process::Command::new("git")
        .args(vec!["pull", "--recurse-submodules"])
        .current_dir(location)
        .status()
        .unwrap();

    if mv.success() {
        let cmds: Vec<&str> = command.split(" ").collect();
        let cmd = process::Command::new(cmds[0])
            .args(&cmds[1..])
            .current_dir(location)
            .status()
            .unwrap();
        println!("{}", cmd.success());
    }

    println!("{}", mv.success());
}
