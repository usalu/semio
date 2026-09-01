/** 🧬️ SemioObjectArtifact schema — real facet mirror of the Rust `🦀️component.rs` sibling. */
export interface SemioTransform {
  translation: { x: number; y: number; z: number };
  rotation: { x: number; y: number; z: number; w: number };
  scale: { x: number; y: number; z: number };
}
export interface ArtifactDialect {
  artifactKind: string;
  standard: string;
  subset: string;
}

export interface ArtifactRef {
  artifactId: string;
  dialect: ArtifactDialect;
}
/** 🌉️ Mirrors `store::ArtifactChild<S>` — `childId`/`target` only; `local_owner` and
 *  `PhantomData<S>` are `#[serde(skip)]`. */
export interface ArtifactChildHandle {
  childId: string;
  target: ArtifactRef;
}
export interface SemioObjectArtifact {
  /** @state artifact */ schema: string;
  /** @state artifact */ transform: SemioTransform;
  /** @state artifact @child kind=s.stdio.semio.brep */ brep?: ArtifactChildHandle;
  /** @state artifact @child kind=s.stdio.semio.mesh */ mesh?: ArtifactChildHandle;
  /** @state artifact @child kind=s.stdio.semio.value */ properties?: ArtifactChildHandle;
}
