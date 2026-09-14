//! 📤️ The `Effect::RequestFileOpen` import law, driven by the language-agnostic fixture
//! `🧫️fixtures/📤️file-open-import/🔣️.json` that `🎠️kernel/🟦️.ts`'s TypeScript twin drives too.
//!
//! The defects this pins, all measured on 6118 for ticket 26/09/09/PROCEDURAL-3D-END-TO-END:
//!   1. the wgpu door dropped `request-file-open` outright (`unmapped effect … dropped`), so
//!      `Import Document…` reached the guest, produced its effect, and stopped there;
//!   2. the two renderers disagreed about what one import invocation carries — React sent
//!      `{payload, name, chunk, chunkCount}` per chunk, the wgpu shell `{json, payload}` once;
//!   3. an unchunked import asks the fixed guest heap for one contiguous block the size of the whole
//!      file, several times its own per-request ceiling;
//!   4. `chunk`/`chunkCount` are `u32` in the guest, and `FromValue`'s unsigned arm refuses a float —
//!      so the envelope's integers may never widen on a lossy hop.

use super::*;

#[derive(serde::Deserialize)]
struct PayloadRepeat {
    character: String,
    count: usize,
}

#[derive(serde::Deserialize)]
struct ChunkCase {
    id: String,
    #[serde(default)]
    payload: Option<String>,
    #[serde(rename = "payloadRepeat", default)]
    payload_repeat: Option<PayloadRepeat>,
    #[serde(default)]
    chunks: Option<Vec<String>>,
    #[serde(rename = "chunkLengths", default)]
    chunk_lengths: Option<Vec<usize>>,
    #[serde(rename = "chunkByteLengths", default)]
    chunk_byte_lengths: Option<Vec<usize>>,
}

impl ChunkCase {
    fn payload(&self) -> String {
        match (&self.payload, &self.payload_repeat) {
            (Some(text), _) => text.clone(),
            (None, Some(repeat)) => repeat.character.repeat(repeat.count),
            (None, None) => panic!("{} states neither `payload` nor `payloadRepeat`", self.id),
        }
    }
}

#[derive(serde::Deserialize)]
struct FanOut {
    index: usize,
    total: usize,
}

#[derive(serde::Deserialize)]
struct ArgumentChunk {
    payload: String,
    chunk: usize,
    #[serde(rename = "chunkCount")]
    chunk_count: usize,
}

#[derive(serde::Deserialize)]
struct ArgumentCase {
    id: String,
    name: String,
    chunk: ArgumentChunk,
    #[serde(rename = "fanOut")]
    fan_out: Option<FanOut>,
    arguments: serde_json::Map<String, serde_json::Value>,
}

#[derive(serde::Deserialize)]
struct FileOpenImportFixture {
    #[serde(rename = "importChunkBytes")]
    import_chunk_bytes: usize,
    #[serde(rename = "chunkCases")]
    chunk_cases: Vec<ChunkCase>,
    #[serde(rename = "argumentCases")]
    argument_cases: Vec<ArgumentCase>,
}

fn fixture() -> FileOpenImportFixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/📤️file-open-import/🔣️.json")).expect("file-open-import fixture JSON")
}

/// 📏️ The chunk extent is derived from the guest's own contiguous-request ceiling, and the fixture
/// records the same number both twins compute.
#[test]
fn the_chunk_extent_is_half_the_guest_contiguous_ceiling() {
    assert_eq!(IMPORT_CHUNK_BYTES, semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2);
    assert_eq!(fixture().import_chunk_bytes, IMPORT_CHUNK_BYTES);
}

