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

macro_rules! projection_string_enum {
    ($name:ty, $($variant:path => $value:literal),+ $(,)?) => {
        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(match self { $($variant => $value),+ })
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                match <String as serde::Deserialize>::deserialize(deserializer)?.as_str() {
                    $($value => Ok($variant),)+
                    value => Err(serde::de::Error::unknown_variant(value, &[$($value),+])),
                }
            }
        }

        impl ToValue for $name {
            fn to_value(&self) -> DslValue {
                DslValue::String(match self { $($variant => $value),+ }.into())
            }
        }

        impl FromValue for $name {
            fn from_value(value: DslValue) -> Result<Self, ValueError> {
                match String::from_value(value)?.as_str() {
                    $($value => Ok($variant),)+
                    _ => Err(ValueError::new("unknown viewport projection value")),
                }
            }
        }
    };
}

/// 📐️ Selected projection family in a complete editable preference bank.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Viewport3dProjectionKind { Orthographic, Axonometric, Oblique, OnePoint, TwoPoint, ThreePoint, Curvilinear }

/// 🧭️ Cardinal viewport face shared by orthographic and active orientations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Viewport3dOrthographicView { Plan, Top, Bottom, Front, Back, Left, Right }

/// 📐️ Axonometric preset family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Viewport3dAxonometricVariant { Isometric, Dimetric, Trimetric }

/// 🧭️ Axonometric corner quadrant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Viewport3dAxonometricQuadrant { Ne, Nw, Se, Sw }

/// 🌐️ Optional active corner hemisphere.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Viewport3dAxonometricHemisphere { Upper, Lower }

/// 📐️ Oblique projection preset family.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Viewport3dObliqueVariant { Cabinet, Cavalier, Military }

/// 🧭️ One-point perspective axis selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Viewport3dOnePointAxis { X, Y, Z }

/// 🐟️ Curvilinear projection mapping.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Viewport3dCurvilinearMapping { Fisheye, Panini }

projection_string_enum!(Viewport3dProjectionKind,
    Viewport3dProjectionKind::Orthographic => "orthographic",
    Viewport3dProjectionKind::Axonometric => "axonometric",
    Viewport3dProjectionKind::Oblique => "oblique",
    Viewport3dProjectionKind::OnePoint => "onePoint",
    Viewport3dProjectionKind::TwoPoint => "twoPoint",
    Viewport3dProjectionKind::ThreePoint => "threePoint",
    Viewport3dProjectionKind::Curvilinear => "curvilinear",
);
projection_string_enum!(Viewport3dOrthographicView,
    Viewport3dOrthographicView::Plan => "plan",
    Viewport3dOrthographicView::Top => "top",
    Viewport3dOrthographicView::Bottom => "bottom",
    Viewport3dOrthographicView::Front => "front",
    Viewport3dOrthographicView::Back => "back",
    Viewport3dOrthographicView::Left => "left",
    Viewport3dOrthographicView::Right => "right",
);
projection_string_enum!(Viewport3dAxonometricVariant,
    Viewport3dAxonometricVariant::Isometric => "isometric",
    Viewport3dAxonometricVariant::Dimetric => "dimetric",
    Viewport3dAxonometricVariant::Trimetric => "trimetric",
);
projection_string_enum!(Viewport3dAxonometricQuadrant,
    Viewport3dAxonometricQuadrant::Ne => "ne",
    Viewport3dAxonometricQuadrant::Nw => "nw",
    Viewport3dAxonometricQuadrant::Se => "se",
    Viewport3dAxonometricQuadrant::Sw => "sw",
);
projection_string_enum!(Viewport3dAxonometricHemisphere,
    Viewport3dAxonometricHemisphere::Upper => "upper",
    Viewport3dAxonometricHemisphere::Lower => "lower",
);
projection_string_enum!(Viewport3dObliqueVariant,
    Viewport3dObliqueVariant::Cabinet => "cabinet",
    Viewport3dObliqueVariant::Cavalier => "cavalier",
    Viewport3dObliqueVariant::Military => "military",
);
projection_string_enum!(Viewport3dOnePointAxis,
    Viewport3dOnePointAxis::X => "x",
    Viewport3dOnePointAxis::Y => "y",
    Viewport3dOnePointAxis::Z => "z",
);
projection_string_enum!(Viewport3dCurvilinearMapping,
    Viewport3dCurvilinearMapping::Fisheye => "fisheye",
    Viewport3dCurvilinearMapping::Panini => "panini",
);

