/** 🧬️ SemioMutation schema — real mirror of `🦀️.rs`. The envelope union's own mutation
 * vocabulary: `SetSnapshot` (the only way to change SUBSET KIND) and 18 wrapper variants each
 * carrying that subset's OWN mutation enum unchanged. `NoMutation` was dropped from the Rust side
 * (`#[derive(dsl::Mutations)]` requires every variant to wrap exactly one leaf payload, and `no` is
 * not an approved semantic verb) — a "do nothing" mutation is now expressed as `SetSnapshot` with
 * the current snapshot unchanged. Adjacently tagged (`#[serde(tag = "mutation", content =
 * "payload")]`, NOT internally tagged like every wrapped subset's own mutation enum — an
 * internally-tagged wrapper here would collide key-for-key with a wrapped variant's OWN `mutation`
 * discriminator field on flatten). */
import type { SemioSnapshot } from "../📸️snapshot/🟦️";
import type { SemioBrepMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioMeshMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioModelMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioValueMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioDocumentMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioCadMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioDrawingMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioImageMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioVideoMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioAudioMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioAnimationMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioPresentationMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioFlowMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioTextMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioTableMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioGraphMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioObjectMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";
import type { SemioKitMutation } from "../../../🧊️brep/🧬️schema/🧬️mutations/🟦️";

export type SemioMutation =
  | { mutation: "setSnapshot"; payload: { snapshot: SemioSnapshot } }
  | { mutation: "brep"; payload: SemioBrepMutation }
  | { mutation: "mesh"; payload: SemioMeshMutation }
  | { mutation: "model"; payload: SemioModelMutation }
  | { mutation: "value"; payload: SemioValueMutation }
  | { mutation: "document"; payload: SemioDocumentMutation }
  | { mutation: "cad"; payload: SemioCadMutation }
  | { mutation: "drawing"; payload: SemioDrawingMutation }
  | { mutation: "image"; payload: SemioImageMutation }
  | { mutation: "video"; payload: SemioVideoMutation }
  | { mutation: "audio"; payload: SemioAudioMutation }
  | { mutation: "animation"; payload: SemioAnimationMutation }
  | { mutation: "presentation"; payload: SemioPresentationMutation }
  | { mutation: "flow"; payload: SemioFlowMutation }
  | { mutation: "text"; payload: SemioTextMutation }
  | { mutation: "table"; payload: SemioTableMutation }
  | { mutation: "graph"; payload: SemioGraphMutation }
  | { mutation: "object"; payload: SemioObjectMutation }
  | { mutation: "kit"; payload: SemioKitMutation };
