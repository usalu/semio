/** 🌱 `create-object` payload — mirrors Rust `CreateObject` (`../🦀️.rs:14`). `index: Option<usize>`
 * carries no `skip_serializing_if`, so the key stays required with a nullable value (confirmed by
 * the committed `per-verb 🧪️tests 🦠️mutation/🔣️.json` fixture). */
import type { Puzzle3dObject } from "../../../📸️snapshot/🟦️.ts";

export interface CreateObject {
  object: Puzzle3dObject;
  index: number | null;
}
