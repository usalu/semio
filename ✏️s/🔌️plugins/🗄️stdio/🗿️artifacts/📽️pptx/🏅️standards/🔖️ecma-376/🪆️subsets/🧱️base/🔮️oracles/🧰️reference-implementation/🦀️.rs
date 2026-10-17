//! 🧰️ Reference implementation of the PPTX oracle: the independent zip and quick-xml composition with the shared imports of the former inline module. See [`super`].

use quick_xml::escape::resolve_xml_entity;
use quick_xml::events::{BytesEnd, BytesRef, BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use quick_xml::XmlVersion;
use semio_repo_test_host::Json;
use std::collections::HashMap;
use std::io::{Cursor, Read, Write};

#[path = "🌳️xml-tree-parse-and-serialize/🦀️.rs"]
mod xml_tree_parse_and_serialize;
use xml_tree_parse_and_serialize::*;

#[path = "📦️package-and-presentation-model/🦀️.rs"]
mod package_and_presentation_model;
use package_and_presentation_model::*;

#[path = "🖼️render-and-write-presentation/🦀️.rs"]
mod render_and_write_presentation;
use render_and_write_presentation::*;

#[path = "🧾️json-wire-and-edit-helpers/🦀️.rs"]
mod json_wire_and_edit_helpers;
use json_wire_and_edit_helpers::*;

#[path = "🧬️mutation-apply/🦀️.rs"]
mod mutation_apply;
use mutation_apply::*;

#[path = "🔁️entry-points-and-plumbing/🦀️.rs"]
mod entry_points_and_plumbing;
pub use entry_points_and_plumbing::*;
