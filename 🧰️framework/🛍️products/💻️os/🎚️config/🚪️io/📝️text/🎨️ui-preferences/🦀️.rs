//! 🎭️ Native external schema facets for the authored first-party UI preference records.

use crate::opening_config::{UiDriver,UiTheme,UserNamedLayout,UiPreferences};
use semio_framework_value::{DslValue,FromValue,ToValue};

fn schema(name:&str,generator:&mut schemars::gen::SchemaGenerator)->schemars::schema::Schema {
    let text=include_str!("../../../../🧬️schema/🎨️ui-preferences/🔣️.json").replace("#/$defs/","#/definitions/OsConfigUi");
    let mut root:serde_json::Value=serde_json::from_str(&text).expect("authored UI preference schema");
    let definitions=root.as_object_mut().expect("schema object").remove("$defs").expect("authored definitions");
    for(key,value)in definitions.as_object().expect("definition object"){
        generator.definitions_mut().entry(format!("OsConfigUi{key}")).or_insert_with(||serde_json::from_value(value.clone()).expect("authored schema definition"));
    }
    if name=="UiPreferences"{serde_json::from_value(root).expect("authored preferences schema")}else{generator.definitions()[&format!("OsConfigUi{name}")].clone()}
}

macro_rules! native_facet {
    ($($owner:ty),+)=>{$(
        impl serde::Serialize for $owner {
            fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{serde::Serialize::serialize(&self.to_value(),serializer)}
        }
        impl<'de> serde::Deserialize<'de> for $owner {
            fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{Self::from_value(<DslValue as serde::Deserialize>::deserialize(deserializer)?).map_err(serde::de::Error::custom)}
        }
        impl schemars::JsonSchema for $owner {
            fn schema_name()->String{stringify!($owner).to_owned()}
            fn json_schema(generator:&mut schemars::gen::SchemaGenerator)->schemars::schema::Schema{schema(stringify!($owner),generator)}
        }
    )+};
}
native_facet!(UiDriver,UiTheme,UserNamedLayout,UiPreferences);
