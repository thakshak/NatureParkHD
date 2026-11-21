use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle, Sink};
use std::collections::HashMap;

const SAMPLE_RATE: u32 = 44100;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SoundId {
    Theme = 0,
    Combo = 1,
    Clear = 2,
    Special = 3,
    LevelUp = 4,
    GameOver = 5,
    Unused6 = 6,
    Intro = 7,
}

// Exact Nokia OTT (FORMAT_TONE) byte arrays from com.nokia.mid.appl.np.a
const OTT_0: &[i8] = &[
    2, 74, 58, 85, 9, 29, 52, -64, -60, 36, 0, 77, 30, -56, -5, -54, 35, 18, 12, 33, 66, -79, 33,
    115, 17, 32, 66, 20, 39, 18, 12, 33, 66, -47, 33, 115, 81, 32, -62, 20, 73, -120, -112, -71,
    -88, -112, -71, -56, -112, -70, -120, -96, -84, 72, 92, -61, 8, 48, -100, 72, 92, -84, 72, 92,
    2, 6, 90, 36, 24, 66, -128, 2, -61, 17, 32, -62, 20, 45, 18, 23, 43, 18, 23, 39, 18, 23, 34,
    -126, -79, 33, 115, 17, 32, -62, 20, 39, 18, 23, 43, 18, 23, 45, 18, 12, 33, 67, 81, 33, 115,
    17, 33, 114, -47, 33, 115, 17, 32, -62, 20, 43, 18, 12, 33, 66, 40, 43, 18, 23, 49, 18, 12, 33,
    66, 113, 33, 114, -79, 33, 112, 8, 25, 104, -112, 97, 10, 0, 46, -52, 68, -125, 8, 80, -76, 72,
    92, -84, 72, 92, -100, 72, 92, -118, 10, -60, -123, -52, 68, -125, 8, 80, -100, 72, 92, -84,
    72, 92, -76, 72, 48, -123, 13, 68, -123, -52, 68, -123, -53, 68, -123, -52, 68, -125, 8, 80,
    -28, 72, 48, -123, 13, 68, -123, -50, 68, -123, -42, 70, 36, 46, -93, 17, 32, -62, 20, 57, 18,
    23, 89, 24, -112, -70, -117, 68, -125, 8, 80, -28, 72, 93, 100, 98, 66, -22, 49, 18, 12, 33,
    66, -47, 33, 114, -79, 32, -62, 20, 53, 18, 23, 57, 18, 23, 89, 24, -112, -70, -116, 68, -125,
    8, 80, -28, 72, 93, 100, 98, 66, -22, 45, 18, 23, 53, 18, 23, 57, 18, 23, 89, 24, -112, -71,
    88, -112, 97, 10, 19, -119, 11, -111, -119, 6, 16, -94, -115, 68, -123, -50, 68, -123, -42, 70,
    36, 46, -93, 17, 32, -62, 20, 57, 18, 23, 89, 24, -112, -70, -117, 68, -125, 8, 80, -28, 72,
    93, 100, 98, 66, -22, 49, 18, 12, 33, 66, -47, 33, 114, -79, 32, -62, 20, 53, 18, 23, 57, 18,
    23, 89, 24, -112, -70, -116, 68, -125, 8, 80, -28, 72, 93, 100, 98, 66, -27, 98, 65, -124, 40,
    78, 36, 46, 70, 36, 46, 76, -28, 46, 70, 36, 24, 66, -118, 56, -126, 8, 32, -60, -104, -119,
    11, -102, -119, 11, -100, -119, 11, -88, -118, 10, -60, -123, -52, 68, -125, 8, 80, -100, 72,
    92, -84, 72, 92, 2, 6, 90, 36, 24, 66, -128, 2, -61, 17, 32, -62, 20, 45, 18, 23, 43, 18, 23,
    39, 18, 23, 34, -126, -79, 33, 115, 17, 32, -62, 20, 39, 18, 23, 43, 18, 23, 45, 18, 12, 33,
    67, 81, 33, 115, 17, 33, 114, -47, 33, 115, 17, 32, -62, 20, 43, 18, 12, 33, 66, 40, 43, 18,
    23, 49, 18, 12, 33, 66, 113, 33, 114, -79, 33, 112, 8, 25, 104, -112, 97, 10, 0, 9, 12, 68,
    -125, 8, 80, -76, 72, 92, -84, 72, 92, -100, 72, 92, -118, 10, -60, -123, -52, 68, -125, 8, 80,
    -100, 72, 92, -84, 72, 92, -76, 72, 48, -123, 13, 68, -123, -52, 68, -123, -53, 68, -123, -52,
    68, -125, 8, 80, -100, 72, 48, -123, 8, -88, -123, 0, 0,
];
const OTT_1: &[i8] = &[
    2, 74, 58, 72, -64, -56, 4, 0, 17, 28, -53, -53, 35, 2, 20, 49, 67, 81, 0,
];
const OTT_2: &[i8] = &[2, 74, 58, 72, -64, -52, 4, 0, 15, 28, -53, -54, 37, 66, -44, 55, 64, 0];
const OTT_3: &[i8] = &[
    2, 74, 58, 72, -64, -60, 4, 0, 25, 32, -53, -53, 41, 73, 2, -76, 45, 66, -12, 49, 67, 52, 54,
    -64, 0,
];
const OTT_4: &[i8] = &[
    2, 74, 58, 64, 4, 0, 123, 22, -53, -54, 43, 8, -78, -48, 49, 3, 80, 33, 3, 16, 53, 3, 16, 45,
    2, -80, 45, 2, 16, 49, 2, 108, 33, 2, 112, 43, 2, -48, 33, 3, 16, 32, -62, -48, 43, 2, 112, 43,
    2, 16, 45, 2, 16, 48, -62, -80, 45, 3, 16, 53, 2, 16, 49, 3, 80, 49, 2, -48, 43, 2, -48, 33, 3,
    16, 89, 22, -88, -100, 10, -64, -76, 12, 64, -124, 11, 64, -60, 11, 64, -84, 9, -64, -118, 0,
    0,
];
const OTT_5: &[i8] = &[
    2, 74, 58, 64, 4, 0, 75, 24, -53, -57, 39, 24, -94, 83, 43, 50, 16, 33, 114, 83, 43, 51, 55,
    33, 2, 23, 57, 52, -100, -102, -115, 77, 39, 38, -93, 51, 73, -55, -88, -68, -46, 114, 106, 43,
    66, -12, 51, 67, -108, 89, 86, -128, 0,
];
const OTT_6: &[i8] = &[
    2, 74, 58, 64, 4, 0, -127, 28, -53, -58, 33, 28, 66, 6, 32, 66, 7, 32, 66, 7, 32, -94, 112, 35,
    2, 16, 73, -56, 16, -127, -88, 16, -127, 120, 16, -127, -120, 16, -127, -120, 16, -127, -56,
    16, -127, -56, 40, -100, 8, -64, -124, 18, 114, 4, 32, 106, 4, 24, 66, 6, -96, 66, 6, -96, 66,
    7, 32, 66, 7, 32, 66, 10, 35, 2, 16, 35, 2, 16, 39, 2, 16, 39, 2, 16, 49, 2, 16, 47, 2, 16, 43,
    2, -16, 32, -61, 14, 0,
];
const OTT_7: &[i8] = &[
    2, 74, 58, 72, -48, -60, 4, 0, 65, 34, -53, -54, 35, 18, 12, 33, 66, -79, 33, 115, 17, 32, 66,
    20, 39, 18, 12, 33, 66, -47, 33, 115, 81, 32, 66, 20, 49, 18, 12, 33, 66, -47, 33, 114, 113, 32,
    -62, 20, 43, 18, 12, 33, 66, 40, 0,
];

