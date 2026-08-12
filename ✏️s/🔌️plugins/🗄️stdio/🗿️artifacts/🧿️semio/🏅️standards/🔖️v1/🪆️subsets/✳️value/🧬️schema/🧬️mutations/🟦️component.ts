/** 🧬️ SemioValueMutation facet mirror — the `🦀️component.rs` sibling is the real source of
 * truth; this discriminated union tracks its fields 1:1 (see `POLICY_FACET_MIRROR_DRIFT`). */
import type { ValueId, SemioValueSnapshot, SemioValue } from "../📸️snapshot/🟦️component";

export type SemioValuePathSegment = { kind: "key"; key: string } | { kind: "index"; index: number };
export type SemioValuePath = SemioValuePathSegment[];

export type SemioValueMutation =
  | { mutation: "noMutation" }
  | { mutation: "setSnapshot"; snapshot: SemioValueSnapshot }
  | { mutation: "setValue"; path: SemioValuePath; value: SemioValue }
  | { mutation: "setMapEntry"; path: SemioValuePath; key: string; value: SemioValue }
  | { mutation: "removeMapEntry"; path: SemioValuePath; key: string }
  | { mutation: "insertListItem"; path: SemioValuePath; index: number; value: SemioValue }
  | { mutation: "removeListItem"; path: SemioValuePath; index: number }
  | { mutation: "setNode"; id: ValueId; value: SemioValue }
  | { mutation: "removeNode"; id: ValueId };
