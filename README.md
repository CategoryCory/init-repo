# repo-man

## About the project

A simple CLI tool for managing remote Git repositories. Ideal for self-hosted 
Git workflows over SSH.

Please note that currently, **repo-man** only supports Unix/Linux targets. Support
for Windows will be added in a future release.

## Features

- Initializes **bare Git repos** with correct `HEAD` configuration
- Configurable **base directory** and **default branch name**
- Supports **multiple remote host profiles**
- Easily installable with `cargo install`
- Configurable through CLI command

## Installation

Before beginning, ensure that you have [Rust](https://rust-lang.org) installed.

1. Clone this repo to your development machine
   ```sh
   git clone https://github.com/CategoryCory/repo-man.git
   ```
2. In the directory where you cloned the repo, build the project
   ```sh
   cargo build --release
   ```
3. Install the crate
   ```sh
   cargo install --path .
   ```
4. Run initial configuration
   ```sh
   repo-man configure
   ```
   The configuration wizard will walk you through several steps:
   - Enter config profile name: This application supports multiple remote host profiles. If you enter the name of
     an existing profile, you can modify its settings; otherwise, a new profile will be created with the name specified.
   - Host domain name or IP address: The remote host IP address or domain name
   - Repository base directory: The base directory for all bare repos on the remote host. Must be an **absolute path.**
   - Default Git branch: The default branch name to use for new Git repos.
   - SSH key path: The path to the SSH key to use for this remote host (optional).
   This config is stored at `~/.repo-man/config.toml`.
   You can run the config again at any time.

## Usage

### Create a new repository

```sh
repo-man new my-project --host host-profile-name
```

This will:
- Create `/home/git/repos/my-project.git` (or your configured base directory) on host `host-profile-name`
- Initialize a bare Git repo
- Set `HEAD` to `refs/heads/master` (or your configured default branch)

You can also override options per command:

```sh
repo-man new my-project --base-dir /tmp/test --default-branch main
```

### Run configuration

```sh
repo-man configure
```

### Show help

```sh
repo-man -h, --help
```

## License

MIT

## Contributions welcome!

Please feel free to fork, contribute, or submit feature ideas.