pub struct AudioEngine {
    _stream: Option<OutputStream>,
    handle: Option<OutputStreamHandle>,
    current_sink: Option<Sink>,
    cache: HashMap<SoundId, Vec<f32>>,
    pub muted: bool,
}

impl AudioEngine {
    pub fn new() -> Self {
        let (stream, handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(e) => {
                eprintln!("Audio disabled (no output device): {e}");
                (None, None)
            }
        };

        let mut cache = HashMap::new();
        let pairs = [
            (SoundId::Theme, OTT_0),
            (SoundId::Combo, OTT_1),
            (SoundId::Clear, OTT_2),
            (SoundId::Special, OTT_3),
            (SoundId::LevelUp, OTT_4),
            (SoundId::GameOver, OTT_5),
            (SoundId::Unused6, OTT_6),
            (SoundId::Intro, OTT_7),
        ];
        for (id, raw) in pairs {
            let bytes: Vec<u8> = raw.iter().map(|&b| b as u8).collect();
            if let Some(samples) = render_ott(&bytes) {
                cache.insert(id, samples);
            }
        }

        Self {
            _stream: stream,
            handle,
            current_sink: None,
            cache,
            muted: false,
        }
    }

    pub fn play(&mut self, id: SoundId) {
        if self.muted {
            return;
        }
        let Some(handle) = &self.handle else { return };
        let Some(samples) = self.cache.get(&id) else {
            return;
        };
        if let Some(sink) = self.current_sink.take() {
            sink.stop();
        }
        if let Ok(sink) = Sink::try_new(handle) {
            let buf = SamplesBuffer::new(1, SAMPLE_RATE, samples.clone());
            sink.append(buf);
            sink.set_volume(0.22);
            self.current_sink = Some(sink);
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.current_sink.take() {
            sink.stop();
        }
    }
}

struct BitReader<'a> {
    data: &'a [u8],
    bit_pos: usize,
}

