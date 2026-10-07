//! 🚪️ IO stdio.pdf (1.7/🧾️vt) — reuses the 🧱️base subset's `binary`/`deflate` raw-codec DAG
//! leaves rather than duplicating them (same `PdfSnapshot` type, same catalog DAG edges).
//! Registration flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level
//! aggregator, and the `SubsetValidator` directly), not per-leaf `register()` — same pattern
//! established by `🗄️a/🚪️io` and `🧱️base/🚪️io` for this artifact. ISO 16612-2:2010 (PDF/VT-1/-2).
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
    use crate::standards::v1_7::subsets::base::io::PdfComposer as PdfAnyComposer;
    use crate::standards::v1_7::subsets::vt::io::check_vt_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::register_subset_validator,semio_framework_plugin::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::ComposeError,semio_framework_plugin::ComposeSource,semio_framework_plugin::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::SubsetValidator,semio_framework_plugin::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_VT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("vt") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("*") };
    const DIALECT_X: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("x") };
    const DEP_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };
    const DEP_DEFLATE: Dialect = Dialect { artifact_kind: "s.stdio.deflate", standard: StandardId("rfc1950"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct PdfVtComposerComposition;

    impl ArtifactComposition for PdfVtComposerComposition {
        type Snapshot = PdfSnapshot;
        const WRITES: Dialect = DIALECT_VT;

        /// 📚️ Reads `🖨️x` alongside `🧱️base`/self/deps -- VT is layered on X-4 (ISO 16612-2 is based
        /// on ISO 15930-7), matching the catalog DAG relationship the roster describes.
        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_X, DIALECT_VT, DEP_BINARY, DEP_DEFLATE]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = PdfAnyComposer::compose(sources)?;
            let checks = check_vt_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("PDF/VT-1/-2 conformance violated: {} hard issue(s) -- not stamping the vt dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    pub struct PdfVtValidator;

    impl SubsetValidator for PdfVtValidator {
        const DIALECT: Dialect = DIALECT_VT;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <PdfSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <PdfSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_vt_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.pdf.vt.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "PDF/VT SubsetValidator: payload did not decode as a PdfSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<PdfVtValidator>)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

#[path = "💾️binary/🦀️.rs"]
pub mod binary;

#[path = "📝️text/🦀️.rs"]
pub mod text;

pub mod derived_construction {
    use crate::standards::v1_7::subsets::base::schema::diff::PdfDiff;
    use crate::standards::v1_7::subsets::base::schema::mutations::{apply_pdf_mutation, InsertPage, PdfMutation, SetInfo};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfInfo, PdfObject, PdfOutputIntent, PdfPage, PdfSnapshot};
    use crate::standards::v1_7::subsets::vt::io::check_vt_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Seed
    /// 🌱️ Seeds a fresh snapshot with a real `/GTS_PDFX` OutputIntent (same shape `🖨️x` seeds) plus
    /// a minimal `/DPartRoot` → `/DParts` → one `/DPart` node carrying `/DPM`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn seeded_snapshot(output_condition: String) -> PdfSnapshot {
        let profile = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🖼️assets/🌈️icc/🌈️sRGB2014.icc")).to_vec();
        let condition_identifier = output_condition.clone();
        let objects = vec![
            PdfIndirectObject { id: ObjRef { num: 1, gen: 0 }, value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Catalog".into()) },
                PdfDictEntry { key: "Pages".into(), value: PdfObject::Ref(ObjRef { num: 4, gen: 0 }) },
                PdfDictEntry { key: "OutputIntents".into(), value: PdfObject::Array(vec![PdfObject::Ref(ObjRef { num: 2, gen: 0 })]) },
                PdfDictEntry::new("DPartRoot",PdfObject::Ref(ObjRef { num: 10, gen: 0 })),
            ]) },
            PdfIndirectObject { id: ObjRef { num: 2, gen: 0 }, value: PdfObject::Dict(vec![
                PdfDictEntry { key: "Type".into(), value: PdfObject::Name("OutputIntent".into()) },
                PdfDictEntry { key: "S".into(), value: PdfObject::Name("GTS_PDFX".into()) },
                PdfDictEntry { key: "OutputConditionIdentifier".into(), value: PdfObject::Str(output_condition.into_bytes()) },
                PdfDictEntry { key: "DestOutputProfile".into(), value: PdfObject::Ref(ObjRef { num: 3, gen: 0 }) },
            ]) },
            PdfIndirectObject { id: ObjRef { num: 3, gen: 0 }, value: PdfObject::Stream { dict: vec![PdfDictEntry { key: "N".into(), value: PdfObject::Int(3) }], data: profile.clone(), filters: Vec::new() } },
            PdfIndirectObject { id: ObjRef { num: 4, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry { key: "Type".into(), value: PdfObject::Name("Pages".into()) }, PdfDictEntry { key: "Kids".into(), value: PdfObject::Array(Vec::new()) }, PdfDictEntry { key: "Count".into(), value: PdfObject::Int(0) }]) },
            PdfIndirectObject { id: ObjRef { num: 10, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry::new("Type",PdfObject::name("DPartRoot")),PdfDictEntry::new("DPartRootNode",PdfObject::Ref(ObjRef { num: 11, gen: 0 }))]) },
            PdfIndirectObject { id: ObjRef { num: 11, gen: 0 }, value: PdfObject::Dict(vec![PdfDictEntry::new("Type",PdfObject::name("DPart")),PdfDictEntry::new("Parent",PdfObject::Ref(ObjRef { num: 10, gen: 0 })),PdfDictEntry::new("DPM",PdfObject::Dict(Vec::new()))]) },
        ];
        PdfSnapshot {
            output_intents: vec![PdfOutputIntent { subtype: "GTS_PDFX".into(), condition_identifier, condition: None, registry_name: None, info: None, profile: Some(profile) }],
            catalog_extra: vec![PdfDictEntry::new("DPartRoot",PdfObject::Ref(ObjRef { num: 10, gen: 0 }))],
            trailer: vec![PdfDictEntry { key: "Root".into(), value: PdfObject::Ref(ObjRef { num: 1, gen: 0 }) }],
            objects,
            ..PdfSnapshot::default()
        }
    }
    //#endregion 🔖️Seed

    //#region 🔖️Builder
    #[derive(Clone, Debug)]
    pub struct PdfVtBuilderConstruction {
        snapshot: PdfSnapshot,
    }

    impl PdfVtBuilderConstruction {
        /// ➕ The recommended entry point: REQUIRES an output-condition identifier up front.
        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn new(output_condition: impl Into<String>) -> Self {
            Self { snapshot: seeded_snapshot(output_condition.into()) }
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn add_page(mut self, mut page: PdfPage) -> Self {
            if page.trim_box.is_none() && page.art_box.is_none() { page.trim_box=Some(page.media_box); }
            let media=page.media_box;
            let trim=page.trim_box;
            let art=page.art_box;
            let root=self.snapshot.trailer.iter().find(|entry|entry.key=="Root").and_then(|entry|entry.value.as_ref());
            let catalog=root.and_then(|reference|self.snapshot.objects.iter().find(|object|object.id==reference)).map(|object|&object.value);
            let pages=catalog.and_then(|object|object.dict_get("Pages")).and_then(PdfObject::as_ref);
            let part_root=catalog.and_then(|object|object.dict_get("DPartRoot")).and_then(PdfObject::as_ref);
            let part_node=part_root.and_then(|reference|self.snapshot.objects.iter().find(|object|object.id==reference)).and_then(|object|object.value.dict_get("DPartRootNode")).and_then(PdfObject::as_ref);
            let index=self.snapshot.pages.len();
            apply_pdf_mutation(&mut self.snapshot,&PdfMutation::InsertPage(InsertPage{index,page}));
            let Some(pages)=pages else{return self};
            let number=if !self.snapshot.objects.iter().any(|object|object.id.num==5){Some(5)}else{self.snapshot.objects.iter().map(|object|object.id.num).max().and_then(|number|number.checked_add(1))};
            let Some(number)=number else{return self};
            let reference=ObjRef{num:number,gen:0};
            let mut entries=vec![PdfDictEntry::new("Type",PdfObject::name("Page")),PdfDictEntry::new("Parent",PdfObject::Ref(pages)),PdfDictEntry::new("MediaBox",PdfObject::numbers(&media))];
            if let Some(trim)=trim{entries.push(PdfDictEntry::new("TrimBox",PdfObject::numbers(&trim)));}
            if let Some(art)=art{entries.push(PdfDictEntry::new("ArtBox",PdfObject::numbers(&art)));}
            let at=self.snapshot.objects.iter().position(|object|object.id.num>number).unwrap_or(self.snapshot.objects.len());
            self.snapshot.objects.insert(at,PdfIndirectObject{id:reference,value:PdfObject::Dict(entries)});
            if let Some(object)=self.snapshot.objects.iter_mut().find(|object|object.id==pages){
                if let PdfObject::Dict(entries)=&mut object.value{
                    if let Some(entry)=entries.iter_mut().find(|entry|entry.key=="Kids"){if let PdfObject::Array(kids)=&mut entry.value{kids.push(PdfObject::Ref(reference));}}
                    if let Some(entry)=entries.iter_mut().find(|entry|entry.key=="Count"){entry.value=PdfObject::Int(self.snapshot.pages.len() as i64);}
                }
            }
            let mut pending=part_node.into_iter().collect::<Vec<_>>();
            let mut visited=std::collections::HashSet::new();
            let mut terminal=None;
            while let Some(node)=pending.pop(){
                if !visited.insert(node){continue;}
                let Some(object)=self.snapshot.objects.iter().find(|object|object.id==node)else{continue};
                if let Some(groups)=object.value.dict_get("DParts").and_then(PdfObject::as_array){
                    for group in groups.iter().rev(){if let Some(items)=group.as_array(){for item in items.iter().rev(){if let Some(reference)=item.as_ref(){pending.push(reference);}}}}
                }else{terminal=Some(node);}
            }
            if let Some(object)=terminal.and_then(|node|self.snapshot.objects.iter_mut().find(|object|object.id==node)){
                if let PdfObject::Dict(entries)=&mut object.value{
                    if !entries.iter().any(|entry|entry.key=="Start"){entries.push(PdfDictEntry::new("Start",PdfObject::Ref(reference)));}
                    else if index>0{if let Some(entry)=entries.iter_mut().find(|entry|entry.key=="End"){entry.value=PdfObject::Ref(reference);}else{entries.push(PdfDictEntry::new("End",PdfObject::Ref(reference)));}}
                }
            }
            self
        }

        // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
        pub fn set_info(mut self, info: PdfInfo) -> Self {
            apply_pdf_mutation(&mut self.snapshot, &PdfMutation::SetInfo(SetInfo { info }));
            self
        }
    }

    impl ArtifactBuilder for PdfVtBuilderConstruction {
        type Snapshot = PdfSnapshot;
        type Mutation = PdfMutation;
        type Diff = PdfDiff;

        fn empty() -> Self {
            let mut page=PdfPage::new(612.0,792.0);
            page.trim_box=Some(page.media_box);
            Self::new("sRGB2014").add_page(page)
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self::from_snapshot(<PdfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let diff = apply_pdf_mutation(&mut self.snapshot, &mutation);
            (self, diff)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <PdfDiff as protocol::MutationDiff<PdfSnapshot>>::apply(&diff, &self.snapshot)?;
            Ok(self)
        }

        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_vt_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot};
    use crate::standards::v1_7::subsets::base::io::PdfAnalyzer as PdfAnyAnalyzer;
    pub use crate::standards::v1_7::subsets::base::io::PdfParts;
    use crate::standards::v1_7::subsets::x::io::check_x_conformance;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::Analysis,semio_framework_plugin::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_plugin::IoConfidence,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.7"), subset: SubsetId("vt") };

    //#region 🔖️Conformance
    pub const CODE_DPART_ROOT: &str = "stdio.pdf.vt.missing-dpartroot";
    pub const CODE_DPM: &str = "stdio.pdf.vt.dpart-missing-dpm";

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dict_name<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a str> {
        dict.iter().find(|e| e.key == key).and_then(|e| e.value.as_name())
    }

    fn partition_graph(snapshot:&PdfSnapshot)->Result<Vec<ObjRef>,String>{
        use std::collections::{HashMap,HashSet};
        let mut index=HashMap::new();
        for object in &snapshot.objects{if index.insert(object.id,&object.value).is_some(){return Err("duplicate indirect object identity".into());}}
        let resolve=|reference:ObjRef|index.get(&reference).copied().ok_or_else(||"partition reference does not resolve".to_owned());
        let catalog=snapshot.trailer.iter().find(|entry|entry.key=="Root").and_then(|entry|entry.value.as_ref()).ok_or("document trailer has no indirect Root")?;
        let catalog=resolve(catalog)?;
        let root_pages=catalog.dict_get("Pages").and_then(PdfObject::as_ref).ok_or("Catalog has no indirect Pages")?;
        let mut page_nodes=HashSet::new();
        let mut pages=HashSet::new();
        let mut pending=vec![(root_pages,None)];
        while let Some((reference,parent))=pending.pop(){
            if !page_nodes.insert(reference){return Err("page tree has duplicate ownership or a cycle".into());}
            let object=resolve(reference)?;
            let PdfObject::Dict(entries)=object else{return Err("page-tree target is not a dictionary".into())};
            if let Some(parent)=parent{if object.dict_get("Parent").and_then(PdfObject::as_ref)!=Some(parent){return Err("page-tree Parent does not name its actual owner".into());}}
            match dict_name(entries,"Type"){
                Some("Pages")=>{
                    let kids=object.dict_get("Kids").and_then(PdfObject::as_array).ok_or("Pages has no Kids array")?;
                    for child in kids.iter().rev(){pending.push((child.as_ref().ok_or("page-tree child is not indirect")?,Some(reference)));}
                }
                Some("Page")=>{pages.insert(reference);}
                _=>return Err("page-tree target has the wrong Type".into())
            }
        }
        if pages.is_empty(){return Err("a terminal document part requires an actual Page".into());}
        if resolve(root_pages)?.dict_get("Count").and_then(PdfObject::as_i64)!=Some(pages.len() as i64){return Err("Pages Count disagrees with its actual leaves".into());}
        let root=catalog.dict_get("DPartRoot").and_then(PdfObject::as_ref).ok_or("Catalog DPartRoot is not indirect")?;
        let root_object=resolve(root)?;
        let PdfObject::Dict(root_dict)=root_object else{return Err("DPartRoot is not a dictionary".into())};
        if root_object.dict_get("Type").is_some()&&dict_name(root_dict,"Type")!=Some("DPartRoot"){return Err("DPartRoot has the wrong Type".into());}
        let node=root_object.dict_get("DPartRootNode").and_then(PdfObject::as_ref).ok_or("DPartRootNode must be an indirect DPart")?;
        let mut visited=HashSet::new();
        let mut missing=Vec::new();
        let mut pending=vec![(node,root)];
        while let Some((reference,parent))=pending.pop(){
            if !visited.insert(reference){return Err("DPart has duplicate ownership or a cycle".into());}
            let object=resolve(reference)?;
            let PdfObject::Dict(entries)=object else{return Err("DPart is not a dictionary".into())};
            if object.dict_get("Type").is_some()&&dict_name(entries,"Type")!=Some("DPart"){return Err("DPart has the wrong Type".into());}
            if object.dict_get("Parent").and_then(PdfObject::as_ref)!=Some(parent){return Err("DPart Parent does not name its actual owner".into());}
            match object.dict_get("DPM"){None=>missing.push(reference),Some(PdfObject::Dict(_))=>{},Some(_)=>return Err("DPM must be a direct dictionary".into())}
            match object.dict_get("DParts"){
                Some(PdfObject::Array(groups)) if !groups.is_empty()=>{
                    for group in groups.iter().rev(){
                        let PdfObject::Array(items)=group else{return Err("DParts owns arrays of indirect part members".into())};
                        if items.is_empty(){return Err("DPart child group is empty".into());}
                        for child in items.iter().rev(){pending.push((child.as_ref().ok_or("DPart child must be indirect")?,reference));}
                    }
                }
                Some(_)=>return Err("DParts must contain nonempty child groups".into()),
                None=>{let start=object.dict_get("Start").and_then(PdfObject::as_ref).ok_or("terminal DPart has no indirect Start Page")?;if !pages.contains(&start){return Err("terminal Start is outside the actual document page tree".into());}}
            }
            if let Some(end)=object.dict_get("End"){if !end.as_ref().is_some_and(|reference|pages.contains(&reference)){return Err("DPart End is outside the actual document page tree".into());}}
        }
        Ok(missing)
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real ISO 16612-2:2010 (PDF/VT-1/-2) conformance checks against one already-decoded
    /// `PdfSnapshot`: the full ISO 15930-7 (PDF/X-4) check suite (`🖨️x::check_x_conformance`) plus
    /// VT's own `/DPartRoot`/`/DPM` checks. Shared single source of truth used by `PdfVtComposer`,
    /// `PdfVtBuilder`, and `PdfVtValidator`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_vt_conformance(snapshot: &PdfSnapshot) -> Vec<Diagnostic> {
        let mut out=check_x_conformance(snapshot);
        let catalog=snapshot.trailer.iter().find(|entry|entry.key=="Root").and_then(|entry|entry.value.as_ref()).and_then(|reference|snapshot.objects.iter().find(|object|object.id==reference)).map(|object|&object.value);
        if !catalog.is_some_and(|catalog|catalog.dict_get("DPartRoot").is_some()){
            out.push(hard(CODE_DPART_ROOT,"the actual document Root carries no DPartRoot".into()));
            return out;
        }
        match partition_graph(snapshot){
            Ok(missing)=>{for reference in missing{out.push(soft(CODE_DPM,format!("DPart node {} {} R has no DPM metadata dictionary",reference.num,reference.gen)));}},
            Err(message)=>out.push(hard("stdio.pdf.vt.invalid-dpart-graph",message))
        }
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    pub struct PdfVtAnalyzerAnalysis;

    impl ArtifactAnalysis for PdfVtAnalyzerAnalysis {
        type Parts = PdfParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            PdfAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = PdfAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_vt_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = IoConfidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec PdfVtBuilderFacets {
        construction: PdfVtBuilderConstruction,
        analysis: PdfVtAnalyzerAnalysis,
        composition: super::io::derived_composition::PdfVtComposerComposition,
    }
    builder: PdfVtBuilder,
    analyzer: PdfVtAnalyzer,
    composer: PdfVtComposer,
);
