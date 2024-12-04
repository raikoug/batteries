use clap::{Arg, Command, ArgAction};
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string_pretty;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tabled::{Table, Tabled};
use zbus::zvariant::{ObjectPath, OwnedObjectPath};
use zbus::{Connection, Proxy};

#[derive(Deserialize, Debug, serde::Serialize)]
pub struct DeviceMapping {
    pub serial: String,
    pub device_type: String,
    pub name: String,
}

#[derive(Deserialize, Debug, serde::Serialize)]
pub struct DeviceSuppress {
    pub serial: Option<String>,
    pub model: Option<String>,
    pub vendor: Option<String>,
    pub device_type: Option<u32>,
}

#[derive(Deserialize, Debug, serde::Serialize)]
pub struct Config {
    pub device_mapping: Vec<DeviceMapping>,
    pub device_suppress: Vec<DeviceSuppress>,
}

#[derive(Tabled, Serialize)]
struct FullDevice {
    name: String,
    percentage: String,
    device_type: String,
    mapped_name: String,
    mapped_type: String,
    suppressed: bool,
    serial: String,
    vendor: String,
    numeric_type: u32,
}

#[derive(Tabled, Serialize)]
struct SmallDevice {
    name: String,
    percentage: String,
    device_type: String,
}

#[tokio::main]
async fn main() -> zbus::Result<()> {
    // Define command-line arguments
    let matches = Command::new("batteries")
        .version("0.1.0")
        .author("Riccardo Bella <riccardobella@gmail.com>")
        .about("Battery management tool")
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .action(ArgAction::SetTrue)
                .help("Print the output in JSON format"),
        )
        .arg(
            Arg::new("list")
                .short('l')
                .long("list")
                .action(ArgAction::SetTrue)
                .help("Print detailed info about each device"),
        )
        .get_matches();
    
    let list: bool = matches.get_flag("list");
    let json: bool = matches.get_flag("json");

    let config_path = Path::new("/etc/batteries/configs.toml");
    let config: Option<Config> = match fs::read_to_string(config_path) {
        Ok(config_content) => toml::from_str(&config_content).ok(),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                // Try to create the file if it doesn't exist
                match File::create(config_path) {
                    Ok(mut file) => {
                        let default_config = Config {
                            device_mapping: Vec::new(),
                            device_suppress: Vec::new(),
                        };
                        let default_config_content = toml::to_string(&default_config).unwrap();
                        if file.write_all(default_config_content.as_bytes()).is_ok() {
                            Some(default_config)
                        } else {
                            None
                        }
                    }
                    Err(_) => None,
                }
            }
            _ => None,
        },
    };

    // Connect to the system D-Bus
    let connection = Connection::system().await?;

    // Create a proxy for UPower
    let upower = Proxy::new(
        &connection,
        "org.freedesktop.UPower",
        "/org/freedesktop/UPower",
        "org.freedesktop.UPower",
    )
    .await?;

    // Retrieve the list of devices
    let devices: Vec<OwnedObjectPath> = upower.call("EnumerateDevices", &()).await?;

    let mut full_device_list: Vec<FullDevice> = Vec::new();
    let mut filtered_device_list: Vec<SmallDevice> = Vec::new();

    for device_path in devices {
        // Convert the device_path to an ObjectPath
        let object_path = ObjectPath::try_from(device_path.as_str())?;

        // Create a proxy for the device
        let device_proxy = Proxy::new(
            &connection,
            "org.freedesktop.UPower",
            object_path,
            "org.freedesktop.UPower.Device",
        )
        .await?;

        // Retrieve device properties
        let name: String = device_proxy.get_property("Model").await?;
        let percentage: f64 = device_proxy.get_property("Percentage").await?;
        let device_type: u32 = device_proxy.get_property("Type").await?;
        let device_serial: String = device_proxy.get_property("Serial").await?;
        let device_vendor: String = device_proxy.get_property("Vendor").await?;

        // Check if the device should be suppressed
        let suppress = config.as_ref().map_or(false, |config| {
            config.device_suppress.iter().any(|suppress| {
                (suppress.serial.as_ref().map_or(false, |s| s == &device_serial))
                    || (suppress.model.as_ref().map_or(false, |m| m == &name))
                    || (suppress.vendor.as_ref().map_or(false, |v| v == &device_vendor))
                    || (suppress.device_type.map_or(false, |t| t == device_type))
            })
        });

        if !list && suppress {
            continue;
        }

        // Convert the device type to a human-readable string
        let device_type_str = match device_type {
            1 => "Line Power",
            2 => "Battery",
            3 => "UPS",
            4 => "Monitor",
            5 => "Mouse",
            6 => "Keyboard",
            7 => "PDA",
            8 => "Phone",
            _ => "Unknown",
        };

        // Check if there is a custom mapping for this device
        let mut mapped_name = name.clone();
        let mut mapped_type = device_type_str.to_string();
        if let Some(config) = &config {
            for device_mapping in &config.device_mapping {
                if device_mapping.serial == device_serial {
                    mapped_name = device_mapping.name.clone();
                    mapped_type = device_mapping.device_type.clone();
                    break;
                }
            }
        }

        // Add the device to the list
        full_device_list.push(FullDevice {
            name: name.clone(),
            percentage: format!("{:.1}%", percentage),
            device_type: device_type_str.to_string(),
            mapped_name: mapped_name.clone(),
            mapped_type: mapped_type.clone(),
            suppressed: suppress,
            serial: device_serial,
            vendor: device_vendor,
            numeric_type: device_type,
        });

        filtered_device_list.push(SmallDevice {
            name: mapped_name.clone(),
            percentage: format!("{:.1}%", percentage),
            device_type: mapped_type.clone(),
        });
    }

    if list && json {
        println!("{}", to_string_pretty(&full_device_list).unwrap());
    } else if list {
        let table = Table::new(full_device_list);
        println!("{}", table);
    } else if json {
        println!("{}", to_string_pretty(&filtered_device_list).unwrap());
    } else {
        let table = Table::new(filtered_device_list);
        println!("{}", table);
    }

    Ok(())
}