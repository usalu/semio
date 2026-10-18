//! 🔤️ Legend strings and sub-element paint: the modelling strings, the legend value formatting and lines, the sub-element defaults and paint and the modelling lane text. See [`super`].

use super::*;

//#region 🔖️Overlay
/// 🏷️ The fixed strings of the modelling layer in one language.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World3dModellingStrings {
    pub annotations: &'static str,
    pub dimension: &'static str,
    pub angle: &'static str,
    pub marker: &'static str,
    pub leader: &'static str,
    pub legend: &'static str,
    pub scale: &'static str,
    pub no_data: &'static str,
    pub mismatch: &'static str,
}

/// 🏷️ The fixed strings for a locale (German for `de*`, English otherwise); the React twin is `world3dModellingStrings`.
pub fn world3d_modelling_strings(locale: &str) -> World3dModellingStrings {
    if is_german(locale) {
        World3dModellingStrings { annotations: "Anmerkungen", dimension: "Bemaßung", angle: "Winkel", marker: "Markierung", leader: "Beschriftung", legend: "Legende", scale: "Skalenwerte", no_data: "Keine Daten", mismatch: "Das Skalarfeld passt nicht zum Netz" }
    } else {
        World3dModellingStrings { annotations: "Annotations", dimension: "Dimension", angle: "Angle", marker: "Marker", leader: "Label", legend: "Legend", scale: "Scale values", no_data: "No data", mismatch: "The scalar field does not match the mesh" }
    }
}

fn is_german(locale: &str) -> bool {
    locale.to_ascii_lowercase().split(['-', '_']).next() == Some("de")
}

/// 🧮️ A legend value with at most four significant digits, grouped and separated for the `en` or `de` language family — the same text as `Intl.NumberFormat(locale, { maximumSignificantDigits: 4 })`,
/// which pins it through the `legendValues` fixture. Rounding is half away from zero on the shortest decimal form of the double, as ICU does.
pub fn format_world3d_legend_value(value: f64, locale: &str) -> String {
    if !value.is_finite() {
        return value.to_string();
    }
    let (decimal, group) = if is_german(locale) { (',', '.') } else { ('.', ',') };
    if value == 0.0 {
        return "0".to_string();
    }
    let scientific = format!("{:e}", value.abs());
    let (mantissa, exponent) = scientific.split_once('e').unwrap_or((scientific.as_str(), "0"));
    let mut digits: Vec<u8> = mantissa.bytes().filter(u8::is_ascii_digit).map(|digit| digit - b'0').collect();
    let mut exponent: i32 = exponent.parse().unwrap_or(0);
    if digits.len() > 4 {
        let round_up = digits[4] >= 5;
        digits.truncate(4);
        if round_up {
            let mut index = 4;
            loop {
                if index == 0 {
                    digits.insert(0, 1);
                    digits.truncate(4);
                    exponent += 1;
                    break;
                }
                index -= 1;
                if digits[index] == 9 {
                    digits[index] = 0;
                } else {
                    digits[index] += 1;
                    break;
                }
            }
        }
    }
    while digits.len() > 1 && digits.last() == Some(&0) {
        digits.pop();
    }
    let text: String = digits.iter().map(|digit| char::from(b'0' + digit)).collect();
    let integer_digits = exponent + 1;
    let (integer, fraction) = if integer_digits <= 0 {
        ("0".to_string(), format!("{}{text}", "0".repeat(integer_digits.unsigned_abs() as usize)))
    } else if integer_digits as usize >= text.len() {
        (format!("{text}{}", "0".repeat(integer_digits as usize - text.len())), String::new())
    } else {
        (text[..integer_digits as usize].to_string(), text[integer_digits as usize..].to_string())
    };
    let mut grouped = String::with_capacity(integer.len() + integer.len() / 3);
    for (index, digit) in integer.chars().enumerate() {
        if index > 0 && (integer.len() - index) % 3 == 0 {
            grouped.push(group);
        }
        grouped.push(digit);
    }
    let sign = if value < 0.0 { "-" } else { "" };
    if fraction.is_empty() { format!("{sign}{grouped}") } else { format!("{sign}{grouped}{decimal}{fraction}") }
}

/// 🌡️ One legend tick line: its text and the ramp colour it names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World3dLegendLine {
    pub text: String,
    pub rgb: [u8; 3],
}

