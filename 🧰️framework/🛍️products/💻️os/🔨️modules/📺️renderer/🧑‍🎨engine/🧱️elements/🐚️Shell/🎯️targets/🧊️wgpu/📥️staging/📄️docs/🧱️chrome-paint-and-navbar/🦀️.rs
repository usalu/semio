//! 📄️ `shell-docs` staging of `chrome-paint-and-navbar`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ActionPanelAndUtilities
//#region 🔀️ChromeParity
/// 📚️ One row of the open `playground.navbar.fixture` dropdown. `control_id` is what the click
/// handler's `shell.example.<id>` arm already dispatches `setActiveExample` for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ShellExampleRow {
    pub control_id: String,
    pub label: String,
    pub selected: bool,
}

/// 📚️ The example rows one surface offers, by DIALECT — `manifest::examples_for_app` is THE
/// predicate, shared with React's `examplesForApp` and pinned by
/// `🛂️manifest/🧫️fixtures/📚️example-picker.json`. Viewers additionally declare the example-switch action.
pub(crate) fn shell_offered_examples<'a>(examples: &'a [semio_framework::manifest::ExampleDefinition], app: &AppDefinition) -> Vec<&'a semio_framework::manifest::ExampleDefinition> {
    if app.role != semio_framework::AppRole::Editor && !app.actions.iter().chain(app.window_kinds.iter().flat_map(|kind| kind.actions.iter())).any(|action| action.id == "setActiveExample") {
        return Vec::new();
    }
    semio_framework::manifest::examples_for_app(examples, app)
}

pub(crate) fn shell_example_rows(examples: &[semio_framework::manifest::ExampleDefinition], app: &AppDefinition, active_example_id: Option<&str>, terminology: Terminology, locale: Locale) -> Vec<ShellExampleRow> {
    shell_offered_examples(examples, app)
        .into_iter()
        .map(|example| ShellExampleRow { control_id: format!("shell.example.{}", example.id), label: example.label.resolve(terminology, locale).to_string(), selected: active_example_id == Some(example.id.as_str()) })
        .collect()
}

//#endregion 🔀️ChromeParity
//#region 🪟️WindowPaneChrome
/// 🔀️ One selectable row of the Projection pane — the flattened Rust twin of React's
/// `createWorldProjectionTemplates` taxonomy (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3097`):
/// `Parallel > Orthographic | Axonometric (Isometric/Dimetric/Trimetric) | Oblique
/// (Cabinet/Cavalier/Military)` and `Perspective > 1-Point/2-Point/3-Point/Curvilinear`, in that
/// order, each carrying its own `projection-*` icon.
///
/// 🌍️ The labels are FROZEN English on both renderers: React's templates hard-code them
/// (`leaf("orthographic", "Orthographic", …)`) rather than resolving a `LocalizedLabel`, so
/// translating them here would make the two shells read differently for the same row.
#[derive(Clone, Copy)]
pub(crate) struct WorldProjectionTemplate {
    pub id: &'static str,
    pub label: &'static str,
    pub icon_id: &'static str,
    /// 🌲️ Depth in React's template tree — a branch is 0, its leaves are 1.
    pub depth: u8,
    pub mode: Viewport3dProjectionMode,
}

impl WorldProjectionTemplate {
    /// 📐️ React's complete template spec for a newly opened pane.
    pub(crate) fn spec(self) -> Viewport3dProjectionSpec {
        let orientation = match self.mode {
            Viewport3dProjectionMode::Orthographic {} => Viewport3dProjectionOrientation::Cardinal { view: Viewport3dOrthographicView::Plan },
            Viewport3dProjectionMode::Axonometric { .. } => Viewport3dProjectionOrientation::Corner { quadrant: Viewport3dAxonometricQuadrant::Ne, hemisphere: Some(Viewport3dAxonometricHemisphere::Upper) },
            Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Military, .. } => Viewport3dProjectionOrientation::Cardinal { view: Viewport3dOrthographicView::Plan },
            Viewport3dProjectionMode::Oblique { .. } | Viewport3dProjectionMode::OnePoint { .. } => Viewport3dProjectionOrientation::Cardinal { view: Viewport3dOrthographicView::Front },
            _ => Viewport3dProjectionOrientation::Free {},
        };
        Viewport3dProjectionSpec { mode: self.mode, orientation }
    }
}

/// 🔀️ React's projection taxonomy, depth-first, branch before its own leaves.
pub(crate) const WORLD_PROJECTION_TEMPLATES: &[WorldProjectionTemplate] = &[
    WorldProjectionTemplate { id: "parallel", label: "Parallel", icon_id: "projection-parallel", depth: 0, mode: Viewport3dProjectionMode::Orthographic {} },
    WorldProjectionTemplate { id: "orthographic", label: "Orthographic", icon_id: "projection-orthographic", depth: 1, mode: Viewport3dProjectionMode::Orthographic {} },
    WorldProjectionTemplate { id: "axonometric", label: "Axonometric", icon_id: "projection-axonometric", depth: 1, mode: Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Isometric, angle_a: 30.0, angle_b: 30.0 } },
    WorldProjectionTemplate { id: "axonometric-isometric", label: "Isometric", icon_id: "projection-isometric", depth: 2, mode: Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Isometric, angle_a: 15.0, angle_b: 30.0 } },
    WorldProjectionTemplate { id: "axonometric-dimetric", label: "Dimetric", icon_id: "projection-dimetric", depth: 2, mode: Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Dimetric, angle_a: 15.0, angle_b: 15.0 } },
    WorldProjectionTemplate { id: "axonometric-trimetric", label: "Trimetric", icon_id: "projection-trimetric", depth: 2, mode: Viewport3dProjectionMode::Axonometric { variant: Viewport3dAxonometricVariant::Trimetric, angle_a: 12.0, angle_b: 42.0 } },
    WorldProjectionTemplate { id: "oblique", label: "Oblique", icon_id: "projection-oblique", depth: 1, mode: Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cavalier, angle: 45.0, depth_scale: 1.0 } },
    WorldProjectionTemplate { id: "oblique-cabinet", label: "Cabinet", icon_id: "projection-oblique-cabinet", depth: 2, mode: Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cabinet, angle: 45.0, depth_scale: 0.5 } },
    WorldProjectionTemplate { id: "oblique-cavalier", label: "Cavalier", icon_id: "projection-oblique-cavalier", depth: 2, mode: Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Cavalier, angle: 45.0, depth_scale: 1.0 } },
    WorldProjectionTemplate { id: "oblique-military", label: "Military", icon_id: "projection-oblique-military", depth: 2, mode: Viewport3dProjectionMode::Oblique { variant: Viewport3dObliqueVariant::Military, angle: 45.0, depth_scale: 1.0 } },
    WorldProjectionTemplate { id: "perspective", label: "Perspective", icon_id: "projection-perspective", depth: 0, mode: Viewport3dProjectionMode::ThreePoint { fov: 50.0 } },
    WorldProjectionTemplate { id: "one-point", label: "1-Point", icon_id: "projection-one-point", depth: 1, mode: Viewport3dProjectionMode::OnePoint { fov: 50.0 } },
    WorldProjectionTemplate { id: "two-point", label: "2-Point", icon_id: "projection-two-point", depth: 1, mode: Viewport3dProjectionMode::TwoPoint { fov: 50.0, vertical_shift: 0.0 } },
    WorldProjectionTemplate { id: "three-point", label: "3-Point", icon_id: "projection-three-point", depth: 1, mode: Viewport3dProjectionMode::ThreePoint { fov: 50.0 } },
    WorldProjectionTemplate { id: "curvilinear", label: "Curvilinear", icon_id: "projection-curvilinear", depth: 1, mode: Viewport3dProjectionMode::Curvilinear { fov: 120.0, strength: 1.0, mapping: Viewport3dCurvilinearMapping::Fisheye } },
];

//#endregion 🪟️WindowPaneChrome
//#endregion ActionPanelAndUtilities

