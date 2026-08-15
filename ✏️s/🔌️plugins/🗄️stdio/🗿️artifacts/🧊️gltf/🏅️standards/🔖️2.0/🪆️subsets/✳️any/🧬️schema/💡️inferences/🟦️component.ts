/** 💡 Universal glTF geometric inference schema. */
export * from './📦bounds/🟦️component.ts';
import type { GltfGeometricInference } from './📦bounds/🟦️component.ts';
export interface GltfInference {
  /** @derived */
  geometry: GltfGeometricInference;
}
