use crate::{prepare_space_document_sources, SpaceDocumentCodec, SpaceDocumentFormat, SpaceDocumentSource};

/// 🧬️ Decodes the selected artifact's declared source format through its owned snapshot codec.
fn decode<S: store::ArtifactDsl + semio_framework_value::ToValue + semio_framework_value::FromValue>(format: SpaceDocumentFormat, text: &str) -> Result<String, String> {
    let snapshot = match format {
        SpaceDocumentFormat::Dsl => <S as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())?,
        SpaceDocumentFormat::Json => semio_framework_pack_json::from_json_str::<S>(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?,
    };
    Ok(semio_framework_pack_json::to_json_string(&snapshot))
}

/// 📚️ Selects the unique original example declarations for this actual composition.
pub(crate) fn sources() -> [SpaceDocumentSource; 2] {
    [
        SpaceDocumentSource { slug: "🖍️semio.draw.json".into(), format: SpaceDocumentFormat::Dsl, codec: "draw.snapshot.v1".into(), text: semio_s_artifact_draw_drawing::examples::demo::source().document_json() },
        SpaceDocumentSource { slug: "✒️jack.writer.json".into(), format: SpaceDocumentFormat::Dsl, codec: "writer.snapshot.v1".into(), text: semio_s_artifact_writer_writer::examples::demo::source().document_json() },
    ]
}

/// 🔑️ Binds source identities to the selected owners' real snapshot implementations.
pub(crate) fn codecs() -> [SpaceDocumentCodec; 2] {
    [
        SpaceDocumentCodec { id: "draw.snapshot.v1", decode: decode::<semio_s_artifact_draw_drawing::DrawingSnapshot> },
        SpaceDocumentCodec { id: "writer.snapshot.v1", decode: decode::<semio_s_artifact_writer_writer::WriterSnapshot> },
    ]
}

/// 📤️ Builds test input through the selected artifact's owned codec.
pub(crate) fn document_for_app(plugin_id: &str, app_id: &str) -> Result<String, String> {
    let bindings = [("draw", "draw", "🖍️semio.draw.json"), ("writer", "writer", "✒️jack.writer.json")];
    let slug = bindings.iter().find(|(plugin, app, _)| *plugin == plugin_id && *app == app_id).map(|(_, _, slug)| *slug).ok_or_else(|| format!("no test input for {plugin_id}/{app_id}"))?;
    prepare_space_document_sources(&sources(), &codecs())?.into_iter().find(|(identity, _)| identity == slug).map(|(_, document)| document).ok_or_else(|| format!("test input {slug} is missing"))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️native/🦀️.rs"]
mod tests;
