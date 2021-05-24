//! # Example: Triangles
//!
//! Shows multiple triangles with different styles.

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment, Triangle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(600, 128));

    let padding = Point::new(10, 20);
    let size = display.bounding_box().size.height as i32 - (padding.y * 2);
    let half_size = size / 2;
    let offset = size + 10;

    // Triangle pointing up
    let base_triangle = Triangle::new(
        Point::new(0, size),
        Point::new(half_size, 0),
        Point::new(size, size),
    )
    .translate(padding);

    // Triangle pointing down
    let flipped_triangle = Triangle::new(
        Point::new(0, 0),
        Point::new(size, 0),
        Point::new(half_size, size),
    )
    .translate(padding);

    let inside_thick_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::CSS_SALMON)
        .stroke_width(10)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();

    let center_stroke_fill = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::CSS_AQUAMARINE)
        .stroke_width(10)
        .fill_color(Rgb888::CSS_CADET_BLUE)
        .build();

    let outside_stroke_fill = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::CSS_FIRE_BRICK)
        .stroke_width(9)
        .stroke_alignment(StrokeAlignment::Outside)
        .fill_color(Rgb888::CSS_WHITE_SMOKE)
        .build();

    let thick_stroke = PrimitiveStyleBuilder::new()
        .stroke_alignment(StrokeAlignment::Inside)
        .stroke_width(20)
        .stroke_color(Rgb888::CSS_DARK_TURQUOISE)
        .build();

    // Inside thick stroke, no fill
    base_triangle
        .into_styled(inside_thick_stroke)
        .draw(&mut display)?;

    // Center stroke alignment with fill
    flipped_triangle
        .translate(Point::new(offset, 0))
        .into_styled(center_stroke_fill)
        .draw(&mut display)?;

    // Outside stroke alignment with fill
    base_triangle
        .translate(Point::new(offset * 2, 0))
        .into_styled(outside_stroke_fill)
        .draw(&mut display)?;

    // Fill only
    flipped_triangle
        .translate(Point::new(offset * 3, 0))
        .into_styled(PrimitiveStyle::with_fill(Rgb888::CSS_CORAL))
        .draw(&mut display)?;

    // 1px stroke, no fill
    base_triangle
        .translate(Point::new(offset * 4, 0))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::WHITE, 1))
        .draw(&mut display)?;

    // Really thick stroke with inside alignment
    flipped_triangle
        .translate(Point::new(offset * 5, 0))
        .into_styled(thick_stroke)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Triangles", &output_settings).show_static(&display);

    Ok(())
}
