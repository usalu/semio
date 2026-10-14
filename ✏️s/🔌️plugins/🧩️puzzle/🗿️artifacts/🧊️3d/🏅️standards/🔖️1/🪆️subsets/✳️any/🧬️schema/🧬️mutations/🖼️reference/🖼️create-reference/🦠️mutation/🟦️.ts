/** 🖼 `create-reference` payload — mirrors Rust `CreateReference` (`../🦀️.rs:14`). `index:
 * Option<usize>` carries no `skip_serializing_if`, so the key stays required with a nullable
 * value. */
import type { Puzzle3dReference } from "../../../📸️snapshot/🟦️.ts";

export interface CreateReference {
  reference: Puzzle3dReference;
  index: number | null;
}
