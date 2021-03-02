//! # Example: Fonts
//!
//! Demonstrate the available builtin fonts.

use embedded_graphics::{
    mono_font::{
        ascii::{Font10x20, Font5x8, Font6x12, Font9x15},
        MonoTextStyle, MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(350, 200));

    // Show smallest font with black font on white background (default value for fonts)
    Text::new("Hello World! - default style 6x8", Point::new(15, 15))
        .into_styled(MonoTextStyle::new(Font5x8, BinaryColor::On))
        .draw(&mut display)?;

    // Show smallest font with white font on black background
    let style = MonoTextStyleBuilder::new()
        .font(Font5x8)
        .text_color(BinaryColor::Off)
        .background_color(BinaryColor::On)
        .build();

    Text::new("Hello World! - inverse 6x8", Point::new(15, 30))
        .into_styled(style)
        .draw(&mut display)?;

    // Show 6x12 Font
    Text::new("Hello 6x12!", Point::new(15, 45))
        .into_styled(MonoTextStyle::new(Font6x12, BinaryColor::On))
        .draw(&mut display)?;

    // Show 9x15 Font
    Text::new("Hello 9x15!", Point::new(15, 70))
        .into_styled(MonoTextStyle::new(Font9x15, BinaryColor::On))
        .draw(&mut display)?;

    // Show 10x20 Font
    Text::new("Hello 10x20!", Point::new(15, 95))
        .into_styled(MonoTextStyle::new(Font10x20, BinaryColor::On))
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