/// 📐️ Complete editable projection preset bank retained by one viewport-owning window.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewport3dProjectionPreferences {
    pub kind: Viewport3dProjectionKind,
    pub orthographic_view: Viewport3dOrthographicView,
    pub axonometric_variant: Viewport3dAxonometricVariant,
    pub axonometric_angle_a: f64,
    pub axonometric_angle_b: f64,
    pub axonometric_quadrant: Viewport3dAxonometricQuadrant,
    pub oblique_variant: Viewport3dObliqueVariant,
    pub oblique_angle: f64,
    pub oblique_depth: f64,
    pub one_point_axis: Viewport3dOnePointAxis,
    pub fov: f64,
    pub two_point_shift: f64,
    pub curvilinear_fov: f64,
    pub curvilinear_strength: f64,
    pub curvilinear_mapping: Viewport3dCurvilinearMapping,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Viewport3dProjectionPreferencesSerde {
    kind: Viewport3dProjectionKind,
    orthographic_view: Viewport3dOrthographicView,
    axonometric_variant: Viewport3dAxonometricVariant,
    axonometric_angle_a: f64,
    axonometric_angle_b: f64,
    axonometric_quadrant: Viewport3dAxonometricQuadrant,
    oblique_variant: Viewport3dObliqueVariant,
    oblique_angle: f64,
    oblique_depth: f64,
    one_point_axis: Viewport3dOnePointAxis,
    fov: f64,
    two_point_shift: f64,
    curvilinear_fov: f64,
    curvilinear_strength: f64,
    curvilinear_mapping: Viewport3dCurvilinearMapping,
}

impl From<Viewport3dProjectionPreferencesSerde> for Viewport3dProjectionPreferences {
    fn from(value: Viewport3dProjectionPreferencesSerde) -> Self {
        Self {
            kind: value.kind,
            orthographic_view: value.orthographic_view,
            axonometric_variant: value.axonometric_variant,
            axonometric_angle_a: value.axonometric_angle_a,
            axonometric_angle_b: value.axonometric_angle_b,
            axonometric_quadrant: value.axonometric_quadrant,
            oblique_variant: value.oblique_variant,
            oblique_angle: value.oblique_angle,
            oblique_depth: value.oblique_depth,
            one_point_axis: value.one_point_axis,
            fov: value.fov,
            two_point_shift: value.two_point_shift,
            curvilinear_fov: value.curvilinear_fov,
            curvilinear_strength: value.curvilinear_strength,
            curvilinear_mapping: value.curvilinear_mapping,
        }
    }
}

impl<'de> serde::Deserialize<'de> for Viewport3dProjectionPreferences {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let preferences = Self::from(<Viewport3dProjectionPreferencesSerde as serde::Deserialize>::deserialize(deserializer)?);
        preferences.validate().map_err(serde::de::Error::custom)?;
        Ok(preferences)
    }
}

impl Default for Viewport3dProjectionPreferences {
    fn default() -> Self {
        Self {
            kind: Viewport3dProjectionKind::ThreePoint,
            orthographic_view: Viewport3dOrthographicView::Top,
            axonometric_variant: Viewport3dAxonometricVariant::Isometric,
            axonometric_angle_a: 15.0,
            axonometric_angle_b: 12.0,
            axonometric_quadrant: Viewport3dAxonometricQuadrant::Ne,
            oblique_variant: Viewport3dObliqueVariant::Cavalier,
            oblique_angle: 45.0,
            oblique_depth: 1.0,
            one_point_axis: Viewport3dOnePointAxis::Y,
            fov: 50.0,
            two_point_shift: 0.0,
            curvilinear_fov: 120.0,
            curvilinear_strength: 1.0,
            curvilinear_mapping: Viewport3dCurvilinearMapping::Fisheye,
        }
    }
}

impl Viewport3dProjectionPreferences {
    pub fn validate(&self) -> Result<(), ValueError> {
        projection_range(self.axonometric_angle_a, 5.0, 75.0, "axonometricAngleA")?;
        projection_range(self.axonometric_angle_b, 5.0, 75.0, "axonometricAngleB")?;
        projection_range(self.oblique_angle, 5.0, 90.0, "obliqueAngle")?;
        projection_range(self.oblique_depth, 0.05, 1.0, "obliqueDepth")?;
        projection_range(self.fov, 15.0, 120.0, "fov")?;
        projection_range(self.two_point_shift, -1.0, 1.0, "twoPointShift")?;
        projection_range(self.curvilinear_fov, 60.0, 160.0, "curvilinearFov")?;
        projection_range(self.curvilinear_strength, 0.0, 1.0, "curvilinearStrength")
    }
}

