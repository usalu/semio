//! 🎯️ Interaction options: the pick granularity and targets, the section plane and cap, the sub-element style, the highlight and the modelling options with their lane codecs. See [`super`].

use super::*;

//#region 🔖️Interaction
string_enum! {
    /// 🎯️ The only granularity hover and selection reach.
    World3dPickGranularity { Shape => "shape", Face => "face", Edge => "edge", Vertex => "vertex" }
}

/// 🎯️ The sub-element kinds hover and selection may reach.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World3dPickTargets {
    pub mesh: bool,
    pub face: bool,
    pub edge: bool,
    pub vertex: bool,
}

impl World3dPickGranularity {
    /// 🎯️ The targets a filter admits; no filter admits all of them.
    pub fn targets(filter: Option<Self>) -> World3dPickTargets {
        match filter {
            None => World3dPickTargets { mesh: true, face: true, edge: true, vertex: true },
            Some(filter) => World3dPickTargets { mesh: filter == Self::Shape, face: filter == Self::Face, edge: filter == Self::Edge, vertex: filter == Self::Vertex },
        }
    }
}

/// ✂️ How a section plane closes the cut.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World3dSectionCap {
    #[serde(default)]
    pub tone: World3dTone,
}

/// ✂️ A section plane through `origin`; geometry on the side `normal` points to is removed. `cap` closes the cut with a solid face.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World3dSection {
    pub origin: World3dVec3,
    pub normal: World3dVec3,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<World3dSectionCap>,
}

impl World3dSection {
    pub fn new(origin: World3dVec3, normal: World3dVec3) -> Self {
        Self { origin, normal, cap: None }
    }

    pub fn capped(mut self, tone: World3dTone) -> Self {
        self.cap = Some(World3dSectionCap { tone });
        self
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        vec3_finite(&self.origin, "origin")?;
        vec3_nonzero(&self.normal, "normal")
    }

    /// ✂️ The plane `[nx, ny, nz, constant]` in three.js clipping convention (negative signed distance is clipped), removing the side `normal` points to.
    pub fn clip_plane(&self) -> [f64; 4] {
        let length = self.normal.iter().map(|component| component * component).sum::<f64>().sqrt();
        let unit = [self.normal[0] / length, self.normal[1] / length, self.normal[2] / length];
        let constant = unit[0] * self.origin[0] + unit[1] * self.origin[1] + unit[2] * self.origin[2];
        [-unit[0], -unit[1], -unit[2], constant]
    }
}

impl ToValue for World3dSectionCap {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "tone", &self.tone);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dSectionCap {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["tone"])?;
        Ok(Self { tone: fields.or("tone", World3dTone::Neutral)? })
    }
}

impl ToValue for World3dSection {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "origin", &self.origin);
        push(&mut entries, "normal", &self.normal);
        push_option(&mut entries, "cap", &self.cap);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dSection {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["origin", "normal", "cap"])?;
        let section = Self { origin: fields.require("origin")?, normal: fields.require("normal")?, cap: fields.option("cap")? };
        section.validate()?;
        Ok(section)
    }
}

/// 🖍️ Hover and selection tokens of one sub-element granularity; an absent token keeps the theme default (secondary for hover, primary for selection).
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dSubElementStyle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hover: Option<World3dTone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected: Option<World3dTone>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width_px: Option<f64>,
}

impl World3dSubElementStyle {
    pub fn validate(&self) -> Result<(), ValueError> {
        match self.width_px {
            Some(width) if !width.is_finite() || !(1.0..=8.0).contains(&width) => Err(refuse("widthPx must be within 1..8")),
            _ => Ok(()),
        }
    }
}

impl ToValue for World3dSubElementStyle {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push_option(&mut entries, "hover", &self.hover);
        push_option(&mut entries, "selected", &self.selected);
        push_option(&mut entries, "widthPx", &self.width_px);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dSubElementStyle {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["hover", "selected", "widthPx"])?;
        let style = Self { hover: fields.option("hover")?, selected: fields.option("selected")?, width_px: fields.option("widthPx")? };
        style.validate()?;
        Ok(style)
    }
}

/// 🖍️ Sub-element highlight tokens per granularity, resolved through the theme so every theme and dark mode customises them.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World3dHighlight {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub face: Option<World3dSubElementStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edge: Option<World3dSubElementStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertex: Option<World3dSubElementStyle>,
}

impl ToValue for World3dHighlight {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push_option(&mut entries, "face", &self.face);
        push_option(&mut entries, "edge", &self.edge);
        push_option(&mut entries, "vertex", &self.vertex);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dHighlight {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["face", "edge", "vertex"])?;
        Ok(Self { face: fields.option("face")?, edge: fields.option("edge")?, vertex: fields.option("vertex")? })
    }
}

/// ⚙️ The scene options of a modelling viewport: pick granularity filter, section plane and highlight tokens.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dModellingOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pick_filter: Option<World3dPickGranularity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section: Option<World3dSection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<World3dHighlight>,
}

impl World3dModellingOptions {
    pub fn validate(&self) -> Result<(), ValueError> {
        if let Some(section) = &self.section {
            section.validate().map_err(|error| error.under("section"))?;
        }
        Ok(())
    }
}

impl ToValue for World3dModellingOptions {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push_option(&mut entries, "pickFilter", &self.pick_filter);
        push_option(&mut entries, "section", &self.section);
        push_option(&mut entries, "highlight", &self.highlight);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dModellingOptions {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["pickFilter", "section", "highlight"])?;
        let options = Self { pick_filter: fields.option("pickFilter")?, section: fields.option("section")?, highlight: fields.option("highlight")? };
        options.validate()?;
        Ok(options)
    }
}
//#endregion 🔖️Interaction
