//! 🗂️ The ModelScene the drawing is made from: the JSON twin of the TypeScript `ModelScene` (metres, layers of polylines and single-line texts), validated once so the writer never sees a degenerate entity.
//! <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf>

use crate::json::{parse, Json};

/// 📍️ A model point in metres.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// 〰️ An open or closed polyline of at least two points.
#[derive(Clone, Debug, PartialEq)]
pub struct Polyline {
    pub points: Vec<Point>,
    pub closed: bool,
}

/// 🔤️ A single-line text.
#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub at: Point,
    pub height_m: f64,
    pub text: String,
}

/// 🧫️ A CAD layer: ACI colour, lineweight in 1/100 mm, polylines and texts.
#[derive(Clone, Debug, PartialEq)]
pub struct Layer {
    pub name: String,
    pub aci: u8,
    pub lineweight: u32,
    pub polylines: Vec<Polyline>,
    pub texts: Vec<Text>,
}

/// 📐️ A validated scene in metres; layer names are unique case-insensitively.
#[derive(Clone, Debug, PartialEq)]
pub struct ModelScene {
    pub layers: Vec<Layer>,
}

fn field<'a>(value: &'a Json, key: &str, owner: &str) -> Result<&'a Json, String> {
    value.get(key).ok_or_else(|| format!("{owner}: missing \"{key}\""))
}

fn number(value: &Json, key: &str, owner: &str) -> Result<f64, String> {
    field(value, key, owner)?.number().ok_or_else(|| format!("{owner}: \"{key}\" must be a number"))
}

fn list<'a>(value: &'a Json, key: &str, owner: &str) -> Result<&'a [Json], String> {
    field(value, key, owner)?.array().ok_or_else(|| format!("{owner}: \"{key}\" must be an array"))
}

fn point(value: &Json, owner: &str) -> Result<Point, String> {
    Ok(Point { x: number(value, "x", owner)?, y: number(value, "y", owner)? })
}

fn whole(value: f64, key: &str, owner: &str, max: f64) -> Result<u32, String> {
    if value.fract() == 0.0 && (0.0..=max).contains(&value) {
        Ok(value as u32)
    } else {
        Err(format!("{owner}: \"{key}\" must be a whole number from 0 to {max}"))
    }
}

const FORBIDDEN: &str = "<>/\\\":;?*|,=`";

fn valid_name(name: &str) -> bool {
    !name.is_empty() && name.chars().count() <= 255 && !name.chars().any(|ch| ch.is_control() || FORBIDDEN.contains(ch))
}

fn layer(value: &Json, index: usize) -> Result<Layer, String> {
    let owner = format!("layers[{index}]");
    let name = field(value, "name", &owner)?.string().ok_or_else(|| format!("{owner}: \"name\" must be a string"))?.to_string();
    if !valid_name(&name) {
        return Err(format!("{owner}: layer name {name:?} is empty, longer than 255 characters or contains one of {FORBIDDEN}"));
    }
    let aci = whole(number(value, "aci", &owner)?, "aci", &owner, 255.0)?;
    if aci == 0 {
        return Err(format!("{owner}: \"aci\" 0 is ByBlock, a layer needs 1 to 255"));
    }
    let lineweight = whole(number(value, "lineweight", &owner)?, "lineweight", &owner, 211.0)?;
    let mut polylines = Vec::new();
    for (at, line) in list(value, "polylines", &owner)?.iter().enumerate() {
        let owner = format!("{owner}.polylines[{at}]");
        let points = list(line, "points", &owner)?.iter().map(|p| point(p, &owner)).collect::<Result<Vec<_>, _>>()?;
        if points.len() < 2 {
            return Err(format!("{owner}: a polyline needs at least two points, got {}", points.len()));
        }
        let closed = field(line, "closed", &owner)?.bool().ok_or_else(|| format!("{owner}: \"closed\" must be a boolean"))?;
        polylines.push(Polyline { points, closed });
    }
    let mut texts = Vec::new();
    for (at, item) in list(value, "texts", &owner)?.iter().enumerate() {
        let owner = format!("{owner}.texts[{at}]");
        let height_m = number(item, "heightM", &owner)?;
        if height_m <= 0.0 {
            return Err(format!("{owner}: \"heightM\" must be positive"));
        }
        let text = field(item, "text", &owner)?.string().ok_or_else(|| format!("{owner}: \"text\" must be a string"))?.to_string();
        texts.push(Text { at: point(field(item, "at", &owner)?, &owner)?, height_m, text });
    }
    Ok(Layer { name, aci: aci as u8, lineweight, polylines, texts })
}

impl ModelScene {
    /// 📥️ Reads and validates ModelScene JSON; units must be metres.
    pub fn from_json(json: &[u8]) -> Result<ModelScene, String> {
        let root = parse(json).map_err(|error| format!("ModelScene JSON: {error}"))?;
        let units = field(&root, "units", "ModelScene")?.string().ok_or("ModelScene: \"units\" must be a string")?;
        if units != "m" {
            return Err(format!("ModelScene units must be \"m\", got {units:?}"));
        }
        let mut layers: Vec<Layer> = Vec::new();
        for (index, value) in list(&root, "layers", "ModelScene")?.iter().enumerate() {
            let next = layer(value, index)?;
            match layers.iter_mut().find(|known| known.name.to_uppercase() == next.name.to_uppercase()) {
                Some(known) if known.aci == next.aci && known.lineweight == next.lineweight => {
                    known.polylines.extend(next.polylines);
                    known.texts.extend(next.texts);
                }
                Some(known) => return Err(format!("layers[{index}]: layer {:?} repeats {:?} with another colour or lineweight", next.name, known.name)),
                None => layers.push(next),
            }
        }
        Ok(ModelScene { layers })
    }
}
