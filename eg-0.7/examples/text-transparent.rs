//! # Example: Transparent fonts
//!
//! Demonstrate the background styles and transparency behaviors of different font styles.

use embedded_graphics::{
    mono_font::{ascii::FONT_6X9, MonoTextStyleBuilder},
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle, Rectangle},
    text::Text,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb565> = SimulatorDisplay::new(Size::new(256, 128));

    Circle::new(Point::new(0, 0), 41)
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(&mut display)
        .unwrap();

    Rectangle::new(Point::new(20, 20), Size::new(80, 60))
        .into_styled(PrimitiveStyle::with_fill(Rgb565::RED))
        .draw(&mut display)
        .unwrap();

    // Can also be written in the shorter form: TextStyle::new(&FONT_6X9, Rgb565::WHITE)
    let no_background = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(Rgb565::WHITE)
        .build();

    let filled_background = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(Rgb565::YELLOW)
        .background_color(Rgb565::BLUE)
        .build();

    let inverse_background = MonoTextStyleBuilder::new()
        .font(&FONT_6X9)
        .text_color(Rgb565::BLUE)
        .background_color(Rgb565::YELLOW)
        .build();

    Text::new(
        "Hello world! - no background",
        Point::new(15, 15),
        no_background,
    )
    .draw(&mut display)
    .unwrap();

    Text::new(
        "Hello world! - filled background",
        Point::new(15, 30),
        filled_background,
    )
    .draw(&mut display)
    .unwrap();

    Text::new(
        "Hello world! - inverse background",
        Point::new(15, 45),
        inverse_background,
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(3).build();
    Window::new("Fonts with transparent background", &output_settings).show_static(&display);

    Ok(())
}
