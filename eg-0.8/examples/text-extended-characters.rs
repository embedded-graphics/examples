//! # Example: Extended characters
//!
//! Demonstrate ability of all built in fonts to render extended characters.

use embedded_graphics::{
    mono_font::{
        iso_8859_1::{FONT_10X20, FONT_6X12, FONT_6X9, FONT_8X13},
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
    Text::new(
        &format!("Font6x9 {}", test_text),
        Point::new(15, 15),
        MonoTextStyle::new(&FONT_6X9, BinaryColor::On),
    )
    .draw(&mut display)?;

    // Show 6x12 font
    Text::new(
        &format!("Font6x12 {}", test_text),
        Point::new(15, 30),
        MonoTextStyle::new(&FONT_6X12, BinaryColor::On),
    )
    .draw(&mut display)?;

    // Show 8x13 Font
    Text::new(
        &format!("Font8x13 {}", test_text),
        Point::new(15, 45),
        MonoTextStyle::new(&FONT_8X13, BinaryColor::On),
    )
    .draw(&mut display)?;

    // Show 10x20 Font
    Text::new(
        &format!("Font10x20 {}", test_text),
        Point::new(15, 65),
        MonoTextStyle::new(&FONT_10X20, BinaryColor::On),
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    Window::new("Fonts", &output_settings).show_static(&display);

    Ok(())
}
