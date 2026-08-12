/** 🧬️ SemioSnapshot schema — real mirror of `🦀️component.rs`. The envelope union over all 13
 * domain subsets — every semio artifact round-trips through this shape. `SemioSubsetSnapshot` is
 * serde-internally-tagged (`#[serde(tag = "subset", rename_all = "camelCase")]`): on the wire each
 * variant flattens the REFERENCED subset's own snapshot fields alongside the `subset`
 * discriminator directly (no nested `value` key). This mirror models each variant as a
 * discriminated-union member carrying the literal `subset` tag plus a single lowerCamelCase field
 * embedding that subset's own snapshot type BY REFERENCE (never redeclaring its internals — see
 * that subset's own `📸️snapshot/🟦️component.ts` for the real shape), the same discriminated-union
 * idiom already used repo-wide (e.g. ✳️presentation's `SlideShape`/`PlaceholderKind`). */
import type { SemioBrepSnapshot } from "../../../brep/schema/snapshot/component";
import type { SemioMeshSnapshot } from "../../../mesh/schema/snapshot/component";
import type { SemioModelSnapshot } from "../../../model/schema/snapshot/component";
import type { SemioObjectSnapshot } from "../../../object/schema/snapshot/component";
import type { SemioDocumentSnapshot } from "../../../document/schema/snapshot/component";
import type { SemioCadSnapshot } from "../../../cad/schema/snapshot/component";
import type { SemioDrawingSnapshot } from "../../../drawing/schema/snapshot/component";
import type { SemioImageSnapshot } from "../../../image/schema/snapshot/component";
import type { SemioVideoSnapshot } from "../../../video/schema/snapshot/component";
import type { SemioAudioSnapshot } from "../../../audio/schema/snapshot/component";
import type { SemioAnimationSnapshot } from "../../../animation/schema/snapshot/component";
import type { SemioPresentationSnapshot } from "../../../presentation/schema/snapshot/component";
import type { SemioWorkflowSnapshot } from "../../../workflow/schema/snapshot/component";

export type SemioSubsetSnapshot =
  | { subset: "brep"; brep: SemioBrepSnapshot }
  | { subset: "mesh"; mesh: SemioMeshSnapshot }
  | { subset: "model"; model: SemioModelSnapshot }
  | { subset: "object"; object: SemioObjectSnapshot }
  | { subset: "document"; document: SemioDocumentSnapshot }
  | { subset: "cad"; cad: SemioCadSnapshot }
  | { subset: "drawing"; drawing: SemioDrawingSnapshot }
  | { subset: "image"; image: SemioImageSnapshot }
  | { subset: "video"; video: SemioVideoSnapshot }
  | { subset: "audio"; audio: SemioAudioSnapshot }
  | { subset: "animation"; animation: SemioAnimationSnapshot }
  | { subset: "presentation"; presentation: SemioPresentationSnapshot }
  | { subset: "workflow"; workflow: SemioWorkflowSnapshot };

export interface SemioSnapshot {
  /** @state persistent */ schema: string;
  /** @state persistent */ subset: SemioSubsetSnapshot;
}
