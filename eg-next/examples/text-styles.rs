//! # Example: Text styles
//!
//! Display a sentence of text using different styles, colors an decorations.

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_8X13, FONT_8X13_BOLD, FONT_8X13_ITALIC},
        MonoTextStyleBuilder,
    },
    pixelcolor::Rgb888,
    prelude::*,
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(512, 128));

    let normal = MonoTextStyleBuilder::new()
        .font(&FONT_8X13)
        .text_color(Rgb888::WHITE)
        .build();

    let underline = MonoTextStyleBuilder::from(&normal)
        .text_color(Rgb888::CSS_YELLOW)
        .underline()
        .build();

    let strikethrough = MonoTextStyleBuilder::from(&normal)
        .strikethrough_with_color(Rgb888::RED)
        .build();

    let background = MonoTextStyleBuilder::from(&normal)
        .background_color(Rgb888::CSS_WHEAT)
        .text_color(Rgb888::CSS_TOMATO)
        .build();

    let bold = MonoTextStyleBuilder::from(&normal)
        .font(&FONT_8X13_BOLD)
        .build();

    let italic = MonoTextStyleBuilder::from(&normal)
        .font(&FONT_8X13_ITALIC)
        .build();

    // First line
    let position =
        Text::new("A sentence with normal, ", Point::new(15, 15), normal).draw(&mut display)?;
    let position = Text::new("yellow underline", position, underline).draw(&mut display)?;
    let position = Text::new(", ", position, normal).draw(&mut display)?;
    let position = Text::new("red strikethrough", position, strikethrough).draw(&mut display)?;
    Text::new(", ", position, normal).draw(&mut display)?;

    // Second line
    let position = Text::new(
        "bold",
        Point::new(15, 15 + FONT_8X13.character_size.height as i32),
        bold,
    )
    .draw(&mut display)?;
    let position = Text::new(", ", position, normal).draw(&mut display)?;
    let position = Text::new("highlighted", position, background).draw(&mut display)?;
    let position = Text::new(" and ", position, normal).draw(&mut display)?;
    let position = Text::new("italic", position, italic).draw(&mut display)?;
    Text::new(" text!", position, normal).draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Text styles", &output_settings).show_static(&display);

    Ok(())
}
