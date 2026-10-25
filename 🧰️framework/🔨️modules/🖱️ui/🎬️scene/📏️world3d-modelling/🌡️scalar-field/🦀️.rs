//! 🌡️ Scalar field: the scalar domain and colour ramp, the scalar range and legend, the legend tick and the scalar field with its retirement and lane codec. See [`super`].

use super::*;

//#region 🔖️ScalarField
string_enum! {
    /// 🕸️ Which mesh element each scalar value belongs to.
    World3dScalarDomain { Vertex => "vertex", Face => "face" }
}

string_enum! {
    /// 🎨️ A named colour ramp.
    World3dColorRamp { Viridis => "viridis", Inferno => "inferno", Coolwarm => "coolwarm", Grayscale => "grayscale" }
}

impl World3dColorRamp {
    /// 🛑️ Equally spaced sRGB stops; colours interpolate linearly per channel.
    pub fn stops(self) -> &'static [[u8; 3]] {
        match self {
            Self::Viridis => &[[0x44, 0x01, 0x54], [0x3b, 0x52, 0x8b], [0x21, 0x91, 0x8c], [0x5e, 0xc9, 0x62], [0xfd, 0xe7, 0x25]],
            Self::Inferno => &[[0x00, 0x00, 0x04], [0x57, 0x10, 0x6e], [0xbc, 0x37, 0x54], [0xf9, 0x8e, 0x09], [0xfc, 0xff, 0xa4]],
            Self::Coolwarm => &[[0x3b, 0x4c, 0xc0], [0x8d, 0xb0, 0xfe], [0xdd, 0xdd, 0xdd], [0xf4, 0x98, 0x7a], [0xb4, 0x04, 0x26]],
            Self::Grayscale => &[[0x00, 0x00, 0x00], [0xff, 0xff, 0xff]],
        }
    }

    /// 🧪️ The ramp colour at `t` (clamped to 0..1).
    pub fn sample(self, t: f64) -> [u8; 3] {
        let stops = self.stops();
        let segments = stops.len() - 1;
        let scaled = if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) } * segments as f64;
        let index = (scaled.floor() as usize).min(segments - 1);
        let fraction = scaled - index as f64;
        let (from, to) = (stops[index], stops[index + 1]);
        let channel = |component: usize| {
            let (a, b) = (f64::from(from[component]) / 255.0, f64::from(to[component]) / 255.0);
            ((a + (b - a) * fraction) * 255.0 + 0.5).floor().clamp(0.0, 255.0) as u8
        };
        [channel(0), channel(1), channel(2)]
    }
}

/// ↔️ The value interval the ramp spans; values outside clamp to its ends.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct World3dScalarRange {
    pub min: f64,
    pub max: f64,
}

impl ToValue for World3dScalarRange {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "min", &self.min);
        push(&mut entries, "max", &self.max);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dScalarRange {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["min", "max"])?;
        let range = Self { min: fields.require("min")?, max: fields.require("max")? };
        if !range.min.is_finite() || !range.max.is_finite() || range.min >= range.max {
            return Err(refuse("range min must be finite and below max"));
        }
        Ok(range)
    }
}

/// 🏷️ The on-screen legend of a scalar field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dScalarLegend {
    pub title: World3dText,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default = "legend_ticks_default")]
    pub ticks: u8,
}

fn legend_ticks_default() -> u8 {
    WORLD3D_LEGEND_TICKS_DEFAULT
}

impl ToValue for World3dScalarLegend {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "title", &self.title);
        push_option(&mut entries, "unit", &self.unit);
        push(&mut entries, "ticks", &self.ticks);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dScalarLegend {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["title", "unit", "ticks"])?;
        let legend = Self { title: fields.require("title")?, unit: fields.option("unit")?, ticks: fields.or("ticks", WORLD3D_LEGEND_TICKS_DEFAULT)? };
        if !(2..=9).contains(&legend.ticks) {
            return Err(refuse("ticks must be within 2..9"));
        }
        if legend.unit.as_deref() == Some("") {
            return Err(refuse("unit must not be empty"));
        }
        Ok(legend)
    }
}

/// 📍️ One legend tick: a value and the ramp colour it maps to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dLegendTick {
    pub value: f64,
    pub rgb: [u8; 3],
}

