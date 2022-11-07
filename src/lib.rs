use nih_plug::prelude::*;
use std::sync::Arc;

struct ParamOrderExample {
    params: Arc<PluginParams>,
}

#[derive(Params)]
struct PluginParams {
    #[id = "one"]
    pub one: FloatParam,

    #[nested(group = "inner")]
    pub two_and_three: InnerParams,

    #[id = "four"]
    pub four: FloatParam,
}

#[derive(Params)]
struct InnerParams {
    #[id = "two"]
    pub two: FloatParam,

    #[id = "three"]
    pub three: FloatParam,
}

impl Default for ParamOrderExample {
    fn default() -> Self {
        Self {
            params: Arc::new(PluginParams::default()),
        }
    }
}

impl Default for PluginParams {
    fn default() -> Self {
        Self {
            one: FloatParam::new("One", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            two_and_three: InnerParams {
                two: FloatParam::new("Two", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
                three: FloatParam::new("Three", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
            },
            four: FloatParam::new("Four", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }),
        }
    }
}

impl Plugin for ParamOrderExample {
    const NAME: &'static str = "Param Order Example";
    const VENDOR: &'static str = "Your Name";
    const URL: &'static str = "https://youtu.be/dQw4w9WgXcQ";
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_INPUT_CHANNELS: u32 = 2;
    const DEFAULT_OUTPUT_CHANNELS: u32 = 2;

    const DEFAULT_AUX_INPUTS: Option<AuxiliaryIOConfig> = None;
    const DEFAULT_AUX_OUTPUTS: Option<AuxiliaryIOConfig> = None;

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn accepts_bus_config(&self, config: &BusConfig) -> bool {
        // This works with any symmetrical IO layout
        config.num_input_channels == config.num_output_channels && config.num_input_channels > 0
    }

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }
}

impl ClapPlugin for ParamOrderExample {
    const CLAP_ID: &'static str = "com.your-domain.param-order-example";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A short description of your plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Utility];
}

impl Vst3Plugin for ParamOrderExample {
    const VST3_CLASS_ID: [u8; 16] = *b"param_order_expl";
    const VST3_CATEGORIES: &'static str = "Fx|Tools";
}

nih_export_clap!(ParamOrderExample);
nih_export_vst3!(ParamOrderExample);
