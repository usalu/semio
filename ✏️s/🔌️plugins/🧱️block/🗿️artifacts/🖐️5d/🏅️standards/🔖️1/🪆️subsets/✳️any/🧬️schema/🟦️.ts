/** 🧬️ Block5d artifact schema — every field with its state class. */

import type { BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation } from "../../../../../../../🟦️";
import type { Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d } from "../../../../../🟦️";

export interface Block5dArtifact {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  partKind: BlockKindIdentity;
  /** @state artifact */
  part2d: Block5dPart2d;
  /** @state artifact */
  part3d: Block5dPart3d;
  /** @state artifact */
  representations: BlockRepresentation[];
  /** @state artifact */
  gripKinds: Block5dGripKind[];
  /** @state artifact */
  grips: Block5dGripTemplate[];
  /** @state artifact */
  compatibility: BlockCompatibilityRule[];
  /** @state artifact */
  attributes: BlockAttribute[];
  /** @state artifact */
  authors: BlockAuthor[];
  /** @state artifact */
  camera2d: BlockCamera2d;
  /** @state artifact */
  camera3d: BlockCamera3d;
  /** @state artifact */
  meta: BlockMeta;
  /** @state presence */
  selectedIds: string[];
  /** @state config */
  locale: string;
}
