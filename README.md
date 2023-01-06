# peaches

A smart switcher for the terminal.

## Why?

I spend a disproportionate amount of time in the terminal on navigation. This is often low value add, repetitive work, annoying to manage manually and easily automated. `peaches` solves for this, by offering an extendable smart switcher for the terminal. Leveraging commonly loved linux tools such as `tmux`, `peaches` offers a fuzzy find experience for navigating between projects, connecting to ssh servers, connecting to docker containers, and launching common terminal applications. 

## Installation

peaches is built to run on linux primarily.

```sh
wget https://raw.githubusercontent.com/KCaverly/peaches/main/install.sh -O - | sh
```

## Upgrade

To upgrade peaches to the latest version, please run `upgrade`.

```sh
peaches upgrade
```

This command will grab the latest install script from github, and update the version installed.

## Usage

### `dirs`

**Launches a fuzzy finder for common directories, cds to the chosen directory in a new tmux window.**

TODO: Add GIF showing the primary functionality.

This functionality is based around quick switching within a directory organized project structure. 
Across my machines, I have both a 'personal' and a 'work' folder in which, I keep a series of git repositories associated with various projects.
Provided with a directory, a minimum and maximum depth, all child directories are discovered and provided in a fuzzy finder to select between.
Once selected, a new tmux window is created (or switched to, if already created), in the target directory.
This provides an intuitive tab like interface for quick switching between various common projects.

#### For example...

Assume you have a series of projects held within the 'personal' folder, as shown below:

```
/home/user/personal/
-- /home/user/personal/project1/
-- /home/user/personal/project2/
-- /home/user/personal/project3/
-- /home/user/personal/.hidden_dir/
```

And your provided '.peaches' config file, includes the following:

```toml
[directories]

  [directories.personal]
  directory = "/home/user/personal/"
  min_depth = 1
  max_depth = 1
  include_hidden = false
  session_name = "projects"

```

A fuzzy finder will be generated with the following options:

```
/home/user/personal/project1/
/home/user/personal/project2/
/home/user/personal/project3/
```

Lets say we then select 'project1'. A new tmux window named 'project1' would then be created inside a session named 'projects', with the active directory set to the directory chosen. If a tmux window with that session & name is already active, this tmux window will then be attached (switched) to.

### `ssh`

**Launches a fuzzy finder for provided ssh server details, and connects to the ssh server in a new tmux window.**

TODO: Add GIF showing the primary functionality.

Especially, in an environment of remote work, you may find yourself frequently sshing into a variety of machines. Provided with ssh details and an authentication method, this command will launch a fuzzy finder for the ssh servers, and connect to the selected server inside a new tmux window.

#### For example, 

Let's presume you frequently connect to two hosts 'prod-server' and 'dev-server' via ssh.

ie. ```ssh user@prod-server``` or ```ssh user@dev-server```

You can define the two servers inside your '.peaches' config file, like so:

```toml
[ssh]
session_name = 'ssh'

  [ssh.prod]
  host = 'prod-server'
  username = 'prod_user'
  password = 'crypt:<encrypted password>'
  auth_method = 'password'

  [ssh.dev]
  host = 'dev-server'
  username = 'dev_user'
  auth_method = 'ssh_key'

  [ssh.dev_admin]
  host = 'dev-server'
  username = 'admin'
  password = 'password1234'
  auth_method = 'password'

```

A fuzzy finder will be generated with the following options:

```
prod: prod_user@prod-server
dev: dev_user@dev-server
dev_admin: admin@dev-server
```

While the configuration suggests different authentication methods, the outcome is largely the same. Lets assume 'prod:...' is selected. In this scenario, a new tmux window will be created named 'prod' (the ssh alias provided) inside a session named 'ssh', with the server actively connected. Similar to the `dirs` command above, if a tmux window with that session & name is already active, this tmux window will then be attached (switched) to.

#### Authentication Configuration

You may notice a few different configuration options for SSH in the example above. The primary decision point is surrounding 'auth_method'.

There are two 'auth_methods' available, 'password' and 'ssh_key'. If you have access to the SSH server directly, I would encourage the 'ssh_key' method for simplicity and security. Simply provide the ssh server with your ssh_key as normal, and whenever selected the ssh server will leverage the ssh key in .ssh to connect.

However, if you do not have access to managing ssh public keys on the ssh server itself, a 'password' option is available. Within the password option, 'peaches' accomodates for both clear-text and encrypted passwords. For more details on encryption, see **HERE**. At a high-level, if encryption is chosen and a correct encrypted password is provided, this password will only be read and decrypted at connection time, and never stored in memory.

