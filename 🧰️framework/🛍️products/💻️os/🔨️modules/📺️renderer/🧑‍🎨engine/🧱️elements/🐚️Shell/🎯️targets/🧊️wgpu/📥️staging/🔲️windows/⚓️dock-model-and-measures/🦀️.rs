//! 🔲️ `shell-windows` staging of `dock-model-and-measures`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 📏️WindowMeasures
/// 📏️ The retained engine surface one window instance's Measures overlay paints into — the window
/// instance, then the reserved section body key, the same `{windowId}/{id}` rule React's
/// `windowMeasureDomId` qualifies measure ids with. Window instance ids are plain identifiers, so the
/// suffix can never collide with an app-authored surface.
pub(crate) fn window_measures_surface_id(window_id: &str) -> String {
    format!("{window_id}/{}", semio_framework::UiRefreshSection::Measures.body_key())
}

/// 📏️ Projects general window measures into a compact retained Tree with one labelled row per
/// measure. Controls preserve their original keys and action bindings; groups retain disclosure
/// and their optional inline header slider. Utility-scoped groups belong to the utility rail.
/// See `🧑‍🎨engine/🧫️fixtures/📏️window-measures/🔣️.json`.
pub(crate) fn window_measures_overlay_records(window_id: &str, measures: &[WindowMeasure], active_utility_id: Option<&str>) -> Result<Option<Vec<ui_contract::UiNodeRecord>>, String> {
    let partition = ui_wgpu::wgpu::partition_window_measures(measures, active_utility_id)
        .map_err(|measure| format!("window '{window_id}' authored more than {} measures, refused at {measure:?}", ui_wgpu::wgpu::component::layout::WINDOW_MEASURE_PARTITION_CAPACITY))?;
    if partition.general.is_empty() {
        return Ok(None);
    }
    let mut projection = WindowMeasuresProjection { window_id, records: Vec::new() };
    let root = projection.reserve();
    let section = projection.reserve();
    let mut children = ui_contract::UiNodeChildren::default();
    for measure in partition.general.iter() {
        if let Some(child) = projection.measure(measure)? {
            children.try_push(child).map_err(|_| format!("window '{window_id}' measures exceed one overlay document"))?;
        }
    }
    projection.place(
        section,
        format!("{window_id}/measures.rows"),
        ui_contract::Component::TreeSection(ui_contract::TreeSectionProps { label: None, default_open: Some(true), header_toolbar: None, window: None }),
        MeasureRecord { children, ..MeasureRecord::default() },
    )?;
    let mut children = ui_contract::UiNodeChildren::default();
    children.try_push(section).map_err(|_| "measure tree admits its section".to_string())?;
    projection.place(
        root,
        semio_framework::UiRefreshSection::Measures.body_key().to_string(),
        ui_contract::Component::Tree(ui_contract::TreeProps { presentation: ui_contract::TreePresentation::Compact, interaction_domain: None }),
        MeasureRecord { children, ..MeasureRecord::default() },
    )?;
    projection.records.into_iter().collect::<Option<Vec<_>>>().map(Some).ok_or_else(|| format!("window '{window_id}' measures projection left a reserved record unplaced"))
}

/// 🧩️ Everything a measure record carries beside its key and component.
#[derive(Default)]
struct MeasureRecord {
    pub(crate) children: ui_contract::UiNodeChildren,
    pub(crate) bindings: ui_contract::UiNodeBindings,
    pub(crate) activity: ui_contract::Activity,
    pub(crate) disabled: bool,
    pub(crate) label: Option<ui_contract::Label>,
}

/// 🌳️ One overlay document under construction: ids are reserved pre-order and filled once a node's
/// children exist, so the published record order is parent-first.
struct WindowMeasuresProjection<'a> {
    pub(crate) window_id: &'a str,
    pub(crate) records: Vec<Option<ui_contract::UiNodeRecord>>,
}

