/** 🧬️ SemioObjectSnapshot schema — real facet mirror of the Rust `🦀️component.rs` sibling.
 * FIRST COMPOSITE subset: `brep`/`mesh`/`properties` are CHILD HANDLES (two strings), never
 * embedded content. */
export interface SemioTransform {
  translation: { x: number; y: number; z: number };
  rotation: { x: number; y: number; z: number; w: number };
  scale: { x: number; y: number; z: number };
}

export interface ArtifactChildHandle {
  childId: string;
  /** target ArtifactRef, flattened to its canonical URI ("<artifact_id>!<kind>@<standard>/<subset>") */
  target: string;
}

export interface SemioObjectSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ transform: SemioTransform;
  /** @state artifact @child kind=s.stdio.semio.brep */ brep?: ArtifactChildHandle;
  /** @state artifact @child kind=s.stdio.semio.mesh */ mesh?: ArtifactChildHandle;
  /** @state artifact @child kind=s.stdio.semio.value */ properties?: ArtifactChildHandle;
}
