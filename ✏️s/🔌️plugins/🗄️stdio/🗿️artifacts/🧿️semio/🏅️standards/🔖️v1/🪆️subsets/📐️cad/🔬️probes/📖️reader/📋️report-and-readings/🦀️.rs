//! 📋️ Report and readings: the probe version, the report record with its rendering, the engine names and the entity and reading records. See [`super`].

use super::*;

//#region 🔬️Report
const PROBE_VERSION: &str = "ruststep@0.4.0 + dxf@0.6.1";

/// 🔬️ The typed report every probe emits. The orchestrator compares `measurements`; it never
/// computes them, and this binary never computes what a mutation SHOULD have produced.
pub(super) struct Report {
    pub(super) probe: &'static str,
    engine_family: &'static str,
    engine_impl: &'static str,
    engine_version: &'static str,
    pub(super) status: &'static str,
    measurements: Vec<(String, Json)>,
    diagnostics: Vec<(&'static str, String, Option<String>)>,
}

impl Report {
    pub(super) fn new(probe: &'static str, engine: (&'static str, &'static str, &'static str)) -> Self {
        Report { probe, engine_family: engine.0, engine_impl: engine.1, engine_version: engine.2, status: "ok", measurements: Vec::new(), diagnostics: Vec::new() }
    }
    pub(super) fn put(&mut self, key: &str, value: Json) {
        self.measurements.push((key.to_string(), value));
    }
    pub(super) fn diag(&mut self, severity: &'static str, message: String, detail: Option<String>) {
        self.diagnostics.push((severity, message, detail));
    }
    pub(super) fn emit(self, duration_ms: u128) -> String {
        let mut fields = vec![
            ("schema".to_string(), s("semio.repository-test.probe-report/v2")),
            ("probe".to_string(), s(self.probe)),
            ("probeVersion".to_string(), s(PROBE_VERSION)),
            ("engine".to_string(), obj(vec![("family", s(self.engine_family)), ("implementation", s(self.engine_impl)), ("version", s(self.engine_version))])),
            ("status".to_string(), s(self.status)),
            ("durationMs".to_string(), Json::Int(duration_ms as i64)),
            ("measurements".to_string(), Json::Obj(self.measurements)),
        ];
        if !self.diagnostics.is_empty() {
            fields.push((
                "diagnostics".to_string(),
                Json::Arr(
                    self.diagnostics
                        .into_iter()
                        .map(|(sev, msg, detail)| {
                            let mut d = vec![("severity".to_string(), s(sev)), ("message".to_string(), Json::Str(msg))];
                            if let Some(detail) = detail {
                                d.push(("detail".to_string(), Json::Str(detail)));
                            }
                            Json::Obj(d)
                        })
                        .collect(),
                ),
            ));
        }
        Json::Obj(fields).to_text()
    }
}

pub(super) const STEP_ENGINE: (&str, &str, &str) = ("ruststep", "ruststep part-21 ast parser", "0.4.0");

pub(super) const DXF_ENGINE: (&str, &str, &str) = ("dxf-rs", "dxf crate ascii reader/writer", "0.6.1");
//#endregion 🔬️Report

//#region 📐️Readings
/// 📐️ One entity as a THIRD-PARTY READER recovered it — never as our schema declares it.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct Ent {
    pub(super) kind: String,
    pub(super) layer: String,
    pub(super) geom: Vec<f64>,
    pub(super) text: String,
}

impl Ent {
    pub(super) fn json(&self) -> Json {
        obj(vec![("kind", s(&self.kind)), ("layer", s(&self.layer)), ("geom", nums(&self.geom)), ("text", s(&self.text))])
    }
}

/// 📐️ A whole drawing as a third-party reader recovered it.
#[derive(Clone, Debug, PartialEq, Default)]
pub(super) struct Reading {
    pub(super) layers: Vec<(String, i64, String, bool)>,
    pub(super) blocks: Vec<(String, [f64; 2], Vec<Ent>)>,
    pub(super) entities: Vec<Ent>,
}

impl Reading {
    pub(super) fn json(&self) -> Json {
        obj(vec![
            ("layers", Json::Arr(self.layers.iter().map(|(n, c, lt, v)| obj(vec![("name", s(n)), ("colorIndex", Json::Int(*c)), ("lineType", s(lt)), ("visible", Json::Bool(*v))])).collect())),
            ("blocks", Json::Arr(self.blocks.iter().map(|(n, bp, ents)| obj(vec![("name", s(n)), ("basePoint", nums(bp)), ("entities", Json::Arr(ents.iter().map(Ent::json).collect()))])).collect())),
            ("entities", Json::Arr(self.entities.iter().map(Ent::json).collect())),
        ])
    }
    pub(super) fn counts(&self) -> (usize, usize, usize) {
        (self.layers.len(), self.blocks.len(), self.entities.len())
    }
}
//#endregion 📐️Readings
