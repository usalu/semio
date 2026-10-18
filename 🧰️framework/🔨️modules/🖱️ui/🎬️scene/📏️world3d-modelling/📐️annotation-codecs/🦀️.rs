//! 📐️ Annotation codecs: the annotation constructors and validation, the annotation lane codec and the annotation layer with its lane codec. See [`super`].

use super::*;

//#region 🔖️Annotations
impl World3dAnnotation {
    pub fn dimension(id: impl Into<String>, from: World3dVec3, to: World3dVec3, offset: World3dVec3, text: World3dText) -> Self {
        Self::Dimension(World3dDimension { id: id.into(), from, to, offset, text, tone: World3dTone::Neutral })
    }

    pub fn angle(id: impl Into<String>, vertex: World3dVec3, direction_a: World3dVec3, direction_b: World3dVec3, text: World3dText) -> Self {
        Self::Angle(World3dAngle { id: id.into(), vertex, direction_a, direction_b, radius_px: WORLD3D_ANGLE_RADIUS_PX_DEFAULT, text, tone: World3dTone::Neutral })
    }

    pub fn marker(id: impl Into<String>, position: World3dVec3, text: World3dText) -> Self {
        Self::Marker(World3dMarker { id: id.into(), position, shape: World3dMarkerShape::Dot, text, tone: World3dTone::Neutral })
    }

    pub fn leader(id: impl Into<String>, anchor: World3dVec3, label_offset_px: [f64; 2], text: World3dText) -> Self {
        Self::Leader(World3dLeader { id: id.into(), anchor, label_offset_px, text, tone: World3dTone::Neutral })
    }

    pub fn with_tone(mut self, tone: World3dTone) -> Self {
        match &mut self {
            Self::Dimension(item) => item.tone = tone,
            Self::Angle(item) => item.tone = tone,
            Self::Marker(item) => item.tone = tone,
            Self::Leader(item) => item.tone = tone,
        }
        self
    }

    pub fn id(&self) -> &str {
        match self {
            Self::Dimension(item) => &item.id,
            Self::Angle(item) => &item.id,
            Self::Marker(item) => &item.id,
            Self::Leader(item) => &item.id,
        }
    }

    pub fn text(&self) -> &World3dText {
        match self {
            Self::Dimension(item) => &item.text,
            Self::Angle(item) => &item.text,
            Self::Marker(item) => &item.text,
            Self::Leader(item) => &item.text,
        }
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        if self.id().is_empty() {
            return Err(refuse("annotation id must not be empty"));
        }
        self.text().validate()?;
        match self {
            Self::Dimension(item) => {
                for (vector, label) in [(&item.from, "from"), (&item.to, "to"), (&item.offset, "offset")] {
                    vec3_finite(vector, label)?;
                }
                if item.from == item.to {
                    return Err(refuse("dimension from and to must differ"));
                }
            }
            Self::Angle(item) => {
                vec3_finite(&item.vertex, "vertex")?;
                vec3_nonzero(&item.direction_a, "directionA")?;
                vec3_nonzero(&item.direction_b, "directionB")?;
                if !item.radius_px.is_finite() || !(16.0..=256.0).contains(&item.radius_px) {
                    return Err(refuse("radiusPx must be within 16..256"));
                }
            }
            Self::Marker(item) => vec3_finite(&item.position, "position")?,
            Self::Leader(item) => {
                vec3_finite(&item.anchor, "anchor")?;
                if !item.label_offset_px.iter().all(|component| component.is_finite()) {
                    return Err(refuse("labelOffsetPx must be finite"));
                }
            }
        }
        Ok(())
    }
}

impl ToValue for World3dAnnotation {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        match self {
            Self::Dimension(item) => {
                push(&mut entries, "kind", &"dimension");
                push(&mut entries, "id", &item.id);
                push(&mut entries, "from", &item.from);
                push(&mut entries, "to", &item.to);
                push(&mut entries, "offset", &item.offset);
                push(&mut entries, "text", &item.text);
                push(&mut entries, "tone", &item.tone);
            }
            Self::Angle(item) => {
                push(&mut entries, "kind", &"angle");
                push(&mut entries, "id", &item.id);
                push(&mut entries, "vertex", &item.vertex);
                push(&mut entries, "directionA", &item.direction_a);
                push(&mut entries, "directionB", &item.direction_b);
                push(&mut entries, "radiusPx", &item.radius_px);
                push(&mut entries, "text", &item.text);
                push(&mut entries, "tone", &item.tone);
            }
            Self::Marker(item) => {
                push(&mut entries, "kind", &"marker");
                push(&mut entries, "id", &item.id);
                push(&mut entries, "position", &item.position);
                push(&mut entries, "shape", &item.shape);
                push(&mut entries, "text", &item.text);
                push(&mut entries, "tone", &item.tone);
            }
            Self::Leader(item) => {
                push(&mut entries, "kind", &"leader");
                push(&mut entries, "id", &item.id);
                push(&mut entries, "anchor", &item.anchor);
                push(&mut entries, "labelOffsetPx", &item.label_offset_px);
                push(&mut entries, "text", &item.text);
                push(&mut entries, "tone", &item.tone);
            }
        }
        DslValue::Object(entries)
    }
}

