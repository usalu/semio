/** 🧬️ SemioDrawingArtifact schema — mirrors `🦀️.rs`'s `SemioDrawingArtifact` (the real
 * source of truth). See `📸️snapshot/🟦️.ts` for the full `DrawCanvas`/`DrawStyle`/
 * `DrawLayer`/`DrawNode` shape re-exported here. */
import type { DrawCanvas, DrawLayer, DrawStyle } from "./📸️snapshot/🟦️";

export interface SemioDrawingArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ canvas: DrawCanvas;
  /** @state artifact */ styles: DrawStyle[];
  /** @state artifact */ layers: DrawLayer[];
}
