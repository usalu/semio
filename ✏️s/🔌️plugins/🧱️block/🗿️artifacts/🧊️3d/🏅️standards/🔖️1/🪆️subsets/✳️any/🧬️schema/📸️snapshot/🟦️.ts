/** 🧬️ Block3d snapshot schema — artifact-lane fields only. */

import type { BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation } from "../../../../../../../../🟦️";
import type { Block3dVortexKindExtra, Block3dVortexTemplate } from "../../../../../../🟦️";
import type { ArtifactChild } from "../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🟦️.ts";

export interface Block3dSnapshot {
  /** @state artifact */
  schema: string;
  /** @state artifact */
  objectKind: BlockKindIdentity;
  /** @state artifact */
  representations: BlockRepresentation[];
  /** @state artifact @child kind=s.stdio.semio */
  catalog: ArtifactChild;
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
}
