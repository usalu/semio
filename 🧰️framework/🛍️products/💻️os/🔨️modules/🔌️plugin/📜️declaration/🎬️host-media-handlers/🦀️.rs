//! 🎬️ Host media handlers: the mesh import and two dimensional svg export handler types, their descriptors, declarations, requests and results, the runtime error and the handler registry. See [`super`].

use super::*;

/// 🧷️ Typed mesh-to-document executable retained by the plugin runtime for mesh import.
pub type MeshDocumentImporter = fn(&semio_framework::MeshData) -> Result<semio_framework_pack_json::Value, String>;

/// 🧷️ Typed document-to-SVG executable retained by the plugin runtime for 2D export.
pub type TwoDSvgDocumentRenderer = fn(&Value) -> Result<(String, u32, u32), String>;

/// 🧭️ The closed set of host-media contribution roles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostMediaHandlerKind {
    MeshImport,
    TwoDSvgExport,
}

impl HostMediaHandlerKind {
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::MeshImport => "mesh-import",
            Self::TwoDSvgExport => "2d-svg-export",
        }
    }
}

/// ⚙️ Complete process-local identity of a typed host-media executable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostMediaExecutableIdentity {
    MeshImport(usize),
    TwoDSvgExport(usize),
}

/// 📋️ Immutable host-media descriptor visible to a runtime host.
#[derive(Clone, Debug, PartialEq)]
pub struct HostMediaHandlerDescriptor {
    pub id: String,
    pub owner: String,
    pub kind: HostMediaHandlerKind,
    pub artifact_kind: ArtifactKindSpec,
    pub artifact_schema: String,
    pub file_stem: Option<String>,
    pub executable_identity: HostMediaExecutableIdentity,
}

/// 🧾️ Descriptor-owned typed host-media contribution, frozen before any aggregate commit.
#[derive(Clone, Debug)]
pub struct HostMediaHandlerDeclaration {
    pub(super) id: ArtifactIdentity,
    pub(super) kind: HostMediaHandlerKind,
    pub(super) artifact_kind: ArtifactKindSpec,
    pub(super) artifact_schema: String,
    file_stem: Option<String>,
    executable: HostMediaExecutable,
}

#[derive(Clone, Copy, Debug)]
enum HostMediaExecutable {
    MeshImport(MeshDocumentImporter),
    TwoDSvgExport(TwoDSvgDocumentRenderer),
}

impl HostMediaHandlerDeclaration {
    /// 🏗️ Describes one mesh bridge without invoking its converter during assembly.
    pub fn mesh_import(id: impl Into<String>, artifact_kind: ArtifactKindSpec, artifact_schema: impl Into<String>, importer: MeshDocumentImporter) -> Result<Self, PluginAssemblyError> {
        let declaration = Self {
            id: ArtifactIdentity::parse(id).map_err(PluginAssemblyError::definition)?,
            kind: HostMediaHandlerKind::MeshImport,
            artifact_kind,
            artifact_schema: artifact_schema.into(),
            file_stem: None,
            executable: HostMediaExecutable::MeshImport(importer),
        };
        declaration.validate_shape()?;
        Ok(declaration)
    }

    /// 🏗️ Describes one 2D SVG renderer without invoking it during assembly.
    pub fn two_d_svg_export(id: impl Into<String>, artifact_kind: ArtifactKindSpec, artifact_schema: impl Into<String>, file_stem: impl Into<String>, renderer: TwoDSvgDocumentRenderer) -> Result<Self, PluginAssemblyError> {
        let declaration = Self {
            id: ArtifactIdentity::parse(id).map_err(PluginAssemblyError::definition)?,
            kind: HostMediaHandlerKind::TwoDSvgExport,
            artifact_kind,
            artifact_schema: artifact_schema.into(),
            file_stem: Some(file_stem.into()),
            executable: HostMediaExecutable::TwoDSvgExport(renderer),
        };
        declaration.validate_shape()?;
        Ok(declaration)
    }

    /// 🪪️ Returns the stable contribution id.
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// ⚙️ Returns the complete executable identity used for idempotence and conflict rejection.
    pub fn executable_identity(&self) -> HostMediaExecutableIdentity {
        match self.executable {
            HostMediaExecutable::MeshImport(importer) => HostMediaExecutableIdentity::MeshImport(importer as usize),
            HostMediaExecutable::TwoDSvgExport(renderer) => HostMediaExecutableIdentity::TwoDSvgExport(renderer as usize),
        }
    }

