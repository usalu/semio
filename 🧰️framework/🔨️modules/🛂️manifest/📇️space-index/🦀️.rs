//! 📇️ Space index contribution: the open topic through which a plugin names the artifact-index document the os shell opens for a hub space, so the framework never names the plugin that supplies it.
use super::*;

//#region 🔖️SpaceIndexContribution
/// 📇️ Topic under which a plugin names the artifact-index document the os shell opens for a hub space; the framework reads the contribution and never names the plugin that supplies it.
pub const SPACE_INDEX_TOPIC: &str = "os.space-index";

/// 📇️ Payload of [`SPACE_INDEX_TOPIC`]: the dialect of the app that edits the space index and the document schema the shell opens it with.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct SpaceIndexContribution {
    pub dialect: ArtifactDialect,
    pub document_schema: String,
}

impl SpaceIndexContribution {
    /// 📤️ Wraps the contribution as the open topic row a plugin publishes.
    pub fn into_topic_contribution(self) -> TopicContribution {
        TopicContribution::new(SPACE_INDEX_TOPIC, ToValue::to_value(&self))
    }

    /// 📥️ Reads the first well-formed [`SPACE_INDEX_TOPIC`] row among a manifest's topic contributions.
    pub fn from_topic_contributions(contributions: &[TopicContribution]) -> Option<Self> {
        contributions.iter().filter(|row| row.topic == SPACE_INDEX_TOPIC).find_map(|row| row.decode::<Self>().ok())
    }
}
//#endregion 🔖️SpaceIndexContribution
