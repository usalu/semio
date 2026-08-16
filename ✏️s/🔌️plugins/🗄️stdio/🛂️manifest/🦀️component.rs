//! 🛂️ Manifest facet for `🗄️stdio` — library plugin stub.

//#region 🔖️FormatCatalog
//! 🗄️ Full `stdio` format roster, plumbed onto the generic `io::FormatDescriptor` registry
//! (ticket 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT wave 3). Rows are transplanted
//! verbatim (mime/extension/dir_name/is_binary) from `mesh::STDIO_FORMAT_CATALOG`
//! (`🧰️framework/🔨️modules/🔺️mesh/🦀️component.rs`), cross-checked against this plugin's own
//! `🗿️artifacts/<dir_name>` directories; `neutral` has no bool equivalent on the closed side so
//! every row is `true` (every stdio format is a neutral interchange format).

use semio_framework_plugin::io::{register_format_descriptors, FormatDescriptor};

/// 🗄️ Full 36-entry stdio format roster, shaped for `register_format_descriptors`.
pub fn stdio_format_descriptors() -> Vec<FormatDescriptor> {
    vec![
        FormatDescriptor {
            kind_id: "stdio.binary".into(),
            short_id: "binary".into(),
            aliases: vec![],
            mime: "application/octet-stream".into(),
            extension: ".bin".into(),
            name: "Binary".into(),
            full_name: "Raw Binary Bytes".into(),
            neutral: true,
            dir_name: "💾️binary".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.txt".into(),
            short_id: "txt".into(),
            aliases: vec![],
            mime: "text/plain".into(),
            extension: ".txt".into(),
            name: "Text".into(),
            full_name: "Plain Text".into(),
            neutral: true,
            dir_name: "📄txt".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.xml".into(),
            short_id: "xml".into(),
            aliases: vec![],
            mime: "application/xml".into(),
            extension: ".xml".into(),
            name: "XML".into(),
            full_name: "Extensible Markup Language".into(),
            neutral: true,
            dir_name: "📰xml".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.deflate".into(),
            short_id: "deflate".into(),
            aliases: vec![],
            mime: "application/zlib".into(),
            extension: ".zz".into(),
            name: "Deflate".into(),
            full_name: "Zlib Deflate Stream".into(),
            neutral: true,
            dir_name: "🗜️deflate".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.zip".into(),
            short_id: "zip".into(),
            aliases: vec![],
            mime: "application/zip".into(),
            extension: ".zip".into(),
            name: "ZIP".into(),
            full_name: "Zip Archive".into(),
            neutral: true,
            dir_name: "🎒️zip".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.json".into(),
            short_id: "json".into(),
            aliases: vec![],
            mime: "application/json".into(),
            extension: ".json".into(),
            name: "JSON".into(),
            full_name: "JavaScript Object Notation".into(),
            neutral: true,
            dir_name: "🔣️json".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.csv".into(),
            short_id: "csv".into(),
            aliases: vec![],
            mime: "text/csv".into(),
            extension: ".csv".into(),
            name: "CSV".into(),
            full_name: "Comma-Separated Values".into(),
            neutral: true,
            dir_name: "📊️csv".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.md".into(),
            short_id: "md".into(),
            aliases: vec![],
            mime: "text/markdown".into(),
            extension: ".md".into(),
            name: "Markdown".into(),
            full_name: "Markdown Text".into(),
            neutral: true,
            dir_name: "📝️md".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.gltf".into(),
            short_id: "gltf".into(),
            aliases: vec![],
            mime: "model/gltf+json".into(),
            extension: ".gltf".into(),
            name: "GLTF".into(),
            full_name: "GL Transmission Format JSON".into(),
            neutral: true,
            dir_name: "🧊️gltf".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.obj".into(),
            short_id: "obj".into(),
            aliases: vec![],
            mime: "model/obj".into(),
            extension: ".obj".into(),
            name: "OBJ".into(),
            full_name: "Wavefront Object".into(),
            neutral: true,
            dir_name: "🧊️obj".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.stl".into(),
            short_id: "stl".into(),
            aliases: vec![],
            mime: "model/stl".into(),
            extension: ".stl".into(),
            name: "STL".into(),
            full_name: "Stereolithography".into(),
            neutral: true,
            dir_name: "🟪️stl".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.ply".into(),
            short_id: "ply".into(),
            aliases: vec![],
            mime: "model/ply".into(),
            extension: ".ply".into(),
            name: "PLY".into(),
            full_name: "Polygon File Format".into(),
            neutral: true,
            dir_name: "☁️ply".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.las".into(),
            short_id: "las".into(),
            aliases: vec![],
            mime: "application/vnd.las".into(),
            extension: ".las".into(),
            name: "LAS".into(),
            full_name: "ASPRS LAS Point Cloud".into(),
            neutral: true,
            dir_name: "☁️las".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.step".into(),
            short_id: "step".into(),
            aliases: vec![],
            mime: "model/step".into(),
            extension: ".step".into(),
            name: "STEP".into(),
            full_name: "ISO 10303 STEP".into(),
            neutral: true,
            dir_name: "📐️step".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.ifc".into(),
            short_id: "ifc".into(),
            aliases: vec![],
            mime: "application/x-ifc".into(),
            extension: ".ifc".into(),
            name: "IFC".into(),
            full_name: "Industry Foundation Classes".into(),
            neutral: true,
            dir_name: "🏗️ifc".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.dwg".into(),
            short_id: "dwg".into(),
            aliases: vec![],
            mime: "image/vnd.dwg".into(),
            extension: ".dwg".into(),
            name: "DWG".into(),
            full_name: "Drawing".into(),
            neutral: true,
            dir_name: "🖊️dwg".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.dxf".into(),
            short_id: "dxf".into(),
            aliases: vec![],
            mime: "image/vnd.dxf".into(),
            extension: ".dxf".into(),
            name: "DXF".into(),
            full_name: "Drawing Exchange Format".into(),
            neutral: true,
            dir_name: "🖊️dxf".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.svg".into(),
            short_id: "svg".into(),
            aliases: vec![],
            mime: "image/svg+xml".into(),
            extension: ".svg".into(),
            name: "SVG".into(),
            full_name: "Scalable Vector Graphics".into(),
            neutral: true,
            dir_name: "🎨️svg".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.png".into(),
            short_id: "png".into(),
            aliases: vec![],
            mime: "image/png".into(),
            extension: ".png".into(),
            name: "PNG".into(),
            full_name: "Portable Network Graphics".into(),
            neutral: true,
            dir_name: "📷️png".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.jpg".into(),
            short_id: "jpg".into(),
            aliases: vec![],
            mime: "image/jpeg".into(),
            extension: ".jpg".into(),
            name: "JPEG".into(),
            full_name: "Joint Photographic Experts Group".into(),
            neutral: true,
            dir_name: "📷️jpg".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.gif".into(),
            short_id: "gif".into(),
            aliases: vec![],
            mime: "image/gif".into(),
            extension: ".gif".into(),
            name: "GIF".into(),
            full_name: "Graphics Interchange Format".into(),
            neutral: true,
            dir_name: "🎞️gif".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.bmp".into(),
            short_id: "bmp".into(),
            aliases: vec![],
            mime: "image/bmp".into(),
            extension: ".bmp".into(),
            name: "BMP".into(),
            full_name: "Bitmap".into(),
            neutral: true,
            dir_name: "🖼️bmp".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.tiff".into(),
            short_id: "tiff".into(),
            aliases: vec![],
            mime: "image/tiff".into(),
            extension: ".tiff".into(),
            name: "TIFF".into(),
            full_name: "Tagged Image File Format".into(),
            neutral: true,
            dir_name: "🖼️tiff".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.pdf".into(),
            short_id: "pdf".into(),
            aliases: vec![],
            mime: "application/pdf".into(),
            extension: ".pdf".into(),
            name: "PDF".into(),
            full_name: "Portable Document Format".into(),
            neutral: true,
            dir_name: "📄️pdf".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.docx".into(),
            short_id: "docx".into(),
            aliases: vec![],
            mime: "application/vnd.openxmlformats-officedocument.wordprocessingml.document".into(),
            extension: ".docx".into(),
            name: "DOCX".into(),
            full_name: "Office Open XML Word".into(),
            neutral: true,
            dir_name: "📜️docx".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.pptx".into(),
            short_id: "pptx".into(),
            aliases: vec![],
            mime: "application/vnd.openxmlformats-officedocument.presentationml.presentation".into(),
            extension: ".pptx".into(),
            name: "PPTX".into(),
            full_name: "Office Open XML PowerPoint".into(),
            neutral: true,
            dir_name: "🎞️pptx".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.xlsx".into(),
            short_id: "xlsx".into(),
            aliases: vec![],
            mime: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".into(),
            extension: ".xlsx".into(),
            name: "XLSX".into(),
            full_name: "Office Open XML Excel".into(),
            neutral: true,
            dir_name: "📕️xlsx".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.bcf".into(),
            short_id: "bcf".into(),
            aliases: vec![],
            mime: "application/vnd.bcf+xml".into(),
            extension: ".bcf".into(),
            name: "BCF".into(),
            full_name: "BIM Collaboration Format".into(),
            neutral: true,
            dir_name: "💬️bcf".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.semio".into(),
            short_id: "semio".into(),
            aliases: vec![],
            mime: "application/vnd.semio".into(),
            extension: ".semio".into(),
            name: "Semio".into(),
            full_name: "Semio".into(),
            neutral: true,
            dir_name: "🧿️semio".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.mp4".into(),
            short_id: "mp4".into(),
            aliases: vec![],
            mime: "video/mp4".into(),
            extension: ".mp4".into(),
            name: "MP4".into(),
            full_name: "MPEG-4 Part 14".into(),
            neutral: true,
            dir_name: "🎥️mp4".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.avi".into(),
            short_id: "avi".into(),
            aliases: vec![],
            mime: "video/x-msvideo".into(),
            extension: ".avi".into(),
            name: "AVI".into(),
            full_name: "Audio Video Interleave".into(),
            neutral: true,
            dir_name: "📼️avi".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.mp3".into(),
            short_id: "mp3".into(),
            aliases: vec![],
            mime: "audio/mpeg".into(),
            extension: ".mp3".into(),
            name: "MP3".into(),
            full_name: "MPEG Audio Layer III".into(),
            neutral: true,
            dir_name: "🎵️mp3".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.wav".into(),
            short_id: "wav".into(),
            aliases: vec![],
            mime: "audio/wav".into(),
            extension: ".wav".into(),
            name: "WAV".into(),
            full_name: "Waveform Audio File Format".into(),
            neutral: true,
            dir_name: "🔊️wav".into(),
            is_binary: true,
        },
        FormatDescriptor {
            kind_id: "stdio.epw".into(),
            short_id: "epw".into(),
            aliases: vec![],
            mime: "".into(),
            extension: ".epw".into(),
            name: "EPW".into(),
            full_name: "EnergyPlus Weather".into(),
            neutral: true,
            dir_name: "🌦️epw".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.tsv".into(),
            short_id: "tsv".into(),
            aliases: vec![],
            mime: "text/tab-separated-values".into(),
            extension: ".tsv".into(),
            name: "TSV".into(),
            full_name: "Tab-Separated Values".into(),
            neutral: true,
            dir_name: "📑️tsv".into(),
            is_binary: false,
        },
        FormatDescriptor {
            kind_id: "stdio.html".into(),
            short_id: "html".into(),
            aliases: vec![],
            mime: "text/html".into(),
            extension: ".html".into(),
            name: "HTML".into(),
            full_name: "Hypertext Markup Language".into(),
            neutral: true,
            dir_name: "🌐️html".into(),
            is_binary: false,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;
    use std::collections::BTreeMap;

    #[derive(Deserialize)]
    struct Catalog {
        stdio_roster: BTreeMap<String, CatalogEntry>,
    }

    #[derive(Deserialize)]
    struct CatalogEntry {
        dir: String,
        mime: Option<String>,
        ext: String,
    }

    #[test]
    fn descriptor_ledger_is_catalog_derived() {
        let catalog: Catalog = serde_json::from_str(include_str!("../📇️registry/📇️catalog.json")).expect("catalog JSON");
        let descriptors = stdio_format_descriptors();
        assert_eq!(descriptors.len(), catalog.stdio_roster.len());
        let descriptor_by_id: BTreeMap<_, _> = descriptors.iter().map(|descriptor| (descriptor.short_id.as_str(), descriptor)).collect();
        assert_eq!(descriptor_by_id.len(), descriptors.len(), "duplicate stdio descriptor id");
        for (id, entry) in catalog.stdio_roster {
            let descriptor = descriptor_by_id.get(id.as_str()).unwrap_or_else(|| panic!("missing descriptor for {id}"));
            assert_eq!(descriptor.kind_id, format!("stdio.{id}"));
            assert_eq!(descriptor.dir_name, entry.dir);
            assert_eq!(descriptor.extension, entry.ext);
            assert_eq!(descriptor.mime, entry.mime.unwrap_or_default());
        }
    }

    #[test]
    fn epw_is_extension_routed_without_claiming_txt_mime() {
        let descriptors = stdio_format_descriptors();
        let txt = descriptors.iter().find(|descriptor| descriptor.short_id == "txt").expect("txt descriptor");
        let epw = descriptors.iter().find(|descriptor| descriptor.short_id == "epw").expect("epw descriptor");
        assert_eq!(txt.mime, "text/plain");
        assert!(epw.mime.trim().is_empty());
        assert_eq!(epw.extension, ".epw");
    }
}

/// 📌️ Registers `stdio_format_descriptors` onto the generic `io::FormatDescriptor` registry.
/// Called from `🗄️stdio`'s `plugin()` (`✏️s/🔌️plugins/🗄️stdio/🦀️component.rs`), alongside its
/// `artifacts::*::engine::register()` calls.
pub fn register_stdio_format_descriptors() {
    register_format_descriptors(stdio_format_descriptors()).expect("stdio format descriptor registry conflict");
}
//#endregion 🔖️FormatCatalog
