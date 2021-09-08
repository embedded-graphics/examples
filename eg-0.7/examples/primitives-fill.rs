//! # Example: Primitive fill styles
//!
//! This example demonstrates the different fill and stroke styles available for primitives.

use core::convert::Infallible;
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        Circle, CornerRadii, Ellipse, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle,
        RoundedRectangle, Triangle,
    },
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

static CIRCLE_SIZE: i32 = 65;
static ELLIPSE_SIZE: Size = Size::new(90, 65);

fn draw_shapes<T>(target: &mut T, style: PrimitiveStyle<Rgb888>) -> Result<(), T::Error>
where
    T: DrawTarget<Color = Rgb888>,
{
    Circle::new(Point::new(0, 0), CIRCLE_SIZE as u32)
        .into_styled(style)
        .draw(target)?;

    Rectangle::new(Point::new(105, 0), Size::new(64, 64))
        .into_styled(style)
        .draw(target)?;

    Triangle::new(Point::new(33, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new(96 * 2 + 16, 0))
        .into_styled(style)
        .draw(target)?;

    Ellipse::new(Point::new(24, 108), ELLIPSE_SIZE)
        .into_styled(style)
        .draw(target)?;

    RoundedRectangle::new(
        Rectangle::new(Point::new(32, 0), Size::new(64, 64)),
        CornerRadii::new(Size::new(16, 16)),
    )
    .translate(Point::new(96 + 24, 108))
    .into_styled(style)
    .draw(target)
}

fn main() -> Result<(), Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(325, 220));

    let stroke = PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1);

    let stroke_off_fill_off = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(1)
        .fill_color(Rgb888::GREEN)
        .build();

    let stroke_off_fill_on = PrimitiveStyle::with_fill(Rgb888::YELLOW);

    draw_shapes(&mut display.translated(Point::new(8, 8)), stroke)?;
    draw_shapes(
        &mut display.translated(Point::new(24, 24)),
        stroke_off_fill_on,
    )?;
    draw_shapes(
        &mut display.translated(Point::new(40, 40)),
        stroke_off_fill_off,
    )?;

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    Window::new("Filled primitives", &output_settings).show_static(&display);

    Ok(())
}
