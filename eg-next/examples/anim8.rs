//! # Example: Analog Clock
//!
//! ![Screenshot of clock example]( data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAfwAAAH9AQMAAADFwFz1AAAABlBMVEUAAAD///+l2Z/dAAAFM0lEQVR42u3dYWrjMBAF4Afz1zBX8QEEc/WBvdAcQKBNussK3EbVpDNSt+SRYMWWPrBsybQJCK+88sp/EWm32BagN+/EcoBuLRUE8K2wA6B7c9yBN2I9QK3+2eItrS0HWv0L9c9LgV6f+p6lADdcAWq6AuiVO9DRhYBYty571wBc8RGApquApkM4H+A6lvOBpkM6H+A6tvOBpkM8H6A61BcAosMetnSA6olBzpYOsI0B0Wyg4cAgSjUZIPsEgCQDoqCRD7DlAvX2Hlyo414lFSD7HJAsoI/lc3An9xGdBFRMpCYCZDOAJAKsMwBrHiCYCVka0I+MI0GAvw97vSxAMBeyLKBiMjUJIJsFJAjwX8VeMwcQzIYsB6iYTk0ByOYBCQL8V7HXzQAE8yHLAAocSQHMA5QEgNQDcALAPkDjAYEnZPFAhSs1HjAfUMIBUh/A4QB7AY0GBL6QRQMFzoQD5gVKMEBuQKIB9QIcDDC8IY0FxA9YLFDgTjBQ/UCNBcwPlFCA1A9wKMBPAE1DgeI/AwoFxN+LJ1kkUHD655NQoLrntAOokYAB6h9NgQAZnohEAvoMwFHA+9HcHuUCaCCAZwAKBOQ5wOKAcgGU7q/WLoULgB8FXMZSlVqkisq1cAHKDwJIrzdSlXZvdysU6YUrwD8ZqHR/3VKpKeRvgVoawHgOIE0DfrX7S6X35q/bpkoaIM8CFgWUy4G3VnegSr21fSt8BOAnA2+75O9cet9WQG7lNMCeBUoW0B7kBwOkzwIcB/iSD+h2oKwGnv8XMWkSgHM7wNsB2Fqg4OlzsDQAuh0o2wHS3QDO7QCvBGxw7JOUTODYDpDuBlC2A6S7AZzbAd4OwBYB/XL5xxMnA6S7AZTtAG0HYNsB3g7AWDcDR6ubAfoGQFsAtGbPAdLal4Fv0gcjAK1+EfgGN9IXgcK6GVBsBhi7gXM7oDdjK8DYDZxrANioD8dASQYYO4F/bY+NAOlu4MBu4NwNkG4FekvKBUZ/WtkEYJkAYTdwbAesz6x7AMJu4FgJ8INDnwOkeQBhN1B68dwD6FKAdNCHQ4CRBpTtgO4G+DKws4H3T9dzEihpgO4GGKuBMujDIWBZgF6G9nKAsRyQQR+OAAoD+DJXTwOaAxxYCPTCaBLVwYyWAZDuBg5sAGDosXmgpACELUAv4XB8fWwpgO0B5NKHcwAFAtz70ANoIPCvaA6AA4E+O3q+QWfEAf2CeAAJBPAUUCKBCqgPOIAaCRQwfAApLBIQnHD+iOCkUIBJvQCThgIVXgAtFCD1A4xIYDyaPtZLLFD9QB0CbqD4AYsFxA1QMMB+QGOB/nH26coIBswLSDAAN1CigeIFLBoQJ0DhAKvv6coaDZAXQDQA8wElHqg+oMYD4gLI4gF2/daTNR4gH4B4AOYBSgZQPIBlAOIAKALw3En6vm4GQDYPCDIA1Hmg5gAyDZDlAKyzAGsOQDb7fBfkAKizQM0CZBIgywJY5wDWLIBsDhBkAZCp5ztZHsD6ALZxLT/g7UU9LicaALiGE4EulfIAUQD6wVXUfj3ZMgGy2xuPAep9mASgfg7UXEAUxwCAgi0XIMM5Ak4IcgE0nKNb+aSaDbDpaDgfotkAVR1NaUdDBOBfWkj7GeYDVEdAQz6Apo+frlxTgUGls+OpwONaZ7dXAFzfA51eAfR6HehyCOBfdu/o8BIAYleg710DUNP3ADesAnrlDnR0EYBWL0Dfkw90oQPU24cA/uVYqTV8Ffj/1rT9Buv6foO1jb/D+s6vvPKKI78B89G0YRkxkl8AAAAASUVORK5CYII=)
//!
//! This example shows some more advanced usage of Embedded Graphics. It draws a round clock face
//! with hour, minute and second hands. A digital clock is drawn in the middle of the clock. The
//! whole thing is updated with your computer's local time every 50ms.

