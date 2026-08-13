/** 🧬️ SemioDrawingArtifact schema — mirrors `🦀️component.rs`'s `SemioDrawingArtifact` (the real
 * source of truth). See `📸️snapshot/🟦️component.ts` for the full `DrawCanvas`/`DrawStyle`/
 * `DrawLayer`/`DrawNode` shape re-exported here. */
import type { DrawCanvas, DrawLayer, DrawStyle } from "./📸️snapshot/🟦️component";

export interface SemioDrawingArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ canvas: DrawCanvas;
  /** @state artifact */ styles: DrawStyle[];
  /** @state artifact */ layers: DrawLayer[];
}
