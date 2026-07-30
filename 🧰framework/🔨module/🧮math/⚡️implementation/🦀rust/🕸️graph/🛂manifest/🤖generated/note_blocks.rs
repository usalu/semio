// Generated from note-blocks.manifest.json

use crate::Manifest;

pub const NOTEBLOCKS_MANIFEST_JSON: &str = "{\"schema\":\"manifest\",\"id\":\"note-blocks\",\"name\":\"Note Document Blocks\",\"blockKinds\":[{\"id\":\"text\",\"name\":\"Text\",\"properties\":[{\"name\":\"paragraphs\",\"kind\":\"data\",\"valueType\":{\"kind\":\"text\"}}]},{\"id\":\"image\",\"name\":\"Image\"},{\"id\":\"table\",\"name\":\"Table\"},{\"id\":\"math\",\"name\":\"Math\",\"properties\":[{\"name\":\"tex\",\"kind\":\"data\",\"valueType\":{\"kind\":\"text\"}}]},{\"id\":\"ink\",\"name\":\"Ink\"},{\"id\":\"group\",\"name\":\"Group\"}]}";

pub fn note_blocks_manifest() -> Manifest {
    serde_json::from_str(NOTEBLOCKS_MANIFEST_JSON).expect("manifest json")
}
