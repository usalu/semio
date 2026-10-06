//! 📝️ Text representation codec surface for `stdio.gltf` (snapshot).

/// 📖️ Grammar include.
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

//#region 🚚️Carrier
/// 🚚️ The carrier this facet's `parse`/`print` speak, named as the schema names the export.
pub type GltfSnapshotText = String;
//#endregion 🚚️Carrier

#[allow(unused_imports)]
mod pack_codec {
use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_dsl_record::DslField;
use crate::standards::v2_0::subsets::any::io::binary::snapshot::owned_pack::*;
impl store::ArtifactDsl for GltfSnapshot{
 const EXTENSION:&'static str="gltf";
 fn envelope_id()->&'static str{"stdio.gltf"}
 fn parse_dsl(text:&str)->Result<Self,semio_framework_diagnostic::TextError>{let body=match store::semio_format::split_text_preamble(text){Ok((envelope,body))=>{if !envelope.matches_identity("stdio.gltf",store::semio_format::Component::Dsl,1){return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("GLTF logical text identity differs").to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))}body},Err(_)=>text};let record=semio_framework_dsl_record::parse_exact(body,&Snapshot::__dsl_spec(),&semio_framework_dsl_record::ParseOptions{limits:semio_framework_diagnostic::Limits::default(),mode:semio_framework_dsl_record::SourceMode::Document})?;Snapshot::__dsl_from_record(&record).map(Into::into)}
 fn print_dsl(&self)->String{let body=semio_framework_dsl_record::print(&record(self),&Snapshot::__dsl_spec(),semio_framework_dsl_record::JoinMode::Document);let envelope=store::semio_format::SemioEnvelope::from_envelope_id("stdio.gltf",store::semio_format::Component::Dsl,1).expect("valid GLTF owned identity");store::semio_format::wrap_text(&envelope,&body)}
}
}

#[cfg(test)]
mod serde_oracle {
use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use serde::de::{MapAccess,SeqAccess,Visitor};
use serde::ser::{SerializeMap,SerializeSeq};
use serde::{Deserialize,Deserializer,Serialize,Serializer};
use std::fmt;
impl Serialize for GltfJson {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            GltfJson::Null => serializer.serialize_unit(),
            GltfJson::Bool(b) => serializer.serialize_bool(*b),
            GltfJson::Number(n) => serializer.serialize_f64(*n),
            GltfJson::String(s) => serializer.serialize_str(s),
            GltfJson::Array(items) => {
                let mut seq = serializer.serialize_seq(Some(items.len()))?;
                for item in items {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            GltfJson::Object(members) => {
                let mut map = serializer.serialize_map(Some(members.len()))?;
                for (k, v) in members {
                    map.serialize_entry(k, v)?;
                }
                map.end()
            }
        }
    }
}

struct GltfJsonVisitor;
impl<'de> Visitor<'de> for GltfJsonVisitor {
    type Value = GltfJson;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a JSON value (glTF extras/extensions)")
    }
    fn visit_unit<E>(self) -> Result<Self::Value, E> {
        Ok(GltfJson::Null)
    }
    fn visit_none<E>(self) -> Result<Self::Value, E> {
        Ok(GltfJson::Null)
    }
    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E> {
        Ok(GltfJson::Bool(v))
    }
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> {
        Ok(GltfJson::Number(v as f64))
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
        Ok(GltfJson::Number(v as f64))
    }
    fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> {
        Ok(GltfJson::Number(v))
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> {
        Ok(GltfJson::String(v.to_string()))
    }
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> {
        Ok(GltfJson::String(v))
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut items = Vec::new();
        while let Some(v) = seq.next_element()? {
            items.push(v);
        }
        Ok(GltfJson::Array(items))
    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut members = Vec::new();
        while let Some((k, v)) = map.next_entry::<String, GltfJson>()? {
            members.push((k, v));
        }
        Ok(GltfJson::Object(members))
    }
}

impl<'de> Deserialize<'de> for GltfJson {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(GltfJsonVisitor)
    }
}


pub(crate) mod ordered_attr_map {
    use super::*;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn serialize<S: Serializer>(attrs: &[(String, usize)], serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(attrs.len()))?;
        for (k, v) in attrs {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }

