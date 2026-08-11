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

// #region PluginDomainIconConcepts
/** @emoji 🔌️ TODO(follow-up): should be plugin-declared metadata (each plugin's own manifest/catalog
 * contributing its own `app.<id>`/`window.<id>.<sub>` icon), not hardcoded here — the framework should not
 * need to know the full roster of installed apps/windows to assign their icons. Isolated here, separate
 * from {@link ICON_CONCEPT_ASSIGNMENTS}, so the layering violation is visible rather than mixed into
 * genuinely generic framework data. See ticket `CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`. */
export const PLUGIN_DOMAIN_ICON_CONCEPTS = {
  "app.animate": "animate",
  "app.architect": "architect",
  "app.cad": "box",
  "app.draw": "draw",
  "app.fem": "fem-app",
  "app.flow": "flow",
  "app.forms": "forms",
  "app.gis2d": "gis2d",
  "app.gis3d": "gis3d",
  "app.imperative": "imperative",
  "app.layout": "layout",
  "app.lowpoly": "shapes",
  "app.mathematical": "math-app",
  "app.note": "note",
  "app.process": "hammer",
  "app.procedural2d": "procedural2d",
  "app.procedural3d": "workflow",
  "app.puzzle": "puzzle",
  "app.reasoning": "reasoning-wires",
  "app.remodel": "remodel-app",
  "app.sequence": "sequence",
  "app.shooting": "camera",
  "app.sourcing": "library",
  "app.s": "s",
  "app.trinity": "trinity",
  "app.trinity-rewrite": "trinity-rewrite",
  "app.vcs": "git-branch",
  "app.writer": "writer",
  "app.raster": "raster",
  "app.dag": "dag",
  "window.cad.energy": "sun",
  "window.cad.shape": "cad-shape",
  "window.cad.structure": "component",
  "window.document.jack": "document-jack",
  "window.document.report": "document-report",
  "window.fem.model": "fem-model",
  "window.fem.results": "bar-chart-3",
  "window.gis.terrain": "terrain-3d",
  "window.preview": "preview",
  "window.lowpoly.model": "lowpoly-model",
  "window.measure.lod-depth": "lod-depth",
  "window.process.workpiece": "process-workpiece",
  "window.puzzle5d.3d": "puzzle5d-3d",
  "window.remodel.model": "remodel-model",
  "window.shooting.scene": "shooting-scene",
  "window.trinity.lhs": "trinity-lhs",
  "window.trinity.rhs": "trinity-rhs",
} as const satisfies Record<string, IconName>;

export type PluginDomainIconConceptId = keyof typeof PLUGIN_DOMAIN_ICON_CONCEPTS;
// #endregion PluginDomainIconConcepts

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
