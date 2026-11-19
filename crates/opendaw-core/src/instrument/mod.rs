pub trait Instrument: Send {
    fn name(&self) -> &str;
    fn note_on(&mut self, pitch: u8, velocity: u8);
    fn note_off(&mut self, pitch: u8);
    fn render(&mut self, buffer: &mut crate::dsp::AudioBuffer);
    fn reset(&mut self);
}
