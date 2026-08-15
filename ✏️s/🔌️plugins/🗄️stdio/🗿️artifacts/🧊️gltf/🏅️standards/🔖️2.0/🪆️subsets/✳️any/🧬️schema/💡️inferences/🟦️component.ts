/** 💡 Universal glTF geometric inference schema. */
export * from './📦️size/🟦️component.ts';
export * from './🧱️area-volume/🟦️component.ts';
export * from './⚪️compactness/🟦️component.ts';
export * from './📏️proportion/🟦️component.ts';
export * from './⚖️mass-distribution/🟦️component.ts';
export * from './🌀️curvature/🟦️component.ts';
export * from './↕️thickness/🟦️component.ts';
export * from './🕳️concavity/🟦️component.ts';
export * from './↔️clearance/🟦️component.ts';
export * from './🔗️adjacency/🟦️component.ts';
export * from './🧭️orientation/🟦️component.ts';
export * from './🪞️symmetry/🟦️component.ts';
export * from './🌊️roughness/🟦️component.ts';
export * from './🕸️topology/🟦️component.ts';
export * from './🧮️geometric-analysis/🟦️component.ts';
import type { GltfGeometricInference } from './🧮️geometric-analysis/🟦️component.ts';
export interface GltfInference {
  /** @derived */
  geometricAnalysis: GltfGeometricInference;
}
