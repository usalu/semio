//! 🎬️ The media window kit.
use semio_framework::WindowKindDefinition;
use semio_framework_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use semio_framework_ui_locale::Locale;
use crate::window_kits::WindowKit;
use crate::window_surface::UiAssemblyResult;
use crate::window_surface::ui_assembly_error;
use crate::window_surface::window_kind_definition;



pub const MEDIA_TRANSPORT_EXTENSION_ID: &str = "framework.media.transport@1";

pub const MEDIA_PLAYBACK_OUTPUT_PORT: &str = "playback:out";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaKind {
    Audio,
    Video,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaCapabilityStatus {
    Ready,
    Loading,
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MediaResource {
    pub controller_id: String,
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub output_port: String,
    pub revision: String,
    pub generation: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MediaView {
    pub duration_ms: Option<u64>,
    pub position_ms: u64,
    pub selection_start_ms: Option<u64>,
    pub selection_end_ms: Option<u64>,
    pub kind: MediaKind,
    pub media_type: String,
    pub revision: String,
    pub locale: Locale,
    pub resource: Option<MediaResource>,
    pub capability: MediaCapabilityStatus,
    pub capability_reason: Option<String>,
    pub host_content_height: f64,
}

pub struct MediaWindowKit;

impl MediaWindowKit {
    fn value_text(value: &str, field: &'static str) -> UiAssemblyResult<UiValue> {
        Ok(UiValue::Text(UiText::try_from_str(value).ok_or_else(|| ui_assembly_error(field))?))
    }

    fn put(map: &mut UiMapBuilder, key: &str, value: UiValue) -> UiAssemblyResult<()> {
        map.try_insert(key.into(), value).map_err(|_| ui_assembly_error("media-window.props"))
    }

    fn labels(locale: Locale) -> UiAssemblyResult<UiValue> {
        let labels = match locale {
            Locale::De => [
                ("play", "Wiedergabe"),
                ("pause", "Pause"),
                ("seek", "Position"),
                ("position", "Position"),
                ("duration", "Dauer"),
                ("selectionStart", "Auswahlbeginn"),
                ("selectionEnd", "Auswahlende"),
                ("loading", "Medium wird geladen…"),
                ("progress", "Ladefortschritt"),
                ("cancel", "Abbrechen"),
                ("unsupported", "Dieses Medium kann auf diesem Gerät nicht wiedergegeben werden."),
                ("unknownDuration", "Dauer unbekannt"),
                ("audio", "Audio"),
                ("video", "Video"),
            ],
            Locale::En => [
                ("play", "Play"),
                ("pause", "Pause"),
                ("seek", "Seek"),
                ("position", "Position"),
                ("duration", "Duration"),
                ("selectionStart", "Selection start"),
                ("selectionEnd", "Selection end"),
                ("loading", "Loading media…"),
                ("progress", "Loading progress"),
                ("cancel", "Cancel"),
                ("unsupported", "This media cannot be played on this device."),
                ("unknownDuration", "Unknown duration"),
                ("audio", "Audio"),
                ("video", "Video"),
            ],
        };
        let mut map = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("media-window.labels"))?;
        for (key, value) in labels {
            Self::put(&mut map, key, Self::value_text(value, "media-window.label")?)?;
        }
        Ok(UiValue::Map(map.finish()))
    }

    fn resource(resource: &MediaResource) -> UiAssemblyResult<UiValue> {
        let mut map = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("media-window.resource"))?;
        Self::put(&mut map, "kind", Self::value_text("artifact-media-export", "media-window.resource-kind")?)?;
        Self::put(&mut map, "controllerId", Self::value_text(&resource.controller_id, "media-window.resource-controller")?)?;
        Self::put(&mut map, "appInstanceId", UiValue::Number(f64::from(resource.app_instance_id)))?;
        Self::put(&mut map, "parentDocumentId", Self::value_text(&resource.parent_document_id, "media-window.resource-parent-document")?)?;
        Self::put(&mut map, "outputPort", Self::value_text(&resource.output_port, "media-window.resource-port")?)?;
        Self::put(&mut map, "revision", Self::value_text(&resource.revision, "media-window.resource-revision")?)?;
        Self::put(&mut map, "generation", Self::value_text(&resource.generation, "media-window.resource-generation")?)?;
        Ok(UiValue::Map(map.finish()))
    }
}

impl WindowKit for MediaWindowKit {
    type ViewModel = MediaView;
    const KIND_ID: &'static str = "framework.window.media";

    fn window_kind() -> WindowKindDefinition {
        window_kind_definition(Self::KIND_ID, "Media", "Medien", SurfaceKind::Canvas2d, "play", Vec::new())
    }

    fn editable_window_kind() -> WindowKindDefinition {
        Self::window_kind()
    }

    fn render(view: &MediaView) -> UiAssemblyResult<BuiltNode> {
        const MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;
        let parsed_revision = view.revision.parse::<u64>().ok().filter(|revision| revision.to_string() == view.revision).ok_or_else(|| ui_assembly_error("media-window.revision"))?;
        let _ = parsed_revision;
        if view.media_type.is_empty() || view.media_type.len() > 128 || !view.host_content_height.is_finite() || !(0.0..=4096.0).contains(&view.host_content_height) {
            return Err(ui_assembly_error("media-window.identity"));
        }
        if view.resource.as_ref().is_some_and(|resource| {
            resource.controller_id.is_empty()
                || resource.parent_document_id.is_empty()
                || resource.parent_document_id.len() > 512
                || resource.revision != view.revision
                || resource.output_port != MEDIA_PLAYBACK_OUTPUT_PORT
                || resource.generation.parse::<u64>().ok().is_none_or(|generation| generation.to_string() != resource.generation)
        }) || matches!(view.capability, MediaCapabilityStatus::Ready) && view.resource.is_none()
        {
            return Err(ui_assembly_error("media-window.resource-authority"));
        }
        let duration_ms = view.duration_ms.filter(|duration| *duration > 0 && *duration <= MAX_SAFE_INTEGER);
        let position_ms = duration_ms.map_or(0, |duration| view.position_ms.min(duration));
        let (selection_start_ms, selection_end_ms) = match (duration_ms, view.selection_start_ms, view.selection_end_ms) {
            (Some(duration), Some(start), Some(end)) if start < end => {
                let start = start.min(duration.saturating_sub(1));
                (Some(start), Some(end.max(start + 1).min(duration)))
            }
            _ => (None, None),
        };
        let kind = match view.kind {
            MediaKind::Audio => "audio",
            MediaKind::Video => "video",
        };
        let locale = match view.locale {
            Locale::En => "en",
            Locale::De => "de",
        };
        let status = match view.capability {
            MediaCapabilityStatus::Ready => "ready",
            MediaCapabilityStatus::Loading => "loading",
            MediaCapabilityStatus::Unsupported => "unsupported",
        };
        let mut capability = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("media-window.capability"))?;
        Self::put(&mut capability, "status", Self::value_text(status, "media-window.capability-status")?)?;
        Self::put(
            &mut capability,
            "reason",
            match &view.capability_reason {
                Some(reason) => Self::value_text(reason, "media-window.capability-reason")?,
                None => UiValue::Null,
            },
        )?;
        let mut props = UiMapBuilder::try_new().ok_or_else(|| ui_assembly_error("media-window.props"))?;
        for (key, value) in [
            ("schemaVersion", UiValue::Number(1.0)),
            ("kind", Self::value_text(kind, "media-window.kind")?),
            ("mediaType", Self::value_text(&view.media_type, "media-window.media-type")?),
            ("revision", Self::value_text(&view.revision, "media-window.revision")?),
            ("durationMs", duration_ms.map_or(UiValue::Null, |value| UiValue::Number(value as f64))),
            ("positionMs", UiValue::Number(position_ms as f64)),
            ("selectionStartMs", selection_start_ms.map_or(UiValue::Null, |value| UiValue::Number(value as f64))),
            ("selectionEndMs", selection_end_ms.map_or(UiValue::Null, |value| UiValue::Number(value as f64))),
            ("locale", Self::value_text(locale, "media-window.locale")?),
            ("labels", Self::labels(view.locale)?),
            (
                "resource",
                match &view.resource {
                    Some(resource) => Self::resource(resource)?,
                    None => UiValue::Null,
                },
            ),
            ("capability", UiValue::Map(capability.finish())),
            ("hostContentHeight", UiValue::Number(view.host_content_height)),
        ] {
            Self::put(&mut props, key, value)?;
        }
        BuiltNode::try_new(
            Self::KIND_ID,
            Component::Extension(ExtensionProps { extension: UiText::try_from_str(MEDIA_TRANSPORT_EXTENSION_ID).ok_or_else(|| ui_assembly_error("media-window.extension"))?, props: UiValue::Map(props.finish()) }),
        )
        .map_err(|_| ui_assembly_error("media-window.build"))
    }
}
