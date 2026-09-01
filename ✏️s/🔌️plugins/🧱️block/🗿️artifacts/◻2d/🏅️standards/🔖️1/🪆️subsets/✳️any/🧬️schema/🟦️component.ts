/** 🧬️ Block2d artifact schema — every field with its state class. */

import type { BlockAttribute, BlockAuthor, BlockCamera2d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta } from "../../../../../../../🟦️component";
import type { Block2dHandleKind, Block2dHandleTemplate, Block2dPresentation } from "../../../../../🟦️component";

export interface Block2dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  nodeKind: BlockKindIdentity;
  /** @state artifact */
  presentation: Block2dPresentation;
  /** @state artifact */
  handleKinds: Block2dHandleKind[];
  /** @state artifact */
  handles: Block2dHandleTemplate[];
  /** @state artifact */
  compatibility: BlockCompatibilityRule[];
  /** @state artifact */
  attributes: BlockAttribute[];
  /** @state artifact */
  authors: BlockAuthor[];
  /** @state artifact */
  camera2d: BlockCamera2d;
  /** @state artifact */
  meta: BlockMeta;
  /** @state presence */
  selectedIds: string[];
  /** @state config */
  locale: string;
}
