/** 🧬️ SemioDrawingArtifact schema — mirrors `🦀️component.rs`'s `SemioDrawingArtifact` (the real
 * source of truth). See `📸️snapshot/🟦️component.ts` for the full `DrawCanvas`/`DrawStyle`/
 * `DrawLayer`/`DrawNode` shape re-exported here. */
import type { DrawCanvas, DrawLayer, DrawStyle } from "./📸️snapshot/🟦️component";

export interface SemioDrawingArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ canvas: DrawCanvas;
  /** @state persistent */ styles: DrawStyle[];
  /** @state persistent */ layers: DrawLayer[];
}
