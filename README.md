# obs-cmd: A Streamlined Command-Line Interface for OBS with obs-websocket v5 Support

"A Rust-Based Alternative to [obs-cli](https://github.com/muesli/obs-cli/pull/64) Compatible with obs-websocket 5"

This tool serves as a Rust-based replacement for obs-cli, offering compatibility with the latest obs-websocket version 5. Unlike obs-cli, which lacks support for this version, this Rust implementation ensures seamless integration and enhanced functionality with obs-websocket 5, providing a more robust and up-to-date solution for users seeking advanced OBS control and automation features.

[![release](https://github.com/grigio/obs-cmd/actions/workflows/release.yml/badge.svg)](https://github.com/grigio/obs-cmd/actions/workflows/release.yml)

## Quick Start

**Get Your OBS Info**

```bash
$ obs-cmd --websocket obsws://localhost:4455/secret info
Version: Version { obs_version: Version { major: 29, minor: 1, patch: 1 }, obs_web_socket_version: Version { major: 5, minor: 2, patch: 2 }, rpc_version: 1, available_requests: ..
```

**Switch the Scene**

```bash
$ obs-cmd --websocket obsws://localhost:4455/secret scene switch my-scene
Switched to scene: my-scene
Result: Ok(())
```

**Start the Virtual Camera**

```bash
$ obs-cmd --websocket obsws://localhost:4455/secret virtual-camera start
Recording started
Result: Ok(())
```

## Usage

```bash
obs-cmd [options] [command] [options] [arguments]
```

### Commands

| Command          | Options | Arguments                      |
| ---------------- | ------- | ------------------------------ |
| info             | -       |                                |
| scene            | switch  | [scene-name]                   |
|                  | list    |                                |
| scene-collection | switch  | [collection-name]              |
| scene-item       | toggle  | [scene-name] [scene-item-name] |
| toggle-mute      | switch  | [device-name]                  |
| recording        | start   |                                |
|                  | stop    |                                |
|                  | toggle  |                                |
| streaming        | start   |                                |
|                  | stop    |                                |
|                  | toggle  |                                |
| virtual-camera   | start   |                                |
|                  | stop    |                                |
|                  | toggle  |                                |
| replay           | start   |                                |
|                  | stop    |                                |
|                  | toggle  |                                |
|                  | save    |                                |

### Options

| Command     | Arguments | Notes                                      |
| ----------- | --------- | ------------------------------------------ |
| --websocket | [url]     | Default is 'obsws://localhost:4455/secret' |
| --help      | -         |                                            |

## Configuring OBS Studio

**Enable OBS WebSocket:**

1. Go to OBS Studio, navigate to Tools -> WebSocket Server Settings.
2. Check [ X ] Enable WebSocket server.

**Default Settings:  
For convenience, use** the default settings:

    Server Port: 4455
    Server Password: secret

With these settings, there's no need to specify the `--websocket` option in obs-cmd.

**Custom Settings:**  
To use custom settings, pass them to obs-cmd using the `--websocket` option. You can view your current configuration in OBS Studio by clicking 'Show Connect Info'.

## Installation

### Using the provided Binaries

#### 1. Download the Binary

Visit the [latest release page](https://github.com/grigio/obs-cmd/releases/latest) of `obs-cmd` and download the appropriate binary for your operating system, e.g., obs-cmd-linux-amd64.

#### 2. Set Execution Permissions and Move to Bin Directory

Make the downloaded binary executable and move it to a directory in your PATH:

```bash
chmod +x obs-cmd-linux-amd64 && sudo mv obs-cmd-linux-amd64 /usr/local/bin/obs-cmd
```

### Installing From Source

#### 1. Install Rust

Run the command to install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 2. Clone the Repository

Clone obs-cmd to your local system:

```bash
git clone https://github.com/grigio/obs-cmd.git
```

#### 3. Build the Application

Navigate to the cloned directory and build the application:

```bash
cd obs-cmd
cargo build --release
```

#### 4. Move the Binary

After a successful build, move the binary to a system-wide location:

```bash
sudo cp target/release/obs-cmd /usr/local/bin/obs-cmd
```

### Installing on Arch Linux

To install the obs-cmd package on Arch Linux, available on the [Arch User Repository (AUR)](https://aur.archlinux.org/packages/obs-cmd), follow these steps. Ensure you have rust installed, as it provides cargo, necessary for the installation.

#### Option 1: Using an AUR Helper

If you prefer an AUR helper, tools like yay or aurman can simplify the process.

#### Option 2: Manual Installation

1. Download the PKGBUILD file:

```bash
wget https://aur.archlinux.org/cgit/aur.git/snapshot/obs-cmd.tar.gz

```

2. Extract the downloaded .tar.gz file:

```bash
tar xvzf obs-cmd.tar.gz
```

3. Change directory to obs-cmd:

```bash
cd obs-cmd
```

4. Build the package using makepkg:

```bash
makepkg -s
```

5. Install the package with pacman (note: version number may vary):

```bash
sudo pacman -U obs-cmd-0.15.3-1-x86_64.pkg.tar.zst
```

## Donations

Donations are welcome and will go towards further development of this project

```
monero:88LyqYXn4LdCVDtPWKuton9hJwbo8ZduNEGuARHGdeSJ79BBYWGpMQR8VGWxGDKtTLLM6E9MJm8RvW9VMUgCcSXu19L9FSv
bitcoin:bc1q6mh77hfv8x8pa0clzskw6ndysujmr78j6se025
lightning:techonsapevole@getalby.com
```
