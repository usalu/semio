/** 🧬️ Block5d snapshot schema — artifact-lane fields only. */

import type { BlockAttribute, BlockAuthor, BlockCamera2d, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation } from "../../../../../../../../🟦️";
import type { Block5dGripKind, Block5dGripTemplate, Block5dPart2d, Block5dPart3d } from "../../../../../../🟦️";

export interface Block5dSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  partKind: BlockKindIdentity;
  /** @state artifact */
  "2d": Block5dPart2d;
  /** @state artifact */
  "3d": Block5dPart3d;
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
}
