//! 📋️ Native clipboard producers and consumers share one escaped wire envelope.
use super::{ClipboardError, DslValue, PasteAnchor, PastePlacement};

pub const CLIPBOARD_TEXT_MAX_BYTES:usize=1_048_576;
pub const CLIPBOARD_METADATA_MAX_WIRE_BYTES:usize=16_384;
pub const CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES:usize=CLIPBOARD_TEXT_MAX_BYTES*6+CLIPBOARD_METADATA_MAX_WIRE_BYTES;
pub const CLIPBOARD_PASTE_MAX_WIRE_BYTES:usize=CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES+CLIPBOARD_METADATA_MAX_WIRE_BYTES;

impl PastePlacement {
    /// 🎯️ Reads the named placement arguments carried alongside a reserved paste fragment.
    pub fn from_action_args(args: &DslValue) -> Result<Self, ClipboardError> {
        let malformed = || ClipboardError::ParseFailed("malformed paste placement".into());
        let anchor = match args.get("anchor").map(|value| value.as_str().ok_or_else(malformed)).transpose()? {
            None | Some("original") => PasteAnchor::Original,
            Some("middle") => PasteAnchor::Middle,
            Some("centroid") => PasteAnchor::Centroid,
            Some("bottomLeft") => PasteAnchor::BottomLeft,
            Some("bottomRight") => PasteAnchor::BottomRight,
            Some("topLeft") => PasteAnchor::TopLeft,
            Some("topRight") => PasteAnchor::TopRight,
            _ => return Err(malformed()),
        };
        let position = args.get("position").filter(|value| !value.is_null()).map(|value| {
            let [x, y, z] = value.as_array().ok_or_else(malformed)? else { return Err(malformed()) };
            let number = |axis: &DslValue| axis.as_f64().filter(|value| value.is_finite()).ok_or_else(malformed);
            Ok([number(x)?, number(y)?, number(z)?])
        }).transpose()?;
        Ok(Self { anchor, position })
    }

    /// 🧭️ Translates a whole fragment so its requested point anchor lands at the paste position.
    pub fn point_translation(&self, points: impl IntoIterator<Item = [f64; 3]>, default: [f64; 3]) -> [f64; 3] {
        let Some(target) = self.position else { return default };
        if self.anchor == PasteAnchor::Original {
            return target;
        }
        let mut points = points.into_iter();
        let Some(first) = points.next() else { return target };
        let (mut min, mut max, mut sum, mut count) = (first, first, first, 1);
        for point in points {
            for (axis, value) in point.into_iter().enumerate() {
                min[axis] = min[axis].min(value);
                max[axis] = max[axis].max(value);
                sum[axis] += value;
            }
            count += 1;
        }
        let middle = std::array::from_fn(|axis| min[axis].midpoint(max[axis]));
        let anchor = match self.anchor {
            PasteAnchor::Original => [0.0; 3],
            PasteAnchor::Middle => middle,
            PasteAnchor::Centroid => sum.map(|value| value / f64::from(count)),
            PasteAnchor::BottomLeft => [min[0], max[1], middle[2]],
            PasteAnchor::BottomRight => [max[0], max[1], middle[2]],
            PasteAnchor::TopLeft => [min[0], min[1], middle[2]],
            PasteAnchor::TopRight => [max[0], min[1], middle[2]],
        };
        std::array::from_fn(|axis| target[axis] - anchor[axis])
    }
}
