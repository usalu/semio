/** 🧬️ Puzzle3d snapshot schema — persistent fields only. */

export interface Puzzle3dSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  domain: string;
  /** @state persistent */
  meta: Puzzle3dMeta;
  /** @state persistent */
  objects: Puzzle3dObject[];
  /** @state persistent */
  attractions: Puzzle3dAttraction[];
  /** @state persistent */
  targetVolumes: Puzzle3dTargetVolume[];
  /** @state persistent */
  references: Puzzle3dReference[];
}

