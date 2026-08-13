/** 🧬️ SemioKitSnapshot schema — real facet mirror of the Rust `🦀️component.rs` sibling. SECOND
 * COMPOSITE subset: `objects`/`models`/`properties` are owned CHILD handles; `representations` is
 * a LINK pool (joined to a type by `role === type.id`). */
export interface ArtifactChildHandle { childId: string; target: string; }
export interface ArtifactLinkRef { target: string; pin: { kind: "head" } | { kind: "checkpoint"; id: string } | { kind: "snapshot"; hash: string; size: number; mediaType: string }; role: string; }
export interface SemioTransform {
  translation: { x: number; y: number; z: number };
  rotation: { x: number; y: number; z: number; w: number };
  scale: { x: number; y: number; z: number };
}
export interface SemioKitType { id: string; name: string; category: string; }
export interface SemioKitPiece { id: string; typeId: string; transform: SemioTransform; }
export interface SemioKitConnection { id: string; connectingPieceId: string; connectingPort: string; connectedPieceId: string; connectedPort: string; }
export interface SemioKitDesign { id: string; name: string; pieces: SemioKitPiece[]; connections: SemioKitConnection[]; }

export interface SemioKitSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ types: SemioKitType[];
  /** @state artifact */ designs: SemioKitDesign[];
  /** @state artifact @child kind=s.stdio.semio.object many */ objects: ArtifactChildHandle[];
  /** @state artifact @child kind=s.stdio.semio.model many */ models: ArtifactChildHandle[];
  /** @state artifact @child kind=s.stdio.semio.value */ properties?: ArtifactChildHandle;
  /** @state artifact @link_slot roles=representation many */ representations: ArtifactLinkRef[];
}
