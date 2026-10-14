/** ➕ `add-object-vortex` payload — mirrors Rust `AddObjectVortex` (`../🦀️.rs:14`). `index:
 * Option<usize>` carries no `skip_serializing_if`, so the key stays required with a nullable
 * value. */
import type { Puzzle3dVortex } from "../../../📸️snapshot/🟦️.ts";

export interface AddObjectVortex {
  objectId: string;
  vortex: Puzzle3dVortex;
  index: number | null;
}
