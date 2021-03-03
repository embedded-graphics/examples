//! # Example: Extended characters
//!
//! Demonstrate ability of most built in fonts to render extended characters.

use embedded_graphics::{
    mono_font::{
        latin1::{Font10x20, Font6x12, Font6x9, Font8x13},
        MonoTextStyle,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(1400, 160));

    let test_text  = "¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";

    // Show smallest font with black font on white background (default value for fonts)
    Text::new(&format!("Font6x9 {}", test_text), Point::new(15, 15))
        .into_styled(MonoTextStyle::new(Font6x9, BinaryColor::On))
        .draw(&mut display)?;

    // Show 6x12 font
    Text::new(&format!("Font6x12 {}", test_text), Point::new(15, 30))
        .into_styled(MonoTextStyle::new(Font6x12, BinaryColor::On))
        .draw(&mut display)?;

    // Show 8x13 Font
    Text::new(&format!("Font8x13 {}", test_text), Point::new(15, 45))
        .into_styled(MonoTextStyle::new(Font8x13, BinaryColor::On))
        .draw(&mut display)?;

    // Show 10x20 Font
    Text::new(&format!("Font10x20 {}", test_text), Point::new(15, 65))
        .into_styled(MonoTextStyle::new(Font10x20, BinaryColor::On))
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
