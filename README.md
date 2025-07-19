[简体中文](README.zh-CN.md)

# rign

`rign` is a command-line tool for managing multiple Node.js versions on Windows. It is written in Rust and designed to be a simple and fast alternative to other Node.js version managers.

## Features

*   List all installed Node.js versions.
*   Install specific Node.js versions from the official repository or a mirror.
*   Uninstall specific Node.js versions.
*   Switch between installed Node.js versions.
*   Show available Node.js versions, including detailed information.

## Usage

### 1. Download the executable file and place it in a suitable location

It is recommended to place the executable file in a separate folder, for example `rign/rign.exe`

### 2. Execute the `add-path` command in the directory where the executable file is located

```bash
.\rign.exe add-path
```

This command will add the `current directory` and the `current directory/nodejs/actived` folders to the path environment variable.

After execution, please open a new terminal for the `path` to take effect.

### 3. Use `rign` to manage node versions

```bash
rign use latest
rign use lts
rign use 16
```

## Commands

- actived

    Check the currently active node version [aliases: cur, current]

- list

    List all installed versions [aliases: ls]

- show

    View released versions of node

- install

    Install the specified version [aliases: i, add]

- uninstall

    Delete the specified version [aliases: rm, remove, delete]

- use

    Switch the activated node version

- add-path

    Add the tool and node directories to the user's environment variables

- clean-path

    Clean up the environment variables added by this tool

- node-mirror

    Set the node download mirror to env. like: https://npmmirror.com/mirrors/node