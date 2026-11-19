#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginFormat { Vst3, Clap, Au }

pub trait PluginHost: Send {
    fn format(&self) -> PluginFormat;
    fn name(&self) -> &str;
    fn load(&mut self, path: &std::path::Path) -> crate::error::Result<()>;
    fn process(&mut self, buffer: &mut crate::dsp::AudioBuffer);
    fn unload(&mut self);
}
