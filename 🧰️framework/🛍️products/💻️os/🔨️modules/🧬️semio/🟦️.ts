/** 📨 Identity coordinates of an OS semio container. */
export interface SemioEnvelope {
  plugin: string;
  artifact: string;
  component: "dsl" | "pack" | "op" | "spr" | "cmd";
  version: number;
}

/** 🪪️ Matches all envelope coordinates before a domain codec interprets its body. */
export function matchesSemioEnvelope(envelope: SemioEnvelope, envelopeId: string, component: SemioEnvelope["component"], version: number): boolean {
  return envelopeId === `${envelope.plugin}.${envelope.artifact}` && envelope.component === component && envelope.version === version;
}
