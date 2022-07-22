# peaches

## Functionality Ideas

### Core

#### Tmux Utilities
* Create New Sessions
* Create New Windows
* Create New Panes
* Send Keys to Windows/Panes
* Attach to Session:Window

#### File System Utilities
* Navigate Through Directories

#### Config
* Expose options through a TOML file.

### Plugins

#### `projects`
* Launch a fuzzy finder with a variety of directories
* Once selected, launch a new tmux window with that directory name
* CD to the desired directory and attach to the new tmux window
* Allow for background commands to run on projects started in background pane.

#### `ssh`
* Infer available SSH Hosts from .ssh/config file
* Once selected, launch a new tmux window with that ssh host name
* SSH into the new server and attach to the new tmux window, prompting for password

#### `dotfiles`
* Git pull latest dotfiles repository, with recurse submodules
* Run stow to replace/update all existing config files
* Run PackerSync in Nvim to get Updated Nvim Build

#### `ansible`
* If Git Repository, pull latest version of Repository
* Launch Ansible Playbook
* TUI with Options to Run/Update for A Variety of Different Tags

#### `healthcheck`
* Run a series of checks against the system to ensure that all commonly accessed tools are available

#### `pyenv`
* Available Pyenv Versions
* Tui to Select Change as Needed

#### `poetry`
* Manage Poetry Environments

## Structure

- core/
-- fs.go
-- config.go
-- tmux.go

- plugins/
-- projects/
-- ssh/
-- pyenv/
-- poetry/
-- ansible/

cmd/
-- root.go

tui/
-- ...
