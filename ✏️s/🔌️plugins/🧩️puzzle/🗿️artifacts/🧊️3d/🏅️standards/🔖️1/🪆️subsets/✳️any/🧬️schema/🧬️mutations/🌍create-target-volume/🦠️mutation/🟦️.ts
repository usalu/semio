/** 🌍 `create-target-volume` payload — mirrors Rust `CreateTargetVolume` (`../🦀️.rs:14`). `index:
 * Option<usize>` carries no `skip_serializing_if`, so the key stays required with a nullable
 * value. */
import type { Puzzle3dTargetVolume } from "../../../📸️snapshot/🟦️.ts";

export interface CreateTargetVolume {
  targetVolume: Puzzle3dTargetVolume;
  index: number | null;
}
