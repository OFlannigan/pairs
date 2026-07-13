# PaiRS

This is a command line tool, which aims to simplify pair programming.
It was hugely inspired by [rstash](https://github.com/otto-de/rstash) and [mob](https://github.com/remotemobprogramming/mob/).

![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/OFlannigan/pairs/ci.yml?branch=main&label=build)
[![GitHub License](https://img.shields.io/github/license/OFlannigan/pairs)](https://opensource.org/license/mit)
![Dynamic TOML Badge](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2FOFlannigan%2Fpairs%2Frefs%2Fheads%2Fmain%2FCargo.toml&query=package.version&label=version)


---

## Installation

### Building from source

```bash
# Install cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone project
git clone https://github.com/OFlannigan/pairs.git

# Switch to project folder
cd pairs

# Build in release mode
cargo build --release

# Put binary somewhere in your PATH, e.g. /usr/local/bin
mv ./target/release/pairs /usr/local/bin/pairs

# Make binary executable
chmod +x /usr/local/bin/pairs
```

### Other ways of installation

Currently, there are no other ways to install `pairs`.
It is planned to include at least installation via [crates.io](https://crates.io/) and an installation script but both have not yet been implemented.

## Usage

The intention of pairs is to simplify pair programming by providing a simple command line interface to manage the process of switching between driver and navigator roles.
All that while leveraging the Rust type system and speed.
As of right now, there are three operations available:

- [Stashing your changes remotely](#stashing-changes)
- [Listing all remote stashes](#listing-remote-stashes)
- [Popping a remote stash](#popping-a-remote-stash)

### Stashing changes

If you want to hand over your current work to your pair-programmer without creating a commit or if you want to save your work for later, you can simply invoke `pairs` as this is the default behavior.
What will this do?
`pairs` will check your working tree for any uncommited changes.
If there are any, it will create a branch with a randomized pin as suffix for easier identification.
It will then commit all your changes to that branch and push them to the remote specified as `origin` in your project settings.
You will then be prompted on whether the local changes should be discarded as well.

### Listing remote stashes

When you want to see all branches, which are affiliated to `pairs`, you can do so by invoking `pairs` with the `list` command: `pairs list`.
This will check the remote origin for any branches and return a list of all branches matching the naming scheme along with its respective pin, the author, and when it was created.
`pairs` will also tell you when there are no branches matching the expected pattern for your project.

### Popping a remote stash

There are two ways how you can apply changes from a remote branch locally.

- Executing `pairs` with the `pop` command, so `pairs pop`
- Specifying a pin directly, e.g. `pairs 666`

Either way, after you applied the remote changes, `pairs` will prompt you to choose whether you would like to delete the temporary branch locally and remotely.

#### Using `pop` command

If you use the `pairs pop` command, you will be prompted to select a stash from the list of available stashes.
You will see the same information as when invoking `pairs list` but you can navigate the list via the arrow keys and choose a branch with the enter key.

#### Using a specific pin

When you specify an existing pin as an argument, like `pairs 666`, pairs will look for the specified pin on the remote origin.
If the pin is present, `pairs` will apply the changes locally as well.
