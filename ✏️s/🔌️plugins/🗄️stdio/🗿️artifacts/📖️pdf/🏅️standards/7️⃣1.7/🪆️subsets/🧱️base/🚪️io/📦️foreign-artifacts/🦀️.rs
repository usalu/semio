//! 📦️ Native PDF foreign artifacts preserve encoded custody behind first-party typed references.
use crate::standards::v1_7::subsets::base::io::error::{PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::schema::snapshot::*;
use semio_framework_artifact_reference::{ArtifactRef,ArtifactDialect};
use semio_framework_value::ToValue;

#[path = "📖️dct-view.rs"]
pub mod dct_view;

#[path = "🔎️dct-lookup.rs"]
pub mod dct_lookup;

#[path = "🗃️install-pack.rs"]
pub mod install_pack;

const DCT_KIND: &str = "s.stdio.jpeg";
pub const SAMPLED_IMAGE_KIND: &str = "s.stdio.samples";
/// 📐️ The largest packed sample body (§8.9.3) an 8-bit image keeps inline in the typed snapshot; a larger one is a sampled artifact.
pub const INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES: usize = 1 << 18;
const RAW_KIND: &str = "s.stdio.binary";
const RAW_STANDARD: &str = "raw";
const RAW_SUBSET: &str = "*";

/// 🎞️ Hashes an admitted large sampled image XObject over its borrowed native bytes.
pub struct PdfSampledResourceIdentity {
    hash: semio_framework_hash::Sha256,
    cursor: usize,
    filter_wire: [u8; 160],
    filter_len: usize,
}

impl PdfSampledResourceIdentity {
    /// 📐️ Bounds the four independently owned reference strings.
    pub const REFERENCE_BYTES: usize = artifact_reference_bytes(SAMPLED_IMAGE_KIND);
    /// 📄️ Native bytes hashed per controlled work unit.
    pub const PAGE_BYTES: usize = 4_096;

    /// 🎛️ Binds a raw or Flate-filtered sampled image stream's compact canonical filter wire.
    pub fn new(filters: &[PdfStreamFilter]) -> Option<Self> {
        use std::io::Write;
        let mut filter_wire = [0u8; 160];
        let filter_len = match filters {
            [] => {
                filter_wire[..2].copy_from_slice(b"[]");
                2
            }
            [PdfStreamFilter::Flate { predictor: None }] => {
                let wire = br#"[{"kind":"flate","predictor":null}]"#;
                filter_wire[..wire.len()].copy_from_slice(wire);
                wire.len()
            }
            [PdfStreamFilter::Flate { predictor: Some(p) }] => {
                let mut cursor = std::io::Cursor::new(&mut filter_wire[..]);
                write!(
                    cursor,
                    r#"[{{"kind":"flate","predictor":{{"predictor":{},"colors":{},"bitsPerComponent":{},"columns":{}}}}}]"#,
                    p.predictor, p.colors, p.bits_per_component, p.columns,
                ).ok()?;
                cursor.position() as usize
            }
            _ => return None,
        };
        let mut hash = semio_framework_hash::Sha256::new();
        hash.update(SAMPLED_IMAGE_KIND.as_bytes());
        Some(Self { hash, cursor: 0, filter_wire, filter_len })
    }

    /// 🗜️ Binds a large sampled image whose retained body is still its encoded file bytes: the identity hashes the declared chain ahead of the body and no typed filter after it.
    pub fn encoded(dict: &[PdfDictEntry]) -> Option<Self> {
        if !encoded_sampled_image(dict) { return None; }
        let mut filter_wire = [0u8; 160];
        filter_wire[..2].copy_from_slice(b"[]");
        let mut hash = semio_framework_hash::Sha256::new();
        hash.update(SAMPLED_IMAGE_KIND.as_bytes());
        hash.update(encoded_chain(dict).as_bytes());
        Some(Self { hash, cursor: 0, filter_wire, filter_len: 2 })
    }

    /// ▶️ Hashes at most `work` pages of native stream bytes and reports page work consumed.
    pub fn advance(&mut self, data: &(impl semio_framework_value::ByteReadSource + ?Sized), work: usize) -> usize {
        let limit = work.saturating_mul(Self::PAGE_BYTES);
        let end = self.cursor.saturating_add(limit).min(data.byte_len());
        let bytes = end - self.cursor;
        let mut page = [0u8; Self::PAGE_BYTES];
        let mut remaining = bytes;
        while remaining != 0 {
            let count = remaining.min(Self::PAGE_BYTES);
            if data.read_bytes(self.cursor, &mut page[..count]) != count { break; }
            self.append(&page[..count]);
            remaining -= count;
        }
        bytes.div_ceil(Self::PAGE_BYTES)
    }

    /// 🧮️ Hashes the next verified native bytes without requiring sample materialization.
    pub fn append(&mut self, data: &[u8]) {
        self.hash.update(data);
        self.cursor += data.len();
    }

    /// ✅️ Indicates that the borrowed native sample stream was read completely.
    pub fn complete(&self, length: usize) -> bool { self.cursor == length }

    /// 🧾️ Materializes the bounded reference after all native sample bytes have been hashed.
    pub fn reference(&self) -> ArtifactRef {
        artifact_reference(SAMPLED_IMAGE_KIND, self.digest())
    }

    /// 🔑️ Computes the fixed-size native identity after hashing admitted input.
    pub fn digest(&self) -> [u8; 32] {
        let mut hash = self.hash.clone();
        hash.update(&self.filter_wire[..self.filter_len]);
        hash.finalize()
    }
}

/// 🔐️ Hashes an admitted one-filter JPEG over its borrowed native bytes.
pub struct PdfDctResourceIdentity {
    hash: semio_framework_hash::Sha256,
    cursor: usize,
    filter_wire: &'static [u8],
}

impl PdfDctResourceIdentity {
    /// 📐️ Bounds the four independently owned reference strings.
    pub const REFERENCE_BYTES: usize = "pdf::sha256:".len() + DCT_KIND.len() + 64
        + RAW_KIND.len() + RAW_STANDARD.len() + RAW_SUBSET.len();

    /// 🎛️ Binds the supported single DCT filter's compact canonical value.
    pub fn new(color_transform: Option<u32>) -> Option<Self> {
        let filter_wire: &[u8] = match color_transform {
            None => br#"[{"kind":"dct","colorTransform":null}]"#,
            Some(0) => br#"[{"kind":"dct","colorTransform":0}]"#,
            Some(1) => br#"[{"kind":"dct","colorTransform":1}]"#,
            _ => return None,
        };
        let mut hash = semio_framework_hash::Sha256::new();
        hash.update(DCT_KIND.as_bytes());
        Some(Self { hash, cursor: 0, filter_wire })
    }

    /// ▶️ Hashes at most the granted number of native stream bytes.
    pub fn advance(&mut self, data: &(impl semio_framework_value::ByteReadSource + ?Sized), work: usize) -> usize {
        let end = self.cursor.saturating_add(work).min(data.byte_len());
        let used = end - self.cursor;
        let mut page = [0u8; 4_096];
        let mut remaining = used;
        while remaining != 0 {
            let count = remaining.min(page.len());
            if data.read_bytes(self.cursor, &mut page[..count]) != count { break; }
            self.append(&page[..count]);
            remaining -= count;
        }
        used
    }

    /// 🔑️ Reads the exact authored DCT identity without allocating reference strings.
    pub fn parse_reference(reference: &ArtifactRef) -> Option<[u8; 32]> {
        if reference.dialect.artifact_kind != RAW_KIND || reference.dialect.standard != RAW_STANDARD
            || reference.dialect.subset != RAW_SUBSET
        {
            return None;
        }
        let hex = reference.artifact_id.strip_prefix("pdf:s.stdio.jpeg:sha256:")?.as_bytes();
        let hex: &[u8; 64] = hex.try_into().ok()?;
        let nibble = |byte| match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        };
        let mut digest = [0; 32];
        for (value, pair) in digest.iter_mut().zip(hex.chunks_exact(2)) {
            *value = nibble(pair[0])? << 4 | nibble(pair[1])?;
        }
        Some(digest)
    }

    /// 📍️ Gives the number of original bytes already hashed.
    pub fn position(&self) -> usize { self.cursor }

    /// 🧮️ Hashes the next verified native bytes without requiring a contiguous source.
    pub fn append(&mut self, data: &[u8]) {
        self.hash.update(data);
        self.cursor += data.len();
    }

    /// ✅️ Indicates that the borrowed encoded stream was read completely.
    pub fn complete(&self, length: usize) -> bool { self.cursor == length }

    /// 🧾️ Materializes the bounded reference after all encoded bytes have been hashed.
    pub fn reference(&self) -> ArtifactRef {
        Self::reference_for_digest(self.digest())
    }

    /// 🪪️ Materializes the native DCT reference from a completed external digest.
    pub fn reference_for_digest(digest: [u8; 32]) -> ArtifactRef {
        artifact_reference(DCT_KIND, digest)
    }

    /// 🔑️ Computes the fixed-size native identity after hashing admitted input.
    pub fn digest(&self) -> [u8; 32] {
        let mut hash = self.hash.clone();
        hash.update(self.filter_wire);
        hash.finalize()
    }
}

