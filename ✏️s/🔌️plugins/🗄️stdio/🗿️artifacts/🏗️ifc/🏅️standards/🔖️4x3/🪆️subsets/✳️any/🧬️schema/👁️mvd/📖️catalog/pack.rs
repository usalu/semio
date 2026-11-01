//! 📥️ Immutable scope rows retained from the official ADD2 listings, decoded once from the committed IFCJ resource `mvd.ifcj` whose neutral form is `🔣️.json`.
use super::{IfcMvdCatalog, IfcMvdEntry, IfcMvdSource};
use semio_s_artifact_stdio_ifc_json_pack::{with_json_pack, Json, JsonPackError, Leaker};
use std::sync::OnceLock;

static CATALOG: OnceLock<IfcMvdCatalog> = OnceLock::new();
const RESOURCE: &[u8] = include_bytes!("mvd.ifcj");

fn entry(leak: &mut Leaker, json: &Json<'_>) -> Result<IfcMvdEntry, JsonPackError> {
    let fields = json.object(&["id", "name", "href", "universal", "views"])?;
    let views = fields.require("views")?.object(&["ReferenceView", "Alignment-basedReferenceView"])?;
    Ok(IfcMvdEntry {
        id: leak.text(fields.require("id")?.string()?),
        name: leak.text(fields.require("name")?.string()?),
        href: leak.text(fields.require("href")?.string()?),
        universal: fields.require("universal")?.boolean()?,
        reference: views.get("ReferenceView").map(|causes| leak.texts(Some(causes))).transpose()?,
        alignment: views.get("Alignment-basedReferenceView").map(|causes| leak.texts(Some(causes))).transpose()?,
    })
}

fn entries(leak: &mut Leaker, json: &Json<'_>) -> Result<&'static [IfcMvdEntry], JsonPackError> {
    let mut rows = json.array()?.iter().map(|item| entry(leak, item)).collect::<Result<Vec<_>, _>>()?;
    rows.sort_by(|left, right| left.id.cmp(right.id));
    Ok(Leaker::slice(rows))
}

/// 📥️ Decode a committed scope resource into immutable statics sorted by id; every refusal is a closed cause.
pub fn decode_mvd(pack: &[u8]) -> Result<IfcMvdCatalog, JsonPackError> {
    with_json_pack(pack, |root| {
        let mut leak = Leaker::default();
        let fields = root.object(&["schema", "sourceCommit", "sources", "conformanceEvaluated", "views", "entities", "concepts"])?;
        let mut sources = Vec::new();
        for item in fields.require("sources")?.array()? {
            let row = item.object(&["url", "sha256"])?;
            sources.push(IfcMvdSource { url: leak.text(row.require("url")?.string()?), sha256: leak.text(row.require("sha256")?.string()?) });
        }
        Ok(IfcMvdCatalog {
            schema: leak.text(fields.require("schema")?.string()?),
            source_commit: leak.text(fields.require("sourceCommit")?.string()?),
            sources: Leaker::slice(sources),
            conformance_evaluated: fields.require("conformanceEvaluated")?.boolean()?,
            entities: entries(&mut leak, fields.require("entities")?)?,
            concepts: entries(&mut leak, fields.require("concepts")?)?,
        })
    })
}

/// 📖️ Borrow the full immutable scope catalog; the first call decodes the embedded resource once, which the per-resource laws prove cannot be refused (a refusal is a corrupted build artifact).
pub fn embedded_mvd() -> &'static IfcMvdCatalog {
    CATALOG.get_or_init(|| match decode_mvd(RESOURCE) {
        Ok(catalog) => catalog,
        Err(error) => panic!("embedded IFC4x3 MVD resource refused: {error:?}"),
    })
}
