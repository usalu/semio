/** 🏷️ World projection templates: the template descriptors and tree, the template id codec, and the labels and icon ids of the projection specs. */
import { type IconName } from "@semio-tech/ui-react";
import { type WorldAxonometricVariant, type WorldObliqueVariant, type WorldOrthographicViewId, type WorldProjectionMode, type WorldProjectionOrientation, type WorldProjectionSpec } from "../../🔭️scene-types/🟦️.ts";
import { WORLD_PROJECTION_COMMAND, worldProjectionDefaults } from "../🧮️projection-pose/🟦️.ts";

// #region 📐️WorldProjection
export interface WorldProjectionTemplateDescriptor {
  readonly id: string;
  readonly label: string;
  readonly iconId: IconName;
  readonly controllerId: string;
  readonly command: string;
  readonly args: { readonly spec: WorldProjectionSpec };
  readonly children?: readonly WorldProjectionTemplateDescriptor[];
}

export interface CreateWorldProjectionTemplatesConfig {
  readonly controllerId: string;
  readonly command?: string;
}

function worldProjectionTemplateLeaf(controllerId: string, command: string, id: string, label: string, iconId: IconName, spec: WorldProjectionSpec): WorldProjectionTemplateDescriptor {
  return { id, label, iconId, controllerId, command, args: { spec } };
}

function worldProjectionTemplateBranch(controllerId: string, command: string, id: string, label: string, iconId: IconName, spec: WorldProjectionSpec, children: readonly WorldProjectionTemplateDescriptor[]): WorldProjectionTemplateDescriptor {
  return { id, label, iconId, controllerId, command, args: { spec }, children };
}

/** 🪟️ Builds the projection-mode taxonomy tree (no Top/Front — those are gizmo orientations):
 * `Parallel > Orthographic | Axonometric (Isometric/Dimetric/Trimetric) | Oblique (Cabinet/Cavalier/Military)`
 * and `Perspective > 1-Point/2-Point/3-Point/Curvilinear`. */
export function createWorldProjectionTemplates(config: CreateWorldProjectionTemplatesConfig): readonly WorldProjectionTemplateDescriptor[] {
  const { controllerId } = config;
  const command = config.command ?? WORLD_PROJECTION_COMMAND;
  const leaf = (id: string, label: string, iconId: IconName, spec: WorldProjectionSpec) => worldProjectionTemplateLeaf(controllerId, command, id, label, iconId, spec);
  const branch = (id: string, label: string, iconId: IconName, spec: WorldProjectionSpec, children: readonly WorldProjectionTemplateDescriptor[]) => worldProjectionTemplateBranch(controllerId, command, id, label, iconId, spec, children);

  const orthographic = leaf("orthographic", "Orthographic", "projection-orthographic", worldProjectionDefaults("orthographic"));

  const axonometricVariants: readonly [WorldAxonometricVariant, string, IconName][] = [
    ["isometric", "Isometric", "projection-isometric"],
    ["dimetric", "Dimetric", "projection-dimetric"],
    ["trimetric", "Trimetric", "projection-trimetric"],
  ];
  const axonometric = branch(
    "axonometric",
    "Axonometric",
    "projection-axonometric",
    worldProjectionDefaults("axonometric"),
    axonometricVariants.map(([variant, label, iconId]) =>
      leaf(`axonometric-${variant}`, label, iconId, {
        mode: { kind: "axonometric", variant, angleA: variant === "trimetric" ? 12 : 15, angleB: variant === "trimetric" ? 42 : variant === "isometric" ? 30 : 15 },
        orientation: { type: "corner", quadrant: "ne", hemisphere: "upper" },
      }),
    ),
  );

  const obliqueVariants: readonly [WorldObliqueVariant, string, IconName][] = [
    ["cabinet", "Cabinet", "projection-oblique-cabinet"],
    ["cavalier", "Cavalier", "projection-oblique-cavalier"],
    ["military", "Military", "projection-oblique-military"],
  ];
  const oblique = branch(
    "oblique",
    "Oblique",
    "projection-oblique",
    worldProjectionDefaults("oblique"),
    obliqueVariants.map(([variant, label, iconId]) =>
      leaf(`oblique-${variant}`, label, iconId, {
        mode: { kind: "oblique", variant, angle: 45, depthScale: variant === "cabinet" ? 0.5 : 1 },
        orientation: { type: "cardinal", view: variant === "military" ? "plan" : "front" },
      }),
    ),
  );

  const parallel = branch("parallel", "Parallel", "projection-parallel", worldProjectionDefaults("orthographic"), [orthographic, axonometric, oblique]);

  const perspective = branch("perspective", "Perspective", "projection-perspective", worldProjectionDefaults("threePoint"), [
    leaf("one-point", "1-Point", "projection-one-point", worldProjectionDefaults("onePoint")),
    leaf("two-point", "2-Point", "projection-two-point", worldProjectionDefaults("twoPoint")),
    leaf("three-point", "3-Point", "projection-three-point", worldProjectionDefaults("threePoint")),
    leaf("curvilinear", "Curvilinear", "projection-curvilinear", worldProjectionDefaults("curvilinear")),
  ]);

  return [parallel, perspective];
}