use chrono::{Local, Timelike};
use core::f32::consts::PI;
use embedded_graphics::{
    mono_font::{ascii::FONT_9X15, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{
        self, Arc, Circle, Line, Polyline, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle,
        Sector, Triangle,
    },
    text::Text,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rand::{thread_rng, Rng};
use std::{
    thread,
    time::{Duration, Instant},
};

fn polar(circle: &Circle, angle: f32, radius: i32) -> Point {
    let radius = radius as f32;

    circle.center()
        + Point::new(
            (angle.sin() * radius) as i32,
            -(angle.cos() * radius) as i32,
        )
}

fn draw_static_crosshair<D>(target: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    let center = Point::new(80, 80);

    // Outside
    Circle::with_center(center, 149)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GREEN, 1))
        .draw(target)?;

    // Inside
    Circle::with_center(center, 15)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GREEN, 1))
        .draw(target)?;

    // Center crosshair point
    Pixel(center, Rgb888::CSS_GREEN).draw(target)?;

    // Crosshair lines
    {
        let offs = Size::new_equal(20);
        let len = Size::new_equal(20);

        let line_style = PrimitiveStyle::with_stroke(Rgb888::CSS_LIME_GREEN, 1);

        Line::new(
            center - offs.y_axis(),
            center - offs.y_axis() - len.y_axis(),
        )
        .into_styled(line_style)
        .draw(target)?;

        Line::new(
            center - offs.x_axis(),
            center - offs.x_axis() - len.x_axis(),
        )
        .into_styled(line_style)
        .draw(target)?;

        Line::new(
            center + offs.x_axis(),
            center + offs.x_axis() + len.x_axis(),
        )
        .into_styled(line_style)
        .draw(target)?;

        Line::new(
            center + offs.y_axis(),
            center + offs.y_axis() + len.y_axis(),
        )
        .into_styled(line_style)
        .draw(target)?;
    }

    Ok(())
}

fn draw_dynamic_crosshair<D>(target: &mut D, t: Duration) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    let center = Point::new(80, 80);

    let angle = (t.as_millis() / 10 % 360) as u32;
    let angle = (angle as f32).deg();

    Arc::with_center(center, 149, 0.0.deg() + angle, 45.0.deg())
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_alignment(primitives::StrokeAlignment::Outside)
                .stroke_width(5)
                .stroke_color(Rgb888::CSS_LIME_GREEN)
                .build(),
        )
        .draw(target)?;

    Arc::with_center(center, 149, -90.0.deg() - angle, -110.0.deg())
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_alignment(primitives::StrokeAlignment::Inside)
                .stroke_width(5)
                .stroke_color(Rgb888::CSS_LIME_GREEN)
                .build(),
        )
        .draw(target)?;

    Ok(())
}

fn draw_radar<D>(target: &mut D, t: Duration) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    let size = 89;
    let center = Point::new(200, 200);

    let base_angle = (t.as_millis() / 5 % 360) as u32;
    let angle = (base_angle as f32).deg();

    let radar = Circle::with_center(center, size);

    radar
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GREEN, 1))
        .draw(target)?;

    Sector::with_center(center, size, 90.0.deg() + angle, -60.0.deg())
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_LIME_GREEN)
                .build(),
        )
        .draw(target)?;

    {
        let pos = polar(&radar, 0.0, 20);
        let brightness = 255u32.saturating_sub(base_angle) as u8;
        let color = Rgb888::new(0, brightness, 0);

        Circle::with_center(pos, 3)
            .into_styled(PrimitiveStyle::with_fill(color))
            .draw(target)?;
    }

    Ok(())
}

