use protocol::value::{DslValue, FromValue, ToValue, ValueError};

/// 🌐️ Orbit navigation pose; projection and authored scene cameras have separate owners.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
pub struct Viewport3dOrbit {
    pub position: [f64; 3],
    pub target: [f64; 3],
    pub zoom: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up: Option<[f64; 3]>,
}

impl<'de> serde::Deserialize<'de> for Viewport3dOrbit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Viewport3dOrbitVisitor;

        impl<'de> serde::de::Visitor<'de> for Viewport3dOrbitVisitor {
            type Value = Viewport3dOrbit;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("a Viewport3dOrbit object") }

            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let (mut position, mut target, mut zoom, mut up) = (None, None, None, None);
                while let Some(field) = map.next_key::<String>()? {
                    match field.as_str() {
                        "position" => {
                            if position.is_some() { return Err(serde::de::Error::duplicate_field("position")); }
                            position = Some(map.next_value()?);
                        }
                        "target" => {
                            if target.is_some() { return Err(serde::de::Error::duplicate_field("target")); }
                            target = Some(map.next_value()?);
                        }
                        "zoom" => {
                            if zoom.is_some() { return Err(serde::de::Error::duplicate_field("zoom")); }
                            zoom = Some(map.next_value()?);
                        }
                        "up" => {
                            if up.is_some() { return Err(serde::de::Error::duplicate_field("up")); }
                            up = Some(map.next_value::<[f64; 3]>()?);
                        }
                        _ => return Err(serde::de::Error::unknown_field(&field, &["position", "target", "zoom", "up"])),
                    }
                }
                let viewport = Viewport3dOrbit {
                    position: position.ok_or_else(|| serde::de::Error::missing_field("position"))?,
                    target: target.ok_or_else(|| serde::de::Error::missing_field("target"))?,
                    zoom: zoom.ok_or_else(|| serde::de::Error::missing_field("zoom"))?,
                    up,
                };
                viewport.validate().map_err(serde::de::Error::custom)?;
                Ok(viewport)
            }
        }

        deserializer.deserialize_map(Viewport3dOrbitVisitor)
    }
}

impl Viewport3dOrbit {
    pub fn validate(&self) -> Result<(), ValueError> {
        for (name, coordinates) in [("position", Some(&self.position)), ("target", Some(&self.target)), ("up", self.up.as_ref())] {
            if let Some(coordinates) = coordinates {
                for value in coordinates { crate::finite(*value, name)?; }
            }
        }
        crate::zoom(self.zoom)
    }
}

impl ToValue for Viewport3dOrbit {
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("position".into(), self.position.to_value()), ("target".into(), self.target.to_value()), ("zoom".into(), self.zoom.to_value())];
        if let Some(up) = &self.up { entries.push(("up".into(), up.to_value())); }
        DslValue::object(entries)
    }
}

impl FromValue for Viewport3dOrbit {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = crate::fields(value, &["position", "target", "zoom", "up"])?;
        let up = if entries.iter().any(|(name, _)| name == "up") { Some(crate::take(&mut entries, "up")?) } else { None };
        let pose = Self { position: crate::take(&mut entries, "position")?, target: crate::take(&mut entries, "target")?, zoom: crate::take(&mut entries, "zoom")?, up };
        pose.validate()?;
        Ok(pose)
    }
}
