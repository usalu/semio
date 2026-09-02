/** 📐 `scale-target-volume` payload — mirrors Rust `ScaleTargetVolume` (`../🦀️.rs:13`). `new_scale:
 * Option<Puzzle3dScale>` carries no `skip_serializing_if`, so the key stays required with a
 * nullable value. */
import type { Puzzle3dScale } from "../../🟦️.ts";

export interface ScaleTargetVolume {
  id: string;
  newScale: Puzzle3dScale | null;
}