fn draw_scanners<D>(target: &mut D, t: Duration) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    let frame = Rectangle::new(Point::new(200 - 89 / 2 + 10, 10), Size::new(79, 10));

    frame
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GREEN, 1))
        .draw(target)?;

    let w = frame.bounding_box().size.width - 10;

    let pos = ((t.as_millis() as u32 / 20) % w * 2) as u32;

    let pos = if pos > w {
        // Reverse
        w + (w - pos)
    } else {
        // Forward
        pos
    };

    Rectangle::new(
        Point::new(200 - 89 / 2 + 10 + pos as i32, 10),
        Size::new(10, 10),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb888::CSS_LIME_GREEN))
    .draw(target)?;

    // ---

    let frame = Rectangle::new(Point::new(200 - 89 / 2 + 10, 30), Size::new(79, 10));

    frame
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GREEN, 1))
        .draw(target)?;

    let w = frame.bounding_box().size.width - 10;

    let pos = ((t.as_millis() as u32 / 20 + 20) % w * 2) as u32;

    let pos = if pos > w {
        // Reverse
        w + (w - pos)
    } else {
        // Forward
        pos
    };

    Rectangle::new(
        Point::new(
            200 - 89 / 2 + 10 + pos as i32,
            frame.bounding_box().top_left.y,
        ),
        Size::new(10, 10),
    )
    .into_styled(PrimitiveStyle::with_fill(Rgb888::CSS_LIME_GREEN))
    .draw(target)?;

    Ok(())
}

fn draw_horizontal_rule<D>(target: &mut D, t: Instant, lerp: &mut Lerp) -> Result<(), D::Error>
where
    D: DrawTarget<Color = Rgb888>,
{
    Polyline::new(&[
        Point::new(10, 200 - 5),
        Point::new(10, 200),
        Point::new(130, 200),
        Point::new(130, 200 - 5),
    ])
    .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_DARK_GREEN, 1))
    .draw(target)?;

    Triangle::new(Point::zero(), Point::new(5, 10), Point::new(-5, 10))
        .translate(Point::new(
            10 + lerp.position(t.elapsed().as_millis() as u64) as i32,
            200,
        ))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CSS_LIME_GREEN, 1))
        .draw(target)?;

    Ok(())
}

struct Lerp {
    is_animating: bool,
    duration: u32,
    final_position: u32,
    start_position: u32,
    start: u64,
}

impl Lerp {
    fn position(&mut self, t: u64) -> u32 {
        let t = t - self.start;
        let t = (t as u32) as f32 / self.duration as f32;

        if t >= 1.0 {
            self.is_animating = false;
        }

        // Floating point LERP
        let pos = (1.0 - t) * self.start_position as f32 + t * self.final_position as f32;

        pos as u32
    }

    fn move_target(&mut self, new_start: u64, new_target: u32) {
        self.start_position = self.final_position;
        self.final_position = new_target;
        self.start = new_start;

        println!(
            "Reset to {} -> {}",
            self.start_position, self.final_position
        );

        self.is_animating = true;
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(256, 256));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Greebles", &output_settings);

    let start = Instant::now();

    let mut h_state = Lerp {
        is_animating: true,
        duration: 500,
        start_position: 0,
        final_position: 50,
        start: 0,
    };

    let mut rng = thread_rng();

    'running: loop {
        display.clear(Rgb888::BLACK)?;

        draw_static_crosshair(&mut display)?;
        draw_dynamic_crosshair(&mut display, start.elapsed())?;

        draw_radar(&mut display, start.elapsed())?;

        draw_scanners(&mut display, start.elapsed())?;

        draw_horizontal_rule(&mut display, start, &mut h_state)?;

        if !h_state.is_animating {
            h_state.move_target(start.elapsed().as_millis() as u64, rng.gen_range(0..=120));
        }

        window.update(&display);

        if window.events().any(|e| e == SimulatorEvent::Quit) {
            break 'running Ok(());
        }
        thread::sleep(Duration::from_millis(16));
    }
}