impl ToValue for Viewport3dProjectionPreferences {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            ("kind".into(), self.kind.to_value()),
            ("orthographicView".into(), self.orthographic_view.to_value()),
            ("axonometricVariant".into(), self.axonometric_variant.to_value()),
            ("axonometricAngleA".into(), self.axonometric_angle_a.to_value()),
            ("axonometricAngleB".into(), self.axonometric_angle_b.to_value()),
            ("axonometricQuadrant".into(), self.axonometric_quadrant.to_value()),
            ("obliqueVariant".into(), self.oblique_variant.to_value()),
            ("obliqueAngle".into(), self.oblique_angle.to_value()),
            ("obliqueDepth".into(), self.oblique_depth.to_value()),
            ("onePointAxis".into(), self.one_point_axis.to_value()),
            ("fov".into(), self.fov.to_value()),
            ("twoPointShift".into(), self.two_point_shift.to_value()),
            ("curvilinearFov".into(), self.curvilinear_fov.to_value()),
            ("curvilinearStrength".into(), self.curvilinear_strength.to_value()),
            ("curvilinearMapping".into(), self.curvilinear_mapping.to_value()),
        ])
    }
}

impl FromValue for Viewport3dProjectionPreferences {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = crate::fields(value, &[
            "kind", "orthographicView", "axonometricVariant", "axonometricAngleA", "axonometricAngleB", "axonometricQuadrant", "obliqueVariant", "obliqueAngle", "obliqueDepth", "onePointAxis", "fov", "twoPointShift", "curvilinearFov", "curvilinearStrength", "curvilinearMapping",
        ])?;
        let preferences = Self {
            kind: crate::take(&mut entries, "kind")?,
            orthographic_view: crate::take(&mut entries, "orthographicView")?,
            axonometric_variant: crate::take(&mut entries, "axonometricVariant")?,
            axonometric_angle_a: crate::take(&mut entries, "axonometricAngleA")?,
            axonometric_angle_b: crate::take(&mut entries, "axonometricAngleB")?,
            axonometric_quadrant: crate::take(&mut entries, "axonometricQuadrant")?,
            oblique_variant: crate::take(&mut entries, "obliqueVariant")?,
            oblique_angle: crate::take(&mut entries, "obliqueAngle")?,
            oblique_depth: crate::take(&mut entries, "obliqueDepth")?,
            one_point_axis: crate::take(&mut entries, "onePointAxis")?,
            fov: crate::take(&mut entries, "fov")?,
            two_point_shift: crate::take(&mut entries, "twoPointShift")?,
            curvilinear_fov: crate::take(&mut entries, "curvilinearFov")?,
            curvilinear_strength: crate::take(&mut entries, "curvilinearStrength")?,
            curvilinear_mapping: crate::take(&mut entries, "curvilinearMapping")?,
        };
        preferences.validate()?;
        Ok(preferences)
    }
}

