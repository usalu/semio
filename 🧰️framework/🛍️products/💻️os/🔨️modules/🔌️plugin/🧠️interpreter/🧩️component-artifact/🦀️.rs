//! 🧩️ Component artifact: the component version, the artifact kinds and sections, the component limits and the wasm artifact with its section readers. See [`super`].

use super::*;

//#region 🧩️ComponentArtifact
const COMPONENT_VERSION: [u8; 4] = [0x0d, 0x00, 0x01, 0x00];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WasmArtifactKind {
    Core,
    Component,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentSectionKind {
    Custom,
    CoreModule,
    CoreInstance,
    CoreType,
    Component,
    Instance,
    Alias,
    Type,
    Canonical,
    Start,
    Import,
    Export,
}

impl ComponentSectionKind {
    pub(super) fn parse(id: u8) -> Result<Self, CoreError> {
        match id {
            0 => Ok(Self::Custom),
            1 => Ok(Self::CoreModule),
            2 => Ok(Self::CoreInstance),
            3 => Ok(Self::CoreType),
            4 => Ok(Self::Component),
            5 => Ok(Self::Instance),
            6 => Ok(Self::Alias),
            7 => Ok(Self::Type),
            8 => Ok(Self::Canonical),
            9 => Ok(Self::Start),
            10 => Ok(Self::Import),
            11 => Ok(Self::Export),
            _ => Err(CoreError::Decode(format!("unknown component section {id}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ComponentSection {
    pub kind: ComponentSectionKind,
    pub offset: usize,
    pub length: usize,
    pub custom_name: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentLimits {
    pub maximum_bytes: usize,
    pub maximum_nesting: u32,
    pub maximum_core_modules: usize,
}

impl Default for ComponentLimits {
    fn default() -> Self {
        Self { maximum_bytes: 512 * 1024 * 1024, maximum_nesting: 32, maximum_core_modules: 1_024 }
    }
}

#[derive(Clone, Debug)]
pub struct ComponentArtifact {
    pub(super) bytes_fingerprint: u64,
    pub(super) sections: Vec<ComponentSection>,
    pub(super) core_modules: Vec<Arc<CoreModule>>,
    nested_components: Vec<Arc<ComponentArtifact>>,
}

impl ComponentArtifact {
    pub fn parse(bytes: &[u8]) -> Result<Self, CoreError> {
        Self::parse_with_limits(bytes, ComponentLimits::default())
    }

    pub fn parse_with_limits(bytes: &[u8], limits: ComponentLimits) -> Result<Self, CoreError> {
        Self::parse_nested(bytes, limits, 0)
    }

    fn parse_nested(bytes: &[u8], limits: ComponentLimits, nesting: u32) -> Result<Self, CoreError> {
        if bytes.len() > limits.maximum_bytes {
            return Err(CoreError::Validation(format!("component size {} exceeds limit {}", bytes.len(), limits.maximum_bytes)));
        }
        if nesting > limits.maximum_nesting {
            return Err(CoreError::Validation(format!("component nesting {nesting} exceeds limit {}", limits.maximum_nesting)));
        }
        let mut decoder = Decoder::new(bytes);
        if decoder.bytes(4)? != b"\0asm" || decoder.bytes(4)? != COMPONENT_VERSION {
            return Err(CoreError::Decode("expected WebAssembly component version 13.1".into()));
        }
        let mut sections = Vec::new();
        let mut core_modules = Vec::new();
        let mut nested_components = Vec::new();
        while !decoder.done() {
            let section_offset = decoder.position;
            let id = decoder.byte()?;
            let kind = ComponentSectionKind::parse(id)?;
            let length = decoder.u32()? as usize;
            let payload_offset = decoder.position;
            let payload = decoder.bytes(length)?;
            let custom_name = if kind == ComponentSectionKind::Custom {
                let mut custom = Decoder::new(payload);
                Some(custom.name()?)
            } else {
                None
            };
            match kind {
                ComponentSectionKind::CoreModule => {
                    if core_modules.len() == limits.maximum_core_modules {
                        return Err(CoreError::Validation(format!("component core module count exceeds limit {}", limits.maximum_core_modules)));
                    }
                    core_modules.push(Arc::new(CoreModule::parse(payload).map_err(|error| CoreError::Decode(format!("core module {}: {error}", core_modules.len())))?));
                }
                ComponentSectionKind::Component => nested_components.push(Arc::new(Self::parse_nested(payload, limits, nesting + 1)?)),
                _ => {}
            }
            sections.push(ComponentSection { kind, offset: section_offset, length: decoder.position - payload_offset, custom_name });
        }
        Ok(Self { bytes_fingerprint: stable_fingerprint(bytes), sections, core_modules, nested_components })
    }

    pub fn fingerprint(&self) -> u64 {
        self.bytes_fingerprint
    }

    pub fn sections(&self) -> &[ComponentSection] {
        &self.sections
    }

    pub fn core_modules(&self) -> &[Arc<CoreModule>] {
        &self.core_modules
    }

    pub fn nested_components(&self) -> &[Arc<ComponentArtifact>] {
        &self.nested_components
    }
}

#[derive(Clone, Debug)]
pub enum WasmArtifact {
    Core(Arc<CoreModule>),
    Component(Arc<ComponentArtifact>),
}

impl WasmArtifact {
    pub fn parse(bytes: &[u8]) -> Result<Self, CoreError> {
        if bytes.get(0..4) != Some(b"\0asm") {
            return Err(CoreError::Decode("expected WebAssembly magic".into()));
        }
        match bytes.get(4..8) {
            Some([1, 0, 0, 0]) => Ok(Self::Core(Arc::new(CoreModule::parse(bytes)?))),
            Some(version) if version == COMPONENT_VERSION => Ok(Self::Component(Arc::new(ComponentArtifact::parse(bytes)?))),
            _ => Err(CoreError::Decode("unsupported WebAssembly encoding version".into())),
        }
    }

    pub fn kind(&self) -> WasmArtifactKind {
        match self {
            Self::Core(_) => WasmArtifactKind::Core,
            Self::Component(_) => WasmArtifactKind::Component,
        }
    }
}
//#endregion 🧩️ComponentArtifact
