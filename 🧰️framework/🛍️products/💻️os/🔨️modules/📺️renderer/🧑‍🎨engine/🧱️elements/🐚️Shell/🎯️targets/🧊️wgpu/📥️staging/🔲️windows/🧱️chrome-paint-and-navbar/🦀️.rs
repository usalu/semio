//! 🔲️ `shell-windows` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🪟️WindowPaneChrome
/// 🪟️ One chip a window pane's own overlay rows carry. React mounts each as a FOLDED `Pane` inside
/// `window-body` at its own anchor (`🪟️Window/🟦️.tsx:320-412`), plus the world's projection pane
/// portalled into the same host (`🌐️World3dHost/🟦️.tsx:5188`): a top row `Actions` (leading) ·
/// `Search` (centre) · `Window Options` (trailing) and a bottom row `Utilities` (leading) ·
/// `Projection` (trailing).
///
/// 🩸️ wgpu painted NONE of them. Its only per-pane chrome was the Measures fold chip — an icon-only
/// square with no label — and the app's utilities sat on a FOOTER rail that React has no counterpart
/// for at all (`Transform · Brush · Volume Brush · Relocate` in the first real boot).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WindowPaneChip {
    Actions,
    Search,
    WindowOptions,
    Utilities,
    Projection,
}

impl WindowPaneChip {
    /// 🪟️ Every chip, in React's own mount order inside `window-body`.
    pub(crate) const ALL: [Self; 5] = [Self::Actions, Self::Search, Self::WindowOptions, Self::Utilities, Self::Projection];

    /// 🧭️ The `Pane` anchor React mounts this chip at.
    pub(crate) fn anchor(self) -> PanelAnchor {
        match self {
            Self::Actions => PanelAnchor::TopLeft,
            Self::Search => PanelAnchor::TopMiddle,
            Self::WindowOptions => PanelAnchor::TopRight,
            Self::Utilities => PanelAnchor::BottomLeft,
            Self::Projection => PanelAnchor::BottomRight,
        }
    }

    /// 🖼️ React's FIXED semantic pane icon, never a fold-direction chevron
    /// (`WINDOW_PANE_*_ICON`, `🖱️ui/🎯️targets/⚛️react/🟦️.tsx:8201-8204`).
    ///
    /// 🔀️ `Projection` is React's one exception: its `Pane` takes `icon={worldProjectionSpecIconId(spec)}`
    /// (`🌐️World3dHost/🟦️.tsx:5202`), so the chip wears the SELECTED template's icon. This answers the
    /// icon of [`WORLD_PROJECTION_DEFAULT_TEMPLATE_ID`] — the spec React synthesises for a pane whose
    /// camera declares none — and [`ShellState::window_pane_chip_icon_id`] resolves the live one.
    pub(crate) fn icon_id(self) -> &'static str {
        match self {
            Self::Actions => "play",
            Self::Search => "search",
            Self::WindowOptions => "settings-2",
            Self::Utilities => "hammer",
            Self::Projection => world_projection_template(WORLD_PROJECTION_DEFAULT_TEMPLATE_ID).icon_id,
        }
    }

    pub(crate) fn label_key(self) -> &'static str {
        match self {
            Self::Actions => "common.actions",
            Self::Search => "window.search.title",
            Self::WindowOptions => "common.windowOptions",
            Self::Utilities => "common.utilities",
            Self::Projection => "common.projection",
        }
    }

    /// 🎯️ The control id this chip's press dispatches — React's `toggleId` VERBATIM, carrying the same
    /// fold/unfold split. `Actions` and `Search` each drive the state of their own pane.
    ///
    /// 🩸️ This said "the wgpu twin of React's `toggleId`" while minting `shell.action.fold.<windowId>`
    /// and four siblings — a shell-private grammar with the window at the END. React mints
    /// `framework.window.<windowIdSegment>.engagement.toggle`, so NOTHING resolved the same chip on
    /// the two renderers, and a behavioural journey addressed by control id could only report every
    /// pane chip as absent on one side (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `📓️w9c-behaviour-parity-run-2.md` steps 9–12). The id is the integration key across i18n,
    /// tooltips, hotkeys, tutorials and introduction anchors — one spelling, or none of them meet.
    pub(crate) fn control_id(self, window_id: &str, folded: bool) -> String {
        match self {
            Self::Actions => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "engagement", "toggle"]),
            Self::Search => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "search", "toggle"]),
            Self::WindowOptions if folded => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "measures", "unfold"]),
            Self::WindowOptions => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "measures", "fold"]),
            Self::Utilities if folded => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "utilityBar", "unfold"]),
            Self::Utilities => semio_framework::child_element_id(WINDOW_PANE_CHIP_PARENT, &[window_id, "utilityBar", "fold"]),
            Self::Projection => semio_framework::child_element_id(WORLD_PROJECTION_PANE_PARENT, &[window_id, "pane", "fold"]),
        }
    }

    /// 🎯️ The chip a control id names, and the `(pane, verb)` pair it is spelled with — the exact
    /// inverse of [`Self::control_id`], so one table states both directions.
    pub(super) fn from_pane_and_verb(pane: &str, verb: &str) -> Option<Self> {
        match (pane, verb) {
            ("engagement", "toggle") => Some(Self::Actions),
            ("search", "toggle") => Some(Self::Search),
            ("measures", "fold" | "unfold") => Some(Self::WindowOptions),
            ("utilityBar", "fold" | "unfold") => Some(Self::Utilities),
            ("pane", "fold") => Some(Self::Projection),
            _ => None,
        }
    }
}

