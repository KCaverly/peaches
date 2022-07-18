# peaches

## Functionality Ideas

### projects
* Launch a fuzzy finder with a variety of directories
* Once selected, launch a new tmux window with that directory name
* CD to the desired directory and attach to the new tmux window

### ssh
* Infer available SSH Hosts from .ssh/config file
* Once selected, launch a new tmux window with that ssh host name
* SSH into the new server and attach to the new tmux window, prompting for password

### dotfiles
* Git pull latest dotfiles repository, with recurse submodules
* Run Stow to replace/update all existing config files
* Run PackerSync in Nvim to get Updated Nvim Build

### healthcheck
* Run a series of checks against the system to ensure that all commonly accessed tools are available

### config
* Provide a configurable experience, so these details can be managed by a centralized configuration module
