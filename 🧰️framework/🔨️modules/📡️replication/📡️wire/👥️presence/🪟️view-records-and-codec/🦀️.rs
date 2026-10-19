//! 🪟️ Presence view: the window view, the view kind and the interface records with their value conversions and encoders. See [`super`].

use super::*;

//#region 🔖️Presence
//#region 🔖️PresenceView
/// 🪟️ One open window/surface's live view for a document — camera/pan-zoom plus in-view
/// pointer, broadcast so peers can render each other's viewport rectangles / camera frustums /
/// cursor markers. `window_id` disambiguates multiple windows viewing the same space; `space` is
/// the coordinate-space id the surface host reports (`"world"`/`"canvas"`/`"geo"`, or an app-declared
/// finer id) — an overlay renders a peer view only in local surfaces with the same `space`.
#[derive(Clone, Debug, PartialEq)]
pub struct PresenceWindowView {
    pub window_id: String,
    pub space: String,
    pub kind: PresenceViewKind,
    /// 📐️ The reporting surface's pixel size — needed to draw a peer's viewport rectangle.
    pub size: [f64; 2],
    /// 📍️ In view coordinates: world point (Orbit), `[x, y, 0]` canvas point (Canvas),
    /// `[lng, lat, 0]` (Geo).
    pub pointer: Option<[f64; 3]>,
    /// 📏️ World-space ray origin for 3D presence (Orbit); absent for canvas/geo.
    pub ray_origin: Option<[f64; 3]>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. The real wire path is
/// `🔖️PresenceViewCodec` below (a hand-rolled binary codec); this exists for capability parity.
impl semio_framework_value::ToValue for PresenceWindowView {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = vec![
            ("windowId".to_string(), semio_framework_value::ToValue::to_value(&self.window_id)),
            ("space".to_string(), semio_framework_value::ToValue::to_value(&self.space)),
            ("kind".to_string(), semio_framework_value::ToValue::to_value(&self.kind)),
            ("size".to_string(), semio_framework_value::ToValue::to_value(&self.size)),
        ];
        if self.pointer.is_some() {
            entries.push(("pointer".to_string(), semio_framework_value::ToValue::to_value(&self.pointer)));
        }
        if self.ray_origin.is_some() {
            entries.push(("rayOrigin".to_string(), semio_framework_value::ToValue::to_value(&self.ray_origin)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}

impl semio_framework_value::FromValue for PresenceWindowView {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceWindowView, found {value:?}")));
        };
        let mut window_id = None;
        let mut space = None;
        let mut kind = None;
        let mut size = None;
        let mut pointer = None;
        let mut ray_origin = None;
        for (key, entry) in fields {
            match key.as_str() {
                "windowId" => window_id = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("windowId"))?),
                "space" => space = Some(<String as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("space"))?),
                "kind" => kind = Some(<PresenceViewKind as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("kind"))?),
                "size" => size = Some(<[f64; 2] as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("size"))?),
                "pointer" => pointer = <Option<[f64; 3]> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("pointer"))?,
                "rayOrigin" => ray_origin = <Option<[f64; 3]> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("rayOrigin"))?,
                _ => {}
            }
        }
        Ok(PresenceWindowView {
            window_id: window_id.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceWindowView missing windowId"))?,
            space: space.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceWindowView missing space"))?,
            kind: kind.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceWindowView missing kind"))?,
            size: size.ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceWindowView missing size"))?,
            pointer,
            ray_origin,
        })
    }
}

