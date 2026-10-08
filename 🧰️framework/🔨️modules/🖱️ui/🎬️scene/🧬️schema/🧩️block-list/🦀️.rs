//! 🧩️ Typed, domain-neutral block-list scene records.
use super::{value_push,value_push_option,value_decode,SceneDoc};
use protocol::value::{DslValue,FromValue,ToValue,ValueError,ValueRefusalKind};
use serde::{Serialize,Deserialize};

/// 🧬️ Enforces closed field ownership and nonempty metadata from the canonical schema.
fn block_list_fields(entries:&[(String,DslValue)],fields:&[&str],nonempty:&[&str])->Result<(),ValueError> {
    for (name,_) in entries {
        if !fields.contains(&name.as_str()) {return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("unknown block-list field `{name}`")));}
    }
    for name in fields {
        if entries.iter().filter(|(key,_)|key.as_str()==*name).count()>1 {return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("duplicate block-list field `{name}`")));}
    }
    for name in nonempty {
        if entries.iter().any(|(key,value)|key.as_str()==*name&&value.as_str()==Some("")) {return Err(ValueError::new(ValueRefusalKind::InvalidValue,format!("empty block-list field `{name}`")));}
    }
    Ok(())
}

/// 🎫️ Validates one required nonempty metadata string through the independent Serde facet.
fn block_list_text<'de,D:serde::Deserializer<'de>>(deserializer:D)->Result<String,D::Error> {
    let value=String::deserialize(deserializer)?;
    if value.is_empty() {return Err(serde::de::Error::custom("block-list metadata must be nonempty"));}
    Ok(value)
}

/// 🟢️ Admits a present optional value while its absent field retains the canonical default.
fn block_list_optional<'de,D:serde::Deserializer<'de>,T:Deserialize<'de>>(deserializer:D)->Result<Option<T>,D::Error> {
    Option::<T>::deserialize(deserializer)?.map(Some).ok_or_else(||serde::de::Error::custom("present block-list fields must not be null"))
}

/// 🌱️ Preserves the distinction between an absent field and a refused explicit null.
fn block_list_optional_value<T:FromValue>(entries:&[(String,DslValue)],name:&str)->Result<Option<T>,ValueError> {
    match entries.iter().find(|(key,_)|key.as_str()==name) {None=>Ok(None),Some((_,value))=>T::from_value(value.clone()).map(Some).map_err(|error|error.under(name))}
}

/// 🧭️ The owned BlockListSelectionTarget scene record.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct BlockListSelectionTarget {
    #[serde(deserialize_with="block_list_text")]
    pub granularity: String,
    #[serde(deserialize_with="block_list_text")]
    pub id: String,
}

impl ToValue for BlockListSelectionTarget {
    fn to_value(&self)->DslValue {
        let mut entries=Vec::new();
        value_push(&mut entries,"granularity",&self.granularity);
        value_push(&mut entries,"id",&self.id);
        DslValue::Object(entries)
    }
}

impl FromValue for BlockListSelectionTarget {
    fn from_value(value:DslValue)->Result<Self,ValueError> {
        let entries=value.into_object()?;
        block_list_fields(&entries,&["granularity", "id"],&["granularity", "id"])?;
        Ok(Self {
            granularity: value_decode(&entries,"granularity")?,
            id: value_decode(&entries,"id")?,
        })
    }
}

/// 🧱️ The owned BlockListBlock scene record.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct BlockListBlock {
    #[serde(deserialize_with="block_list_text")]
    pub id: String,
    pub label: String,
    #[serde(deserialize_with="block_list_text")]
    pub kind: String,
    #[serde(default,deserialize_with="block_list_optional",skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    #[serde(default,deserialize_with="block_list_optional",skip_serializing_if="Option::is_none")]
    pub target: Option<BlockListSelectionTarget>,
}

impl ToValue for BlockListBlock {
    fn to_value(&self)->DslValue {
        let mut entries=Vec::new();
        value_push(&mut entries,"id",&self.id);
        value_push(&mut entries,"label",&self.label);
        value_push(&mut entries,"kind",&self.kind);
        value_push_option(&mut entries,"description",&self.description);
        value_push_option(&mut entries,"target",&self.target);
        DslValue::Object(entries)
    }
}

impl FromValue for BlockListBlock {
    fn from_value(value:DslValue)->Result<Self,ValueError> {
        let entries=value.into_object()?;
        block_list_fields(&entries,&["id", "label", "kind", "description", "target"],&["id", "kind"])?;
        Ok(Self {
            id: value_decode(&entries,"id")?,
            label: value_decode(&entries,"label")?,
            kind: value_decode(&entries,"kind")?,
            description: block_list_optional_value(&entries,"description")?,
            target: block_list_optional_value(&entries,"target")?,
        })
    }
}