impl FromValue for World3dAnnotation {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let kind = value.get("kind").and_then(DslValue::as_str).map(str::to_string).ok_or_else(|| refuse("annotation needs a kind"))?;
        let annotation = match kind.as_str() {
            "dimension" => {
                let mut fields = Fields::read(value, &["kind", "id", "from", "to", "offset", "text", "tone"])?;
                Self::Dimension(World3dDimension { id: fields.require("id")?, from: fields.require("from")?, to: fields.require("to")?, offset: fields.require("offset")?, text: fields.require("text")?, tone: fields.or("tone", World3dTone::Neutral)? })
            }
            "angle" => {
                let mut fields = Fields::read(value, &["kind", "id", "vertex", "directionA", "directionB", "radiusPx", "text", "tone"])?;
                Self::Angle(World3dAngle {
                    id: fields.require("id")?,
                    vertex: fields.require("vertex")?,
                    direction_a: fields.require("directionA")?,
                    direction_b: fields.require("directionB")?,
                    radius_px: fields.or("radiusPx", WORLD3D_ANGLE_RADIUS_PX_DEFAULT)?,
                    text: fields.require("text")?,
                    tone: fields.or("tone", World3dTone::Neutral)?,
                })
            }
            "marker" => {
                let mut fields = Fields::read(value, &["kind", "id", "position", "shape", "text", "tone"])?;
                Self::Marker(World3dMarker { id: fields.require("id")?, position: fields.require("position")?, shape: fields.or("shape", World3dMarkerShape::Dot)?, text: fields.require("text")?, tone: fields.or("tone", World3dTone::Neutral)? })
            }
            "leader" => {
                let mut fields = Fields::read(value, &["kind", "id", "anchor", "labelOffsetPx", "text", "tone"])?;
                Self::Leader(World3dLeader { id: fields.require("id")?, anchor: fields.require("anchor")?, label_offset_px: fields.require("labelOffsetPx")?, text: fields.require("text")?, tone: fields.or("tone", World3dTone::Neutral)? })
            }
            other => return Err(refuse(format!("unknown annotation kind `{other}`"))),
        };
        annotation.validate()?;
        Ok(annotation)
    }
}

/// 📏️ The annotation layer; every item is listed for assistive technology through its text.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dAnnotationLayer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<World3dText>,
    pub items: Vec<World3dAnnotation>,
}

impl World3dAnnotationLayer {
    pub fn new(items: Vec<World3dAnnotation>) -> Self {
        Self { title: None, items }
    }

    pub fn titled(mut self, title: World3dText) -> Self {
        self.title = Some(title);
        self
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        if self.items.len() > WORLD3D_ANNOTATIONS_MAX {
            return Err(refuse(format!("at most {WORLD3D_ANNOTATIONS_MAX} annotations")));
        }
        if let Some(title) = &self.title {
            title.validate().map_err(|error| error.under("title"))?;
        }
        let mut identities = std::collections::HashSet::new();
        for (index, item) in self.items.iter().enumerate() {
            item.validate().map_err(|error| error.under(format!("items[{index}]")))?;
            if !identities.insert(item.id()) {
                return Err(refuse(format!("duplicate annotation id `{}`", item.id())));
            }
        }
        Ok(())
    }
}

impl ToValue for World3dAnnotationLayer {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push_option(&mut entries, "title", &self.title);
        push(&mut entries, "items", &self.items);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dAnnotationLayer {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["title", "items"])?;
        let layer = Self { title: fields.option("title")?, items: fields.require("items")? };
        layer.validate()?;
        Ok(layer)
    }
}
//#endregion 🔖️Annotations