impl<'a> BitReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, bit_pos: 0 }
    }

    fn read_bits(&mut self, n: usize) -> Option<u32> {
        if self.bit_pos + n > self.data.len() * 8 {
            return None;
        }
        let mut val = 0u32;
        for _ in 0..n {
            let byte = self.data[self.bit_pos / 8];
            let bit = (byte >> (7 - (self.bit_pos % 8))) & 1;
            val = (val << 1) | (bit as u32);
            self.bit_pos += 1;
        }
        Some(val)
    }
}

const TEMPO_TABLE: [f32; 32] = [
    25.0, 28.0, 31.0, 35.0, 40.0, 45.0, 50.0, 56.0, 63.0, 70.0, 80.0, 90.0, 100.0, 112.0, 125.0,
    140.0, 160.0, 180.0, 200.0, 225.0, 250.0, 285.0, 320.0, 355.0, 400.0, 450.0, 500.0, 565.0,
    635.0, 715.0, 800.0, 900.0,
];

fn ott_note_freq(note: u32, scale: u32) -> f32 {
    if note == 0 {
        return 0.0;
    }
    // OTT note 1..12 = C..B, scale 0..3 = octaves 4..7 (approx)
    let semitone = (note - 1) as i32;
    let octave = match scale {
        0 => 4,
        1 => 5,
        2 => 6,
        _ => 7,
    };
    let midi = (octave + 1) * 12 + semitone;
    440.0 * 2.0_f32.powf((midi as f32 - 69.0) / 12.0)
}

fn render_ott(data: &[u8]) -> Option<Vec<f32>> {
    let mut br = BitReader::new(data);
    let _cmd_len = br.read_bits(8)?;
    let _cmd_part = br.read_bits(7)?;
    let _sound = br.read_bits(8)?;
    let song_type = br.read_bits(3)?;
    if song_type == 1 {
        let title_len = br.read_bits(4)?;
        for _ in 0..title_len {
            let _ = br.read_bits(8)?;
        }
    }
    let pattern_count = br.read_bits(8)?;

    let mut notes: Vec<(f32, f32)> = Vec::new();
    let mut bpm = 120.0_f32;
    let mut scale = 1_u32;
    let mut style = 0_u32;

    for _ in 0..pattern_count {
        let _pat_id = br.read_bits(3)?;
        let _pat_header = br.read_bits(2)?;
        let _loop_count = br.read_bits(4)?;
        let instr_count = br.read_bits(8)?;
        for _ in 0..instr_count {
            let Some(instr_id) = br.read_bits(3) else {
                break;
            };
            match instr_id {
                1 => {
                    // Note
                    let note = br.read_bits(4)?;
                    let dur_code = br.read_bits(3)?;
                    let spec = br.read_bits(2)?;
                    let beats = match dur_code {
                        0 => 4.0,
                        1 => 2.0,
                        2 => 1.0,
                        3 => 0.5,
                        4 => 0.25,
                        5 => 0.125,
                        _ => 0.25,
                    };
                    let mult = match spec {
                        1 => 1.5,
                        2 => 2.0,
                        _ => 1.0,
                    };
                    let sec = (60.0 / bpm) * beats * mult;
                    let freq = ott_note_freq(note, scale);
                    notes.push((freq, sec));
                }
                2 => {
                    scale = br.read_bits(2)?;
                }
                3 => {
                    style = br.read_bits(2)?;
                }
                4 => {
                    let t_idx = br.read_bits(5)? as usize;
                    bpm = TEMPO_TABLE[t_idx.min(31)];
                }
                5 => {
                    let _vol = br.read_bits(4)?;
                }
                _ => break,
            }
        }
    }
    let _ = style;

    if notes.is_empty() {
        return None;
    }

    let total_sec: f32 = notes.iter().map(|(_, d)| d).sum::<f32>() + 0.02;
    let total_samples = (total_sec * SAMPLE_RATE as f32).ceil() as usize;
    let mut out = vec![0.0_f32; total_samples];
    let mut cursor = 0usize;

    for (freq, dur) in notes {
        let len = ((dur * SAMPLE_RATE as f32).round() as usize).min(total_samples - cursor);
        if freq > 0.0 {
            let tone_len = (len as f32 * 0.88) as usize;
            for i in 0..tone_len {
                let t = i as f32 / SAMPLE_RATE as f32;
                let phase = (t * freq) % 1.0;
                let sq = if phase < 0.5 { 0.5 } else { -0.5 };
                out[cursor + i] = sq;
            }
        }
        cursor += len;
    }

    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ott_tracks_render_non_empty() {
        for raw in [OTT_0, OTT_1, OTT_2, OTT_3, OTT_4, OTT_5, OTT_6, OTT_7] {
            let bytes: Vec<u8> = raw.iter().map(|&b| b as u8).collect();
            let rendered = render_ott(&bytes);
            assert!(rendered.is_some());
            assert!(!rendered.unwrap().is_empty());
        }
    }
}