impl WindowMeasuresProjection<'_> {
    pub(crate) fn reserve(&mut self) -> ui_contract::UiNodeId {
        self.records.push(None);
        ui_contract::UiNodeId(self.records.len() as u64)
    }

    pub(crate) fn place(&mut self, id: ui_contract::UiNodeId, key: String, component: ui_contract::Component, record: MeasureRecord) -> Result<ui_contract::UiNodeId, String> {
        if self.records.len() > ui_contract::UI_DOCUMENT_NODES {
            return Err(format!("window '{}' measures exceed {} overlay nodes", self.window_id, ui_contract::UI_DOCUMENT_NODES));
        }
        let key = UiText::try_from_string(key).map_err(|key| format!("measure key '{key}' exceeds the retained contract"))?;
        let MeasureRecord { children, bindings, activity, disabled, label } = record;
        self.records[(id.0 - 1) as usize] = Some(ui_contract::UiNodeRecord {
            id,
            key,
            component,
            layout: ui_contract::LayoutSpec::Stack(ui_contract::StackLayout { axis: ui_contract::Axis::Vertical, gap: ui_contract::SpaceToken::None, ..Default::default() }),
            style: Default::default(),
            activity,
            disabled,
            transition: None,
            accessibility: ui_contract::AccessibilitySpec { label, ..Default::default() },
            bindings,
            menu: None,
            children,
        });
        Ok(id)
    }

    pub(crate) fn control(&mut self, id: &str, label: Option<&str>, component: ui_contract::Component, record: MeasureRecord) -> Result<ui_contract::UiNodeId, String> {
        let key = format!("{}/{id}", self.window_id);
        let row = self.reserve();
        let control = self.reserve();
        let row_label = label.map(measure_label).or_else(|| record.label.clone()).unwrap_or_else(|| measure_label(""));
        self.place(control, key.clone(), component, record)?;
        let mut children = ui_contract::UiNodeChildren::default();
        children.try_push(control).map_err(|_| "measure row admits its control".to_string())?;
        self.place(row, format!("{key}.row"), measure_tree_item(row_label.clone(), None), MeasureRecord { children, label: Some(row_label), ..MeasureRecord::default() })
    }

    pub(crate) fn measure(&mut self, measure: &WindowMeasure) -> Result<Option<ui_contract::UiNodeId>, String> {
        match measure {
            WindowMeasure::Number { id, label, value, min, max, step, loading, waiting, disabled, on_change, .. } => {
                let input = ui_contract::InputProps { kind: ui_contract::InputKind::Number, value: UiText::clipped(&value.to_string()), placeholder: None, commit: Some(UiText::clipped("blur")), min: *min, max: *max, step: *step, accept: None, precision: None, ..Default::default() };
                let record = MeasureRecord {
                    bindings: measure_bindings(ui_contract::Trigger::Commit, on_change, None)?,
                    activity: measure_activity(*loading, *waiting),
                    disabled: disabled.unwrap_or(false),
                    label: label.as_deref().map(measure_label),
                    ..MeasureRecord::default()
                };
                self.control(id, label.as_deref(), ui_contract::Component::Input(input), record).map(Some)
            }
            WindowMeasure::Slider { id, label, value, min, max, step, loading, waiting, disabled, on_change, .. } => {
                let record = MeasureRecord {
                    bindings: measure_bindings(ui_contract::Trigger::Change, on_change, None)?,
                    activity: measure_activity(*loading, *waiting),
                    disabled: disabled.unwrap_or(false),
                    label: label.as_deref().map(measure_label),
                    ..MeasureRecord::default()
                };
                self.control(id, label.as_deref(), measure_slider(*value, *min, *max, *step, None), record).map(Some)
            }
            WindowMeasure::Select { id, label, value, items, on_change } => {
                let mut options = UiFixedList::default();
                for item in items {
                    options.try_push(ui_contract::SelectItem { value: UiText::clipped(&item.value), label: measure_label(&item.label) }).map_err(|_| format!("measure '{id}' has more options than one select admits"))?;
                }
                let record = MeasureRecord { bindings: measure_bindings(ui_contract::Trigger::Change, on_change, None)?, label: label.as_deref().map(measure_label), ..MeasureRecord::default() };
                self.control(id, label.as_deref(), ui_contract::Component::Select(ui_contract::SelectProps { value: UiText::clipped(value), items: options, placeholder: None, appearance: ui_contract::SelectAppearance::Menu }), record).map(Some)
            }
            WindowMeasure::Toggle { id, icon_id, label, pressed, text, on_change } => {
                let toggle = ui_contract::ToggleProps { appearance: ui_contract::ToggleAppearance::Checkbox, on: *pressed, icon: UiText::clipped(icon_id.as_str()), text: text.as_deref().map(measure_label) };
                let record =
                    MeasureRecord { bindings: measure_bindings(ui_contract::Trigger::Change, on_change, Some(("pressed", semio_framework_value::DslValue::Bool(!pressed))))?, label: label.as_deref().or(text.as_deref()).map(measure_label), ..MeasureRecord::default() };
                self.control(id, label.as_deref(), ui_contract::Component::Toggle(toggle), record).map(Some)
            }
            WindowMeasure::Group { id, label, default_open, value, min, max, step, loading, waiting, on_change, children, .. } => {
                let key = format!("{}/{id}", self.window_id);
                let section = self.reserve();
                let mut ids = ui_contract::UiNodeChildren::default();
                if let (Some(value), Some(on_change)) = (value, on_change) {
                    let header = self.reserve();
                    let record = MeasureRecord { bindings: measure_bindings(ui_contract::Trigger::Change, on_change, None)?, activity: measure_activity(*loading, *waiting), label: Some(measure_label(label)), ..MeasureRecord::default() };
                    ids.try_push(self.place(header, format!("{key}.header-slider"), measure_slider(*value, min.unwrap_or(0.0), max.unwrap_or(1.0), *step, None), record)?).map_err(|_| format!("measure group '{id}' exceeds one overlay document"))?;
                }
                for child in children {
                    if let Some(child) = self.measure(child)? {
                        ids.try_push(child).map_err(|_| format!("measure group '{id}' exceeds one overlay document"))?;
                    }
                }
                let record = MeasureRecord { children: ids, activity: measure_activity(*loading, *waiting), label: Some(measure_label(label)), ..MeasureRecord::default() };
                self.place(section, format!("{key}.row"), measure_tree_item(measure_label(label), Some(default_open.unwrap_or(false))), record).map(Some)
            }
        }
    }
}