/// 📏️ Reports storage for the four native artifact reference strings.
pub const fn artifact_reference_bytes(kind: &str) -> usize {
    "pdf::sha256:".len() + kind.len() + 64 + RAW_KIND.len() + RAW_STANDARD.len() + RAW_SUBSET.len()
}

/// 🪪️ Materializes one PDF foreign artifact reference from a complete native digest.
pub fn artifact_reference(kind: &str, digest: [u8; 32]) -> ArtifactRef {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut artifact_id = String::with_capacity("pdf::sha256:".len() + kind.len() + 64);
    artifact_id.push_str("pdf:");
    artifact_id.push_str(kind);
    artifact_id.push_str(":sha256:");
    for byte in digest {
        artifact_id.push(char::from(HEX[usize::from(byte >> 4)]));
        artifact_id.push(char::from(HEX[usize::from(byte & 15)]));
    }
    ArtifactRef { artifact_id, dialect: ArtifactDialect { artifact_kind: RAW_KIND.into(), standard: RAW_STANDARD.into(), subset: RAW_SUBSET.into() } }
}

/// 🖼️ Qualifies an 8-bit gray or RGB image dictionary whose only reference is an indirect soft mask.
pub fn qualified_image_dictionary(dict: &[PdfDictEntry]) -> bool { qualified_image_entries(dict, false) }