/// 📥️ Every fixture payload slices into exactly the chunks it declares, positioned as its own run.
#[test]
fn every_fixture_payload_slices_into_its_declared_chunks() {
    let fixture = fixture();
    assert!(fixture.chunk_cases.len() >= 6, "fixture rows: {}", fixture.chunk_cases.len());
    for case in &fixture.chunk_cases {
        let payload = case.payload();
        let chunks = import_payload_chunks(&payload);
        if let Some(expected) = &case.chunks {
            assert_eq!(chunks.iter().map(|chunk| chunk.payload.clone()).collect::<Vec<_>>(), *expected, "{}", case.id);
        }
        if let Some(expected) = &case.chunk_lengths {
            assert_eq!(chunks.iter().map(|chunk| chunk.payload.chars().count()).collect::<Vec<_>>(), *expected, "{} character lengths", case.id);
        }
        if let Some(expected) = &case.chunk_byte_lengths {
            assert_eq!(chunks.iter().map(|chunk| chunk.payload.len()).collect::<Vec<_>>(), *expected, "{} byte lengths", case.id);
        }
        for (position, chunk) in chunks.iter().enumerate() {
            assert_eq!(chunk.chunk, position, "{} names its position", case.id);
            assert_eq!(chunk.chunk_count, chunks.len(), "{} names its run length", case.id);
        }
    }
}

/// 🧊️ No chunk may exceed the extent the guest can serve as one contiguous block, and reassembling
/// the run must give the payload back byte for byte — the whole reason the slicing exists.
#[test]
fn no_chunk_exceeds_the_extent_and_the_run_reassembles_exactly() {
    for case in fixture().chunk_cases {
        let payload = case.payload();
        let chunks = import_payload_chunks(&payload);
        assert!(!chunks.is_empty(), "{} must produce at least one chunk", case.id);
        for chunk in &chunks {
            assert!(chunk.payload.len() <= IMPORT_CHUNK_BYTES, "{} chunk {} is {} B", case.id, chunk.chunk, chunk.payload.len());
        }
        assert_eq!(chunks.iter().map(|chunk| chunk.payload.as_str()).collect::<String>(), payload, "{} must reassemble", case.id);
    }
}

/// 🔬️ Third-party oracle: `std`'s own UTF-8 boundary discipline. Every chunk boundary of every
/// fixture payload must be a character boundary of the ORIGINAL string — a slice counted in UTF-16
/// units would land inside a code point on the multibyte rows.
#[test]
fn every_chunk_boundary_is_a_utf8_character_boundary_of_the_payload() {
    for case in fixture().chunk_cases {
        let payload = case.payload();
        let mut cut = 0usize;
        for chunk in import_payload_chunks(&payload) {
            assert!(payload.is_char_boundary(cut), "{} cut {cut} is not a character boundary", case.id);
            assert_eq!(&payload[cut..cut + chunk.payload.len()], chunk.payload, "{} chunk {} is not the payload's own slice", case.id, chunk.chunk);
            cut += chunk.payload.len();
        }
        assert_eq!(cut, payload.len(), "{} must consume the whole payload", case.id);
    }
}

/// 📥️ One chunk's dispatch arguments are exactly the fixture's, with the fan-out present only for a
/// multi-file pick.
#[test]
fn every_chunk_dispatches_its_declared_arguments() {
    let fixture = fixture();
    assert!(fixture.argument_cases.len() >= 3, "fixture rows: {}", fixture.argument_cases.len());
    for case in &fixture.argument_cases {
        let chunk = ImportChunk { payload: case.chunk.payload.clone(), chunk: case.chunk.chunk, chunk_count: case.chunk.chunk_count };
        let arguments = import_chunk_arguments(&case.name, &chunk, case.fan_out.as_ref().map(|fan_out| (fan_out.index, fan_out.total)));
        let DslValue::Object(entries) = &arguments else { panic!("{} must build an object", case.id) };
        assert_eq!(entries.len(), case.arguments.len(), "{} argument count", case.id);
        for (key, expected) in &case.arguments {
            let actual = entries.iter().find(|(name, _)| name == key).map(|(_, value)| value).unwrap_or_else(|| panic!("{} is missing {key}", case.id));
            match expected {
                serde_json::Value::String(text) => assert_eq!(actual, &DslValue::String(text.clone()), "{} {key}", case.id),
                serde_json::Value::Number(number) => {
                    let DslValue::Number(actual_number) = actual else { panic!("{} {key} must be a number", case.id) };
                    assert!(actual_number.is_integer(), "{} {key} widened onto a float, which the guest's `u32` decode refuses", case.id);
                    assert_eq!(actual_number.as_u64(), number.as_u64(), "{} {key}", case.id);
                }
                other => panic!("{} {key} has an unsupported fixture shape {other:?}", case.id),
            }
        }
        assert_eq!(case.fan_out.is_some(), case.arguments.contains_key(IMPORT_ARGUMENT_INDEX), "{} fan-out presence", case.id);
    }
}

