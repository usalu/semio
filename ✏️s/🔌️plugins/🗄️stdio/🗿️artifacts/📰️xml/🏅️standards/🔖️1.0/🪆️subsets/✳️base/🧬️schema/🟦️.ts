/** 🧬️ XmlArtifact schema — full persisted state. */
import type { XmlDocument } from './📸️snapshot/🟦️.ts';

export interface XmlArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ doc: XmlDocument;
}