/// 🧾️ The qualified image keys, plus its declared `Filter` and `DecodeParms` when the body is still encoded.
fn qualified_image_entries(dict: &[PdfDictEntry], encoded: bool) -> bool {
    use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_name};
    dict.len() <= if encoded { 11 } else { 9 } && dict.iter().enumerate().all(|(index, entry)| {
        !dict[..index].iter().any(|previous| previous.key == entry.key) && match entry.key.as_str() {
            "Type" => entry.value.as_name() == Some("XObject"),
            "Subtype" => entry.value.as_name() == Some("Image"),
            "BitsPerComponent" => entry.value == PdfObject::Int(8),
            "ImageMask" => entry.value.as_bool() == Some(false),
            "Interpolate" => entry.value.as_bool().is_some(),
            "Width" | "Height" | "ColorSpace" => true,
            "SMask" => entry.value.as_ref().is_some(),
            "Filter" | "DecodeParms" => encoded,
            _ => false,
        }
    }) && dict_name(dict, "Subtype") == Some("Image") && dict_get(dict, "BitsPerComponent").is_some()
}

/// 🖼️ Whether an import keeps this image XObject as its encoded file body: a qualified 8-bit DeviceGray or DeviceRGB image whose declared chain is exactly one Flate filter and whose samples would exceed the inline sample ceiling, so it is typed as a sampled artifact identified by its encoded bytes and chain and decoded only when a consumer resolves its pixels.
pub fn encoded_sampled_image(dict: &[PdfDictEntry]) -> bool {
    use crate::standards::v1_7::subsets::base::modules::lexer::{dict_get, dict_name};
    let flate = |value: &PdfObject| matches!(value.as_name(), Some("FlateDecode" | "Fl"));
    let single = match dict_get(dict, "Filter") { Some(PdfObject::Array(items)) => matches!(items.as_slice(), [item] if flate(item)), Some(value) => flate(value), None => false };
    let parameters = match dict_get(dict, "DecodeParms") {
        None | Some(PdfObject::Null | PdfObject::Dict(_)) => true,
        Some(PdfObject::Array(items)) => matches!(items.as_slice(), [PdfObject::Null | PdfObject::Dict(_)]),
        _ => false,
    };
    let components = match dict_name(dict, "ColorSpace") { Some("DeviceGray") => 1usize, Some("DeviceRGB") => 3, _ => return false };
    let dimension = |key| match dict_get(dict, key) { Some(PdfObject::Int(raw)) => usize::try_from(*raw).ok().filter(|size| *size != 0), _ => None };
    let (Some(width), Some(height)) = (dimension("Width"), dimension("Height")) else { return false; };
    single && parameters && qualified_image_entries(dict, true)
        && width.saturating_mul(height).saturating_mul(components) > INLINE_IMAGE_SAMPLES_MAXIMUM_BYTES
}