/// 🔀️ The template a pane with no explicit selection reads. React resolves
/// `cameraState.projectionSpec ?? worldProjectionDefaults("threePoint")` for a non-orthographic
/// camera (`🌐️World3dHost/🟦️.tsx`'s `worldProjectionSpec`), and this renderer's orbit is
/// perspective-only (`♾️infinite/🌍️world/🦀️.rs`'s `WORLD3D_PERSPECTIVE_CAMERA_ZOOM`), so both
/// shells open on 3-Point.
pub(crate) const WORLD_PROJECTION_DEFAULT_TEMPLATE_ID: &str = "three-point";

const WORLD_PROJECTION_TEMPLATE_PREFIX: &str = "world-projection:";

fn decoded_world_projection_template(template_id: &str) -> Option<Viewport3dProjectionSpec> {
    serde_json::from_str(template_id.strip_prefix(WORLD_PROJECTION_TEMPLATE_PREFIX)?).ok()
}

pub(crate) fn world_projection_template_selection_id(template_id: &str) -> Option<&'static str> {
    if let Some(template) = WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == template_id) {
        return Some(template.id);
    }
    Some(match decoded_world_projection_template(template_id)?.mode {
        Viewport3dProjectionMode::Orthographic {} => "orthographic",
        Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Isometric, .. } => "axonometric-isometric",
        Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Dimetric, .. } => "axonometric-dimetric",
        Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Trimetric, .. } => "axonometric-trimetric",
        Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cabinet, .. } => "oblique-cabinet",
        Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cavalier, .. } => "oblique-cavalier",
        Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Military, .. } => "oblique-military",
        Viewport3dProjectionMode::OnePoint { .. } => "one-point",
        Viewport3dProjectionMode::TwoPoint { .. } => "two-point",
        Viewport3dProjectionMode::ThreePoint { .. } => "three-point",
        Viewport3dProjectionMode::Curvilinear { .. } => "curvilinear",
    })
}

pub(crate) fn world_projection_initial_seed(template_id: &str) -> Option<Viewport3dProjectionSpec> {
    WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == template_id).copied().map(WorldProjectionTemplate::spec).or_else(|| decoded_world_projection_template(template_id))
}

