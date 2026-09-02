/** 🧬️ SemioPresentationArtifact — full artifact state, mirrors SemioPresentationSnapshot. */
import type { SlideMaster, SlideLayout, Slide } from "./📸️snapshot/🟦️";

export interface SemioPresentationArtifact {
  schema: string;
  masters: SlideMaster[];
  layouts: SlideLayout[];
  slides: Slide[];
}
