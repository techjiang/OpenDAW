#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiNote {
    pub channel: u8,
    pub pitch: u8,
    pub velocity: u8,
    pub start_tick: u64,
    pub duration_ticks: u64,
}

#[derive(Debug, Default)]
pub struct MidiSequence {
    notes: Vec<MidiNote>,
}

impl MidiSequence {
    pub fn add_note(&mut self, note: MidiNote) { self.notes.push(note); }
    pub fn notes(&self) -> &[MidiNote] { &self.notes }
    pub fn notes_at(&self, tick: u64) -> impl Iterator<Item = &MidiNote> {
        self.notes.iter().filter(move |note| note.start_tick <= tick && tick < note.start_tick + note.duration_ticks)
    }
}
