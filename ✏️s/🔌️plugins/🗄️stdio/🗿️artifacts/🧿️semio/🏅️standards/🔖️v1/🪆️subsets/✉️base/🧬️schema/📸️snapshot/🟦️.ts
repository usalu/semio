/** 🧬️ SemioSnapshot schema — real mirror of `🦀️.rs`. The envelope union over all 13
 * domain subsets — every semio artifact round-trips through this shape. `SemioSubsetSnapshot` is
 * serde-internally-tagged (`#[serde(tag = "subset", rename_all = "camelCase")]`): on the wire each
 * variant flattens the REFERENCED subset's own snapshot fields alongside the `subset`
 * discriminator directly (no nested `value` key). This mirror models each variant as a
 * discriminated-union member carrying the literal `subset` tag plus a single lowerCamelCase field
 * embedding that subset's own snapshot type BY REFERENCE (never redeclaring its internals — see
 * that subset's own `📸️snapshot/🟦️.ts` for the real shape), the same discriminated-union
 * idiom already used repo-wide (e.g. 📽️presentation's `SlideShape`/`PlaceholderKind`). */
import type { SemioBrepSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioMeshSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioModelSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioValueSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioDocumentSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioCadSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioDrawingSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioImageSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioVideoSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioAudioSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioAnimationSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioPresentationSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";
import type { SemioFlowSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️";

export type SemioSubsetSnapshot =
  | { subset: "brep"; brep: SemioBrepSnapshot }
  | { subset: "mesh"; mesh: SemioMeshSnapshot }
  | { subset: "model"; model: SemioModelSnapshot }
  | { subset: "value"; value: SemioValueSnapshot }
  | { subset: "document"; document: SemioDocumentSnapshot }
  | { subset: "cad"; cad: SemioCadSnapshot }
  | { subset: "drawing"; drawing: SemioDrawingSnapshot }
  | { subset: "image"; image: SemioImageSnapshot }
  | { subset: "video"; video: SemioVideoSnapshot }
  | { subset: "audio"; audio: SemioAudioSnapshot }
  | { subset: "animation"; animation: SemioAnimationSnapshot }
  | { subset: "presentation"; presentation: SemioPresentationSnapshot }
  | { subset: "flow"; flow: SemioFlowSnapshot };

export interface SemioSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ subset: SemioSubsetSnapshot;
}