/// 📐️ Active mathematical projection mode without renderer state.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Viewport3dProjectionMode {
    Orthographic {},
    Axonometric { variant: Viewport3dAxonometricVariant, #[serde(rename = "angleA")] angle_a: f64, #[serde(rename = "angleB")] angle_b: f64 },
    Oblique { variant: Viewport3dObliqueVariant, angle: f64, #[serde(rename = "depthScale")] depth_scale: f64 },
    OnePoint { fov: f64 },
    TwoPoint { fov: f64, #[serde(rename = "verticalShift")] vertical_shift: f64 },
    ThreePoint { fov: f64 },
    Curvilinear { fov: f64, strength: f64, mapping: Viewport3dCurvilinearMapping },
}

#[derive(serde::Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
enum Viewport3dProjectionModeSerde {
    Orthographic {},
    Axonometric { variant: Viewport3dAxonometricVariant, #[serde(rename = "angleA")] angle_a: f64, #[serde(rename = "angleB")] angle_b: f64 },
    Oblique { variant: Viewport3dObliqueVariant, angle: f64, #[serde(rename = "depthScale")] depth_scale: f64 },
    OnePoint { fov: f64 },
    TwoPoint { fov: f64, #[serde(rename = "verticalShift")] vertical_shift: f64 },
    ThreePoint { fov: f64 },
    Curvilinear { fov: f64, strength: f64, mapping: Viewport3dCurvilinearMapping },
}

impl From<Viewport3dProjectionModeSerde> for Viewport3dProjectionMode {
    fn from(value: Viewport3dProjectionModeSerde) -> Self {
        match value {
            Viewport3dProjectionModeSerde::Orthographic {} => Self::Orthographic {},
            Viewport3dProjectionModeSerde::Axonometric { variant, angle_a, angle_b } => Self::Axonometric { variant, angle_a, angle_b },
            Viewport3dProjectionModeSerde::Oblique { variant, angle, depth_scale } => Self::Oblique { variant, angle, depth_scale },
            Viewport3dProjectionModeSerde::OnePoint { fov } => Self::OnePoint { fov },
            Viewport3dProjectionModeSerde::TwoPoint { fov, vertical_shift } => Self::TwoPoint { fov, vertical_shift },
            Viewport3dProjectionModeSerde::ThreePoint { fov } => Self::ThreePoint { fov },
            Viewport3dProjectionModeSerde::Curvilinear { fov, strength, mapping } => Self::Curvilinear { fov, strength, mapping },
        }
    }
}

impl<'de> serde::Deserialize<'de> for Viewport3dProjectionMode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mode = Self::from(<Viewport3dProjectionModeSerde as serde::Deserialize>::deserialize(deserializer)?);
        mode.validate().map_err(serde::de::Error::custom)?;
        Ok(mode)
    }
}

impl Viewport3dProjectionMode {
    pub fn validate(&self) -> Result<(), ValueError> {
        match self {
            Self::Orthographic {} => Ok(()),
            Self::Axonometric { angle_a, angle_b, .. } => {
                crate::finite(*angle_a, "angleA")?;
                crate::finite(*angle_b, "angleB")
            }
            Self::Oblique { angle, depth_scale, .. } => {
                crate::finite(*angle, "angle")?;
                crate::finite(*depth_scale, "depthScale")
            }
            Self::OnePoint { fov } | Self::ThreePoint { fov } => crate::finite(*fov, "fov"),
            Self::TwoPoint { fov, vertical_shift } => {
                crate::finite(*fov, "fov")?;
                crate::finite(*vertical_shift, "verticalShift")
            }
            Self::Curvilinear { fov, strength, .. } => {
                crate::finite(*fov, "fov")?;
                crate::finite(*strength, "strength")
            }
        }
    }
}

impl ToValue for Viewport3dProjectionMode {
    fn to_value(&self) -> DslValue {
        match self {
            Self::Orthographic {} => DslValue::object([("kind".into(), "orthographic".to_value())]),
            Self::Axonometric { variant, angle_a, angle_b } => DslValue::object([("kind".into(), "axonometric".to_value()), ("variant".into(), variant.to_value()), ("angleA".into(), angle_a.to_value()), ("angleB".into(), angle_b.to_value())]),
            Self::Oblique { variant, angle, depth_scale } => DslValue::object([("kind".into(), "oblique".to_value()), ("variant".into(), variant.to_value()), ("angle".into(), angle.to_value()), ("depthScale".into(), depth_scale.to_value())]),
            Self::OnePoint { fov } => DslValue::object([("kind".into(), "onePoint".to_value()), ("fov".into(), fov.to_value())]),
            Self::TwoPoint { fov, vertical_shift } => DslValue::object([("kind".into(), "twoPoint".to_value()), ("fov".into(), fov.to_value()), ("verticalShift".into(), vertical_shift.to_value())]),
            Self::ThreePoint { fov } => DslValue::object([("kind".into(), "threePoint".to_value()), ("fov".into(), fov.to_value())]),
            Self::Curvilinear { fov, strength, mapping } => DslValue::object([("kind".into(), "curvilinear".to_value()), ("fov".into(), fov.to_value()), ("strength".into(), strength.to_value()), ("mapping".into(), mapping.to_value())]),
        }
    }
}

impl FromValue for Viewport3dProjectionMode {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = crate::fields(value, &["kind", "variant", "angleA", "angleB", "angle", "depthScale", "fov", "verticalShift", "strength", "mapping"])?;
        let kind: Viewport3dProjectionKind = crate::take(&mut entries, "kind")?;
        let mode = match kind {
            Viewport3dProjectionKind::Orthographic => Self::Orthographic {},
            Viewport3dProjectionKind::Axonometric => Self::Axonometric { variant: crate::take(&mut entries, "variant")?, angle_a: crate::take(&mut entries, "angleA")?, angle_b: crate::take(&mut entries, "angleB")? },
            Viewport3dProjectionKind::Oblique => Self::Oblique { variant: crate::take(&mut entries, "variant")?, angle: crate::take(&mut entries, "angle")?, depth_scale: crate::take(&mut entries, "depthScale")? },
            Viewport3dProjectionKind::OnePoint => Self::OnePoint { fov: crate::take(&mut entries, "fov")? },
            Viewport3dProjectionKind::TwoPoint => Self::TwoPoint { fov: crate::take(&mut entries, "fov")?, vertical_shift: crate::take(&mut entries, "verticalShift")? },
            Viewport3dProjectionKind::ThreePoint => Self::ThreePoint { fov: crate::take(&mut entries, "fov")? },
            Viewport3dProjectionKind::Curvilinear => Self::Curvilinear { fov: crate::take(&mut entries, "fov")?, strength: crate::take(&mut entries, "strength")?, mapping: crate::take(&mut entries, "mapping")? },
        };
        crate::finish(entries)?;
        mode.validate()?;
        Ok(mode)
    }
}

struct NonNullOption<T>(Option<T>);

impl<T> Default for NonNullOption<T> {
    fn default() -> Self { Self(None) }
}

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for NonNullOption<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor<T>(std::marker::PhantomData<T>);

        impl<'de, T: serde::Deserialize<'de>> serde::de::Visitor<'de> for Visitor<T> {
            type Value = NonNullOption<T>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { formatter.write_str("a non-null optional viewport projection value") }

            fn visit_some<D: serde::Deserializer<'de>>(self, deserializer: D) -> Result<Self::Value, D::Error> {
                <T as serde::Deserialize>::deserialize(deserializer).map(|value| NonNullOption(Some(value)))
            }

            fn visit_none<E: serde::de::Error>(self) -> Result<Self::Value, E> { Err(E::custom("viewport projection option cannot be null")) }

            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> { Err(E::custom("viewport projection option cannot be null")) }
        }

        deserializer.deserialize_option(Visitor(std::marker::PhantomData))
    }
}

