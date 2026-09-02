/** 🔺️ SemioDiff schema — real mirror of `🦀️.rs`. The envelope union's own diff:
 * `NoChange`, 13 same-kind wrappers each nesting that subset's own REAL diff type unchanged, and
 * `Replace` (the escape hatch for a genuine cross-kind change or an explicit `SetSnapshot`
 * mutation — there is no sparse representation for "this artifact used to be a video, now it's a
 * flow"). Tag key is `kind` (`#[serde(tag = "kind", rename_all = "camelCase")]` — distinct
 * from the snapshot facet's own `subset` tag key). */
import type { SemioSnapshot } from "../📸️snapshot/🟦️component";
import type { SemioBrepDiff } from "../../../brep/schema/diff/component";
import type { SemioMeshDiff } from "../../../mesh/schema/diff/component";
import type { SemioModelDiff } from "../../../model/schema/diff/component";
import type { SemioValueTreeDiff } from "../../../value/schema/diff/component";
import type { SemioDocumentDiff } from "../../../document/schema/diff/component";
import type { SemioCadDiff } from "../../../cad/schema/diff/component";
import type { SemioDrawingDiff } from "../../../drawing/schema/diff/component";
import type { SemioImageDiff } from "../../../image/schema/diff/component";
import type { SemioVideoDiff } from "../../../video/schema/diff/component";
import type { SemioAudioDiff } from "../../../audio/schema/diff/component";
import type { SemioAnimationDiff } from "../../../animation/schema/diff/component";
import type { SemioPresentationDiff } from "../../../presentation/schema/diff/component";
import type { SemioFlowDiff } from "../../../flow/schema/diff/component";

export type SemioDiff =
  | { kind: "noChange" }
  | { kind: "brep"; brep: SemioBrepDiff }
  | { kind: "mesh"; mesh: SemioMeshDiff }
  | { kind: "model"; model: SemioModelDiff }
  | { kind: "value"; value: SemioValueTreeDiff }
  | { kind: "document"; document: SemioDocumentDiff }
  | { kind: "cad"; cad: SemioCadDiff }
  | { kind: "drawing"; drawing: SemioDrawingDiff }
  | { kind: "image"; image: SemioImageDiff }
  | { kind: "video"; video: SemioVideoDiff }
  | { kind: "audio"; audio: SemioAudioDiff }
  | { kind: "animation"; animation: SemioAnimationDiff }
  | { kind: "presentation"; presentation: SemioPresentationDiff }
  | { kind: "flow"; flow: SemioFlowDiff }
  | { kind: "replace"; snapshot: SemioSnapshot };
