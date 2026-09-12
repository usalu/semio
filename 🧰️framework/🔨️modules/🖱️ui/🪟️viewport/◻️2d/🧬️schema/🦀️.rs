use protocol::value::{DslValue, FromValue, ToValue, ValueError};

/// 🪟️ Two-dimensional navigation retained by one concrete window instance.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
pub struct Viewport2d {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

impl<'de> serde::Deserialize<'de> for Viewport2d {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Viewport2dVisitor;

        impl<'de> serde::de::Visitor<'de> for Viewport2dVisitor {
            type Value = Viewport2d;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("a Viewport2d object") }

            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let (mut x, mut y, mut zoom) = (None, None, None);
                while let Some(field) = map.next_key::<String>()? {
                    match field.as_str() {
                        "x" => {
                            if x.is_some() { return Err(serde::de::Error::duplicate_field("x")); }
                            x = Some(map.next_value()?);
                        }
                        "y" => {
                            if y.is_some() { return Err(serde::de::Error::duplicate_field("y")); }
                            y = Some(map.next_value()?);
                        }
                        "zoom" => {
                            if zoom.is_some() { return Err(serde::de::Error::duplicate_field("zoom")); }
                            zoom = Some(map.next_value()?);
                        }
                        _ => return Err(serde::de::Error::unknown_field(&field, &["x", "y", "zoom"])),
                    }
                }
                let viewport = Viewport2d {
                    x: x.ok_or_else(|| serde::de::Error::missing_field("x"))?,
                    y: y.ok_or_else(|| serde::de::Error::missing_field("y"))?,
                    zoom: zoom.ok_or_else(|| serde::de::Error::missing_field("zoom"))?,
                };
                viewport.validate().map_err(serde::de::Error::custom)?;
                Ok(viewport)
            }
        }

        deserializer.deserialize_map(Viewport2dVisitor)
    }
}

impl Viewport2d {
    pub fn validate(&self) -> Result<(), ValueError> {
        crate::finite(self.x, "x")?;
        crate::finite(self.y, "y")?;
        crate::zoom(self.zoom)
    }
}

impl Default for Viewport2d {
    fn default() -> Self { Self { x: 0.0, y: 0.0, zoom: 1.0 } }
}

impl ToValue for Viewport2d {
    fn to_value(&self) -> DslValue {
        DslValue::object([("x".into(), self.x.to_value()), ("y".into(), self.y.to_value()), ("zoom".into(), self.zoom.to_value())])
    }
}

impl FromValue for Viewport2d {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = crate::fields(value, &["x", "y", "zoom"])?;
        let pose = Self { x: crate::take(&mut entries, "x")?, y: crate::take(&mut entries, "y")?, zoom: crate::take(&mut entries, "zoom")? };
        pose.validate()?;
        Ok(pose)
    }
}
