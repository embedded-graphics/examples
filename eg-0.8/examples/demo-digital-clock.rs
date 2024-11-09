//! # Example: 7-Segment Digital Clock
//!
//! An example displaying a digital clock using the `eg-seven-segment` crate.

use std::time::Duration;

use chrono::prelude::*;
use eg_seven_segment::SevenSegmentStyleBuilder;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

/// Draws a digital clock with the current local time to the specified display
fn draw_clock<D>(display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    // Create a BinaryColor style
    let character_style = SevenSegmentStyleBuilder::new()
        .segment_color(BinaryColor::On)
        .build();

    // Create a centered alignment for text
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();

    let time = Local::now().format("%H:%M:%S").to_string();

    // Create text from current time and draw to display
    Text::with_text_style(
        &time,
        display.bounding_box().center(),
        character_style,
        text_style,
    )
    .draw(display)?;

    Ok(())
}

fn main() {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let mut window = Window::new("Digital clock", &settings);

    loop {
        display.clear(BinaryColor::Off).unwrap();
        draw_clock(&mut display).unwrap();

        window.update(&mut display);

        if window.events().any(|event| event == SimulatorEvent::Quit) {
            break;
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}
