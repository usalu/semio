/** 🧬️ Playbook diff schema — sparse field delta over the artifact. */

export interface PlaybookDiff {
  /** @state artifact */
  artifact?: PlaybookArtifact;
  /** @state artifact */
  schema?: string;
  /** @state artifact */
  id?: string;
  /** @state artifact */
  version?: string;
  /** @state artifact */
  title?: string | null;
  /** @state artifact */
  document?: ArtifactChildHandle;
  /** @state artifact */
  flow?: ArtifactChildHandle;
  /** @state presence */
  selectedIds?: PlaybookStringList;
  /** @state config */
  locale?: string;
  /** @state config */
  contributionsJson?: string;
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

export interface PlaybookStringList {
  values: string[];
}

export interface PlaybookArtifact { [key: string]: unknown; }
