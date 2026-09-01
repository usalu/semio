//! 🧱️ Reads a glTF 2.0 document through `gltf` 1.4's own public API and projects the six RESOURCE
//! arrays the specification defines: accessors, buffers, bufferViews, images, samplers, textures.
//!
//! This exists because the subset's other two readers cannot answer the question:
//!
//! * `three`'s `GLTFLoader` builds a SCENE GRAPH. A resource nothing references never becomes an
//!   object, so an unreferenced accessor or texture is carried through as opaque JSON and interpreted
//!   by nothing — its own projection says so and omits these six deliberately.
//! * `@gltf-transform/core` models a document, but folds `images` + `samplers` + `textures` into a
//!   single `Texture` and exposes no `listBufferViews`. It cannot distinguish `create-image` from
//!   `create-texture`, and cannot see a bufferView at all.
//!
//! `gltf` 1.4 exposes `accessors()`, `buffers()`, `views()`, `images()`, `samplers()` and `textures()`
//! as separate typed iterators, matching the specification's own structure — so each of the twelve
//! kinds lands on exactly one observable list.
//!
//! Nothing here applies a mutation or predicts what one should produce: it MARSHALS and READS. The
//! expected state is the `after` half of a committed fixture, written by a DIFFERENT third-party
//! library (`@gltf-transform/core`), and this reader judges both sides.
//!
//! usage: reader project <path.gltf> | reader compare <expected.gltf> <actual.gltf>

use std::env;
use std::fmt::Write as _;
use std::process::exit;

fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out
}

fn opt_str(value: Option<&str>) -> String {
    match value {
        Some(v) => format!("\"{}\"", escape(v)),
        None => "null".to_string(),
    }
}

/// 📄️ The six resource arrays, each entry carrying the fields a create/delete of it changes.
fn project(path: &str) -> Result<String, String> {
    // 🚪️`Gltf::open` parses the DOCUMENT only. `gltf::import` would additionally decode every referenced
    // buffer and image file, so a fixture carrying a deliberately tiny or malformed image payload would
    // fail to read at all — and the resource ARRAYS, which is all this projection reports, are fully
    // determined by the document. Reading pixel data to count images would be work done to lose
    // information.
    let document = gltf::Gltf::open(path).map_err(|error| error.to_string())?.document;
    let mut out = String::from("{");

    let _ = write!(out, "\"accessors\":[");
    for (index, accessor) in document.accessors().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"name\":{},\"count\":{},\"dimensions\":\"{:?}\",\"dataType\":\"{:?}\",\"normalized\":{},\"hasView\":{}}}",
            opt_str(accessor.name()),
            accessor.count(),
            accessor.dimensions(),
            accessor.data_type(),
            accessor.normalized(),
            accessor.view().is_some()
        );
    }

    let _ = write!(out, "],\"buffers\":[");
    for (index, buffer) in document.buffers().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(out, "{{\"name\":{},\"length\":{}}}", opt_str(buffer.name()), buffer.length());
    }

    let _ = write!(out, "],\"bufferViews\":[");
    for (index, view) in document.views().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"name\":{},\"offset\":{},\"length\":{},\"stride\":{},\"buffer\":{}}}",
            opt_str(view.name()),
            view.offset(),
            view.length(),
            view.stride().map(|s| s.to_string()).unwrap_or_else(|| "null".to_string()),
            view.buffer().index()
        );
    }

    let _ = write!(out, "],\"images\":[");
    for (index, image) in document.images().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let source = match image.source() {
            gltf::image::Source::View { mime_type, .. } => format!("{{\"kind\":\"view\",\"mimeType\":\"{}\"}}", escape(mime_type)),
            gltf::image::Source::Uri { mime_type, .. } => format!("{{\"kind\":\"uri\",\"mimeType\":{}}}", opt_str(mime_type)),
        };
        let _ = write!(out, "{{\"name\":{},\"source\":{}}}", opt_str(image.name()), source);
    }

    let _ = write!(out, "],\"samplers\":[");
    for (index, sampler) in document.samplers().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"name\":{},\"magFilter\":\"{:?}\",\"minFilter\":\"{:?}\",\"wrapS\":\"{:?}\",\"wrapT\":\"{:?}\"}}",
            opt_str(sampler.name()),
            sampler.mag_filter(),
            sampler.min_filter(),
            sampler.wrap_s(),
            sampler.wrap_t()
        );
    }

    let _ = write!(out, "],\"textures\":[");
    for (index, texture) in document.textures().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"name\":{},\"image\":{},\"sampler\":{}}}",
            opt_str(texture.name()),
            texture.source().index(),
            texture.sampler().index().map(|i| i.to_string()).unwrap_or_else(|| "null".to_string())
        );
    }

    let _ = write!(
        out,
        "],\"counts\":{{\"accessors\":{},\"buffers\":{},\"bufferViews\":{},\"images\":{},\"samplers\":{},\"textures\":{}}}}}",
        document.accessors().count(),
        document.buffers().count(),
        document.views().count(),
        document.images().count(),
        document.samplers().count(),
        document.textures().count()
    );
    Ok(out)
}

fn report(probe: &str, status: &str, measurements: &str, diagnostic: Option<&str>) -> String {
    let diagnostics = match diagnostic {
        Some(message) => format!(",\"diagnostics\":[{{\"severity\":\"error\",\"message\":\"{}\"}}]", escape(message)),
        None => String::new(),
    };
    format!(
        "{{\"schema\":\"semio.repository-test.probe-report/v2\",\"probe\":\"{probe}\",\"probeVersion\":\"gltf@1.4.1\",\"engine\":{{\"family\":\"gltf-rs\",\"implementation\":\"gltf 1.4 document resource arrays\",\"version\":\"1.4.1\"}},\"status\":\"{status}\",\"durationMs\":0,\"measurements\":{measurements}{diagnostics}}}"
    )
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("project") => {
            let path = args.get(2).expect("usage: reader project <path.gltf>");
            match project(path) {
                Ok(json) => println!("{}", report("gltf-resource-project", "ok", &json, None)),
                Err(error) => {
                    println!("{}", report("gltf-resource-project", "failed", "{}", Some(&error)));
                    exit(1);
                }
            }
        }
        Some("compare") => {
            let expected = args.get(2).expect("usage: reader compare <expected> <actual>");
            let actual = args.get(3).expect("usage: reader compare <expected> <actual>");
            match (project(expected), project(actual)) {
                (Ok(left), Ok(right)) => {
                    let equal = left == right;
                    let measurements = format!("{{\"equal\":{equal},\"expected\":{left},\"actual\":{right}}}");
                    println!("{}", report("gltf-resource-compare", "ok", &measurements, None));
                }
                (Err(error), _) | (_, Err(error)) => {
                    println!("{}", report("gltf-resource-compare", "failed", "{}", Some(&error)));
                    exit(1);
                }
            }
        }
        _ => {
            eprintln!("usage: reader project <path.gltf> | reader compare <expected.gltf> <actual.gltf>");
            exit(2);
        }
    }
}