/// 🚨️ The envelope's positions are exact unsigned integers. A float here is the shape that took every
/// wgpu dispatch down once the view state first carried a `u64`.
#[test]
fn the_chunk_envelope_never_widens_its_integers_onto_floats() {
    let chunk = ImportChunk { payload: "x".into(), chunk: 2, chunk_count: 5 };
    let DslValue::Object(entries) = import_chunk_arguments("a.stl", &chunk, Some((1, 3))) else { panic!("object") };
    for key in [IMPORT_ARGUMENT_CHUNK, IMPORT_ARGUMENT_CHUNK_COUNT, IMPORT_ARGUMENT_INDEX, IMPORT_ARGUMENT_TOTAL] {
        let (_, value) = entries.iter().find(|(name, _)| name == key).unwrap_or_else(|| panic!("missing {key}"));
        let DslValue::Number(number) = value else { panic!("{key} must be a number") };
        assert!(number.is_integer(), "{key} must stay an exact integer");
        assert_eq!(<u32 as dsl::FromValue>::from_value(value.clone()).unwrap_or_else(|error| panic!("{key} must decode as the guest's own u32: {error}")), match key {
            IMPORT_ARGUMENT_CHUNK => 2,
            IMPORT_ARGUMENT_CHUNK_COUNT => 5,
            IMPORT_ARGUMENT_INDEX => 1,
            _ => 3,
        });
    }
}

/// 📦️ The effect really carries the envelope the shells read — `read_as` is optional, `multiple` is
/// not, and `import_action` is the verb a shell re-dispatches once per chunk.
#[test]
fn the_effect_envelope_names_the_verb_a_shell_redispatches() {
    let effect = Effect::RequestFileOpen { req: RequestId(131), accept: ".stl".into(), read_as: Some("dataUrl".into()), import_action: "importDocument".into(), multiple: false };
    let Effect::RequestFileOpen { req, accept, read_as, import_action, multiple } = &effect else { panic!("variant") };
    assert_eq!(req.0, 131);
    assert_eq!(accept, ".stl");
    assert_eq!(read_as.as_deref(), Some("dataUrl"));
    assert_eq!(import_action, "importDocument");
    assert!(!multiple);
}

/// 🌉️ The friendly JSON each renderer door produces is exactly what this crate parses back into an
/// `Effect` — the seam the wgpu bridge crosses as `InvocationResult.requested_effects` text.
#[test]
fn every_fixture_friendly_effect_parses_back_into_the_kernel_effect() {
    #[derive(serde::Deserialize)]
    struct WireCase {
        id: String,
        friendly: serde_json::Value,
    }
    #[derive(serde::Deserialize)]
    struct WireFixture {
        #[serde(rename = "wireCases")]
        wire_cases: Vec<WireCase>,
    }
    let fixture: WireFixture = serde_json::from_str(include_str!("../../🧫️fixtures/📤️file-open-import/🔣️.json")).expect("fixture JSON");
    assert!(fixture.wire_cases.len() >= 4, "fixture rows: {}", fixture.wire_cases.len());
    for case in fixture.wire_cases {
        let effect: Effect = serde_json::from_value(case.friendly.clone()).unwrap_or_else(|error| panic!("{} must parse: {error}", case.id));
        let Effect::RequestFileOpen { accept, import_action, .. } = &effect else { panic!("{} must be a RequestFileOpen", case.id) };
        let expected = case.friendly.get("requestFileOpen").expect("friendly shape");
        assert_eq!(accept, expected.get("accept").and_then(serde_json::Value::as_str).unwrap_or_default(), "{}", case.id);
        assert_eq!(import_action, expected.get("importAction").and_then(serde_json::Value::as_str).unwrap_or_default(), "{}", case.id);
    }
}
