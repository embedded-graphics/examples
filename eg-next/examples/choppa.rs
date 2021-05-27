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
    image::Image,
    mono_font::{
        ascii::{FONT_10X20, FONT_6X13, FONT_7X13, FONT_8X13, FONT_9X15},
        MonoTextStyle,
    },
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    primitives::{
        self, Arc, Circle, Line, Polyline, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle,
        Sector, Triangle,
    },
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    sdl2::Keycode, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rand::{thread_rng, Rng};
use std::{
    thread,
    time::{Duration, Instant},
};
use tinybmp::Bmp;

fn polar(circle: &Circle, angle: f32, radius: i32) -> Point {
    let radius = radius as f32;

    circle.center()
        + Point::new(
            (angle.sin() * radius) as i32,
            -(angle.cos() * radius) as i32,
        )
}

enum State {
    Off,
    TitleOpening { start: Duration, offset: u32 },
    TitleIdle,
}

struct Game {
    state: State,
}

impl Game {
    fn new() -> Self {
        Self { state: State::Off }
    }

    fn update<D>(&mut self, target: &mut D, t: Duration) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        let new_state = match &mut self.state {
            State::Off => Some(State::TitleOpening {
                start: t,
                offset: 0,
            }),
            State::TitleOpening { start, offset } => {
                if *offset > target.bounding_box().size.height / 2 {
                    Some(State::TitleIdle)
                } else {
                    Self::draw_title(target, *offset, target.bounding_box().size.height / 2)?;

                    *offset = ((t - *start).as_millis() / 10) as u32;

                    None
                }
            }
            State::TitleIdle => {
                Self::draw_title(target, 0, 0)?;

                let opacity = 128.0 + ((((t.as_millis() / 100) as f32).sin() + 1.0) / 2.0) * 127.0;

                Self::draw_start_button(target, Rgb888::new(0, opacity as u8, 0))?;

                None
            }
        };

        if let Some(new_state) = new_state {
            self.state = new_state;
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.state = State::Off
    }

    fn draw_start_button<D>(target: &mut D, color: Rgb888) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        let small_font = MonoTextStyle::new(&FONT_6X13, color);

        Text::with_text_style(
            "Press space to start",
            target.bounding_box().center() + FONT_10X20.character_size.y_axis(),
            small_font,
            TextStyleBuilder::new()
                .alignment(Alignment::Center)
                .baseline(Baseline::Top)
                .build(),
        )
        .draw(target)?;

        Ok(())
    }

    fn draw_title<D>(target: &mut D, offset: u32, limit: u32) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        let limit = limit as i32;

        let small_font = MonoTextStyle::new(&FONT_6X13, Rgb888::CSS_LIME);
        let big_font = MonoTextStyle::new(&FONT_10X20, Rgb888::CSS_LIME);

        let base = TextStyleBuilder::new().alignment(Alignment::Center);

        let offset = Point::new(0, offset as i32);

        Text::with_text_style(
            "GET TO DA",
            target.bounding_box().center() - Point::new(0, limit) + offset,
            small_font,
            base.baseline(Baseline::Bottom).build(),
        )
        .draw(target)?;

        Text::with_text_style(
            "CHOPPA",
            target.bounding_box().center() + Point::new(0, limit) - offset,
            big_font,
            base.baseline(Baseline::Top).build(),
        )
        .draw(target)?;

        Ok(())
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(384, 256));

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("CHOPPA", &output_settings);

    let start = Instant::now();

    let mut game = Game::new();

    'running: loop {
        display.clear(Rgb888::BLACK)?;

        game.update(&mut display, start.elapsed())?;

        window.update(&display);

        // if window.events().any(|e| e == SimulatorEvent::Quit) {
        //     break 'running Ok(());
        // }

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running Ok(()),
                SimulatorEvent::KeyDown { keycode, .. } => {
                    match keycode {
                        Keycode::R => game.reset(),
                        _ => (), // TODO: More input
                    }
                }

                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(16));
    }
}