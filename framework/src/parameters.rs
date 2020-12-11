use std::convert::TryFrom;

use embedded_graphics::{
    fonts::{Font6x8, Text},
    prelude::*,
    style::{MonoTextStyle, MonoTextStyleBuilder},
};
use embedded_graphics_simulator::{SimulatorEvent, Window};
use sdl2::{
    keyboard::Keycode,
    mouse::{MouseButton, MouseWheelDirection},
};

use crate::Parameter;

pub struct Parameters {
    selected: usize,
    active: bool,
    mouse_button_down: bool,
    parameters: Vec<(String, Parameter)>,
}

impl Parameters {
    pub(crate) fn new(parameters: Vec<(String, Parameter)>) -> Self {
        Self {
            selected: 0,
            active: false,
            mouse_button_down: false,
            parameters,
        }
    }

    pub fn get<T: TryFrom<Parameter>>(&self, name: &str) -> T {
        T::try_from(*self.get_by_name(name))
            .unwrap_or_else(|_| panic!("wrong parameter type: {}", name))
    }

    fn get_by_name(&self, name: &str) -> &Parameter {
        &self
            .parameters
            .iter()
            .find(|(n, _)| n == name)
            .unwrap_or_else(|| panic!("couldn't find parameter: {}", name))
            .1
    }

    pub(crate) fn draw_menu<T>(&self, target: &mut T, color: T::Color) -> Result<(), T::Error>
    where
        T: DrawTarget,
    {
        let max_name_width = self
            .parameters
            .iter()
            .map(|(name, _)| name.len())
            .max()
            .unwrap_or(0);

        let name_delta = Point::new(6, 0);
        let value_delta = name_delta + Point::new((max_name_width as i32 + 1) * 6, 0);

        let style = MonoTextStyle::new(Font6x8, color);
        let style_inverted = MonoTextStyleBuilder::new(Font6x8)
            .background_color(color)
            .build();

        let mut position = Point::new(2, 2);

        for (index, (name, value)) in self.parameters.iter().enumerate() {
            if index == self.selected {
                Text::new(">", position).into_styled(style).draw(target)?;
            }

            if index == self.selected && self.active {
                Text::new(name, position + name_delta)
                    .into_styled(style_inverted)
                    .draw(target)?;
            } else {
                Text::new(name, position + name_delta)
                    .into_styled(style)
                    .draw(target)?;
            }

            Text::new(&value.to_string(), position + value_delta)
                .into_styled(style)
                .draw(target)?;

            position.y += 8;
        }

        Ok(())
    }

    pub(crate) fn handle_events(&mut self, window: &mut Window) -> bool {
        for event in window.events() {
            let event = match event {
                SimulatorEvent::Quit => return true,
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Up => Event::Up,
                    Keycode::Down => Event::Down,
                    Keycode::Left => Event::Left,
                    Keycode::Right => Event::Right,
                    Keycode::Space | Keycode::Return => Event::Activate,
                    _ => continue,
                },
                SimulatorEvent::MouseButtonDown { mouse_btn, point }
                    if mouse_btn == MouseButton::Left =>
                {
                    self.mouse_button_down = true;
                    Event::MouseMove(point)
                }
                SimulatorEvent::MouseButtonDown { mouse_btn, .. }
                    if mouse_btn == MouseButton::Middle =>
                {
                    Event::Activate
                }
                SimulatorEvent::MouseMove { point } if self.mouse_button_down => {
                    Event::MouseMove(point)
                }
                SimulatorEvent::MouseButtonUp { .. } => {
                    self.mouse_button_down = false;
                    continue;
                }
                SimulatorEvent::MouseWheel {
                    mut scroll_delta,
                    direction,
                } => {
                    if direction == MouseWheelDirection::Flipped {
                        scroll_delta = scroll_delta.component_mul(Point::new(-1, -1));
                    }

                    if scroll_delta == Point::new(0, 1) {
                        Event::Up
                    } else if scroll_delta == Point::new(0, -1) {
                        Event::Down
                    } else {
                        continue;
                    }
                }
                _ => continue,
            };

            match event {
                Event::Up if !self.active => {
                    if self.selected > 0 {
                        self.selected -= 1;
                    } else {
                        self.selected = self.parameters.len() - 1;
                    }
                }
                Event::Down if !self.active => {
                    self.selected += 1;
                    if self.selected >= self.parameters.len() {
                        self.selected = 0;
                    }
                }
                Event::Activate => self.active ^= true,
                _ => self.parameters[self.selected].1.handle_event(event),
            }
        }

        false
    }
}

pub(crate) enum Event {
    Up,
    Down,
    Left,
    Right,
    Activate,
    MouseMove(Point),
}
