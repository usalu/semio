//! 🗒️ Core capabilities of the capabilities crate: Gateway capability declarations and the core catalog.

use crate::capability_declarations::{artifact_capabilities, inference_capabilities, inference_job_capabilities, ui_capabilities};
use crate::catalog_source_discovery::discover_catalog_source;
use semio_os_mcp_catalog::catalog::{CapabilityAudience, CapabilityDefinition, CapabilityKind, CapabilityOwner, CapabilityPresentation, CapabilityRef, CapabilitySource, Catalog, CatalogSource, ToolExposure, compile};
use semio_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};
use semio_os_mcp_schema::schema::{capabilities_describe_input_schema, capabilities_describe_output_schema, capabilities_search_input_schema, capabilities_search_output_schema, context_resolve_input_schema, context_resolve_output_schema};

//#region 🔖️CoreCapabilities

// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P2-catalog: the glob re-exports [Design notes](README.md#source-notes-11).

// 🎫️ ticket 26/08/17/LLM-FIRST-OS-VIA-THE-SEMIO-OS-MCP-GATEWAY packet P2-catalog: the small, stable [Design notes](README.md#source-notes-21).
fn capabilities_search_capability() -> CapabilityDefinition {
    let input_schema = capabilities_search_input_schema();
    CapabilityDefinition {
        id: CapabilityRef("capabilities.search".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Meta,
        audience: CapabilityAudience::Agent,
        title: "Search Capabilities".to_string(),
        description: "Deterministic BM25 search over the compiled capability catalog — no LLM.".to_string(),
        artifact_kind: None,
        use_when: vec!["find a capability".to_string(), "what can I do".to_string(), "search for an action".to_string()],
        input_schema,
        output_schema: capabilities_search_output_schema(),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "capabilities_search".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("search".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn capabilities_describe_capability() -> CapabilityDefinition {
    let input_schema = capabilities_describe_input_schema();
    CapabilityDefinition {
        id: CapabilityRef("capabilities.describe".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Meta,
        audience: CapabilityAudience::Agent,
        title: "Describe Capability".to_string(),
        description: "Returns the full CapabilityDefinition for one capability id.".to_string(),
        artifact_kind: None,
        use_when: vec!["show me the details of a capability".to_string(), "what arguments does this take".to_string()],
        input_schema,
        output_schema: capabilities_describe_output_schema(),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "capabilities_describe".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("info".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

fn context_resolve_capability() -> CapabilityDefinition {
    let input_schema = context_resolve_input_schema();
    CapabilityDefinition {
        id: CapabilityRef("context.resolve".to_string()),
        version: 1,
        owner: CapabilityOwner::Gateway,
        kind: CapabilityKind::Meta,
        audience: CapabilityAudience::Agent,
        title: "Resolve Context".to_string(),
        description: "Opens/refreshes the calling session and returns a token-cheap ContextSummary.".to_string(),
        artifact_kind: None,
        use_when: vec!["start a session".to_string(), "what can this session do".to_string()],
        input_schema,
        output_schema: context_resolve_output_schema(),
        effects: Default::default(),
        policy: Default::default(),
        execution: Default::default(),
        exposure: ToolExposure::Direct { tool_name: "context_resolve".to_string() },
        presentation: CapabilityPresentation { icon_id: Some("plug".to_string()), category: Some("gateway".to_string()), keys: None, in_palette: false, args: Vec::new() },
        examples: Vec::new(),
        source: CapabilitySource::Gateway,
    }
}

/// 🎯️ The three real, backend-free core tools this packet registers — folded into
/// `CatalogSource.gateway` so they appear in the compiled catalog exactly like every other
/// capability (D5: the catalog is the one source of truth `tools/list` and `capabilities.search`
/// both read from).
pub fn core_tool_capabilities() -> Vec<CapabilityDefinition> {
    let mut capabilities = vec![capabilities_search_capability(), capabilities_describe_capability(), context_resolve_capability()];
    capabilities.extend(artifact_capabilities());
    capabilities.extend(inference_capabilities());
    capabilities.extend(inference_job_capabilities());
    capabilities.extend(ui_capabilities());
    capabilities
}

//#endregion 🔖️CoreCapabilities

//#region 🔖️Catalog

/// 🗂️ Compiles the live gateway's catalog from the REAL installed plugin registry [Design notes](README.md#source-notes-116).
pub fn build_catalog() -> Catalog {
    compile(&discover_catalog_source(None), semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native).unwrap_or_else(|error| {
        eprintln!("[mcp registry] catalog compile failed ({error}) — falling back to gateway-only capabilities");
        let gateway_only = CatalogSource { descriptors: Vec::new(), os_commands: Vec::new(), shell: Vec::new(), gateway: core_tool_capabilities() };
        compile(&gateway_only, semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native).expect("core gateway capabilities alone never collide with themselves")
    })
}

/// 🔐 Compiles a capability catalog from descriptors already verified by an external authority.
/// Unlike installed discovery, this function never reads a registry or descriptor path.
pub fn catalog_from_descriptors(descriptors: Vec<semio_framework::PackageDescriptor>) -> Result<Catalog, GatewayError> {
    let source = CatalogSource { descriptors, os_commands: Vec::new(), shell: Vec::new(), gateway: core_tool_capabilities() };
    compile(&source, semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native)
        .map_err(|error| GatewayError::new(GatewayErrorCode::PluginUnavailable, format!("authenticated Hub descriptor catalog did not compile: {error}")).retryable())
}

pub fn gateway_only_catalog() -> std::sync::Arc<Catalog> {
    std::sync::Arc::new(catalog_from_descriptors(Vec::new()).expect("core gateway capabilities alone never collide with themselves"))
}

//#endregion 🔖️Catalog
