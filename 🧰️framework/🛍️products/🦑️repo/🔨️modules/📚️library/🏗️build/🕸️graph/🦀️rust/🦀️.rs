use std::sync::OnceLock;

/// 🗂️ Resolves the neutral graph publication from its declared partition, mounted namespace and filename.
pub fn graph_file() -> &'static str {
    static PATH: OnceLock<String> = OnceLock::new();
    PATH.get_or_init(|| {
        let publication: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/📣️publication.json")).expect("graph publication declaration");
        let policy: serde_json::Value = serde_json::from_str(include_str!("../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json")).expect("cache partition declaration");
        let image: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json")).expect("native image declaration");
        let fields = &publication["properties"];
        let partition = fields["partition"]["const"].as_str().expect("graph partition identity");
        format!("{}/{}/{}", policy["partitions"][partition]["directory"].as_str().expect("graph partition directory"), image["definitions"]["ImageProtocol"]["const"]["mountDirectory"].as_str().expect("graph mount directory"), fields["graph"]["const"].as_str().expect("graph publication filename"))
    }).as_str()
}
