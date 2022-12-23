use std::process;

pub fn git_pull_dotfiles(location: &str) {
    println!("{}", location);

    let mv = process::Command::new("git")
        .args(vec!["pull", "--recurse-submodules"])
        .current_dir(location)
        .status()
        .unwrap();

    if mv.success() {
        let cmd = process::Command::new("zsh")
            .args(vec!["install"])
            .current_dir(location)
            .status()
            .unwrap();
        println!("{}", cmd.success());
    }

    println!("{}", mv.success());
}
