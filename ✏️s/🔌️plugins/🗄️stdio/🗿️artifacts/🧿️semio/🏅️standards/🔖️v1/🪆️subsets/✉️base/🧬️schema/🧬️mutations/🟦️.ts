/** 🧬️ SemioMutation — mirror of `🦀️.rs` and of the published JSON carrier `🔣️.json`. Adjacently
 * tagged (`mutation` + `payload`) so a wrapped arm mutation's own `mutation` discriminator never collides
 * with the envelope's: `setSnapshot` replaces the envelope, and each arm wrapper is tagged `apply<Arm>`
 * and carries that arm's own mutation as `payload.mutation`. */
import type { SemioSnapshot } from "../📸️snapshot/🟦️.ts";
import type { SemioBrepMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioMeshMutation } from "../../../🔺️mesh/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioModelMutation } from "../../../🏛️model/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioValueMutation } from "../../../🔢️value/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioDocumentMutation } from "../../../📑️document/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioCadMutation } from "../../../📐️cad/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioDrawingMutation } from "../../../🖊️drawing/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioImageMutation } from "../../../🖼️image/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioVideoMutation } from "../../../🎬️video/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioAudioMutation } from "../../../🔊️audio/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioAnimationMutation } from "../../../🎞️animation/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioPresentationMutation } from "../../../📽️presentation/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioFlowMutation } from "../../../🌊️flow/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioTextMutation } from "../../../🔤️text/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioTableMutation } from "../../../📊️table/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioGraphMutation } from "../../../🕸️graph/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioObjectMutation } from "../../../📦️object/🧬️schema/🧬️mutations/🟦️.ts";
import type { SemioKitMutation } from "../../../🧰️kit/🧬️schema/🧬️mutations/🟦️.ts";

export type SemioMutation =
  | { mutation: "setSnapshot"; payload: { snapshot: SemioSnapshot } }
  | { mutation: "applyBrep"; payload: { mutation: SemioBrepMutation } }
  | { mutation: "applyMesh"; payload: { mutation: SemioMeshMutation } }
  | { mutation: "applyModel"; payload: { mutation: SemioModelMutation } }
  | { mutation: "applyValue"; payload: { mutation: SemioValueMutation } }
  | { mutation: "applyDocument"; payload: { mutation: SemioDocumentMutation } }
  | { mutation: "applyCad"; payload: { mutation: SemioCadMutation } }
  | { mutation: "applyDrawing"; payload: { mutation: SemioDrawingMutation } }
  | { mutation: "applyImage"; payload: { mutation: SemioImageMutation } }
  | { mutation: "applyVideo"; payload: { mutation: SemioVideoMutation } }
  | { mutation: "applyAudio"; payload: { mutation: SemioAudioMutation } }
  | { mutation: "applyAnimation"; payload: { mutation: SemioAnimationMutation } }
  | { mutation: "applyPresentation"; payload: { mutation: SemioPresentationMutation } }
  | { mutation: "applyFlow"; payload: { mutation: SemioFlowMutation } }
  | { mutation: "applyText"; payload: { mutation: SemioTextMutation } }
  | { mutation: "applyTable"; payload: { mutation: SemioTableMutation } }
  | { mutation: "applyGraph"; payload: { mutation: SemioGraphMutation } }
  | { mutation: "applyObject"; payload: { mutation: SemioObjectMutation } }
  | { mutation: "applyKit"; payload: { mutation: SemioKitMutation } };
