use embedded_graphics::prelude::*;
use std::{convert::TryFrom, fmt};

use crate::parameters::Event;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Parameter {
    U32(u32),
    I32(i32),
    Point(Point),
}

impl Parameter {
    pub(crate) fn handle_event(&mut self, event: Event) {
        match self {
            Self::U32(value) => match event {
                Event::Up | Event::Left => *value = value.saturating_sub(1),
                Event::Down | Event::Right => *value = value.saturating_add(1),
                _ => {}
            },
            Self::I32(value) => match event {
                Event::Up | Event::Left => *value = value.saturating_sub(1),
                Event::Down | Event::Right => *value = value.saturating_add(1),
                _ => {}
            },
            Self::Point(point) => match event {
                Event::Left => point.x = point.x.saturating_sub(1),
                Event::Right => point.x = point.x.saturating_add(1),
                Event::Up => point.y = point.y.saturating_sub(1),
                Event::Down => point.y = point.y.saturating_add(1),
                Event::MouseMove(p) => *point = p,
                _ => {}
            },
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Parameter::U32(value) => value.fmt(f),
            Parameter::I32(value) => value.fmt(f),
            Parameter::Point(Point { x, y }) => write!(f, "({}, {})", x, y),
        }
    }
}

impl From<i32> for Parameter {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl TryFrom<Parameter> for i32 {
    type Error = ();

    fn try_from(value: Parameter) -> Result<Self, Self::Error> {
        match value {
            Parameter::I32(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl From<u32> for Parameter {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl TryFrom<Parameter> for u32 {
    type Error = ();

    fn try_from(value: Parameter) -> Result<Self, Self::Error> {
        match value {
            Parameter::U32(value) => Ok(value),
            _ => Err(()),
        }
    }
}

impl From<Point> for Parameter {
    fn from(value: Point) -> Self {
        Self::Point(value)
    }
}

impl TryFrom<Parameter> for Point {
    type Error = ();

    fn try_from(value: Parameter) -> Result<Self, Self::Error> {
        match value {
            Parameter::Point(value) => Ok(value),
            _ => Err(()),
        }
    }
}