/// 🧬️ The canonical spelling of an encoded stream's declared chain (its `Filter` then its `DecodeParms`), hashed into its identity ahead of its body.
fn encoded_chain(dict: &[PdfDictEntry]) -> String {
    let spell = |key: &str| semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&dict.iter().find(|entry| entry.key == key).map(|entry| &entry.value).unwrap_or(&PdfObject::Null).to_value()));
    spell("Filter") + &spell("DecodeParms")
}

/// 🗜️ Whether a retained stream still carries the encoded body of its declared logical chain.
fn encoded_stream(dict: &[PdfDictEntry], filters: &[PdfStreamFilter]) -> bool {
    filters.is_empty() && crate::standards::v1_7::subsets::base::modules::filters::declares_logical_codec(dict)
}

/// 🔓️ Decodes a resolved encoded stream into the form its consumers read (decoded body, typed chain, dictionary without its filter declaration); every other value is returned as it is.
fn decoded_resource(value: PdfObject) -> PResult<PdfObject> {
    let PdfObject::Stream { dict, data, filters } = value else { return Ok(value); };
    if !encoded_stream(&dict, &filters) { return Ok(PdfObject::Stream { dict, data, filters }); }
    let (body, chain) = crate::standards::v1_7::subsets::base::modules::filters::decode_stream_admitted(&dict, &data.to_vec_owner())?;
    let dict = dict.into_iter().filter(|entry| crate::standards::v1_7::subsets::base::modules::lexer::retain_stream_entry(entry, true)).collect();
    Ok(PdfObject::Stream { dict, data: stream_bytes(&body), filters: chain })
}

/// 🛂️ Native admission and resolution are the only access to foreign encoded bodies.
pub trait PdfArtifactResourcePort {
    fn admit(&mut self,kind:&str,object:PdfObject)->PResult<ArtifactRef>;
    fn resolve(&self,reference:&ArtifactRef)->PResult<PdfObject>;
}