/// 🔥️ A scalar analysis field painted as a heatmap on every instance of `mesh_id`: one value per vertex, or one per triangle. `None` is no data.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct World3dScalarField {
    pub mesh_id: String,
    pub domain: World3dScalarDomain,
    pub values: Vec<Option<f64>>,
    pub ramp: World3dColorRamp,
    pub range: World3dScalarRange,
    pub legend: World3dScalarLegend,
}

semio_framework_value::artifact_retire_leaf!(World3dScalarDomain,World3dColorRamp,World3dScalarRange);

semio_framework_value::artifact_retire_struct!(World3dText{en,de});

semio_framework_value::artifact_retire_struct!(World3dScalarLegend{title,unit,ticks});

semio_framework_value::artifact_retire_struct!(World3dScalarField{mesh_id,domain,values,ramp,range,legend});

impl World3dScalarField {
    pub fn new(mesh_id: impl Into<String>, domain: World3dScalarDomain, values: Vec<Option<f64>>, ramp: World3dColorRamp, range: World3dScalarRange, legend_title: World3dText) -> Self {
        Self { mesh_id: mesh_id.into(), domain, values, ramp, range, legend: World3dScalarLegend { title: legend_title, unit: None, ticks: WORLD3D_LEGEND_TICKS_DEFAULT } }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.legend.unit = Some(unit.into());
        self
    }

    pub fn with_ticks(mut self, ticks: u8) -> Self {
        self.legend.ticks = ticks;
        self
    }

    pub fn validate(&self) -> Result<(), ValueError> {
        if self.mesh_id.is_empty() {
            return Err(refuse("meshId must not be empty"));
        }
        if self.values.is_empty() || self.values.len() > WORLD3D_SCALAR_VALUES_MAX {
            return Err(refuse(format!("values must hold 1..{WORLD3D_SCALAR_VALUES_MAX} entries")));
        }
        if self.values.iter().flatten().any(|value| !value.is_finite()) {
            return Err(refuse("values must be finite or null"));
        }
        if !self.range.min.is_finite() || !self.range.max.is_finite() || self.range.min >= self.range.max {
            return Err(refuse("range min must be finite and below max"));
        }
        self.legend.title.validate().map_err(|error| error.under("legend.title"))?;
        if !(2..=9).contains(&self.legend.ticks) {
            return Err(refuse("ticks must be within 2..9"));
        }
        if self.legend.unit.as_deref() == Some("") {
            return Err(refuse("unit must not be empty"));
        }
        Ok(())
    }

    pub(super) fn t(&self, value: f64) -> f64 {
        (value - self.range.min) / (self.range.max - self.range.min)
    }

    /// 🌈️ The heatmap colour of one value; `None` is the no-data grey.
    pub fn color_of(&self, value: Option<f64>) -> [u8; 3] {
        value.map_or(WORLD3D_SCALAR_NO_DATA_RGB, |value| self.ramp.sample(self.t(value)))
    }

    /// 📦️ Every value of the field as packed `r, g, b` bytes, ready for a vertex or triangle colour attribute.
    pub fn color_bytes(&self) -> Vec<u8> {
        self.values.iter().flat_map(|value| self.color_of(*value)).collect()
    }

    /// 📏️ `legend.ticks` evenly spaced values from range min to max, each with its ramp colour.
    pub fn legend_ticks(&self) -> Vec<World3dLegendTick> {
        let count = usize::from(self.legend.ticks.max(2));
        (0..count)
            .map(|index| {
                let value = self.range.min + (index as f64 * (self.range.max - self.range.min)) / (count - 1) as f64;
                World3dLegendTick { value, rgb: self.ramp.sample(self.t(value)) }
            })
            .collect()
    }
}

impl ToValue for World3dScalarField {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        push(&mut entries, "meshId", &self.mesh_id);
        push(&mut entries, "domain", &self.domain);
        push(&mut entries, "values", &self.values);
        push(&mut entries, "ramp", &self.ramp);
        push(&mut entries, "range", &self.range);
        push(&mut entries, "legend", &self.legend);
        DslValue::Object(entries)
    }
}

impl FromValue for World3dScalarField {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let mut fields = Fields::read(value, &["meshId", "domain", "values", "ramp", "range", "legend"])?;
        let field = Self { mesh_id: fields.require("meshId")?, domain: fields.require("domain")?, values: fields.require("values")?, ramp: fields.require("ramp")?, range: fields.require("range")?, legend: fields.require("legend")? };
        field.validate()?;
        Ok(field)
    }
}
//#endregion 🔖️ScalarField
