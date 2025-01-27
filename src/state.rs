use strum::VariantArray;

#[derive(serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::VariantArray, PartialEq, Copy, Clone)]
pub enum AudioSource {
    #[strum(to_string = "Analog 1")]
    Analog1,
    #[strum(to_string = "Analog 2")]
    Analog2,
    #[strum(to_string = "Analog 3")]
    Analog3,
    #[strum(to_string = "Analog 4")]
    Analog4,
    #[strum(to_string = "Analog 5")]
    Analog5,
    #[strum(to_string = "Analog 6")]
    Analog6,
    #[strum(to_string = "Analog 7")]
    Analog7,
    #[strum(to_string = "Analog 8")]
    Analog8,
    #[strum(to_string = "PCM 1")]
    PCM1,
    #[strum(to_string = "PCM 2")]
    PCM2,
    #[strum(to_string = "PCM 3")]
    PCM3,
    #[strum(to_string = "PCM 4")]
    PCM4,
    #[strum(to_string = "PCM 5")]
    PCM5,
    #[strum(to_string = "PCM 6")]
    PCM6,
    #[strum(to_string = "SPDIF 1")]
    SPDIF1,
    #[strum(to_string = "SPDIF 2")]
    SPDIF2,
    #[strum(to_string = "ADAT 1")]
    ADAT1,
    #[strum(to_string = "ADAT 2")]
    ADAT2,
    #[strum(to_string = "ADAT 3")]
    ADAT3,
    #[strum(to_string = "ADAT 4")]
    ADAT4,
    #[strum(to_string = "ADAT 5")]
    ADAT5,
    #[strum(to_string = "ADAT 6")]
    ADAT6,
    #[strum(to_string = "ADAT 7")]
    ADAT7,
    #[strum(to_string = "ADAT 8")]
    ADAT8,
    #[strum(to_string = "Mix A")]
    MixA,
    #[strum(to_string = "Mix B")]
    MixB,
    #[strum(to_string = "Mix C")]
    MixC,
    #[strum(to_string = "Mix D")]
    MixD,
    #[strum(to_string = "Mix E")]
    MixE,
    #[strum(to_string = "Mix F")]
    MixF
}

#[derive(serde::Serialize, serde::Deserialize, strum_macros::Display, strum_macros::VariantArray, PartialEq, Copy, Clone)]
pub enum AudioDestination {
    #[strum(to_string = "Mix A")]
    MixA,
    #[strum(to_string = "Mix B")]
    MixB,
    #[strum(to_string = "Mix C")]
    MixC,
    #[strum(to_string = "Mix D")]
    MixD,
    #[strum(to_string = "Mix E")]
    MixE,
    #[strum(to_string = "Mix F")]
    MixF
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MixerDestination {
    pub stereo: bool,
    pub dest: AudioDestination,
    pub dest_r: AudioDestination,
    pub split: bool,
    pub gain: f32
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MixerEntry {
    pub name: String,
    pub enabled: bool,
    pub stereo: bool,
    pub split: bool,
    pub source: AudioSource,
    pub source_r: AudioSource,
    pub dests: Vec<MixerDestination>
}

// given a slice of true and false values, find the index of the start of a pair of false values
// warn: not tail recursive!
fn find_false_pair(sl: &[bool]) -> Option<usize> {
    match sl {
        [false, false, ..] => Some(0),
        [_, tail @ ..] => find_false_pair(tail).map(|v| v + 1),
        [] => None
    }
}

impl MixerEntry {
    pub fn new() -> Self {
        let mut e = Self {
            name: "Unnamed".to_owned(),
            enabled: true,
            stereo: false,
            split: false,
            source: AudioSource::Analog1,
            source_r: AudioSource::Analog2,
            dests: Vec::new()
        };
        e.add_dest();
        e
    }

    pub fn add_dest(&mut self) {
        let mut used = [false; 6];
        self.dests.iter()
            .flat_map(|d| if d.stereo { vec![ d.dest, d.dest_r ] } else { vec![ d.dest ]})
            .map(|d| AudioDestination::VARIANTS.iter().position(|v| *v == d).unwrap())
            .for_each(|cur| { used[cur] = true; });

        let stereo = self.stereo;
        let available = if stereo {
            find_false_pair(&used)
        } else {
            used.iter().position(|v| !v)
        };

        // stereo source -> mono dest = mix down (route both source l and source r to dest)
        // mono source to stereo dest = route source to dest l and dest r
        // mono src -> mono dest, stereo src -> stereo dest = self explanatory

        self.dests.push(MixerDestination {
            gain: 0.0,
            stereo,
            dest: available.map_or(AudioDestination::MixA, |i| AudioDestination::VARIANTS[i]),
            dest_r: available.map_or(AudioDestination::MixB, |i| if i + 1 < AudioDestination::VARIANTS.len() {
                AudioDestination::VARIANTS[i + 1]
            } else {
                AudioDestination::MixF
            }),
            split: false
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct MixerOutput {
    pub name: String,
    pub gain: f32,
    pub mute: bool,
    pub source: (AudioSource, AudioSource),
    pub split: bool
}
