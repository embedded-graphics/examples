//! # Example: Custom font
//!
//! Shows how to implement the `Font` trait for a custom `SevenSegmentFont` font. This font renders
//! numbers only and emulates a classic 7 segment display.

use embedded_graphics::{
    mono_font::MonoTextStyle,
    pixelcolor::BinaryColor,
    prelude::*,
    text::{HorizontalAlignment, Text, TextStyleBuilder, VerticalAlignment},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct SevenSegmentFont;

impl MonoFont for SevenSegmentFont {
    const FONT_IMAGE: &'static [u8] = include_bytes!("assets/seven-segment-font.raw");
    const FONT_IMAGE_WIDTH: u32 = 224;

    const CHARACTER_SIZE: Size = Size::new(22, 40);
    const CHARACTER_SPACING: u32 = 4;

    // TODO: add line height when MonoFont is updated
    // const LINE_HEIGHT: u32 = 50;

    fn char_offset(c: char) -> u32 {
        c.to_digit(10).unwrap_or(0)
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 128));

    let character_style = MonoTextStyle::new(SevenSegmentFont, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .character_style(character_style)
        .vertical_alignment(VerticalAlignment::Bottom)
        .horizontal_alignment(HorizontalAlignment::Center)
        .build();

    Text::new("123\n456", display.bounding_box().center())
        .into_styled(text_style)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Custom font", &output_settings).show_static(&display);

    Ok(())
}