/// 🧭️ Active cardinal, corner, or free orientation independent of projection mode.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Viewport3dProjectionOrientation {
    Cardinal { view: Viewport3dOrthographicView },
    Corner { quadrant: Viewport3dAxonometricQuadrant, #[serde(skip_serializing_if = "Option::is_none")] hemisphere: Option<Viewport3dAxonometricHemisphere> },
    Free {},
}

#[derive(serde::Deserialize)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
enum Viewport3dProjectionOrientationSerde {
    Cardinal { view: Viewport3dOrthographicView },
    Corner { quadrant: Viewport3dAxonometricQuadrant, #[serde(default)] hemisphere: NonNullOption<Viewport3dAxonometricHemisphere> },
    Free {},
}

impl<'de> serde::Deserialize<'de> for Viewport3dProjectionOrientation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match <Viewport3dProjectionOrientationSerde as serde::Deserialize>::deserialize(deserializer)? {
            Viewport3dProjectionOrientationSerde::Cardinal { view } => Self::Cardinal { view },
            Viewport3dProjectionOrientationSerde::Corner { quadrant, hemisphere } => Self::Corner { quadrant, hemisphere: hemisphere.0 },
            Viewport3dProjectionOrientationSerde::Free {} => Self::Free {},
        })
    }
}

impl ToValue for Viewport3dProjectionOrientation {
    fn to_value(&self) -> DslValue {
        match self {
            Self::Cardinal { view } => DslValue::object([("type".into(), "cardinal".to_value()), ("view".into(), view.to_value())]),
            Self::Corner { quadrant, hemisphere } => {
                let mut entries = vec![("type".into(), "corner".to_value()), ("quadrant".into(), quadrant.to_value())];
                if let Some(hemisphere) = hemisphere { entries.push(("hemisphere".into(), hemisphere.to_value())); }
                DslValue::object(entries)
            }
            Self::Free {} => DslValue::object([("type".into(), "free".to_value())]),
        }
    }
}

impl FromValue for Viewport3dProjectionOrientation {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = crate::fields(value, &["type", "view", "quadrant", "hemisphere"])?;
        let kind: String = crate::take(&mut entries, "type")?;
        let orientation = match kind.as_str() {
            "cardinal" => Self::Cardinal { view: crate::take(&mut entries, "view")? },
            "corner" => {
                let quadrant = crate::take(&mut entries, "quadrant")?;
                let hemisphere = if entries.iter().any(|(name, _)| name == "hemisphere") { Some(crate::take(&mut entries, "hemisphere")?) } else { None };
                Self::Corner { quadrant, hemisphere }
            }
            "free" => Self::Free {},
            _ => return Err(ValueError::new("unknown viewport projection orientation").under("type")),
        };
        crate::finish(entries)?;
        Ok(orientation)
    }
}