    pub(super) fn validate_shape(&self) -> Result<(), PluginAssemblyError> {
        if self.artifact_kind.id.trim().is_empty() || self.artifact_kind.schema.trim().is_empty() || self.artifact_schema.trim().is_empty() {
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-shape", "host-media contributions require non-empty artifact kind and document schema"));
        }
        if self.artifact_kind.schema != self.artifact_schema {
            return Err(PluginAssemblyError::new(
                "plugin-assembly.host-media-schema",
                format!("host-media contribution {:?} schema {:?} does not equal artifact-kind schema {:?}", self.id.as_str(), self.artifact_schema, self.artifact_kind.schema),
            ));
        }
        if matches!(self.kind, HostMediaHandlerKind::TwoDSvgExport) && self.file_stem.as_deref().is_none_or(|file_stem| file_stem.trim().is_empty()) {
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-file-stem", "2D SVG export contributions require a non-empty file stem"));
        }
        Ok(())
    }

    pub fn preflight(&self, owner: &str, declared_media_kinds: &BTreeMap<String, ArtifactKindSpec>) -> Result<(), PluginAssemblyError> {
        ArtifactIdentity::parse(owner.to_string()).map_err(PluginAssemblyError::definition)?;
        self.validate_shape()?;
        let self_id = self.id.as_str();
        let declared = declared_media_kinds
            .get(&self.artifact_kind.id)
            .ok_or_else(|| PluginAssemblyError::new("plugin-assembly.host-media-owner", format!("host-media contribution {:?} targets undeclared artifact kind {:?}", self_id, self.artifact_kind.id)))?;
        if declared != &self.artifact_kind {
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-kind", format!("host-media contribution {:?} does not use the plugin-owned descriptor for {:?}", self.id.as_str(), self.artifact_kind.id)));
        }
        Ok(())
    }

    pub(super) fn descriptor(&self, owner: &str) -> HostMediaHandlerDescriptor {
        HostMediaHandlerDescriptor {
            id: self.id.as_str().to_string(),
            owner: owner.to_string(),
            kind: self.kind,
            artifact_kind: self.artifact_kind.clone(),
            artifact_schema: self.artifact_schema.clone(),
            file_stem: self.file_stem.clone(),
            executable_identity: self.executable_identity(),
        }
    }
}

/// 📥️ Typed mesh bridge runtime request.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshImportRequest {
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub mesh: semio_framework::MeshData,
}

/// 📤️ Typed mesh bridge runtime result.
#[derive(Clone, Debug, PartialEq)]
pub struct MeshImportResult {
    pub document: semio_framework_pack_json::Value,
}

/// 📥️ Typed 2D SVG export runtime request.
#[derive(Clone, Debug, PartialEq)]
pub struct TwoDSvgExportRequest {
    pub artifact_kind: String,
    pub artifact_schema: String,
    pub document: Value,
}

/// 📤️ Typed 2D SVG export runtime result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TwoDSvgExportResult {
    pub file_stem: String,
    pub svg: String,
    pub width: u32,
    pub height: u32,
}

/// 🚧️ Typed host-media runtime rejection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HostMediaRuntimeError {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for HostMediaRuntimeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for HostMediaRuntimeError {}

struct RegisteredHostMediaHandler {
    pub(super) descriptor: HostMediaHandlerDescriptor,
    executable: HostMediaExecutable,
}

impl RegisteredHostMediaHandler {
    pub(super) fn identical_to(&self, other: &Self) -> bool {
        self.descriptor == other.descriptor
            && match (self.executable, other.executable) {
                (HostMediaExecutable::MeshImport(left), HostMediaExecutable::MeshImport(right)) => std::ptr::fn_addr_eq(left, right),
                (HostMediaExecutable::TwoDSvgExport(left), HostMediaExecutable::TwoDSvgExport(right)) => std::ptr::fn_addr_eq(left, right),
                _ => false,
            }
    }
}

pub struct HostMediaHandlerRegistry {
    pub(super) by_id: BTreeMap<String, RegisteredHostMediaHandler>,
    by_target: BTreeMap<(HostMediaHandlerKind, String), String>,
}

