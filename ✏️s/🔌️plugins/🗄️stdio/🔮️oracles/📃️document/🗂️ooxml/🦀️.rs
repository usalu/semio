//! 🗂️ The OOXML conformance-class engine of the document test oracle: the OPC container, the XML edits, the content types, the package facts, the wire form and the conformance mutations. See [`super`].

use quick_xml::events::{BytesStart, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use quick_xml::XmlVersion;
use semio_repo_test_host::Json;
use std::io::{Cursor, Read, Write};

#[path = "📦️container-and-xml-edits/🦀️.rs"]
mod container_and_xml_edits;
pub use container_and_xml_edits::*;

#[path = "🏷️content-types-and-package-facts/🦀️.rs"]
mod content_types_and_package_facts;
pub use content_types_and_package_facts::*;

#[path = "🔌️package-wire-and-reading/🦀️.rs"]
mod package_wire_and_reading;
pub use package_wire_and_reading::*;

#[path = "🏅️conformance-mutations-and-projection/🦀️.rs"]
mod conformance_mutations_and_projection;
pub use conformance_mutations_and_projection::*;
