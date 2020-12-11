use embedded_graphics::{pixelcolor::Rgb565, prelude::*, primitives::Line, style::PrimitiveStyle};
use embedded_graphics_simulator::SimulatorDisplay;
use framework::{Builder, Parameters};

fn main() {
    Builder::new()
        .title("Line debugger")
        .display_size(Size::new(256, 256))
        .output_settings(|builder| builder.scale(3))
        .parameter("start", Point::new(128, 128))
        .parameter("end", Point::new(150, 170))
        .parameter("stroke", 1u32)
        .run(draw);
}

fn draw(
    display: &mut SimulatorDisplay<Rgb565>,
    parameters: &Parameters,
) -> Result<(), std::convert::Infallible> {
    let start: Point = parameters.get("start");
    let end: Point = parameters.get("end");
    let stroke_width = parameters.get("stroke");

    Line::new(start, end)
        .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, stroke_width))
        .draw(display)
}