impl HostMediaHandlerRegistry {
    pub fn new() -> Self {
        Self { by_id: BTreeMap::new(), by_target: BTreeMap::new() }
    }

    pub fn from_declarations(owner: &str, declarations: &[HostMediaHandlerDeclaration]) -> Result<Self, PluginAssemblyError> {
        let mut registry = Self::new();
        for declaration in declarations {
            registry.register(owner, declaration)?;
        }
        Ok(registry)
    }

    pub(super) fn register(&mut self, owner: &str, declaration: &HostMediaHandlerDeclaration) -> Result<(), PluginAssemblyError> {
        let descriptor = declaration.descriptor(owner);
        let entry = RegisteredHostMediaHandler { descriptor: descriptor.clone(), executable: declaration.executable };
        if let Some(existing) = self.by_id.get(&descriptor.id) {
            if existing.identical_to(&entry) {
                return Ok(());
            }
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-id", format!("host-media contribution {:?} has a conflicting descriptor or executable identity", descriptor.id)));
        }
        let target = (descriptor.kind, descriptor.artifact_kind.id.clone());
        if let Some(existing) = self.by_target.get(&target) {
            return Err(PluginAssemblyError::new("plugin-assembly.host-media-target", format!("host-media target {}:{} is already owned by {:?}", descriptor.kind.as_str(), descriptor.artifact_kind.id, existing)));
        }
        self.by_target.insert(target, descriptor.id.clone());
        self.by_id.insert(descriptor.id, entry);
        Ok(())
    }

    pub fn descriptors(&self) -> Vec<HostMediaHandlerDescriptor> {
        self.by_id.values().map(|entry| entry.descriptor.clone()).collect()
    }

    pub fn mesh_import(&self, request: &MeshImportRequest) -> Result<MeshImportResult, HostMediaRuntimeError> {
        let entry = self.lookup(HostMediaHandlerKind::MeshImport, &request.artifact_kind, &request.artifact_schema)?;
        let HostMediaExecutable::MeshImport(importer) = entry.executable else {
            return Err(HostMediaRuntimeError { code: "host-media.executable-kind".into(), message: format!("host-media target {:?} has no mesh executable", request.artifact_kind) });
        };
        importer(&request.mesh).map(|document| MeshImportResult { document }).map_err(|message| HostMediaRuntimeError { code: "host-media.mesh-import".into(), message })
    }

    pub fn two_d_svg_export(&self, request: &TwoDSvgExportRequest) -> Result<TwoDSvgExportResult, HostMediaRuntimeError> {
        let entry = self.lookup(HostMediaHandlerKind::TwoDSvgExport, &request.artifact_kind, &request.artifact_schema)?;
        let HostMediaExecutable::TwoDSvgExport(renderer) = entry.executable else {
            return Err(HostMediaRuntimeError { code: "host-media.executable-kind".into(), message: format!("host-media target {:?} has no 2D SVG executable", request.artifact_kind) });
        };
        let (svg, width, height) = renderer(&request.document).map_err(|message| HostMediaRuntimeError { code: "host-media.2d-svg".into(), message })?;
        Ok(TwoDSvgExportResult { file_stem: entry.descriptor.file_stem.clone().expect("2D SVG declaration requires a file stem"), svg, width, height })
    }

    fn lookup(&self, kind: HostMediaHandlerKind, artifact_kind: &str, artifact_schema: &str) -> Result<&RegisteredHostMediaHandler, HostMediaRuntimeError> {
        let kind_str = kind.as_str();
        let id = self.by_target.get(&(kind, artifact_kind.to_string())).ok_or_else(|| HostMediaRuntimeError { code: "host-media.not-declared".into(), message: format!("no {} declaration for artifact kind {:?}", kind_str, artifact_kind) })?;
        let entry = self.by_id.get(id).expect("host-media target index always points to an entry");
        if entry.descriptor.artifact_schema != artifact_schema {
            return Err(HostMediaRuntimeError {
                code: "host-media.schema-mismatch".into(),
                message: format!("host-media declaration {:?} expects schema {:?}, request supplied {:?}", entry.descriptor.id, entry.descriptor.artifact_schema, artifact_schema),
            });
        }
        Ok(entry)
    }
}
