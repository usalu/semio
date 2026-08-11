/** 🧬️ SemioMeshMutation schema. 🚧 scaffolded by W1b — generic facet mirror; the SemioMeshMutation
 * `🦀️component.rs` sibling is the real source of truth (matches existing repo convention). */
export interface SemioMeshMutationEntry {
  key: string;
  value: string;
}
export interface SemioMeshMutation {
  /** @state persistent */ schema: string;
  /** @state persistent */ entries: SemioMeshMutationEntry[];
}
