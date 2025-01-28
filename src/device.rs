use std::{collections::HashMap, ops::{Deref, DerefMut}};

use crate::ScarlettControlApp;

enum ElemValue {
    Enum(u32),
    Knob { db: f32, muted: bool }
}

struct DeviceState(HashMap<String, ElemValue>);

impl Deref for DeviceState {
    type Target = HashMap<String, ElemValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DeviceState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DeviceState {
    pub fn new() -> Self {
        DeviceState(HashMap::new())
    }

    pub fn diff(&self, p: &DeviceState) -> DeviceState {
        todo!()
    }
}

impl From<ScarlettControlApp> for DeviceState {
    fn from(a: ScarlettControlApp) -> Self {
        let mut d = DeviceState::new();

        // capture
        for (i, v) in a.capture.iter().enumerate() {
            d.insert(format!("Input Source {:02}", i), ElemValue::Enum(0)); // TODO
        }

        // mixer
        for entry in a.mixer_entries {
            for dest in entry.dests {
                match (entry.stereo, dest.stereo) {
                    (true, true) => {
                        // d.insert(format!("Matrix {:02} Mix {}", ))
                    }
                    (true, false) => todo!(),
                    (false, true) => todo!(),
                    (false, false) => todo!(),
                }
            }
        }

        // global state

        // hi z

        // outputs

        d
    }
}

impl From<Device> for DeviceState {
    fn from(d: Device) -> Self {
        d.mixer.iter()
            .flat_map(|e| alsa::mixer::Selem::new(e))
            .map(|s| s.)

        todo!()
    }
}

const CARD_NAME: &str = "Scarlett 18i6";

pub struct Device {
    mixer: alsa::Mixer,
    audio_sources: Vec<String>,
    audio_destinations: Vec<String>
}

impl Device {
    pub fn new() -> Option<Device> {
        let c = alsa::card::Iter::new().find_map(|r| {
            let c = r.unwrap();
            if c.get_name().unwrap() == CARD_NAME { Some(c) } else { None }
        })?;
        // let ctl = alsa::Ctl::from_card(&c, false).unwrap();
        let mixer = alsa::Mixer::new(format!("hw:{}", c.get_name().unwrap()).as_str(), false).unwrap();
        Some(Device {
            mixer
        })
    }

    pub fn update(&mut self, state: ScarlettControlApp) {
        // diff device state and app state
        // 
    }
}