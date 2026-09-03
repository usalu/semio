/** 🔺️ SemioDiff schema — real mirror of `🦀️.rs`. The envelope union's own diff:
 * `NoChange`, 13 same-kind wrappers each nesting that subset's own REAL diff type unchanged, and
 * `Replace` (the escape hatch for a genuine cross-kind change or an explicit `SetSnapshot`
 * mutation — there is no sparse representation for "this artifact used to be a video, now it's a
 * flow"). Tag key is `kind` (`#[serde(tag = "kind", rename_all = "camelCase")]` — distinct
 * from the snapshot facet's own `subset` tag key). */
import type { SemioSnapshot } from "../📸️snapshot/🟦️";
import type { SemioBrepDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioMeshDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioModelDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioValueTreeDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioDocumentDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioCadDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioDrawingDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioImageDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioVideoDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioAudioDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioAnimationDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioPresentationDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioFlowDiff } from "../../../✳️brep/🧬️schema/🔺️diff/🟦️";

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
