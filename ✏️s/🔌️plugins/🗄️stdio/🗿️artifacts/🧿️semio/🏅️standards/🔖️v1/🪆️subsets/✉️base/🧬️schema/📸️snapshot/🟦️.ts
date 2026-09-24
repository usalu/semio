/** 🧬️ SemioSnapshot — mirror of `🦀️.rs` and of the published JSON carrier `🔣️.json`. The envelope
 * union over all eighteen semio subsets: `SemioSubsetSnapshot` is internally tagged on `subset`, so
 * each variant is that subset's own snapshot with the `subset` tag beside its own fields. */
import type { SemioBrepSnapshot } from "../../../🧊️brep/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioMeshSnapshot } from "../../../🔺️mesh/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioModelSnapshot } from "../../../🏛️model/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioValueSnapshot } from "../../../🔢️value/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioDocumentSnapshot } from "../../../📑️document/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioCadSnapshot } from "../../../📐️cad/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioDrawingSnapshot } from "../../../🖊️drawing/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioImageSnapshot } from "../../../🖼️image/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioVideoSnapshot } from "../../../🎬️video/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioAudioSnapshot } from "../../../🔊️audio/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioAnimationSnapshot } from "../../../🎞️animation/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioPresentationSnapshot } from "../../../📽️presentation/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioFlowSnapshot } from "../../../🌊️flow/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioTextSnapshot } from "../../../🔤️text/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioTableSnapshot } from "../../../📊️table/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioGraphSnapshot } from "../../../🕸️graph/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioObjectSnapshot } from "../../../📦️object/🧬️schema/📸️snapshot/🟦️.ts";
import type { SemioKitSnapshot } from "../../../🧰️kit/🧬️schema/📸️snapshot/🟦️.ts";

export type SemioSubsetSnapshot =
  | ({ subset: "brep" } & SemioBrepSnapshot)
  | ({ subset: "mesh" } & SemioMeshSnapshot)
  | ({ subset: "model" } & SemioModelSnapshot)
  | ({ subset: "value" } & SemioValueSnapshot)
  | ({ subset: "document" } & SemioDocumentSnapshot)
  | ({ subset: "cad" } & SemioCadSnapshot)
  | ({ subset: "drawing" } & SemioDrawingSnapshot)
  | ({ subset: "image" } & SemioImageSnapshot)
  | ({ subset: "video" } & SemioVideoSnapshot)
  | ({ subset: "audio" } & SemioAudioSnapshot)
  | ({ subset: "animation" } & SemioAnimationSnapshot)
  | ({ subset: "presentation" } & SemioPresentationSnapshot)
  | ({ subset: "flow" } & SemioFlowSnapshot)
  | ({ subset: "text" } & SemioTextSnapshot)
  | ({ subset: "table" } & SemioTableSnapshot)
  | ({ subset: "graph" } & SemioGraphSnapshot)
  | ({ subset: "object" } & SemioObjectSnapshot)
  | ({ subset: "kit" } & SemioKitSnapshot);

export interface SemioSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ subset: SemioSubsetSnapshot;
}
