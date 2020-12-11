use embedded_graphics::{
    pixelcolor::{BinaryColor, PixelColor, Rgb888},
    prelude::*,
};
use embedded_graphics_simulator::{
    OutputSettings, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use std::time::{Duration, Instant};

use crate::{Parameter, Parameters};

const MIN_FRAME_DURATION: Duration = Duration::from_millis(1000 / 60);

pub struct Builder<C: PixelColor> {
    title: String,
    display_size: Size,
    output_settings: OutputSettings,
    clear_color: C,
    menu_color: C,
    parameters: Vec<(String, Parameter)>,
}

impl<C> Builder<C>
where
    C: PixelColor + From<BinaryColor> + Into<Rgb888>,
{
    pub fn new() -> Self {
        Self {
            title: "No title".to_string(),
            display_size: Size::new(256, 256),
            output_settings: OutputSettings::default(),
            clear_color: BinaryColor::Off.into(),
            menu_color: BinaryColor::On.into(),
            parameters: Vec::new(),
        }
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();

        self
    }

    pub fn display_size(mut self, size: Size) -> Self {
        self.display_size = size;

        self
    }

    pub fn output_settings<F>(mut self, f: F) -> Self
    where
        F: FnOnce(OutputSettingsBuilder) -> OutputSettingsBuilder,
    {
        let builder = OutputSettingsBuilder::new();

        self.output_settings = f(builder).build();

        self
    }

    pub fn clear_color(mut self, color: C) -> Self {
        self.clear_color = color;

        self
    }

    pub fn menu_color(mut self, color: C) -> Self {
        self.menu_color = color;

        self
    }

    pub fn parameter<T: Into<Parameter>>(mut self, name: &str, value: T) -> Self {
        self.parameters.push((name.to_string(), value.into()));

        self
    }

    pub fn run<F>(self, f: F)
    where
        F: Fn(
            &mut SimulatorDisplay<C>,
            &Parameters,
        ) -> Result<(), <SimulatorDisplay<C> as DrawTarget>::Error>,
    {
        let mut parameters = Parameters::new(self.parameters);

        let mut window = Window::new(&self.title, &self.output_settings);
        let mut display = SimulatorDisplay::new(self.display_size);

        loop {
            let start = Instant::now();

            display.clear(self.clear_color).unwrap();

            f(&mut display, &parameters).unwrap();

            parameters.draw_menu(&mut display, self.menu_color).unwrap();

            window.update(&display);

            if parameters.handle_events(&mut window) {
                break;
            }

            let frame_duration = start.elapsed();
            if frame_duration < MIN_FRAME_DURATION {
                std::thread::sleep(MIN_FRAME_DURATION - frame_duration);
            }
        }
    }
}
