/** 📚 `replace-kind-catalogs` payload — mirrors Rust `ReplaceKindCatalogs` (`../🦀️.rs:15`).
 * `new_catalogs: Option<Puzzle3dKindCatalogs>` carries no `skip_serializing_if`, so the key stays
 * required with a nullable value. */
import type { Puzzle3dKindCatalogs } from "../../🟦️.ts";

export interface ReplaceKindCatalogs {
  newCatalogs: Puzzle3dKindCatalogs | null;
}
