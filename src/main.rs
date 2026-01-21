use std::{error::Error, fs};
use evdev::{Device, EventSummary, KeyCode};
use std::process::Command;

const INPUT_PATH: &str = "/dev/input";
const UTILITY: &str = "wpctl";
const SET_MUTE: &str = "set-mute";
const SET_VOLUME: &str = "set-volume";
const DEFAULT_AUDIO_SINK: &str = "@DEFAULT_AUDIO_SINK@";
const TOGGLE: &str = "toggle";
const VID: u16 = 0x1b1c; 
const PID: u16 = 0x1bb9;
const VOLUME_PERCENT_UP: &str = "5%+";
const VOLUME_PERCENT_DOWN: &str = "5%-";

enum ValumeAction {
   VolumeUp(String),
   VolumeDown(String),
   Mute
}

struct DeviceProperties {
    vid: u16,
    pid: u16,
}

fn map_keys(mut device: Device) -> Result<(), Box<dyn Error>>{
    
    loop {
        match device.fetch_events() {
            Ok(events) => {
                for event in events {
                    let _ = match event.destructure() {
                        EventSummary::Key(_,KeyCode::KEY_VOLUMEUP, value)  if value != 0 => execute_command(ValumeAction::VolumeUp(VOLUME_PERCENT_UP.into())),
                        EventSummary::Key(_,KeyCode::KEY_VOLUMEDOWN, value) if value != 0 => execute_command(ValumeAction::VolumeDown(VOLUME_PERCENT_DOWN.into())),
                        EventSummary::Key(_,KeyCode::KEY_MUTE, value) if value != 0 => execute_command(ValumeAction::Mute),
                        _ => Ok(()),
                    };
                };
            },
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound || e.raw_os_error() == Some(19) {
                    _ = init();
                }
            }
        }
        
    }
}

fn execute_command(value: ValumeAction) -> Result<(), Box<dyn Error>> {
    
    match  value {
       ValumeAction::Mute => {
            Command::new(UTILITY)
                .arg(SET_MUTE)
                .arg(DEFAULT_AUDIO_SINK)
                .arg(TOGGLE)
                .status()
                .expect("Failed to run command");
       }
       ValumeAction::VolumeUp(v) | ValumeAction::VolumeDown(v) => {
            Command::new(UTILITY)
                .arg(SET_VOLUME)
                .arg(DEFAULT_AUDIO_SINK)
                .arg(v)
                .status()
                .expect("Failed to run command");
       }
    }
    Ok(())
}

fn find_device(properties: &DeviceProperties) -> Result<Vec<Device>, Box<dyn Error>> {

    let paths = fs::read_dir(INPUT_PATH)?;
    let mut found_device: Vec<Device> = Vec::new();
    for dir_entry in paths.flatten() {
        let path = dir_entry.path();

        if let Ok(device) = Device::open(&path) {
            if device.input_id().vendor() == properties.vid && device.input_id().product() == properties.pid
            {
                found_device.push(device);
            }
        }
    }

    Ok(found_device)
}

fn init() -> Result<(), Box<dyn Error>> {
    let properties = &DeviceProperties { vid: VID, pid: PID };
    let mut device_vec = find_device(properties)?;
    if let Some(device) = device_vec.pop() {
        map_keys(device)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let _ = init();
    Ok(())
}