/// 🎯️ Active mathematical projection snapshot transported to a renderer.
#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Viewport3dProjectionSpec {
    pub mode: Viewport3dProjectionMode,
    pub orientation: Viewport3dProjectionOrientation,
}

impl ToValue for Viewport3dProjectionSpec {
    fn to_value(&self) -> DslValue { DslValue::object([("mode".into(), self.mode.to_value()), ("orientation".into(), self.orientation.to_value())]) }
}

impl FromValue for Viewport3dProjectionSpec {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut entries = crate::fields(value, &["mode", "orientation"])?;
        Ok(Self { mode: crate::take(&mut entries, "mode")?, orientation: crate::take(&mut entries, "orientation")? })
    }
}

/// 🧩️ Derives one active renderer-neutral projection while retaining the complete preference bank.
pub fn derive_active_projection(preferences: &Viewport3dProjectionPreferences) -> Viewport3dProjectionSpec {
    let orientation;
    let mode = match preferences.kind {
        Viewport3dProjectionKind::Orthographic => {
            orientation = Viewport3dProjectionOrientation::Cardinal { view: preferences.orthographic_view };
            Viewport3dProjectionMode::Orthographic {}
        }
        Viewport3dProjectionKind::Axonometric => {
            let angle_a = if preferences.axonometric_variant == Viewport3dAxonometricVariant::Isometric { 30.0 } else { preferences.axonometric_angle_a };
            let angle_b = match preferences.axonometric_variant {
                Viewport3dAxonometricVariant::Isometric => 30.0,
                Viewport3dAxonometricVariant::Dimetric => preferences.axonometric_angle_a,
                Viewport3dAxonometricVariant::Trimetric => preferences.axonometric_angle_b,
            };
            orientation = Viewport3dProjectionOrientation::Corner { quadrant: preferences.axonometric_quadrant, hemisphere: Some(Viewport3dAxonometricHemisphere::Upper) };
            Viewport3dProjectionMode::Axonometric { variant: preferences.axonometric_variant, angle_a, angle_b }
        }
        Viewport3dProjectionKind::Oblique => {
            orientation = Viewport3dProjectionOrientation::Cardinal { view: if preferences.oblique_variant == Viewport3dObliqueVariant::Military { Viewport3dOrthographicView::Plan } else { Viewport3dOrthographicView::Front } };
            Viewport3dProjectionMode::Oblique { variant: preferences.oblique_variant, angle: preferences.oblique_angle, depth_scale: preferences.oblique_depth }
        }
        Viewport3dProjectionKind::OnePoint => {
            orientation = Viewport3dProjectionOrientation::Cardinal { view: match preferences.one_point_axis { Viewport3dOnePointAxis::X => Viewport3dOrthographicView::Left, Viewport3dOnePointAxis::Y => Viewport3dOrthographicView::Front, Viewport3dOnePointAxis::Z => Viewport3dOrthographicView::Top } };
            Viewport3dProjectionMode::OnePoint { fov: preferences.fov }
        }
        Viewport3dProjectionKind::TwoPoint => {
            orientation = Viewport3dProjectionOrientation::Free {};
            Viewport3dProjectionMode::TwoPoint { fov: preferences.fov, vertical_shift: preferences.two_point_shift }
        }
        Viewport3dProjectionKind::ThreePoint => {
            orientation = Viewport3dProjectionOrientation::Free {};
            Viewport3dProjectionMode::ThreePoint { fov: preferences.fov }
        }
        Viewport3dProjectionKind::Curvilinear => {
            orientation = Viewport3dProjectionOrientation::Free {};
            Viewport3dProjectionMode::Curvilinear { fov: preferences.curvilinear_fov, strength: preferences.curvilinear_strength, mapping: preferences.curvilinear_mapping }
        }
    };
    Viewport3dProjectionSpec { mode, orientation }
}

fn projection_range(value: f64, minimum: f64, maximum: f64, name: &str) -> Result<(), ValueError> {
    if value.is_finite() && value >= minimum && value <= maximum { Ok(()) } else { Err(ValueError::new("viewport projection value is outside its declared range").under(name)) }
}
