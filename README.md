# Batteries

`batteries` is a simple CLI tool written in Rust to monitor the battery status of devices connected to your system. It supports user-friendly and software-compatible output formats, and allows customization via configuration files.

## Features

- Displays the battery status of all connected devices.
- Supports both table and JSON output formats.
- Customizable through a configuration file to map names and device types or to filter out unwanted devices.
- Detailed mode to show all devices, including suppressed ones.

## Usage Examples

### Default Table
```bash
batteries
```

Output:
```
+-----------+------------+-------------+
| name      | percentage | device_type |
+-----------+------------+-------------+
| LG        | 52.0%      | mouse       |
+-----------+------------+-------------+
| FreeeWolf | 100.0%     | keyboard    |
+-----------+------------+-------------+
```

### JSON Output
```bash
batteries -j
```

Output:
```json
[
  {
    "name": "LG",
    "percentage": "52.0%",
    "device_type": "mouse"
  },
  {
    "name": "FreeeWolf",
    "percentage": "100.0%",
    "device_type": "keyboard"
  }
]
```

### Detailed Mode
Unfiltered mode to show all devices, including suppressed ones.
```bash
batteries -l
```

Output:
```
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
| name                                  | percentage | device_type | mapped_name | mapped_type | suppressed | serial            | vendor | numeric_type |
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
| BIF0_9                                | 50.0%      | Battery     | BIF0_9      | Battery     | true       |                   | MSI    | 2            |
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
|                                       | 0.0%       | Line Power  |             | Line Power  | true       |                   |        | 1            |
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
| G502 LIGHTSPEED Wireless Gaming Mouse | 52.0%      | Battery     | LG          | mouse       | false      | 9e-7c-81-dd       |        | 2            |
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
| K8BT5.0-2                             | 100.0%     | Keyboard    | FreeeWolf   | keyboard    | false      | 82:AD:9A:2E:4F:8D |        | 6            |
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
```

fitered modee to show all devices, including suppressed ones.
```bash
batteries -i
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
| name                                  | percentage | device_type | mapped_name | mapped_type | suppressed | serial            | vendor | numeric_type |
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
| G502 LIGHTSPEED Wireless Gaming Mouse | 52.0%      | Battery     | LG          | mouse       | false      | 9e-7c-81-dd       |        | 2            |
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
| K8BT5.0-2                             | 100.0%     | Keyboard    | FreeeWolf   | keyboard    | false      | 82:AD:9A:2E:4F:8D |        | 6            |
+---------------------------------------+------------+-------------+-------------+-------------+------------+-------------------+--------+--------------+
```

## Installation

### Build Your Own

#### Requirements

- **Rust**: Make sure Rust and Cargo are installed. Follow [this guide](https://www.rust-lang.org/tools/install) to set them up.

#### Build

Git clone the project (also in /tmp) and then build it:
```bash
git clone https://github.com/raikoug/batteries
cd batteries
cargo build --release
mkdir -p /etc/batteries/
touch /etc/batteries/configs.toml
```

If you want to filter out "Line Power" devices, you can create the configuration file with the following content:
```bash
echo '[[device_suppress]]' >> /etc/batteries/configs.toml
echo 'device_type = 1' >> /etc/batteries/configs.toml
```

If you want to use it everywhere, you can move the binary to `/usr/local/bin`:
```bash
sudo mv target/release/batteries /usr/local/bin/
```

### Deb package
Get the latest deb package from the [releases page](https://github.com/raikoug/batteries/releases)
```bash
sudo dpkg -i batteries_0.1.0_amd64.deb
```
For dependencies, you may need to run:
```bash
sudo apt-get install -f
```

### Make your own deb package
Build the solution following the Build section and then run:
```bash
# always inside batteries base folder
chmod +x ./packaging/make_deb.sh
./packaging/make_deb.sh
```

## Configuration

The configuration file is located at `/etc/batteries/configs.toml`. You can create this file manually (like explained in the Build section) or let the app generate it on the first run.

### Configuration Example

To suppress "Line Power" devices:
```toml
[[device_suppress]]
device_type = 1
```

To map custom names to devices:
```toml
[[device_mapping]]
serial = "9e-7c-81-dd"
name = "LG Mouse"
device_type = "Mouse"

[[device_mapping]]
serial = "82:AD:9A:2E:4F:8D"
name = "FreeWolf Keyboard"
device_type = "Keyboard"
```

## Usage

```bash
Battery management tool

Usage: batteries [OPTIONS]

Options:
  -j, --json     Print the output in JSON format
  -l, --list     Print extended unfiltered info about each device
  -i, --info     Print extended fiiltered info about each device
  -h, --help     Print help
  -V, --version  Print version
```

## License

This project is distributed under a **custom open-source license**. You may use it for personal or educational purposes with proper attribution. For commercial use, please contact 
[Riccardo Bella](mailto:raikoug@gmail.com).

## Contributing

Contributions are welcome! You can contribute by:
- **Merge requests**
- **Issues**

## Features in Development

- CLI configuration editing.
- Precompiled installation packages for easy distribution.


```
Copyright © 2021 Riccardo Bella
```