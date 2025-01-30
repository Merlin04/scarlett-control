use std::{collections::HashMap, ops::{Deref, DerefMut}};

use alsa::mixer::{MilliBel, Selem, SelemChannelId};

use crate::ScarlettControlApp;

const CHANNEL: SelemChannelId = SelemChannelId::FrontLeft /*SelemChannelId::mono()*/;

pub type EnumIndex = usize;

#[derive(PartialEq)]
enum ElemValue {
    Enum(EnumIndex),
    Knob { db: f32, muted: bool }
}

impl<'a> From<&Selem<'a>> for ElemValue {
    fn from(value: &Selem) -> Self {
        if value.is_enumerated() {
            ElemValue::Enum(value.get_enum_item(CHANNEL).unwrap() as EnumIndex)
        } else {
            ElemValue::Knob {
                db: value.get_playback_vol_db(CHANNEL).unwrap().to_db(),
                muted: value.get_playback_switch(CHANNEL).unwrap() == 0
            }
        }
    }
}

trait ElemSettable {
    fn set_value(&self, val: &ElemValue);
}

impl ElemSettable for Selem<'_> {
    fn set_value(&self, val: &ElemValue) {
        match val {
            ElemValue::Enum(val) => self.set_enum_item(CHANNEL, *val as u32).unwrap(),
            ElemValue::Knob { db, muted } => {
                self.set_playback_db(CHANNEL, MilliBel::from_db(*db),
                    if self.has_playback_channel(CHANNEL) { alsa::Round::Floor } else { alsa::Round::Ceil }
                ).unwrap();
                self.set_playback_switch(CHANNEL, if *muted { 0 } else { 1 }).unwrap()
            },
        }
    }
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

    // identify keys in both `self` and `new` where their values differ
    pub fn diff(&self, new: &DeviceState) -> Vec<String> {
        self.iter().filter_map(|(k, v)| new.get(k).filter(|v2| v == *v2)
            .map(|_| k.clone())).collect()
    }
}

impl From<&ScarlettControlApp> for DeviceState {
    fn from(a: &ScarlettControlApp) -> Self {
        let mut d = DeviceState::new();

        // capture
        for (i, v) in a.state.capture.iter().enumerate() {
            d.insert(format!("Input Source {:02}", i), ElemValue::Enum(0)); // TODO
        }

        // mixer
        for entry in &a.state.mixer_entries {
            for dest in &entry.dests {
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

impl From<&Device> for DeviceState {
    fn from(d: &Device) -> Self {
        Self (HashMap::from_iter(
            d.selems.iter()
                .map(|(k, s)| (k.clone(), ElemValue::from(s)))
        ))
    }
}

const CARD_NAME: &str = "Scarlett 18i6";

type Selems<'a> = HashMap<String, Selem<'a>>;

pub struct Device {
    pub selems: Selems<'static>,
    pub capture_sources: Vec<String>,
    // audio sources for mixer entries and outputs
    pub audio_sources: Vec<String>,
    // mixes that a mixer entry can send audio to
    pub mixer_destinations: Vec<String>
}

fn get_enums(selem: &Selem) -> Vec<String> {
    selem.iter_enum().unwrap().map(|i| i.unwrap()).collect()
}

impl Device {
    pub fn new() -> Option<Device> {
        let c = alsa::card::Iter::new().find_map(|r| {
            let c = r.unwrap();
            if c.get_name().unwrap() == CARD_NAME { Some(c) } else { None }
        })?;
        // let ctl = alsa::Ctl::from_card(&c, false).unwrap();
        let mixer = Box::<alsa::Mixer>::leak(Box::from(
            alsa::Mixer::new(format!("hw:{}", c.get_name().unwrap()).as_str(), false).unwrap()
        ));
        let selems: Selems = HashMap::from_iter(mixer.iter()
            .flat_map(|e| Selem::new(e))
            .map(|s| (s.get_id().get_name().unwrap().to_owned(), s)));

        Some(Device {
            capture_sources: get_enums(selems.get("Input Source 01").unwrap()),
            audio_sources: get_enums(selems.get("TODO").unwrap()),
            mixer_destinations: /*get_enums(selems.iter().find_map(|(k, v)|
                    if k.starts_with("Master") && k.ends_with("Source") { Some(v) } else { None }
                ).unwrap())*/
                selems.iter().filter_map(|(k, _v)|
                    if k.starts_with("Matrix 01 Mix ") { Some(k.strip_prefix("Matrix 01 ").unwrap().to_owned()) } else { None }).collect(),
            selems,
        })
    }

    pub fn update(&mut self, app: &ScarlettControlApp) {
        let old = DeviceState::from(&*self);
        let new = DeviceState::from(app);
        for k in old.diff(&new) {
            self.selems.get(&k).unwrap().set_value(new.get(&k).unwrap());
        }
    }
}