const WORLD_PROJECTION_TEMPLATE_PREFIX = "world-projection:";

/** 🪟️ Encodes a {@link WorldProjectionSpec} into a `WindowTemplateDropPayload.templateId` string,
 * for the Display "Windows" drag palette to seed a freshly-opened pane's initial camera. */
export function encodeWorldProjectionTemplateId(spec: WorldProjectionSpec): string {
  return `${WORLD_PROJECTION_TEMPLATE_PREFIX}${JSON.stringify(spec)}`;
}

/** 🪟️ Inverse of {@link encodeWorldProjectionTemplateId}; `null` for anything else (including `undefined`). */
export function decodeWorldProjectionTemplateId(templateId: string | undefined): WorldProjectionSpec | null {
  if (!templateId || !templateId.startsWith(WORLD_PROJECTION_TEMPLATE_PREFIX)) {
    return null;
  }
  try {
    const parsed = JSON.parse(templateId.slice(WORLD_PROJECTION_TEMPLATE_PREFIX.length)) as { readonly mode?: { readonly kind?: unknown }; readonly orientation?: unknown };
    return typeof parsed.mode?.kind === "string" && parsed.orientation ? (parsed as WorldProjectionSpec) : null;
  } catch {
    return null;
  }
}

const WORLD_ORTHOGRAPHIC_VIEW_LABELS: Record<WorldOrthographicViewId, string> = { plan: "Plan", top: "Top", bottom: "Bottom", front: "Front", back: "Back", left: "Left", right: "Right" };
const WORLD_AXONOMETRIC_VARIANT_LABELS: Record<WorldAxonometricVariant, string> = { isometric: "Isometric", dimetric: "Dimetric", trimetric: "Trimetric" };
const WORLD_OBLIQUE_VARIANT_LABELS: Record<WorldObliqueVariant, string> = { cabinet: "Cabinet", cavalier: "Cavalier", military: "Military" };

/** 🧭️ Short label for a gizmo orientation. */
export function worldProjectionOrientationLabel(orientation: WorldProjectionOrientation): string {
  switch (orientation.type) {
    case "cardinal":
      return WORLD_ORTHOGRAPHIC_VIEW_LABELS[orientation.view];
    case "corner": {
      const q = orientation.quadrant.toUpperCase();
      return (orientation.hemisphere ?? "upper") === "lower" ? `${q}↓` : q;
    }
    case "free":
      return "3D";
  }
}

/** 📐️ Mode label matching {@link createWorldProjectionTemplates} leaves. */
export function worldProjectionModeLabel(mode: WorldProjectionMode): string {
  switch (mode.kind) {
    case "orthographic":
      return "Orthographic";
    case "axonometric":
      return WORLD_AXONOMETRIC_VARIANT_LABELS[mode.variant];
    case "oblique":
      return WORLD_OBLIQUE_VARIANT_LABELS[mode.variant];
    case "onePoint":
      return "1-Point";
    case "twoPoint":
      return "2-Point";
    case "threePoint":
      return "3-Point";
    case "curvilinear":
      return "Curvilinear";
  }
}

/** 🪟️ Window title for a live projection — matches {@link createWorldProjectionTemplates} mode labels. */
export function worldProjectionSpecLabel(spec: WorldProjectionSpec): string {
  return worldProjectionModeLabel(spec.mode);
}

/** 🖼️ Catalog icon for a live projection — matches {@link createWorldProjectionTemplates} icons. */
export function worldProjectionSpecIconId(spec: WorldProjectionSpec): string {
  switch (spec.mode.kind) {
    case "orthographic":
      return "projection-orthographic";
    case "axonometric":
      switch (spec.mode.variant) {
        case "isometric":
          return "projection-isometric";
        case "dimetric":
          return "projection-dimetric";
        case "trimetric":
          return "projection-trimetric";
      }
      break;
    case "oblique":
      switch (spec.mode.variant) {
        case "cabinet":
          return "projection-oblique-cabinet";
        case "cavalier":
          return "projection-oblique-cavalier";
        case "military":
          return "projection-oblique-military";
      }
      break;
    case "onePoint":
      return "projection-one-point";
    case "twoPoint":
      return "projection-two-point";
    case "threePoint":
      return "projection-three-point";
    case "curvilinear":
      return "projection-curvilinear";
  }
}
// #endregion 📐️WorldProjection
