/** 💡️ SemioBrepInference facet mirror — real facet mirror of the Rust `🦀️.rs` sibling. */
import { SemioPoint3 } from "../📸️snapshot/🟦️.ts";

export interface Vec3 { x: number; y: number; z: number; }
export interface BrepFaceGroup { start: number; count: number; entityId: string; }
export interface BrepEdgeGroup { start: number; count: number; entityId: string; }
export type BrepSurfaceKind = "plane" | "cylinder" | "cone" | "sphere" | "torus" | "nurbs";
export type BrepCurveKind = "line" | "circle" | "ellipse" | "nurbs";
export interface BrepFaceInfo { entityId: string; surfaceKind: BrepSurfaceKind; area: number; normal: Vec3; }
export interface BrepEdgeInfo { entityId: string; curveKind: BrepCurveKind; length: number; }
export interface BrepMeshTransfer {
  position: number[];
  normal: number[];
  index: number[];
  edges: number[];
  points: number[];
  faceGroups: BrepFaceGroup[];
  edgeGroups: BrepEdgeGroup[];
  faceInfos: BrepFaceInfo[];
  edgeInfos: BrepEdgeInfo[];
}
export interface BrepMassProperties { volume: number; area: number; centroid: SemioPoint3; errorEstimate: number; }

export interface SemioBrepInference {
  validationReport: import("./✅validation-report/🟦️.ts").BrepValidationDiagnostic[];
  tessellation: BrepMeshTransfer;
  massProperties: BrepMassProperties;
}
