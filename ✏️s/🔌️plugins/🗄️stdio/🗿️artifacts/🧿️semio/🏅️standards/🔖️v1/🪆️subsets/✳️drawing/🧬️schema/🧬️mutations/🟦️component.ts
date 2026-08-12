/** 🧬️ SemioDrawingMutation — mirrors the real Rust 🧬️mutations/🦀️component.rs. SMO-approved
 * vocabulary (📌️important.md's binding ruling), tagged union on the `mutation` field. `NodePath`
 * addresses a scene-graph node (`DrawNode` carries no stable id): `layer` selects `layers[layer]`,
 * `path` is a chain of child indices from that layer's root (`path == []` = the root itself).
 *
 * ⚠️ Facet still partial — more SMO-approved verbs land one at a time (see the Rust sibling's own
 * doc comment). */
import type { DrawLayer, DrawNode, SemioPoint2 } from "../📸️snapshot/🟦️component";

export interface NodePath {
  layer: number;
  path: number[];
}

export type SemioDrawingMutation =
  | { mutation: "createLayer"; payload: { index: number; layer: DrawLayer } }
  | { mutation: "deleteLayer"; payload: { id: string } }
  | { mutation: "createNode"; payload: { parent: NodePath; index: number; node: DrawNode } }
  | { mutation: "deleteNode"; payload: { at: NodePath } }
  | { mutation: "moveNode"; payload: { at: NodePath; newOrigin: SemioPoint2 } }
  | { mutation: "dragNodes"; payload: { ats: NodePath[]; offset: SemioPoint2 } }
  | { mutation: "rotate"; payload: { at: NodePath; newRotation: { x: number; y: number; z: number; w: number } } }
  | { mutation: "scale"; payload: { at: NodePath; newScale: { x: number; y: number; z: number } } }
  | { mutation: "reorderNodes"; payload: { parent: NodePath; from: number; to: number } };
