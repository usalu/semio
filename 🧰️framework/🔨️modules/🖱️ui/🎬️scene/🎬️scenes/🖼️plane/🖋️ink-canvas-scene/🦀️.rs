//! 🖋️ Ink canvas scene: the ink canvas interaction domain and the `InkCanvasScene` payload with its scene doc and value conversions. See [`super`].

use super::*;

//#region 🔖️InkCanvasScene
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InkCanvasInteractionDomain {
    #[serde(deserialize_with = "deserialize_non_empty_interaction_domain_text")]
    pub id: String,
    #[serde(deserialize_with = "deserialize_non_empty_interaction_domain_text")]
    pub granularity_id: String,
}

impl ToValue for InkCanvasInteractionDomain {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "id", &self.id);
        value_push(&mut entries, "granularityId", &self.granularity_id);
        DslValue::Object(entries)
    }
}

impl FromValue for InkCanvasInteractionDomain {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        if let Some((field, _)) = entries.iter().find(|(field, _)| field != "id" && field != "granularityId") {
            return Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("unknown field `{field}`")));
        }
        for field in ["id", "granularityId"] {
            if entries.iter().filter(|(candidate, _)| candidate == field).count() > 1 {
                return Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, format!("duplicate field `{field}`")));
            }
        }
        let domain = Self { id: value_decode(&entries, "id")?, granularity_id: value_decode(&entries, "granularityId")? };
        if domain.id.is_empty() || domain.granularity_id.is_empty() {
            return Err(ValueError::new(protocol::value::ValueRefusalKind::InvalidValue, "ink canvas interaction domain fields must be non-empty"));
        }
        Ok(domain)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InkCanvasScene {
    pub document_json: String,
    #[serde(default = "ink_canvas_default_selection_json")]
    pub selection_json: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hovered_id: Option<String>,
    pub active_utility: String,
    pub view_mode: String,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interaction_domain: Option<InkCanvasInteractionDomain>,
}

impl SceneDoc for InkCanvasScene {
    const SCHEMA: &'static str = "ink-canvas@1";
}

// 🚫️async: E1 pure accessor consumed by external-trait impls (serde default) — see R9
pub fn ink_canvas_default_selection_json() -> String {
    "[]".into()
}

impl InkCanvasScene {
    /** 🖊️ Builds an ink canvas scene with the default empty selection. */
    pub fn base(document_json: String, active_utility: String, view_mode: String, interactive: bool) -> Self {
        Self { document_json, selection_json: ink_canvas_default_selection_json(), hovered_id: None, active_utility, view_mode, interactive, interaction_domain: None }
    }
}

impl ToValue for InkCanvasScene {
    fn to_value(&self) -> DslValue {
        let mut entries = Vec::new();
        value_push(&mut entries, "documentJson", &self.document_json);
        value_push(&mut entries, "selectionJson", &self.selection_json);
        value_push_option(&mut entries, "hoveredId", &self.hovered_id);
        value_push(&mut entries, "activeUtility", &self.active_utility);
        value_push(&mut entries, "viewMode", &self.view_mode);
        value_push(&mut entries, "interactive", &self.interactive);
        value_push_option(&mut entries, "interactionDomain", &self.interaction_domain);
        DslValue::Object(entries)
    }
}

impl FromValue for InkCanvasScene {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = value.into_object()?;
        Ok(Self {
            document_json: value_decode(&entries, "documentJson")?,
            selection_json: value_decode_default(&entries, "selectionJson", ink_canvas_default_selection_json)?,
            hovered_id: value_decode_option(&entries, "hoveredId")?,
            active_utility: value_decode(&entries, "activeUtility")?,
            view_mode: value_decode(&entries, "viewMode")?,
            interactive: value_decode_default(&entries, "interactive", Default::default)?,
            interaction_domain: value_decode_option(&entries, "interactionDomain")?,
        })
    }
}
//#endregion 🔖️InkCanvasScene
