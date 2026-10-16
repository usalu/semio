/** 🔺️ SemioDiff schema — real mirror of `🦀️.rs`. The envelope union's own diff:
 * `NoChange` and 13 same-kind wrappers each nesting that subset's own REAL diff type unchanged. Tag key is `kind` (`#[serde(tag = "kind", rename_all = "camelCase")]` — distinct
 * from the snapshot facet's own `subset` tag key). */
import type { SemioSnapshot } from "../📸️snapshot/🟦️";
import type { SemioBrepDiff } from "../../../🧊️brep/🧬️schema/🔺️diff/🟦️";
import type { SemioMeshDiff } from "../../../🔺️mesh/🧬️schema/🔺️diff/🟦️";
import type { SemioModelDiff } from "../../../🏛️model/🧬️schema/🔺️diff/🟦️";
import type { SemioValueTreeDiff } from "../../../🔢️value/🧬️schema/🔺️diff/🟦️";
import type { SemioDocumentDiff } from "../../../📑️document/🧬️schema/🔺️diff/🟦️";
import type { SemioCadDiff } from "../../../📐️cad/🧬️schema/🔺️diff/🟦️";
import type { SemioDrawingDiff } from "../../../🖊️drawing/🧬️schema/🔺️diff/🟦️";
import type { SemioImageDiff } from "../../../🖼️image/🧬️schema/🔺️diff/🟦️";
import type { SemioVideoDiff } from "../../../🎬️video/🧬️schema/🔺️diff/🟦️";
import type { SemioAudioDiff } from "../../../🔊️audio/🧬️schema/🔺️diff/🟦️";
import type { SemioAnimationDiff } from "../../../🎞️animation/🧬️schema/🔺️diff/🟦️";
import type { SemioPresentationDiff } from "../../../📽️presentation/🧬️schema/🔺️diff/🟦️";
import type { SemioFlowDiff } from "../../../🌊️flow/🧬️schema/🔺️diff/🟦️";

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
  | { kind: "flow"; flow: SemioFlowDiff };