/// 🔀️ The template row `id` names, or the default one for an id no template declares.
pub(crate) fn world_projection_template(id: &str) -> &'static WorldProjectionTemplate {
    let id = world_projection_template_selection_id(id).unwrap_or(WORLD_PROJECTION_DEFAULT_TEMPLATE_ID);
    WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == id).or_else(|| WORLD_PROJECTION_TEMPLATES.iter().find(|template| template.id == WORLD_PROJECTION_DEFAULT_TEMPLATE_ID)).expect("the default projection template is declared")
}

/// 🔀️ Materializes React's nested Projection Tree for one exact window pane. Row ids follow
/// `childElementId(paneId, template.id)` at every level. Nesting is structural; React deliberately
/// keeps every row id directly qualified by the pane rather than encoding its ancestor path.
fn world_projection_pane_rows(prefix: &str, window_id: &str, selected_id: &str, depth: u8, cursor: &mut usize) -> Vec<UiTreeItemNode> {
    let mut rows = Vec::new();
    while let Some(template) = WORLD_PROJECTION_TEMPLATES.get(*cursor) {
        if template.depth < depth {
            break;
        }
        if template.depth > depth {
            *cursor += 1;
            continue;
        }
        let id = semio_framework::child_element_id(prefix, &[template.id]);
        *cursor += 1;
        let children = world_projection_pane_rows(prefix, window_id, selected_id, depth + 1, cursor);
        rows.push(UiTreeItemNode {
            id,
            label: Label::data(template.label),
            icon_id: IconName::from_str(template.icon_id),
            presence: UiPresence { selected: template.id == selected_id, ..UiPresence::default() },
            default_open: Some(true),
            action: Some(ActionDescriptor {
                controller_id: "framework".into(),
                action: "setWorldProjectionTemplate".into(),
                args: crate::action_args_json!({ "windowId": window_id.to_string(), "templateId": template.id }),
            }),
            items: (!children.is_empty()).then_some(children),
            ..UiTreeItemNode::base(String::new(), Label::data(String::new()))
        });
    }
    rows
}

impl WindowState {
    /// 📐️ Hands a dock template to its World surface once the producer's delivered camera is
    /// resident. The map records template identity before scene paint creates the surface owner, so
    /// this boundary retries harmlessly until both halves exist.
    pub(crate) fn apply_initial_world_projection_template(&mut self, window_id: &str, surfaces: &mut SurfaceState) {
        let Some(template_id) = self.world_projection_template.get(window_id) else { return };
        let Some(seed) = world_projection_initial_seed(template_id) else { return };
        let Some(world) = surfaces.world3d_state_for_window_mut(window_id) else { return };
        let _ = infinite_world::world::apply_world3d_initial_projection_seed(&mut world.session, &mut world.camera, &mut world.scene, &mut world.bridge, seed);
    }

