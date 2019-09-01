mod error;
mod midi_message;
mod note;

pub use error::Error;
pub use midi_message::{
    Channel, ControlNumber, ControlValue, MidiMessage, PitchBend, ProgramNumber, Song,
    SongPosition, Velocity, U14, U7,
};
pub use note::Note;

/// The frequency for `note` using the standard 440Hz tuning.
#[inline(always)]
#[deprecated(since = "3.0.0", note = "Use note.to_freq_f32() instead.")]
pub fn note_to_frequency_f32(note: Note) -> f32 {
    note.to_freq_f32()
}

/// The frequency for `note` using the standard 440Hz tuning.
#[inline(always)]
#[deprecated(since = "3.0.0", note = "Use note.to_freq_f64() instead.")]
pub fn note_to_frequency_f64(note: Note) -> f64 {
    note.to_freq_f64()
}