impl WindowState {
    /// 🎬️ Installs one canonical engagements section and retires its transport lease before the
    /// shell republishes Actions/Search bodies. A malformed payload reports a fault and preserves
    /// the last valid snapshot.
    pub(crate) fn install_window_engagements_section(&mut self, document: UiDocumentLease, faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>, docs: &mut DocumentState) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Engagements.body_key();
        let engagements = crate::program_bridge::window_engagements_from_section(&document);
        docs.retire_one_surface_document(Some(document))?;
        match engagements {
            Ok(engagements) => self.window_engagements = engagements,
            Err(error) => faults.push((body_key.to_string(), body_key.to_string(), error.into())),
        }
        Ok(())
    }

    /// 🎬️ Reads the canonical reserved engagements surface on either ProgramBridge backend.
    pub(crate) async fn refresh_window_engagements(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>, faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>, docs: &mut DocumentState) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Engagements.body_key();
        let section = match read_ahead {
            Some(section) => section,
            None => program.window_engagements_section(instance_id, view_state).await,
        };
        match section {
            Ok(document) => self.install_window_engagements_section(document, faults, docs)?,
            Err(error) => faults.push((body_key.to_string(), body_key.to_string(), error.into())),
        }
        Ok(())
    }

    /// 📏️ Reads the instance's reserved measures surface once per refresh and republishes every live
    /// window instance's Measures overlay document. A window whose measures fail to project keeps no
    /// overlay and reports a surface fault instead of blanking the refresh.
    pub(crate) async fn refresh_window_measures(
        &mut self,
        program: &ProgramBridgeEntry,
        instance_id: u32,
        view_state: &ViewModel,
        read_ahead: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>,
        windows: &[String],
        faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>, docs: &mut DocumentState
    ) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Measures.body_key();
        let section = match read_ahead {
            Some(section) => section,
            None => program.window_measures_section(instance_id, view_state).await,
        };
        let measures = match section {
            Ok(document) => {
                let measures = crate::program_bridge::window_measures_from_section(&document).map_err(crate::program_bridge::ProgramFault::from);
                docs.retire_one_surface_document(Some(document))?;
                measures
            }
            Err(error) => Err(error),
        };
        let measures = measures.unwrap_or_else(|error| {
            faults.push((body_key.to_string(), body_key.to_string(), error.into()));
            HashMap::new()
        });
        let stale: Vec<String> = docs.window_measures_documents.keys().filter(|window_id| !windows.contains(window_id)).cloned().collect();
        for window_id in stale.into_iter().chain(windows.iter().cloned()) {
            let previous = docs.window_measures_documents.remove(&window_id);
            docs.retire_one_surface_document(previous)?;
            if !windows.contains(&window_id) {
                continue;
            }
            let surface = window_measures_surface_id(&window_id);
            match self.publish_window_measures(&window_id, &surface, measures.get(&window_id).map_or(&[], Vec::as_slice)) {
                Ok(Some(document)) => {
                    docs.window_measures_documents.insert(window_id, document);
                }
                Ok(None) => {}
                Err(error) => faults.push((surface, body_key.to_string(), error.into())),
            }
        }
        Ok(())
    }

    /// 🛠️ Reads the instance's reserved TOOLS section once per refresh into
    /// [`ShellState::tool_measures`] — the wgpu twin of React's `toolMeasuresByToolId` ref, which its
    /// `buildToolTabs` `resolveTree` reads fresh at render time. A read failure reports a surface
    /// fault and leaves the previous roster rather than blanking every armed tool's options.
    pub(crate) async fn refresh_tool_measures(&mut self, program: &ProgramBridgeEntry, instance_id: u32, view_state: &ViewModel, read_ahead: Option<Result<UiDocumentLease, crate::program_bridge::ProgramFault>>, faults: &mut Vec<(String, String, crate::program_bridge::ProgramFault)>, docs: &mut DocumentState) -> Result<(), String> {
        let body_key = semio_framework::UiRefreshSection::Tools.body_key();
        let section = match read_ahead {
            Some(section) => section,
            None => program.tool_measures_section(instance_id, view_state).await,
        };
        let measures = match section {
            Ok(document) => {
                let measures = crate::program_bridge::tool_measures_from_section(&document).map_err(crate::program_bridge::ProgramFault::from);
                docs.retire_one_surface_document(Some(document))?;
                measures
            }
            Err(error) => Err(error),
        };
        match measures {
            Ok(measures) => self.tool_measures = measures,
            Err(error) => faults.push((body_key.to_string(), body_key.to_string(), error.into())),
        }
        Ok(())
    }

    /// 🧾️ One window's overlay document, minted through the engine's own ingress-generation rule
    /// keyed on the projected content, so an unchanged overlay pays no ingress.
    pub(super) fn publish_window_measures(&mut self, window_id: &str, surface: &str, measures: &[WindowMeasure]) -> Result<Option<UiDocumentLease>, String> {
        let Some(records) = window_measures_overlay_records(window_id, measures, self.active_utility_for_window(window_id))? else { return Ok(None) };
        self.publish_surface_records(surface, records).map(Some)
    }

    /// 🧾️ Mints one SHELL-OWNED retained document from already-projected records: the revision is the
    /// content's own hash, so republishing an unchanged body answers `Published` and pays no ingress.
    /// Every shell-owned panel (the Measures overlay, the Command dock, Settings' five leaves, the Tool
    /// leaves, the Marketplace leaf) goes through this one ingress.
    pub(crate) fn publish_surface_records(&mut self, surface: &str, records: Vec<ui_contract::UiNodeRecord>) -> Result<UiDocumentLease, String> {
        let revision = {
            use std::hash::{Hash, Hasher};
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            serde_json::to_string(&records).map_err(|error| error.to_string())?.hash(&mut hasher);
            hasher.finish()
        };
        let generation = ui_wgpu::wgpu::engine::ui_document_ingress_generation(self.window_measures_minted.get(surface).copied(), revision);
        self.window_measures_minted.insert(surface.to_string(), (revision, generation));
        let identity = ui_contract::UiDocumentAssemblyIdentity { generation, revision: ui_contract::UiRevision(revision), root: Some(ui_contract::UiNodeId(1)), layout_epoch: 0 };
        let surface_id = SurfaceId::try_from(surface).map_err(|_| format!("shell surface '{surface}' exceeds the retained contract"))?;
        UiDocumentLease::try_publish(surface_id, identity, records).map_err(|error| crate::program_bridge::retained_publication_refusal(surface, error))
    }

    /// 🎛️ React's Window Options fold state: `useState(true)` — the rail starts FOLDED and is opened
    /// by its own chip (`🖱️ui/🧱️elements/🪟️Window/🟦️.tsx:169-173`).
    pub(crate) fn measures_rail_folded(&self, window_id: &str) -> bool {
        self.measures_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 📐️ The Measures overlay's rect inside a window body: right-aligned like React's measures pane,
    /// at the window's own resized width (or the theme default), inset by the panel inset.
    pub(super) fn window_measures_rect(&self, window_id: &str, body: Rect, theme: &Theme) -> Rect {
        let width = self.measures_width.get(window_id).copied().unwrap_or(theme.window_measures_default_width).min((body.w - theme.panel_inset * 2.0).max(0.0));
        let top = theme.panel_inset + theme.control_height + theme.gap_standard;
        let available = (body.h - top - theme.panel_inset).max(0.0);
        let height = crate::interpreter::retained_content_height(&window_measures_surface_id(window_id)).unwrap_or(available).min(available).max(0.0);
        Rect::new(body.x + body.w - width - theme.panel_inset, body.y + top, width, height)
    }
}

//#endregion 📏️WindowMeasures
//#endregion ShellLifecycle

