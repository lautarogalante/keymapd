use std::error::Error;
use evdev::{Device, EventSummary, KeyCode};
use std::process::Command;

enum ValumeAction {
   VolumeUp(String),
   VolumeDown(String),
   Mute
}

fn map_keys(mut device: Device) -> Result<(), Box<dyn Error>>{
    loop {
        for event in device.fetch_events()?{
            let _ = match event.destructure() {
                EventSummary::Key(_,KeyCode::KEY_VOLUMEUP, value)  if value != 0 => execute_command(ValumeAction::VolumeUp("5%+".into())),
                EventSummary::Key(_,KeyCode::KEY_VOLUMEDOWN, value) if value != 0 => execute_command(ValumeAction::VolumeDown("5%-".into())),
                EventSummary::Key(_,KeyCode::KEY_MUTE, value) if value != 0 => execute_command(ValumeAction::Mute),
               _ => Ok(())
            };
        }
    }
}

fn execute_command(value: ValumeAction) -> Result<(), Box<dyn Error>> {
    
    match  value {
       ValumeAction::Mute => {
            Command::new("wpctl")
                .arg("set-mute")
                .arg("@DEFAULT_AUDIO_SINK@")
                .arg("toggle")
                .status()
                .expect("Failed to run command");
       }
       ValumeAction::VolumeUp(v) | ValumeAction::VolumeDown(v) => {
            Command::new("wpctl")
                .arg("set-volume")
                .arg("@DEFAULT_AUDIO_SINK@")
                .arg(v)
                .status()
                .expect("Failed to run command");
       }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>>{
    let device = Device::open("/dev/input/event4")?;
    map_keys(device)?;
    Ok(())
}
