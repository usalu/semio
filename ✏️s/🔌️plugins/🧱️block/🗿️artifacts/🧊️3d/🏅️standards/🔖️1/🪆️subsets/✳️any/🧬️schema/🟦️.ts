/** 🧬️ Block3d artifact schema — every field with its state class. */

import type { BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation } from "../../../../../../../🟦️";
import type { Block3dBrushPreview, Block3dVortexKindExtra, Block3dVortexTemplate, Block3dWindowView } from "../../../../../🟦️";

/** 🗂️ Dialect coordinate a child artifact is claimed against. */
export interface ArtifactDialect { artifactKind: string; standard: string; subset: string; }

/** 🎯️ What a child handle points at — verified against the real fixture
 * `…/🙅remove-author/🧪️tests/uncredits-ada/📸️snapshot/⬅️before/🔣️.json`'s `catalog.target`
 * (NOT a plain string, unlike some sibling plugins' unverified `ArtifactChildHandle` stubs). */
export interface ArtifactChildTarget { artifactId: string; dialect: ArtifactDialect; }

/** 🧒️ `store::ArtifactChild<T>` wire handle — child artifact id plus its kind claim. */
export interface ArtifactChildHandle { childId: string; target: ArtifactChildTarget; }

export interface Block3dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objectKind: BlockKindIdentity;
  /** @state artifact */
  representations: BlockRepresentation[];
  /** @state artifact @child kind=s.stdio.semio.kit */
  catalog: ArtifactChildHandle;
  /** @state artifact */
  vortexKindExtra: Block3dVortexKindExtra[];
  /** @state artifact */
  vortices: Block3dVortexTemplate[];
  /** @state artifact */
  compatibility: BlockCompatibilityRule[];
  /** @state artifact */
  attributes: BlockAttribute[];
  /** @state artifact */
  authors: BlockAuthor[];
  /** @state artifact */
  camera3d: BlockCamera3d;
  /** @state artifact */
  meta: BlockMeta;
  /** @state presence */
  selectedIds: string[];
  /** @state presence */
  activeRepresentationId?: string;
  /** @state presence */
  wantedTags: string[];
  /** @state config */
  locale: string;
  /** @state config */
  windows: Block3dWindowView[];
  /** @state config */
  brushVortexKindId?: string;
  /** @state config */
  brushRadius: number;
  /** @state config */
  brushFlip: boolean;
  /** @state artifact */
  brushPreview?: Block3dBrushPreview;
  /** @state config */
  camera?: BlockCamera3d;
  /** @state artifact */
  hoveredVortexFullId?: string;
}
