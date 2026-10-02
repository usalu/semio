use crate::{register_space_fixture_sources, SpaceFixtureCodec, SpaceFixtureFormat, SpaceFixtureSource};
use std::sync::OnceLock;

/// 🧬️ Decodes the selected artifact's declared source format through its owned snapshot codec.
fn decode<S: store::ArtifactDsl + semio_framework_value::ToValue + semio_framework_value::FromValue>(format: SpaceFixtureFormat, text: &str) -> Result<String, String> {
    let snapshot = match format {
        SpaceFixtureFormat::Dsl => <S as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())?,
        SpaceFixtureFormat::Json => store::os_pack::json::from_json_str::<S>(text).map_err(|error| error.to_string())?,
    };
    Ok(store::os_pack::json::to_json_string(&snapshot))
}

/// 📚️ Selects the unique original example declarations for this actual composition.
pub(crate) fn sources() -> [SpaceFixtureSource; 2] {
    [
        SpaceFixtureSource { slug: "🖍️semio.draw.json".into(), format: SpaceFixtureFormat::Dsl, codec: "draw.snapshot.v1".into(), text: semio_s_artifact_draw_drawing::examples::demo::source().document_json() },
        SpaceFixtureSource { slug: "✒️jack.writer.json".into(), format: SpaceFixtureFormat::Dsl, codec: "writer.snapshot.v1".into(), text: semio_s_artifact_writer_writer::examples::demo::source().document_json() },
    ]
}

/// 🔑️ Binds source identities to the selected owners' real snapshot implementations.
pub(crate) fn codecs() -> [SpaceFixtureCodec; 2] {
    [
        SpaceFixtureCodec { id: "draw.snapshot.v1", decode: decode::<semio_s_artifact_draw_drawing::DrawingSnapshot> },
        SpaceFixtureCodec { id: "writer.snapshot.v1", decode: decode::<semio_s_artifact_writer_writer::WriterSnapshot> },
    ]
}

/// 🌱️ Admits real demo documents before catalog construction or export can consume them.
pub(crate) fn admit() -> Result<(), String> {
    static ADMISSION: OnceLock<Result<(), String>> = OnceLock::new();
    ADMISSION.get_or_init(|| register_space_fixture_sources(&sources(), &codecs())).clone()
}

/// 📤️ Resolves the actual admitted document for a selected demo app binding.
pub(crate) fn document_for_app(plugin_id: &str, app_id: &str) -> Result<String, String> {
    admit()?;
    let bindings = [("draw", "draw", "🖍️semio.draw.json"), ("writer", "writer", "✒️jack.writer.json")];
    let slug = bindings.iter().find(|(plugin, app, _)| *plugin == plugin_id && *app == app_id).map(|(_, _, slug)| *slug).ok_or_else(|| format!("no admitted demo fixture for {plugin_id}/{app_id}"))?;
    semio_framework_os::os_fixture_document(slug).ok_or_else(|| format!("admitted fixture {slug} is missing"))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️native/🦀️.rs"]
mod tests;