    struct AttrVisitor;
    impl<'de> Visitor<'de> for AttrVisitor {
        type Value = Vec<(String, usize)>;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("a JSON object mapping attribute semantic to accessor index")
        }
        fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
            let mut out = Vec::new();
            while let Some((k, v)) = map.next_entry::<String, usize>()? {
                out.push((k, v));
            }
            Ok(out)
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<(String, usize)>, D::Error> {
        deserializer.deserialize_map(AttrVisitor)
    }
}
}
#[cfg(test)]
pub(crate) use serde_oracle::ordered_attr_map;

mod semantic_cache_codec {
use crate::standards::v2_0::subsets::any::schema::snapshot::*;
use crate::engine::GltfAccessorType;
use crate::engine::GltfComponentType;
use crate::STDIO_GLTF_DOCUMENT_SCHEMA;
use framework_schema::ArtifactSchema;
#[cfg(test)]
use serde::{Deserialize, Serialize};
impl<'de> Deserialize<'de> for GltfCameraProjection {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive()]
#[cfg_attr(test, derive(Deserialize))]
        #[cfg_attr(test, serde(tag = "type", rename_all = "lowercase"))]
        enum Wire {
            Perspective { perspective: GltfPerspective },
            Orthographic { orthographic: GltfOrthographic },
        }
        Ok(match Wire::deserialize(deserializer)? {
            Wire::Perspective { perspective } => Self::Perspective(perspective),
            Wire::Orthographic { orthographic } => Self::Orthographic(orthographic),
        })
    }
}
/// 🌱️ See the identical note on `GltfCameraProjection`'s `Serialize`/`Deserialize` impls above —
/// additive alongside [`dsl::ToValue`]/[`dsl::FromValue`] below, kept unconditional for the same
/// reason.
impl Serialize for GltfCamera {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive()]
#[cfg_attr(test, derive(Serialize))]
        #[cfg_attr(test, serde(rename_all = "camelCase"))]
        struct Wire<'a> {
            #[cfg_attr(test, serde(rename = "type"))]
            kind: &'static str,
            #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
            perspective: Option<&'a GltfPerspective>,
            #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
            orthographic: Option<&'a GltfOrthographic>,
            #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
            name: Option<&'a String>,
            #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
            extensions: Option<&'a GltfJson>,
            #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
            extras: Option<&'a GltfJson>,
        }
        let (kind, perspective, orthographic) = match &self.projection {
            GltfCameraProjection::Perspective(p) => ("perspective", Some(p), None),
            GltfCameraProjection::Orthographic(o) => ("orthographic", None, Some(o)),
        };
        Wire { kind, perspective, orthographic, name: self.name.as_ref(), extensions: self.extensions.as_ref(), extras: self.extras.as_ref() }.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for GltfCamera {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive()]
#[cfg_attr(test, derive(Deserialize))]
        #[cfg_attr(test, serde(rename_all = "camelCase"))]
        struct Wire {
            #[cfg_attr(test, serde(rename = "type"))]
            kind: String,
            #[cfg_attr(test, serde(default))]
            perspective: Option<GltfPerspective>,
            #[cfg_attr(test, serde(default))]
            orthographic: Option<GltfOrthographic>,
            #[cfg_attr(test, serde(default))]
            name: Option<String>,
            #[cfg_attr(test, serde(default))]
            extensions: Option<GltfJson>,
            #[cfg_attr(test, serde(default))]
            extras: Option<GltfJson>,
        }
        let wire = Wire::deserialize(deserializer)?;
        let projection = match wire.kind.as_str() {
            "perspective" => GltfCameraProjection::Perspective(wire.perspective.ok_or_else(|| serde::de::Error::missing_field("perspective"))?),
            "orthographic" => GltfCameraProjection::Orthographic(wire.orthographic.ok_or_else(|| serde::de::Error::missing_field("orthographic"))?),
            other => return Err(serde::de::Error::custom(format!("camera.type must be 'perspective' or 'orthographic', got {other:?}"))),
        };
        Ok(GltfCamera { projection, name: wire.name, extensions: wire.extensions, extras: wire.extras })
    }
}
}