/// 📚️ Owns foreign native resources independently of every semantic role body. Graph streams are identified once per kind (one hash pass over the graph), so resolving many references costs one scan, not one scan per reference.
#[derive(Default)]
pub struct NativePdfArtifactResources<'a> {resources:Vec<(ArtifactRef,PdfObject)>,graph:&'a [PdfIndirectObject],identities:std::cell::RefCell<std::collections::HashMap<String,std::collections::HashMap<String,usize>>>}
fn identity(kind:&str,object:&PdfObject)->PResult<ArtifactRef> {
    let PdfObject::Stream {dict,data,filters}=object else{return Err(PdfEngineError::Unsupported("foreign PDF artifact requires a native stream".into()));};
    let mut hash=semio_framework_hash::Sha256::new();hash.update(kind.as_bytes());if encoded_stream(dict,filters){hash.update(encoded_chain(dict).as_bytes());}{let mut page=[0u8;4096];let mut at=0;while at<data.len(){let count=(data.len()-at).min(page.len());if semio_framework_value::ByteReadSource::read_bytes(data,at,&mut page[..count])!=count{return Err(PdfEngineError::Malformed("foreign PDF artifact lost a stream byte".into()))}hash.update(&page[..count]);at+=count;}}
    let value=semio_framework_pack_json::from_dsl_value(&filters.to_value());hash.update(semio_framework_pack_json::to_string(&value).as_bytes());
    if kind=="s.stdio.font.type1" {for name in ["Length1","Length2","Length3"]{let entry=dict.iter().find(|entry|entry.key==name).map(|entry|&entry.value).unwrap_or(&PdfObject::Null);hash.update(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&entry.to_value())).as_bytes());}}
    Ok(artifact_reference(kind, hash.finalize()))
}
impl<'a> NativePdfArtifactResources<'a> {
    /// 📚️ Admits the native graph's existing foreign resources without semantic decoding.
    pub fn into_objects(self)->Vec<PdfIndirectObject>{self.resources.into_iter().enumerate().map(|(index,(_,value))|PdfIndirectObject{id:ObjRef{num:index as u32+1,gen:0},value}).collect()}
    pub fn from_objects(objects:&'a [PdfIndirectObject])->Self {Self {resources:Vec::new(),graph:objects,identities:Default::default()}}

}
impl PdfArtifactResourcePort for NativePdfArtifactResources<'_> {
    fn admit(&mut self,kind:&str,object:PdfObject)->PResult<ArtifactRef> {let reference=identity(kind,&object)?;if !self.resources.iter().any(|(current,_)|current==&reference){self.resources.push((reference.clone(),object));}Ok(reference)}
    fn resolve(&self,reference:&ArtifactRef)->PResult<PdfObject> {
        if png_source::PngPlane::match_reference(reference).is_some() {
            let (role, source) = png_source::PngGraphSource::resolve(self.graph, reference)
                .map_err(|_| PdfEngineError::Malformed("retained PDF PNG source binding is incomplete".into()))?;
            return png_source::samples::lower_source_plane(role, source);
        }
        if let Some((_,object))=self.resources.iter().find(|(current,_)|current==reference){return Ok(object.clone());}
        let kind=reference.artifact_id.strip_prefix("pdf:").and_then(|id|id.split_once(":sha256:")).map(|(kind,_)|kind).ok_or_else(||PdfEngineError::Unsupported("foreign PDF artifact identity is invalid".into()))?;
        if !self.identities.borrow().contains_key(kind) {
            let mut index=std::collections::HashMap::new();
            for (position,object) in self.graph.iter().enumerate() {if matches!(object.value,PdfObject::Stream {..}){index.entry(identity(kind,&object.value)?.artifact_id).or_insert(position);}}
            self.identities.borrow_mut().insert(kind.to_string(),index);
        }
        let native=reference.dialect.artifact_kind==RAW_KIND&&reference.dialect.standard==RAW_STANDARD&&reference.dialect.subset==RAW_SUBSET;
        if let Some(position)=self.identities.borrow().get(kind).filter(|_|native).and_then(|index|index.get(&reference.artifact_id)).copied(){return decoded_resource(self.graph[position].value.clone());}
        Err(PdfEngineError::Unsupported(format!("foreign PDF artifact {} has not been admitted",reference.artifact_id)))
    }

}

/// 🎟️ Admits a detached native resource and retains its original representation in the native graph.
pub fn admit_pdf_artifact(snapshot:&mut PdfSnapshot,kind:&str,object:PdfObject)->PResult<ArtifactRef> {
    let reference=identity(kind,&object)?;let number=snapshot.objects.iter().map(|object|object.id.num).max().unwrap_or(0).checked_add(1).ok_or_else(||PdfEngineError::Unsupported("PDF native resource identity exhausted".into()))?;
    snapshot.objects.push(PdfIndirectObject {id:ObjRef {num:number,gen:0},value:object});Ok(reference)
}

/// 🛂️ Refuses a native resource reference outside the requested physical artifact kind.
pub fn require_pdf_artifact_kind(reference:&ArtifactRef,kind:&str)->PResult<()> {if reference.dialect.artifact_kind!="s.stdio.binary"||reference.dialect.standard!="raw"||reference.dialect.subset!="*"||!reference.artifact_id.starts_with(&format!("pdf:{kind}:sha256:")){return Err(PdfEngineError::Unsupported("foreign PDF artifact kind does not match its semantic resource".into()));}Ok(())}
/// 🔤️ Binds a semantic font variant to its actual independent native resource kind.
pub fn require_font_artifact_kind(program:&PdfFontProgram)->PResult<()> {let kind=match program {PdfFontProgram::Type1 {..}=>"type1",PdfFontProgram::TrueType {..}=>"truetype",PdfFontProgram::Cff {..}=>"cff",PdfFontProgram::CidCff {..}=>"cid-cff",PdfFontProgram::OpenType {..}=>"opentype"};require_pdf_artifact_kind(program.reference(),&format!("s.stdio.font.{kind}"))}