/// 🌡️ The text of a scalar-field legend in one language: the title with its unit, one line per tick bottom to top, and the no-data caption when a value is missing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct World3dLegendLines {
    pub title: String,
    pub ticks: Vec<World3dLegendLine>,
    pub no_data: Option<&'static str>,
}

impl World3dScalarField {
    /// 🌡️ The legend text for a locale; the React twin is `world3dLegendLines`, pinned against the same `legendLines` fixture.
    pub fn legend_lines(&self, locale: &str) -> World3dLegendLines {
        let title = self.legend.title.resolve(locale);
        let unit = self.legend.unit.as_deref();
        World3dLegendLines {
            title: unit.map_or_else(|| title.to_string(), |unit| format!("{title} ({unit})")),
            ticks: self.legend_ticks().into_iter().map(|tick| World3dLegendLine { text: format!("{}{}", format_world3d_legend_value(tick.value, locale), unit.map_or_else(String::new, |unit| format!(" {unit}"))), rgb: tick.rgb }).collect(),
            no_data: self.values.iter().any(Option::is_none).then(|| world3d_modelling_strings(locale).no_data),
        }
    }
}

impl World3dSection {
    /// ✂️ The unit normal of the removed side.
    pub fn unit_normal(&self) -> World3dVec3 {
        let length = self.normal.iter().map(|component| component * component).sum::<f64>().sqrt();
        [self.normal[0] / length, self.normal[1] / length, self.normal[2] / length]
    }
}

/// 🖍️ The theme defaults of sub-element hover and selection that a highlight token overrides.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dSubElementDefaults<T> {
    pub select: T,
    pub hover: T,
    pub edge_hover: T,
    pub edge_width: f64,
    pub vertex_mark_px: f64,
}

/// 🖍️ The one paint table of sub-element hover and selection per granularity.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dSubElementPaint<T> {
    pub face_select: T,
    pub face_hover: T,
    pub edge_select: T,
    pub edge_hover: T,
    pub vertex_select: T,
    pub vertex_hover: T,
    pub edge_width: f64,
    pub vertex_mark_px: f64,
}

/// 🖍️ Resolves the highlight tokens through `tone` over the theme defaults: a token overrides exactly its granularity, an absent token keeps the default (secondary hover, primary selection in the engine theme);
/// the React twin is `world3dSubElementPaint` and both are pinned against the `subElementPaint` fixture.
pub fn world3d_sub_element_paint<T: Copy>(highlight: Option<&World3dHighlight>, tone: impl Fn(World3dTone) -> T, defaults: World3dSubElementDefaults<T>) -> World3dSubElementPaint<T> {
    let style = |pick: fn(&World3dHighlight) -> Option<World3dSubElementStyle>| highlight.and_then(pick);
    let (face, edge, vertex) = (style(|highlight| highlight.face), style(|highlight| highlight.edge), style(|highlight| highlight.vertex));
    World3dSubElementPaint {
        face_select: face.and_then(|style| style.selected).map_or(defaults.select, &tone),
        face_hover: face.and_then(|style| style.hover).map_or(defaults.hover, &tone),
        edge_select: edge.and_then(|style| style.selected).map_or(defaults.select, &tone),
        edge_hover: edge.and_then(|style| style.hover).map_or(defaults.edge_hover, &tone),
        vertex_select: vertex.and_then(|style| style.selected).map_or(defaults.select, &tone),
        vertex_hover: vertex.and_then(|style| style.hover).map_or(defaults.hover, &tone),
        edge_width: edge.and_then(|style| style.width_px).unwrap_or(defaults.edge_width),
        vertex_mark_px: vertex.and_then(|style| style.width_px).unwrap_or(defaults.vertex_mark_px),
    }
}
//#endregion 🔖️Overlay

//#region 🔖️Lanes
/// 🚚️ Encodes one modelling payload as its lane text.
pub fn world3d_modelling_lane_text<T: ToValue>(value: &T) -> String {
    to_json_string(value)
}

/// 🚚️ Decodes one lane text, refusing anything the schema or its semantic rules refuse.
pub fn world3d_modelling_from_lane_text<T: FromValue>(text: &str) -> Result<T, ValueError> {
    from_json_str(text, JsonMemberPolicy::Reject)
}
//#endregion 🔖️Lanes
