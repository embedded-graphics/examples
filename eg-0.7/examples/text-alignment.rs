//! # Example: Text alignment
//!
//! Draw left/center/right aligned text in a containing box.

use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(512, 128));

    let bounding_box = display.bounding_box();

    let character_style = MonoTextStyleBuilder::new()
        .font(&FONT_8X13)
        .text_color(Rgb888::CSS_TOMATO)
        .build();

    let left_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .baseline(Baseline::Top)
        .build();

    let center_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();

    let right_aligned = TextStyleBuilder::new()
        .alignment(Alignment::Right)
        .baseline(Baseline::Bottom)
        .build();

    Text::with_text_style(
        "Left aligned text, origin top left",
        bounding_box.top_left,
        character_style,
        left_aligned,
    )
    .draw(&mut display)?;

    Text::with_text_style(
        "Center aligned text, origin center center",
        bounding_box.center(),
        character_style,
        center_aligned,
    )
    .draw(&mut display)?;

    Text::with_text_style(
        "Right aligned text, origin bottom right",
        bounding_box.bottom_right().unwrap(),
        character_style,
        right_aligned,
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Text alignment", &output_settings).show_static(&display);

    Ok(())
}
