/** 🧬️ SemioMutation schema — real mirror of `🦀️component.rs`. The envelope union's own mutation
 * vocabulary: `NoMutation`, `SetSnapshot` (the only way to change SUBSET KIND), and 13 wrapper
 * variants each carrying that subset's OWN mutation enum unchanged. Adjacently tagged
 * (`#[serde(tag = "mutation", content = "payload")]`, NOT internally tagged like every wrapped
 * subset's own mutation enum — an internally-tagged wrapper here would collide key-for-key with a
 * wrapped variant's OWN `mutation` discriminator field on flatten). */
import type { SemioSnapshot } from "../📸️snapshot/🟦️component";
import type { SemioBrepMutation } from "../../../brep/schema/mutations/component";
import type { SemioMeshMutation } from "../../../mesh/schema/mutations/component";
import type { SemioModelMutation } from "../../../model/schema/mutations/component";
import type { SemioValueMutation } from "../../../value/schema/mutations/component";
import type { SemioDocumentMutation } from "../../../document/schema/mutations/component";
import type { SemioCadMutation } from "../../../cad/schema/mutations/component";
import type { SemioDrawingMutation } from "../../../drawing/schema/mutations/component";
import type { SemioImageMutation } from "../../../image/schema/mutations/component";
import type { SemioVideoMutation } from "../../../video/schema/mutations/component";
import type { SemioAudioMutation } from "../../../audio/schema/mutations/component";
import type { SemioAnimationMutation } from "../../../animation/schema/mutations/component";
import type { SemioPresentationMutation } from "../../../presentation/schema/mutations/component";
import type { SemioFlowMutation } from "../../../flow/schema/mutations/component";

export type SemioMutation =
  | { mutation: "noMutation" }
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
  | { mutation: "flow"; payload: SemioFlowMutation };
