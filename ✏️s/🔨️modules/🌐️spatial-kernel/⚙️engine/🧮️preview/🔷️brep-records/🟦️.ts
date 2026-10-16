/** 🔷️ Kernel brep records
 *
 * The kernel geometry record and reference aliases of `kernelGeometry`, the `geom` accessor that resolves the brep buckets of a `Model`, and the namespace re-export the preview consumers import.
 */

// #region 🔌️Adapters
import { kernelGeometry } from "@semio-tech/framework-3d-js";
import { Model } from "../../📐️geometry/🧊️model/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧱️kernelGeometry
export { kernelGeometry };

export type VertexRef = kernelGeometry.VertexRef;
export type VertexRecord = kernelGeometry.VertexRecord;
export type EdgeRef = kernelGeometry.EdgeRef;
export type WireRef = kernelGeometry.WireRef;
export type FaceRef = kernelGeometry.FaceRef;
export type ShellRef = kernelGeometry.ShellRef;
export type SolidRef = kernelGeometry.SolidRef;
export type AnchorAttachment = kernelGeometry.AnchorAttachment;
export type AnchorRecord = kernelGeometry.AnchorRecord;
export type EdgeRecord = kernelGeometry.EdgeRecord;
export type WireRecord = kernelGeometry.WireRecord;
export type FaceRecord = kernelGeometry.FaceRecord;
type ShellRecord = kernelGeometry.ShellRecord;
export type SolidPrimitive = kernelGeometry.SolidPrimitive;
export type SolidRecord = kernelGeometry.SolidRecord;
export type MutableSolidRecord = SolidRecord & { solid?: SolidPrimitive };

type KernelGeomBuckets = {
  anchors: Record<string, AnchorRecord>;
  vertices: Record<string, VertexRecord>;
  edges: Record<string, EdgeRecord>;
  wires: Record<string, WireRecord>;
  faces: Record<string, FaceRecord>;
  shells: Record<string, ShellRecord>;
  solids: Record<string, SolidRecord>;
};

type ModelWithGeom = Model & { readonly geometry?: KernelGeomBuckets };

/** 🧱️ Resolves kernel-private brep buckets (`model.geometry` on `Model`, else flat graph fields). */
export function geom(model: Model): KernelGeomBuckets {
  const g = (model as ModelWithGeom).geometry;
  if (g) return g;
  return model as unknown as KernelGeomBuckets;
}
// #endregion 🧱️kernelGeometry