pub mod png_source {
    //! 🧩️ Binds an exact encoded PNG to durable PDF graph parts and two sampled image roles.

    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfDictEntry, PdfIndirectObject, PdfObject};
    use semio_framework_artifact_reference::ArtifactRef;
    use semio_framework_hash::Sha256;
    use semio_framework_value::paged::ByteReadSource;

    const TYPE: &str = "SemioPngSourceV1";
    pub const PNG_GRAPH_CHUNK_BYTES: usize = 32 * 1024;
    pub const PNG_GRAPH_MAXIMUM_BYTES: usize = 512 * 1024;
    pub const PNG_GRAPH_MAXIMUM_PIXELS: usize = 262_144;
    const PARTS: usize = PNG_GRAPH_MAXIMUM_BYTES / PNG_GRAPH_CHUNK_BYTES;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PngSourceError { Reference, Extent, Missing, Duplicate, Part, Digest }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PngPlane { Rgb, Alpha }

    impl PngPlane {
        fn kind(self) -> &'static str {
            match self { Self::Rgb => "s.stdio.png.rgb", Self::Alpha => "s.stdio.png.alpha" }
        }

        /// 🏷️ Addresses one plane by the complete original PNG digest.
        pub fn reference(self, digest: [u8; 32]) -> ArtifactRef {
            super::artifact_reference(self.kind(), digest)
        }

        pub fn match_reference(reference: &ArtifactRef) -> Option<Self> { Self::from_reference(reference).ok().map(|(role, _)| role) }

        pub fn channels(self) -> usize { match self { Self::Rgb => 3, Self::Alpha => 1 } }

        fn from_reference(reference: &ArtifactRef) -> Result<(Self, [u8; 32]), PngSourceError> {
            let id = reference.artifact_id.strip_prefix("pdf:").ok_or(PngSourceError::Reference)?;
            let (kind, hex) = id.split_once(":sha256:").ok_or(PngSourceError::Reference)?;
            let plane = [Self::Rgb, Self::Alpha].into_iter().find(|plane| plane.kind() == kind).ok_or(PngSourceError::Reference)?;
            let mut digest = [0; 32];
            if hex.len() != 64 || !hex.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) { return Err(PngSourceError::Reference); }
            for (index, value) in digest.iter_mut().enumerate() {
                *value = u8::from_str_radix(&hex[2 * index..2 * index + 2], 16).map_err(|_| PngSourceError::Reference)?;
            }
            if reference.dialect.artifact_kind != "s.stdio.binary" || reference.dialect.standard != "raw" || reference.dialect.subset != "*" {
                return Err(PngSourceError::Reference);
            }
            Ok((plane, digest))
        }
    }

    /// 🎚️ Binds an expected exact byte extent to the authenticated original digest.
    #[derive(Clone, Copy)]
    pub struct PngGraphExtent {
        pub digest: [u8; 32],
        pub length: usize,
    }

    impl PngGraphExtent {
        pub fn chunks(self) -> Result<usize, PngSourceError> {
            if !(1..=PNG_GRAPH_MAXIMUM_BYTES).contains(&self.length) { return Err(PngSourceError::Extent); }
            Ok(self.length.div_ceil(PNG_GRAPH_CHUNK_BYTES))
        }

        pub fn chunk_len(self, ordinal: usize) -> Result<usize, PngSourceError> {
            if ordinal >= self.chunks()? { return Err(PngSourceError::Part); }
            Ok((self.length - ordinal * PNG_GRAPH_CHUNK_BYTES).min(PNG_GRAPH_CHUNK_BYTES))
        }

        /// 📥️ Moves one encoded window into an ordinary retained native graph dictionary.
        pub fn part(self, ordinal: usize, octets: String) -> Result<PdfObject, PngSourceError> {
            if octets.len() != self.chunk_len(ordinal)? * 2 { return Err(PngSourceError::Part); }
            Ok(PdfObject::Dict(vec![
                PdfDictEntry::new("Type", PdfObject::Name(TYPE.into())),
                PdfDictEntry::new("Digest", PdfObject::Str(self.digest.to_vec())),
                PdfDictEntry::new("Extent", PdfObject::Int(self.length as i64)),
                PdfDictEntry::new("Ordinal", PdfObject::Int(ordinal as i64)),
                PdfDictEntry::new("Octets", PdfObject::Text(octets)),
            ]))
        }
    }

    /// 🔒️ Identifies the private source parts that the physical writer replaces with lowered XObjects.
    pub fn is_png_graph_source(value: &PdfObject) -> bool {
        matches!(value, PdfObject::Dict(entries) if entries.first().is_some_and(|entry|
            entry.key == "Type" && entry.value.as_name() == Some(TYPE)))
    }

    /// 📃️ Borrows the exact ordinal and hexadecimal octets of one matching private native declaration.
    pub fn matching_png_part<'a>(value: &'a PdfObject, digest: &[u8; 32])
        -> Result<Option<(usize, usize, &'a [u8])>, PngSourceError> {
        let PdfObject::Dict(dict) = value else { return Ok(None); };
        if !is_png_graph_source(value) || !dict.get(1).is_some_and(|entry|
            entry.key == "Digest" && matches!(&entry.value, PdfObject::Str(value) if value == digest)) {
            return Ok(None);
        }
        let [_, _, extent, order, data] = dict.as_slice() else { return Err(PngSourceError::Part); };
        if extent.key != "Extent" || order.key != "Ordinal" || data.key != "Octets" { return Err(PngSourceError::Part); }
        let (PdfObject::Int(total), PdfObject::Int(ordinal), PdfObject::Text(data)) = (&extent.value, &order.value, &data.value)
            else { return Err(PngSourceError::Part); };
        let total = usize::try_from(*total).map_err(|_| PngSourceError::Extent)?;
        let ordinal = usize::try_from(*ordinal).map_err(|_| PngSourceError::Part)?;
        if data.len() != (PngGraphExtent { digest: *digest, length: total }).chunk_len(ordinal)? * 2 { return Err(PngSourceError::Part); }
        Ok(Some((total, ordinal, data.as_bytes())))
    }

    /// ✍️ Appends the canonical ASCII image of an original byte window to its funded backing.
    pub fn append_hex(output: &mut String, bytes: &[u8]) {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        for &byte in bytes {
            output.push(char::from(HEX[(byte >> 4) as usize]));
            output.push(char::from(HEX[(byte & 15) as usize]));
        }
    }

    pub fn hex_octet(pair: &[u8]) -> Option<u8> {
        fn digit(byte: u8) -> Option<u8> {
            match byte {
                b'0'..=b'9' => Some(byte - b'0'),
                b'a'..=b'f' => Some(byte - b'a' + 10),
                _ => None,
            }
        }
        let [high, low] = pair else { return None; };
        Some((digit(*high)? << 4) | digit(*low)?)
    }

    /// 📖 Validates and borrows each exact native page without joining its source allocation.
    pub struct PngGraphSource<'a> {
        chunks: [Option<&'a [u8]>; PARTS],
        length: usize,
    }

    impl<'a> PngGraphSource<'a> {
        pub fn resolve(objects: &'a [PdfIndirectObject], reference: &ArtifactRef) -> Result<(PngPlane, Self), PngSourceError> {
            let (plane, digest) = PngPlane::from_reference(reference)?;
            let mut length = None;
            let mut chunks: [Option<&[u8]>; PARTS] = [None; PARTS];
            for item in objects {
                let Some((total, ordinal, data)) = matching_png_part(&item.value, &digest)? else { continue; };
                if length.is_some_and(|old| old != total) { return Err(PngSourceError::Part); }
                if chunks[ordinal].replace(data).is_some() { return Err(PngSourceError::Duplicate); }
                length = Some(total);
            }
            let length = length.ok_or(PngSourceError::Missing)?;
            let mut hash = Sha256::new();
            let mut scratch = [0; 4096];
            for chunk in &chunks[..length.div_ceil(PNG_GRAPH_CHUNK_BYTES)] {
                for window in chunk.ok_or(PngSourceError::Missing)?.chunks(scratch.len() * 2) {
                    for (out, pair) in scratch.iter_mut().zip(window.chunks_exact(2)) {
                        *out = hex_octet(pair).ok_or(PngSourceError::Part)?;
                    }
                    hash.update(&scratch[..window.len() / 2]);
                }
            }
            if hash.finalize() != digest { return Err(PngSourceError::Digest); }
            Ok((plane, Self { chunks, length }))
        }
    }

    /// 🧾 Excludes only native owners whose exact digest is still bound to a typed image.
    pub fn referenced_source_object(
        value: &PdfObject, images: &[crate::standards::v1_7::subsets::base::schema::snapshot::PdfImage],
    ) -> bool {
        use crate::standards::v1_7::subsets::base::schema::snapshot::PdfImageBody;
        if !is_png_graph_source(value) { return false; }
        let Some(PdfObject::Str(digest)) = value.dict_get("Digest") else { return false; };
        images.iter().any(|image| {
            let PdfImageBody::Artifact { reference } = &image.body else { return false; };
            PngPlane::from_reference(reference).is_ok_and(|(_, expected)| digest == &expected)
        })
    }

    impl ByteReadSource for PngGraphSource<'_> {
        fn byte_len(&self) -> usize { self.length }
        fn byte_at(&self, index: usize) -> Option<u8> {
            let start = (index % PNG_GRAPH_CHUNK_BYTES) * 2;
            hex_octet(self.chunks.get(index / PNG_GRAPH_CHUNK_BYTES)?.as_ref()?.get(start..start + 2)?)
        }

        fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize {
            let available = self.length.saturating_sub(offset).min(output.len());
            let mut written = 0;
            while written < available {
                let offset = offset + written;
                let chunk = self.chunks[offset / PNG_GRAPH_CHUNK_BYTES].expect("validated PNG graph source part");
                let start = (offset % PNG_GRAPH_CHUNK_BYTES) * 2;
                let count = (available - written).min((chunk.len() - start) / 2);
                for (out, pair) in output[written..written + count].iter_mut().zip(chunk[start..start + count * 2].chunks_exact(2)) {
                    *out = hex_octet(pair).expect("validated original PDF PNG hexadecimal pair");
                }
                written += count;
            }
            written
        }
    }

    pub(super) mod samples {
        //! 🧪️ Opens the bounded native PNG source as one straight color or grayscale alpha plane.

        use super::{PngGraphSource, PngPlane, PNG_GRAPH_MAXIMUM_PIXELS};
        use crate::standards::v1_7::subsets::base::{
            io::error::{PResult, PdfEngineError},
            schema::snapshot::{PdfDictEntry, PdfObject, PdfStreamFilter},
        };
        use semio_framework_pixels::PngScanlineDecoder;
        use semio_framework_value::paged::ByteReadSource;

        pub(crate) fn lower_source_plane(plane: PngPlane, source: PngGraphSource<'_>) -> PResult<PdfObject> {
            let malformed = || PdfEngineError::Malformed("PDF PNG source is incomplete or outside its admitted raster extent".into());
            let mut data = Vec::new();
            data.try_reserve_exact(source.byte_len()).map_err(|_| malformed())?;
            data.resize(source.byte_len(), 0);
            if source.read_bytes(0, &mut data) != data.len() { return Err(malformed()); }
            let geometry = data.get(16..24).ok_or_else(malformed)?;
            let width = u32::from_be_bytes(geometry[..4].try_into().unwrap());
            let height = u32::from_be_bytes(geometry[4..].try_into().unwrap());
            let count = (width as usize).checked_mul(height as usize).filter(|pixels| (1..=PNG_GRAPH_MAXIMUM_PIXELS).contains(pixels))
                .ok_or_else(malformed)? * plane.channels();
            let mut decoder = PngScanlineDecoder::from_vec(data).map_err(|_| malformed())?;
            if (decoder.width(), decoder.height()) != (width, height) { return Err(malformed()); }
            let mut samples = Vec::new();
            samples.try_reserve_exact(count).map_err(|_| malformed())?;
            while let Some(row) = decoder.next_row().map_err(|_| malformed())? {
                if row.len() != width as usize * 4 || samples.len() + width as usize * plane.channels() > count {
                    return Err(malformed());
                }
                for pixel in row.chunks_exact(4) {
                    match plane { PngPlane::Rgb => samples.extend_from_slice(&pixel[..3]), PngPlane::Alpha => samples.push(pixel[3]) }
                }
            }
            if samples.len() != count { return Err(malformed()); }
            let dict = vec![
                PdfDictEntry::new("Width", PdfObject::Int(i64::from(width))),
                PdfDictEntry::new("Height", PdfObject::Int(i64::from(height))),
                PdfDictEntry::new("SemioChannels", PdfObject::Int(plane.channels() as i64)),
            ];
            Ok(PdfObject::Stream { dict, data: crate::standards::v1_7::subsets::base::schema::snapshot::stream_bytes(&samples), filters: vec![PdfStreamFilter::Flate { predictor: None }] })
        }
    }
}
