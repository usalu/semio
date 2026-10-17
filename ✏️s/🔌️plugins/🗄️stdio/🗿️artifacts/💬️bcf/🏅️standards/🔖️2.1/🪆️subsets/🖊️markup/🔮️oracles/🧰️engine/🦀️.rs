//! 🧰️ Independent BCF 2.1 oracle engine: the model, the XML tree, the zip archive, the markup and visualization XML, the JSON codec and the forward and inverse mutations. See [`super`].

use quick_xml::escape::resolve_xml_entity;
use quick_xml::events::{BytesDecl, BytesEnd, BytesRef, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use quick_xml::XmlVersion;
use semio_repo_test_host::{digest, Json};
use std::collections::{BTreeMap, HashSet};
use std::io::Cursor;

#[path = "📐️model-and-xml/🦀️.rs"]
mod model_and_xml;
use model_and_xml::*;

#[path = "💬️markup-and-viewpoint-xml/🦀️.rs"]
mod markup_and_viewpoint_xml;
use markup_and_viewpoint_xml::*;

#[path = "🧬️document-codec-and-json/🦀️.rs"]
mod document_codec_and_json;
use document_codec_and_json::*;

#[path = "🦠️forward-and-inverse/🦀️.rs"]
mod forward_and_inverse;
use forward_and_inverse::*;

#[path = "🧭️routing-and-projection/🦀️.rs"]
mod routing_and_projection;
pub use routing_and_projection::*;
