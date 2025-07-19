[English](README.md)

# rign

`rign` 是一个用于在 Windows 上管理多个 Node.js 版本的命令行工具。使用 Rust 编写，旨在成为其他 Node.js 版本管理器的简单快速的替代品。

## 功能

*   列出所有已安装的 Node.js 版本。
*   从官方存储库或镜像安装特定的 Node.js 版本。
*   卸载特定的 Node.js 版本。
*   在已安装的 Node.js 版本之间切换。
*   显示可用的 Node.js 版本，包括详细信息。

## 用法

### 1. 下载可执行文件，并放到一个合适的位置

推荐将可执行文件放在一个单独文件夹中，比如 `rign/rign.exe`

### 2. 在可执行文件所在目录执行 `add-path` 命令

```bash
.\rign.exe add-path
```

该命令会将 `当前目录` 和 `当前目录/nodejs/actived` 这两个文件夹添加到 `path` 环境变量。

执行完成后请打开新的终端，使 `path` 生效。

### 3. 使用 `rign` 管理 node 版本

```bash
rign use latest
rign use lts
rign use 16
```

## 命令

- actived

    检查当前激活的 node 版本 [别名: cur, current]

- list

    列出所有已安装的版本 [别名: ls]

- show

    查看已发布的 node 版本

- install

    安装指定的版本 [别名: i, add]

- uninstall

    删除指定的版本 [别名: rm, remove, delete]

- use

    切换激活的 node 版本

- add-path

    将工具和 node 目录添加到用户的环境变量中

- clean-path

    清理此工具添加的环境变量

- node-mirror

    设置 node 下载镜像到环境变量。例如：https://npmmirror.com/mirrors/node