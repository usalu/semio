/** 🧬️ Puzzle5d snapshot schema — persistent fields only. */

export interface Puzzle5dSnapshot {
  /** @state persistent */
  schema: string;
  /** @state persistent */
  domain: string;
  /** @state persistent */
  label?: string;
  /** @state persistent */
  meta: Puzzle5dMeta;
  /** @state persistent */
  kindCatalogs?: Puzzle5dKindCatalogs;
  /** @state persistent */
  kindCompatibility: Puzzle5dKindCompatibility[];
  /** @state persistent */
  parts: Puzzle5dPart[];
  /** @state persistent */
  fasteners: Puzzle5dFastener[];
}

