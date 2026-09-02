/** 🧬️ StlMutation union — discriminated on `mutation`, mirroring the Rust `StlMutation` enum. */

import type { StlSnapshot, StlTriangle } from '../📸️snapshot/🟦️.ts';

export type StlMutation =
  | { mutation: 'setSnapshot'; snapshot: StlSnapshot }
  | { mutation: 'setSolidName'; name: string }
  | { mutation: 'insertTriangle'; index: number; triangle: StlTriangle }
  | { mutation: 'removeTriangle'; index: number }
  | { mutation: 'setTriangleNormal'; index: number; normal: [number, number, number] }
  | { mutation: 'setTriangleVertices'; index: number; vertices: [[number, number, number], [number, number, number], [number, number, number]] };