/// 🎥️ A peer's live camera/pan-zoom, tagged by surface family.
#[derive(Clone, Debug, PartialEq)]
pub enum PresenceViewKind {
    Canvas { x: f64, y: f64, zoom: f64 },
    Orbit { position: [f64; 3], target: [f64; 3], up: [f64; 3], fov: f64 },
    Geo { lng: f64, lat: f64, zoom: f64, bearing: f64, pitch: f64 },
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above. Internally tagged on
/// `"kind"`, mirroring `#[serde(tag = "kind", rename_all = "camelCase")]` byte-for-byte.
impl semio_framework_value::ToValue for PresenceViewKind {
    fn to_value(&self) -> semio_framework_value::DslValue {
        match self {
            PresenceViewKind::Canvas { x, y, zoom } => semio_framework_value::DslValue::object(vec![
                ("kind".to_string(), semio_framework_value::DslValue::String("canvas".to_string())),
                ("x".to_string(), semio_framework_value::ToValue::to_value(x)),
                ("y".to_string(), semio_framework_value::ToValue::to_value(y)),
                ("zoom".to_string(), semio_framework_value::ToValue::to_value(zoom)),
            ]),
            PresenceViewKind::Orbit { position, target, up, fov } => semio_framework_value::DslValue::object(vec![
                ("kind".to_string(), semio_framework_value::DslValue::String("orbit".to_string())),
                ("position".to_string(), semio_framework_value::ToValue::to_value(position)),
                ("target".to_string(), semio_framework_value::ToValue::to_value(target)),
                ("up".to_string(), semio_framework_value::ToValue::to_value(up)),
                ("fov".to_string(), semio_framework_value::ToValue::to_value(fov)),
            ]),
            PresenceViewKind::Geo { lng, lat, zoom, bearing, pitch } => semio_framework_value::DslValue::object(vec![
                ("kind".to_string(), semio_framework_value::DslValue::String("geo".to_string())),
                ("lng".to_string(), semio_framework_value::ToValue::to_value(lng)),
                ("lat".to_string(), semio_framework_value::ToValue::to_value(lat)),
                ("zoom".to_string(), semio_framework_value::ToValue::to_value(zoom)),
                ("bearing".to_string(), semio_framework_value::ToValue::to_value(bearing)),
                ("pitch".to_string(), semio_framework_value::ToValue::to_value(pitch)),
            ]),
        }
    }
}

impl semio_framework_value::FromValue for PresenceViewKind {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceViewKind, found {value:?}")));
        };
        let get = |key: &str| fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let kind = match get("kind") {
            Some(semio_framework_value::DslValue::String(s)) => s,
            _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind missing kind")),
        };
        match kind.as_str() {
            "canvas" => Ok(PresenceViewKind::Canvas {
                x: <f64 as semio_framework_value::FromValue>::from_value(get("x").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.canvas missing x"))?).map_err(|e| e.under("x"))?,
                y: <f64 as semio_framework_value::FromValue>::from_value(get("y").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.canvas missing y"))?).map_err(|e| e.under("y"))?,
                zoom: <f64 as semio_framework_value::FromValue>::from_value(get("zoom").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.canvas missing zoom"))?).map_err(|e| e.under("zoom"))?,
            }),
            "orbit" => Ok(PresenceViewKind::Orbit {
                position: <[f64; 3] as semio_framework_value::FromValue>::from_value(get("position").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.orbit missing position"))?).map_err(|e| e.under("position"))?,
                target: <[f64; 3] as semio_framework_value::FromValue>::from_value(get("target").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.orbit missing target"))?).map_err(|e| e.under("target"))?,
                up: <[f64; 3] as semio_framework_value::FromValue>::from_value(get("up").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.orbit missing up"))?).map_err(|e| e.under("up"))?,
                fov: <f64 as semio_framework_value::FromValue>::from_value(get("fov").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.orbit missing fov"))?).map_err(|e| e.under("fov"))?,
            }),
            "geo" => Ok(PresenceViewKind::Geo {
                lng: <f64 as semio_framework_value::FromValue>::from_value(get("lng").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing lng"))?).map_err(|e| e.under("lng"))?,
                lat: <f64 as semio_framework_value::FromValue>::from_value(get("lat").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing lat"))?).map_err(|e| e.under("lat"))?,
                zoom: <f64 as semio_framework_value::FromValue>::from_value(get("zoom").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing zoom"))?).map_err(|e| e.under("zoom"))?,
                bearing: <f64 as semio_framework_value::FromValue>::from_value(get("bearing").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing bearing"))?).map_err(|e| e.under("bearing"))?,
                pitch: <f64 as semio_framework_value::FromValue>::from_value(get("pitch").ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "PresenceViewKind.geo missing pitch"))?).map_err(|e| e.under("pitch"))?,
            }),
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown PresenceViewKind kind `{other}`"))),
        }
    }
}

/// 🖱️ A peer's live `data-ui-path` hover/focus/press state (APP scope) — the grammar
/// `type[idx]#id/...`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PresenceUi {
    pub hovered_path: Option<String>,
    pub focused_path: Option<String>,
    pub pressed_path: Option<String>,
}

/// 🌱️ Hand-written, not derived — same DAG reason as `SelectionMode` above.
impl semio_framework_value::ToValue for PresenceUi {
    fn to_value(&self) -> semio_framework_value::DslValue {
        let mut entries = Vec::new();
        if self.hovered_path.is_some() {
            entries.push(("hoveredPath".to_string(), semio_framework_value::ToValue::to_value(&self.hovered_path)));
        }
        if self.focused_path.is_some() {
            entries.push(("focusedPath".to_string(), semio_framework_value::ToValue::to_value(&self.focused_path)));
        }
        if self.pressed_path.is_some() {
            entries.push(("pressedPath".to_string(), semio_framework_value::ToValue::to_value(&self.pressed_path)));
        }
        semio_framework_value::DslValue::object(entries)
    }
}

impl semio_framework_value::FromValue for PresenceUi {
    fn from_value(value: semio_framework_value::DslValue) -> Result<Self, semio_framework_value::ValueError> {
        let semio_framework_value::DslValue::Object(fields) = value else {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for PresenceUi, found {value:?}")));
        };
        let mut out = PresenceUi::default();
        for (key, entry) in fields {
            match key.as_str() {
                "hoveredPath" => out.hovered_path = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("hoveredPath"))?,
                "focusedPath" => out.focused_path = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("focusedPath"))?,
                "pressedPath" => out.pressed_path = <Option<String> as semio_framework_value::FromValue>::from_value(entry).map_err(|e| e.under("pressedPath"))?,
                _ => {}
            }
        }
        Ok(out)
    }
}

//#region 🔖️PresenceViewCodec
async fn encode_presence_view_kind(kind: &PresenceViewKind, out: &mut Vec<u8>) {
    match kind {
        PresenceViewKind::Canvas { x, y, zoom } => {
            out.push(0);
            semio_framework_replication_wire_core::write_f64(out, *x);
            semio_framework_replication_wire_core::write_f64(out, *y);
            semio_framework_replication_wire_core::write_f64(out, *zoom);
        }
        PresenceViewKind::Orbit { position, target, up, fov } => {
            out.push(1);
            for value in position.iter().chain(target.iter()).chain(up.iter()) {
                semio_framework_replication_wire_core::write_f64(out, *value);
            }
            semio_framework_replication_wire_core::write_f64(out, *fov);
        }
        PresenceViewKind::Geo { lng, lat, zoom, bearing, pitch } => {
            out.push(2);
            semio_framework_replication_wire_core::write_f64(out, *lng);
            semio_framework_replication_wire_core::write_f64(out, *lat);
            semio_framework_replication_wire_core::write_f64(out, *zoom);
            semio_framework_replication_wire_core::write_f64(out, *bearing);
            semio_framework_replication_wire_core::write_f64(out, *pitch);
        }
    }
}

async fn encode_presence_window_view(view: &PresenceWindowView, out: &mut Vec<u8>) {
    semio_framework_replication_wire_core::write_str(out, &view.window_id);
    semio_framework_replication_wire_core::write_str(out, &view.space);
    encode_presence_view_kind(&view.kind, out).await;
    semio_framework_replication_wire_core::write_f64(out, view.size[0]);
    semio_framework_replication_wire_core::write_f64(out, view.size[1]);
    semio_framework_replication_wire_core::write_bool(out, view.pointer.is_some());
    if let Some(pointer) = view.pointer {
        for value in pointer {
            semio_framework_replication_wire_core::write_f64(out, value);
        }
    }
    semio_framework_replication_wire_core::write_bool(out, view.ray_origin.is_some());
    if let Some(ray_origin) = view.ray_origin {
        for value in ray_origin {
            semio_framework_replication_wire_core::write_f64(out, value);
        }
    }
}

pub(super) async fn write_vec_presence_window_view(out: &mut Vec<u8>, values: &[PresenceWindowView]) {
    semio_framework_replication_wire_core::write_varint_u64(out, values.len() as u64);
    for value in values {
        encode_presence_window_view(value, out).await;
    }
}

pub(super) async fn encode_presence_ui(ui: &PresenceUi, out: &mut Vec<u8>) {
    write_opt_str(out, &ui.hovered_path).await;
    write_opt_str(out, &ui.focused_path).await;
    write_opt_str(out, &ui.pressed_path).await;
}
//#endregion 🔖️PresenceViewCodec
//#endregion 🔖️PresenceView
//#endregion 🔖️Presence
