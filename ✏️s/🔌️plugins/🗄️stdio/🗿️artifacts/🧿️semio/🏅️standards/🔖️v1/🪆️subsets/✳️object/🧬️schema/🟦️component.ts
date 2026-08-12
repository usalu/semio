/** 🧬️ SemioObjectArtifact schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export interface SemioTransform {
  translation: { x: number; y: number; z: number };
  rotation: { x: number; y: number; z: number; w: number };
  scale: { x: number; y: number; z: number };
}
export interface ArtifactChildHandle { childId: string; target: string; }
export interface SemioObjectArtifact {
  /** @state persistent */ schema: string;
  /** @state persistent */ transform: SemioTransform;
  /** @state persistent @child kind=s.stdio.semio.brep */ brep?: ArtifactChildHandle;
  /** @state persistent @child kind=s.stdio.semio.mesh */ mesh?: ArtifactChildHandle;
  /** @state persistent @child kind=s.stdio.semio.value */ properties?: ArtifactChildHandle;
}
