/** @emoji 🎯️ Canonical concept → catalog icon assignments — each distinct meaning maps to exactly one icon. */
import type { IconName } from "./📦️index.ts";

// #region FrameworkIconConcepts
/** @emoji 🧰️ Domain-neutral icon concepts owned by the framework itself (chrome, generic graph shapes,
 * projection kinds, generic tools, generic utilities) — none of these name a specific plugin/app. */
export const ICON_CONCEPT_ASSIGNMENTS = {
  "chrome.delete": "trash-2",
  "chrome.display-windows": "display-windows",
  "chrome.hud": "hud-overlay",
  "chrome.panel.catalogue": "panel-catalogue",
  "chrome.panel.document": "file-text",
  "chrome.panel.inspection": "panel-inspection",
  "chrome.panel.parameters": "panel-parameters",
  "chrome.scene": "scene-3d",
  "chrome.select-all": "select-all",
  "chrome.typography": "typography",
  "chrome.utility-bar": "wrench",
  "chrome.workbench": "workbench",
  "graph.architect": "architect-graph",
  "graph.dag": "graph-dag",
  "graph.flow": "flow-graph",
  "graph.math": "math-graph",
  "graph.media": "graph-media",
  "graph.network": "network",
  "projection.axonometric": "projection-axonometric",
  "projection.curvilinear": "projection-curvilinear",
  "projection.dimetric": "projection-dimetric",
  "projection.isometric": "projection-isometric",
  "projection.oblique": "projection-oblique",
  "projection.oblique.cabinet": "projection-oblique-cabinet",
  "projection.oblique.cavalier": "projection-oblique-cavalier",
  "projection.oblique.military": "projection-oblique-military",
  "projection.one-point": "projection-one-point",
  "projection.orthographic": "projection-orthographic",
  "projection.parallel": "projection-parallel",
  "projection.perspective": "projection-perspective",
  "projection.three-point": "projection-three-point",
  "projection.trimetric": "projection-trimetric",
  "projection.two-point": "projection-two-point",
  "tool.rectangle": "rectangle-tool",
  "tool.select": "mouse-pointer-2",
  "utility.note.math": "note-math",
  "utility.relocate-3d": "relocate-3d",
  "utility.transform-3d": "transform-3d",
  "utility.volume-brush": "volume-brush",
} as const satisfies Record<string, IconName>;

export type IconConceptId = keyof typeof ICON_CONCEPT_ASSIGNMENTS;
// #endregion FrameworkIconConcepts

/** @emoji 🔍️ Ensures no two distinct concepts share the same icon id. */
export function assertUniqueIconConceptAssignments(assignments: Record<string, IconName> = ICON_CONCEPT_ASSIGNMENTS): void {
  const iconToConcepts = new Map<IconName, string[]>();
  for (const [concept, icon] of Object.entries(assignments)) {
    const bucket = iconToConcepts.get(icon) ?? [];
    bucket.push(concept);
    iconToConcepts.set(icon, bucket);
  }
  const collisions = [...iconToConcepts.entries()].filter(([, concepts]) => concepts.length > 1);
  if (collisions.length > 0) {
    throw new Error(`Duplicate icon assignments: ${collisions.map(([icon, concepts]) => `${icon} ← ${concepts.join(", ")}`).join("; ")}`);
  }
}
