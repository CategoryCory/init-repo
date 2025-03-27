# init-repo

## About the project

A simple CLI tool for initializing bare Git repositories. Ideal for self-hosted 
Git workflows over SSH.

## Features

- Initializes **bare Git repos** with correct `HEAD` configuration
- Configurable **base directory** and **default branch name**
- Easily installable with `cargo install`
- Configurable through CLI command

## Installation

Before beginning, ensure that you have [Rust](https://rust-lang.org) installed.

1. Clone this repo to your self-hosted Git server
   ```sh
   git clone https://github.com/CategoryCory/init-repo.git
   ```
1. In the directory where you cloned the repo, build the project
   ```sh
   cargo build --release
   ```
1. Install the crate
   ```sh
   cargo install --path .
   ```
1. Run initial configuration
   ```sh
   init-repo configure
   ```
   You will be prompted to set a default base directory (e.g. `/home/git/repos`).
   This config is stored at `~/.init-repo/config.toml`.
   You can run the config again at any time.

## Usage

### Create a new repository

```sh
init-repo new my-project
```

This will:
- Create `/home/git/repos/my-project.git` (or your configured base directory)
- Initialize a bare Git repo
- Set `HEAD` to `refs/heads/master` (or your configured default branch)

You can also override options per command:

```sh
init-repo new my-project --base-dir /tmp/test --default-branch main
```

### Run configuration

```sh
init-repo configure
```

### Show help

```sh
init-repo -h, --help
```

## License

MIT

## Contributions welcome!

Please feel free to fork, contribute, or submit feature ideas.