    /// 🎛️ React's `Window` `utilityBarFolded` — `useState(true)`, so a pane's Utilities rail starts
    /// FOLDED and is opened by its own chip (`🪟️Window/🟦️.tsx:176-177`).
    pub(crate) fn utility_bar_folded(&self, window_id: &str) -> bool {
        self.utility_bar_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 🔀️ React's `WorldOrbitProjectionSwitchPane` `folded` — `useState(true)`
    /// (`🌐️World3dHost/🟦️.tsx:5190`), so absent means folded.
    pub(crate) fn projection_pane_folded(&self, window_id: &str) -> bool {
        self.projection_pane_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 🔀️ The projection template this pane shows as selected — React's `worldProjectionSpec`
    /// fallback chain (`cameraState.projectionSpec ?? (cameraState.projection === "orthographic" ?
    /// worldProjectionDefaults("orthographic") : worldProjectionDefaults("threePoint"))`,
    /// `🌐️World3dHost/🟦️.tsx`), collapsed onto this renderer's flat template ids: an explicit press
    /// wins, otherwise the pane's LIVE camera family answers, which is what makes a `Top` pane open
    /// on Orthographic instead of the static default.
    pub(crate) fn world_projection_template_id(&self, window_id: &str, surfaces: &SurfaceState) -> &str {
        if let Some(selected) = self.world_projection_template.get(window_id) {
            return world_projection_template(selected).id;
        }
        match surfaces.world3d_state_for_window(window_id).map(|world| infinite_world::world::world3d_camera_projection(&world.camera)) {
            Some(ui_wgpu::wgpu::CameraProjection3d::Orthographic) => "orthographic",
            _ => WORLD_PROJECTION_DEFAULT_TEMPLATE_ID,
        }
    }

    /// 🎛️ React's `Window` Actions fold; absent means folded.
    pub(crate) fn window_actions_folded(&self, window_id: &str) -> bool {
        self.action_panel_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 🔎️ React's `Window` Search fold; absent means folded.
    pub(crate) fn window_search_folded(&self, window_id: &str) -> bool {
        self.search_panel_folded.get(window_id).copied().unwrap_or(true)
    }

    /// 🖼️ The icon ONE pane chip wears this frame — [`WindowPaneChip::icon_id`] for every fixed one,
    /// and the SELECTED projection template's icon for `Projection`, which is the one React resolves
    /// per pane (`icon={worldProjectionSpecIconId(spec)}`, `🌐️World3dHost/🟦️.tsx:5202`).
    pub(crate) fn window_pane_chip_icon_id(&self, chip: WindowPaneChip, window_id: &str, surfaces: &SurfaceState) -> &'static str {
        match chip {
            WindowPaneChip::Projection => world_projection_template(self.world_projection_template_id(window_id, surfaces)).icon_id,
            _ => chip.icon_id(),
        }
    }

    /// 🔀️ React's `WorldProjectionKindSwitch` as one retained Tree document. The generic Tree owns
    /// the full-width rows, icons, disclosure chevrons and AX hierarchy; Projection adds only the
    /// canonical taxonomy and exact per-window action payload.
    pub(super) fn build_window_projection_ui(&self, window_id: &str, surfaces: &SurfaceState) -> UiNode {
        let pane_id = format!("{WORLD_PROJECTION_PANE_PARENT}.{}", semio_framework::element_id_segment(window_id));
        let selected = self.world_projection_template_id(window_id, surfaces);
        let mut cursor = 0;
        let items = world_projection_pane_rows(&pane_id, window_id, selected, 0, &mut cursor);
        UiNode::Stack(UiStackNode {
            direction: "column".into(),
            gap: None,
            padding: None,
            id: Some(format!("{pane_id}.body")),
            children: vec![UiNode::Tree(UiTreeNode {
                presentation: Default::default(),
                sections: vec![UiTreeSectionNode {
                    header_toolbar: None,
                    id: format!("{pane_id}.projection-modes"),
                    label: None,
                    default_open: Some(true),
                    presence: UiPresence::default(),
                    items,
                    window: None,
                }],
                presence: UiPresence::default(),
                drop_action: None,
                menu: None,
                interaction_domain: None,
            })],
            presence: UiPresence::default(),
            activate: None,
            drop_action: None,
            drop_overlay: None,
            menu: None,
        })
    }

    /// 🔀️ Rebuilds one Projection document through the same revision-keyed shell ingress as Actions,
    /// Search and Measures. The prior lease retires before the replacement is admitted.
    pub(super) fn refresh_window_projection_document(&mut self, window_id: &str, surfaces: &SurfaceState, docs: &mut DocumentState) -> Result<(), String> {
        let surface = window_projection_surface_id(window_id);
        let records = panel_ui_scroll_records(&surface, &self.build_window_projection_ui(window_id, surfaces))?;
        let previous = docs.window_projection_documents.remove(window_id);
        docs.retire_one_surface_document(previous)?;
        let document = self.publish_surface_records(&surface, records)?;
        docs.window_projection_documents.insert(window_id.to_string(), document);
        Ok(())
    }
}

//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities

