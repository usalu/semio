/** 📐️ World projections: the mode options, the template selection ids and the kind and orbit projection switches. The poses, matrices, gizmo specs, rig and templates live in the sibling folders.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | r3f}, which re-exports every public name of the projection folders so import specifiers stay unchanged.
 */

import { childElementId, cn, floatingRibbonSurfaceClass, LevelProvider, reactHostPort, Tree, Icon, type TreeDataItem } from "@semio-tech/ui-react";
import { type ReactElement } from "react";
import { type WorldAxonometricVariant, type WorldObliqueVariant, type WorldProjectionKind, type WorldProjectionSpec } from "../🔭️scene-types/🟦️.ts";
import { type OrbitCameraProjection } from "../🧷️r3f-foundation/🟦️.ts";
import { type OrbitCameraViewId } from "../📷️camera-view/🟦️.tsx";
import { resolveProjectionGizmoHitFromDirection, resolveProjectionGizmoSpec, worldProjectionSpecToOrbitView } from "./🧭️projection-gizmo-specs/🟦️.ts";
import { createWorldProjectionTemplates, type WorldProjectionTemplateDescriptor } from "./🏷️projection-templates/🟦️.ts";

// #region 📷️OrbitCameraView
/** 🧭️ Maps a CAD viewport gizmo axis click to a named orbit view (Z-up). */
export function resolveOrbitGizmoViewFromDirection(direction: { readonly x: number; readonly y: number; readonly z: number }): OrbitCameraViewId {
  const spec = resolveProjectionGizmoSpec(resolveProjectionGizmoHitFromDirection(direction));
  return worldProjectionSpecToOrbitView(spec) ?? "top";
}

export const WORLD_PROJECTION_KINDS: readonly WorldProjectionKind[] = ["orthographic", "axonometric", "oblique", "onePoint", "twoPoint", "threePoint", "curvilinear"];

export interface WorldProjectionModeOption {
  readonly id: string;
  readonly label: string;
  readonly spec: WorldProjectionSpec;
  readonly active: boolean;
}

/** 🔀️ Non-spatial mode variants that sit beside the template tree (kept for callers that still want a flat list).
 * Prefer {@link createWorldProjectionTemplates} / {@link WorldProjectionKindSwitch} for the canonical taxonomy. */
export function worldProjectionModeOptions(spec: WorldProjectionSpec): readonly WorldProjectionModeOption[] {
  const mode = spec.mode;
  switch (mode.kind) {
    case "axonometric": {
      const variants: readonly { readonly variant: WorldAxonometricVariant; readonly label: string; readonly angleA: number; readonly angleB: number }[] = [
        { variant: "isometric", label: "Iso", angleA: 30, angleB: 30 },
        { variant: "dimetric", label: "Di", angleA: 15, angleB: 15 },
        { variant: "trimetric", label: "Tri", angleA: 12, angleB: 42 },
      ];
      return variants.map((row) => ({
        id: `axonometric-${row.variant}`,
        label: row.label,
        spec: { mode: { kind: "axonometric", variant: row.variant, angleA: row.angleA, angleB: row.angleB }, orientation: spec.orientation },
        active: mode.variant === row.variant,
      }));
    }
    case "oblique": {
      const variants: readonly { readonly variant: WorldObliqueVariant; readonly label: string; readonly depthScale: number }[] = [
        { variant: "cabinet", label: "Cab", depthScale: 0.5 },
        { variant: "cavalier", label: "Cav", depthScale: 1 },
        { variant: "military", label: "Mil", depthScale: 1 },
      ];
      return variants.map((row) => ({
        id: `oblique-${row.variant}`,
        label: row.label,
        spec: { mode: { kind: "oblique", variant: row.variant, angle: mode.angle, depthScale: row.depthScale }, orientation: spec.orientation },
        active: mode.variant === row.variant,
      }));
    }
    case "curvilinear":
      return (["fisheye", "panini"] as const).map((mapping) => ({
        id: `curvilinear-${mapping}`,
        label: mapping === "fisheye" ? "Fish" : "Pan",
        spec: { mode: { kind: "curvilinear", fov: mode.fov, strength: mode.strength, mapping }, orientation: spec.orientation },
        active: mode.mapping === mapping,
      }));
    default:
      return [];
  }
}

/** 🌲️ Template-tree node id for the active *mode* (orientation is gizmo-owned and does not change selection). */
export function worldProjectionTemplateSelectionId(spec: WorldProjectionSpec): string {
  switch (spec.mode.kind) {
    case "orthographic":
      return "orthographic";
    case "axonometric":
      return `axonometric-${spec.mode.variant}`;
    case "oblique":
      return `oblique-${spec.mode.variant}`;
    case "onePoint":
      return "one-point";
    case "twoPoint":
      return "two-point";
    case "threePoint":
      return "three-point";
    case "curvilinear":
      return "curvilinear";
  }
}

/** 🌲️ Applies a template-tree *mode* while preserving the current gizmo orientation. */
export function worldProjectionTemplateApplySpec(templateSpec: WorldProjectionSpec, currentSpec?: WorldProjectionSpec): WorldProjectionSpec {
  return {
    mode: templateSpec.mode,
    orientation: currentSpec?.orientation ?? templateSpec.orientation,
  };
}

/** 🌲️ Maps {@link createWorldProjectionTemplates} into selectable {@link TreeDataItem}s for the live projection pane.
 * `id` is the OWNING pane's element id: `TreeSection`/`TreeItem` render their item id as the row's DOM `id`, and one
 * world surface is mounted per open window instance, so a bare `template.id` puts `orthographic` (and every sibling)
 * in the document once per pane. Every row id is therefore a {@link childElementId} of the pane. */
export function worldProjectionSwitchTreeItems(id: string, templates: readonly WorldProjectionTemplateDescriptor[], onSelect: (spec: WorldProjectionSpec) => void, currentSpec?: WorldProjectionSpec): TreeDataItem[] {
  return templates.map((template) => ({
    id: childElementId(id, template.id),
    label: template.label,
    icon: <Icon icon={template.iconId} size={12} className="size-tiny shrink-0" />,
    defaultOpen: true,
    onClick: () => onSelect(worldProjectionTemplateApplySpec(template.args.spec, currentSpec)),
    ...(template.children?.length ? { items: worldProjectionSwitchTreeItems(id, template.children, onSelect, currentSpec) } : {}),
  }));
}

export interface WorldProjectionKindSwitchProps {
  /** 🪪️ Element id of the pane this switch is mounted in — qualifies every row id per window instance. */
  readonly id: string;
  readonly spec: WorldProjectionSpec;
  readonly onSpecChange: (spec: WorldProjectionSpec) => void;
  readonly className?: string;
  /** 🪟️ `"pane"` (default) renders a transparent tree inside {@link Pane} body — same silhouette as window options. `"ribbon"` keeps {@link floatingRibbonSurfaceClass} for standalone window-level overlays. */
  readonly surface?: "pane" | "ribbon";
}

/** 🌲️ Projection-mode switcher — same Parallel/Perspective taxonomy as {@link createWorldProjectionTemplates}; cube owns spatial angles. */
export function WorldProjectionKindSwitch(props: WorldProjectionKindSwitchProps): ReactElement {
  const surface = props.surface ?? "pane";
  const shellClass =
    props.className ??
    (surface === "ribbon"
      ? cn("pointer-events-auto min-w-40 text-2xs font-medium", floatingRibbonSurfaceClass)
      : cn("pointer-events-auto w-full min-w-0 text-tiny font-medium"));
  const templates = reactHostPort.useMemo(() => createWorldProjectionTemplates({ controllerId: "projection-switch" }), []);
  const items = reactHostPort.useMemo(() => worldProjectionSwitchTreeItems(props.id, templates, props.onSpecChange, props.spec), [props.id, templates, props.onSpecChange, props.spec]);
  const selectedId = childElementId(props.id, worldProjectionTemplateSelectionId(props.spec));
  const tree = <Tree showLines={surface === "pane"} sortableSections={false} selectionMode="single" selectedIds={[selectedId]} sections={[{ id: childElementId(props.id, "projection-modes"), items }]} />;
  return (
    <div className={shellClass} data-world-projection-kind-switch {...(surface === "ribbon" ? { "data-level": "window" as const } : {})}>
      {surface === "ribbon" ? <LevelProvider level="window">{tree}</LevelProvider> : tree}
    </div>
  );
}

export interface WorldOrbitProjectionSwitchProps {
  /** 🪪️ Element id of the pane this switch is mounted in — see {@link WorldProjectionKindSwitchProps.id}. */
  readonly id: string;
  readonly projection: OrbitCameraProjection;
  readonly onProjectionChange: (projection: OrbitCameraProjection) => void;
  readonly className?: string;
  readonly spec?: WorldProjectionSpec;
  readonly onSpecChange?: (spec: WorldProjectionSpec) => void;
}

/** 🔀️ Orthographic / perspective toggle, or full {@link WorldProjectionKindSwitch} when `spec` is provided. */
export function WorldOrbitProjectionSwitch(props: WorldOrbitProjectionSwitchProps): ReactElement {
  if (props.spec && props.onSpecChange) {
    return <WorldProjectionKindSwitch id={props.id} spec={props.spec} onSpecChange={props.onSpecChange} className={props.className} />;
  }
  const shellClass = props.className ?? cn("pointer-events-auto flex text-2xs font-medium", floatingRibbonSurfaceClass);
  const buttonClass = (active: boolean) => cn("px-2 py-1 transition-colors text-muted-foreground hover:text-emphasized", active && "text-emphasized");
  return (
    <div className={shellClass} data-world-projection-switch data-level="window">
      <button type="button" className={buttonClass(props.projection === "orthographic")} aria-pressed={props.projection === "orthographic"} onClick={() => props.onProjectionChange("orthographic")}>
        Ortho
      </button>
      <button type="button" className={buttonClass(props.projection === "perspective")} aria-pressed={props.projection === "perspective"} onClick={() => props.onProjectionChange("perspective")}>
        Persp
      </button>
    </div>
  );
}
// #endregion 📷️OrbitCameraView
