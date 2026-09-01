/** 🧬️ XmlArtifact schema — full persisted state. */
import type { XmlDocument } from './📸️snapshot/🟦️component.ts';

export interface XmlArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ doc: XmlDocument;
}
