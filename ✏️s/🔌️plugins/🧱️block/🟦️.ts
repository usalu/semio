/** 🤝️ Block plugin — record types shared by all three artifacts' document entities, mirroring
 * `🦀️.rs`. Dimension-specific nouns stay per-artifact — only the identity/metadata/
 * compatibility/representation/camera shapes common to every dimension live here.
 */

/** 🪪️ The single kind definition a block document edits. */
export interface BlockKindIdentity {
  id: string;
  name: string;
  label: string;
  variant?: string;
  description: string;
  icon?: string;
  unit?: string;
}

/** 🏷️ One free-form key/value attribute on a kind. */
export interface BlockAttribute {
  key: string;
  value: string;
  definition?: string;
}

/** 👤️ One author credited on a kind. */
export interface BlockAuthor {
  id: string;
  name: string;
  email?: string;
}

/** 🔗️ One allowed compatibility pair between two handle/vortex/grip kind ids. */
export interface BlockCompatibilityRule {
  id: string;
  source: string;
  target: string;
  bidirectional: boolean;
}

/** 🧱️ One representation (mesh at a LOD/tag combination) a kind ships with. */
export interface BlockRepresentation {
  id: string;
  name: string;
  meshUrl?: string;
  tags: string[];
  lod?: string;
  description: string;
  attributes: BlockAttribute[];
}

export interface BlockCamera2d {
  x: number;
  y: number;
  zoom: number;
}

export interface BlockCamera3d {
  position: [number, number, number];
  target: [number, number, number];
  zoom: number;
}

/** 📝️ Free-text description carried alongside a block document. */
export interface BlockMeta {
  description: string;
}
