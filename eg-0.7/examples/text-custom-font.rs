//! # Example: Custom font
//!
//! Shows how to implement a custom `SevenSegmentFont` font using the `MonoFont` struct. This font
//! renders numbers only and emulates a classic 7 segment display.

use embedded_graphics::{
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

const SEVENT_SEGMENT_FONT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(include_bytes!("assets/seven-segment-font.raw"), 224),
    glyph_mapping: &StrGlyphMapping::new("0123456789", 0),
    character_size: Size::new(22, 40),
    character_spacing: 4,
    baseline: 7,
    underline: DecorationDimensions::default_underline(40),
    strikethrough: DecorationDimensions::default_strikethrough(40),
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 128));

    let character_style = MonoTextStyle::new(&SEVENT_SEGMENT_FONT, BinaryColor::On);
    let text_style = TextStyleBuilder::new()
        .baseline(Baseline::Bottom)
        .alignment(Alignment::Center)
        .build();

    Text::with_text_style(
        "123\n456",
        display.bounding_box().center(),
        character_style,
        text_style,
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Custom font", &output_settings).show_static(&display);

    Ok(())
}
