//! # Example: Multiline text box using embedded-text
//!
//! This example demonstrates drawing a piece of multiline text that is both horizontally and
//! vertically centered. The example is using the TextBox from embedded-text. For more examples,
//! check out the [embedded-text repository](https://github.com/embedded-graphics/embedded-text).

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::new(Size::new(128, 128));

    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();

    TextBox::with_textbox_style(
        "This is a\nmultiline\nHello World!",
        display.bounding_box(),
        character_style,
        textbox_style,
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("TextBox alignment", &output_settings).show_static(&display);

    Ok(())
}