/// 🪜️ The owned BlockListStep scene record.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct BlockListStep {
    #[serde(deserialize_with="block_list_text")]
    pub id: String,
    pub title: String,
    #[serde(default,deserialize_with="block_list_optional",skip_serializing_if="Option::is_none")]
    pub description: Option<String>,
    pub blocks: Vec<BlockListBlock>,
    #[serde(default,deserialize_with="block_list_optional",skip_serializing_if="Option::is_none")]
    pub target: Option<BlockListSelectionTarget>,
}

impl ToValue for BlockListStep {
    fn to_value(&self)->DslValue {
        let mut entries=Vec::new();
        value_push(&mut entries,"id",&self.id);
        value_push(&mut entries,"title",&self.title);
        value_push_option(&mut entries,"description",&self.description);
        value_push(&mut entries,"blocks",&self.blocks);
        value_push_option(&mut entries,"target",&self.target);
        DslValue::Object(entries)
    }
}

impl FromValue for BlockListStep {
    fn from_value(value:DslValue)->Result<Self,ValueError> {
        let entries=value.into_object()?;
        block_list_fields(&entries,&["id", "title", "description", "blocks", "target"],&["id"])?;
        Ok(Self {
            id: value_decode(&entries,"id")?,
            title: value_decode(&entries,"title")?,
            description: block_list_optional_value(&entries,"description")?,
            blocks: value_decode(&entries,"blocks")?,
            target: block_list_optional_value(&entries,"target")?,
        })
    }
}

/// 🎨️ The owned BlockListPaletteEntry scene record.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct BlockListPaletteEntry {
    #[serde(deserialize_with="block_list_text")]
    pub block_kind: String,
    pub label: String,
    pub icon_id: String,
}

impl ToValue for BlockListPaletteEntry {
    fn to_value(&self)->DslValue {
        let mut entries=Vec::new();
        value_push(&mut entries,"blockKind",&self.block_kind);
        value_push(&mut entries,"label",&self.label);
        value_push(&mut entries,"iconId",&self.icon_id);
        DslValue::Object(entries)
    }
}

impl FromValue for BlockListPaletteEntry {
    fn from_value(value:DslValue)->Result<Self,ValueError> {
        let entries=value.into_object()?;
        block_list_fields(&entries,&["blockKind", "label", "iconId"],&["blockKind"])?;
        Ok(Self {
            block_kind: value_decode(&entries,"blockKind")?,
            label: value_decode(&entries,"label")?,
            icon_id: value_decode(&entries,"iconId")?,
        })
    }
}

/// 🎬️ The owned BlockListScene scene record.
#[derive(Clone,Debug,PartialEq,Serialize,Deserialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct BlockListScene {
    pub steps: Vec<BlockListStep>,
    pub palette: Vec<BlockListPaletteEntry>,
    #[serde(default,deserialize_with="block_list_optional",skip_serializing_if="Option::is_none")]
    pub selected_id: Option<String>,
    #[serde(default,deserialize_with="block_list_optional",skip_serializing_if="Option::is_none")]
    pub dragging_id: Option<String>,
    #[serde(default,deserialize_with="block_list_optional",skip_serializing_if="Option::is_none")]
    pub domain_id: Option<String>,
}

impl ToValue for BlockListScene {
    fn to_value(&self)->DslValue {
        let mut entries=Vec::new();
        value_push(&mut entries,"steps",&self.steps);
        value_push(&mut entries,"palette",&self.palette);
        value_push_option(&mut entries,"selectedId",&self.selected_id);
        value_push_option(&mut entries,"draggingId",&self.dragging_id);
        value_push_option(&mut entries,"domainId",&self.domain_id);
        DslValue::Object(entries)
    }
}

impl FromValue for BlockListScene {
    fn from_value(value:DslValue)->Result<Self,ValueError> {
        let entries=value.into_object()?;
        block_list_fields(&entries,&["steps", "palette", "selectedId", "draggingId", "domainId"],&[])?;
        Ok(Self {
            steps: value_decode(&entries,"steps")?,
            palette: value_decode(&entries,"palette")?,
            selected_id: block_list_optional_value(&entries,"selectedId")?,
            dragging_id: block_list_optional_value(&entries,"draggingId")?,
            domain_id: block_list_optional_value(&entries,"domainId")?,
        })
    }
}

impl SceneDoc for BlockListScene { const SCHEMA: &'static str="block-list@1"; }
