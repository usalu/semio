/** 📏 `scale-object` payload — mirrors Rust `ScaleObject` (`../🦀️.rs:13`). `new_scale:
 * Option<Puzzle3dScale>` carries no `skip_serializing_if`, so the key stays required with a
 * nullable value. */
import type { Puzzle3dScale } from "../../🟦️component.ts";

export interface ScaleObject {
  id: string;
  newScale: Puzzle3dScale | null;
}
