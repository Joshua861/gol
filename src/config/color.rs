use nannou::color::Srgb;
use serde::de::{SeqAccess, Visitor};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

struct ColorVisitor;

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of three integers")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let r = seq.next_element::<u8>()?.unwrap_or(0);
        let g = seq.next_element::<u8>()?.unwrap_or(0);
        let b = seq.next_element::<u8>()?.unwrap_or(0);

        Ok(Color::new_u8(r, g, b))
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_seq(ColorVisitor)
    }
}

impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let r = (self.r * 255.0).round() as i64;
        let g = (self.g * 255.0).round() as i64;
        let b = (self.b * 255.0).round() as i64;

        let color_array = [r, g, b];
        color_array.serialize(serializer)
    }
}

impl Color {
    pub fn to_srgb(&self) -> Srgb {
        Srgb::new(self.r, self.g, self.b)
    }
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
    pub fn new_u8(r: u8, g: u8, b: u8) -> Self {
        Color {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
        }
    }
}
