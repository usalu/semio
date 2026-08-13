/** 🧬️ SemioKitArtifact schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export interface ArtifactChildHandle { childId: string; target: string; }
export interface ArtifactLinkRef { target: string; pin: unknown; role: string; }
export interface SemioKitType { id: string; name: string; category: string; }
export interface SemioKitDesign { id: string; name: string; pieces: unknown[]; connections: unknown[]; }
export interface SemioKitArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ types: SemioKitType[];
  /** @state artifact */ designs: SemioKitDesign[];
  /** @state artifact @child kind=s.stdio.semio.object many */ objects: ArtifactChildHandle[];
  /** @state artifact @child kind=s.stdio.semio.model many */ models: ArtifactChildHandle[];
  /** @state artifact @child kind=s.stdio.semio.value */ properties?: ArtifactChildHandle;
  /** @state artifact @link_slot roles=representation many */ representations: ArtifactLinkRef[];
}
