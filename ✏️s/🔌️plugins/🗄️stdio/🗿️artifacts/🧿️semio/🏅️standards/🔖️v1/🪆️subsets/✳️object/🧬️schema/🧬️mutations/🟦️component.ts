/** 🧬️ SemioObjectMutation facet mirror — the `🦀️component.rs` sibling is the real source of
 * truth; this discriminated union tracks its fields 1:1 (see `POLICY_FACET_MIRROR_DRIFT`). */
import type { ObjectId, SemioObjectSnapshot, SemioValue } from "../📸️snapshot/🟦️component";

export type SemioObjectPathSegment = { kind: "key"; key: string } | { kind: "index"; index: number };
export type SemioObjectPath = SemioObjectPathSegment[];

export type SemioObjectMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: SemioObjectSnapshot }
  | { mutation: "setValue"; path: SemioObjectPath; value: SemioValue }
  | { mutation: "setMapEntry"; path: SemioObjectPath; key: string; value: SemioValue }
  | { mutation: "removeMapEntry"; path: SemioObjectPath; key: string }
  | { mutation: "insertListItem"; path: SemioObjectPath; index: number; value: SemioValue }
  | { mutation: "removeListItem"; path: SemioObjectPath; index: number }
  | { mutation: "setObject"; id: ObjectId; value: SemioValue }
  | { mutation: "removeObject"; id: ObjectId };
