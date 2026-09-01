/** 🤝 `connect-kind-compatibility` payload — mirrors Rust `ConnectKindCompatibility`
 * (`../🦀️.rs:13`). */
import type { Puzzle3dCompatSpecificity } from "../../../📸️snapshot/🟦️component.ts";

export interface ConnectKindCompatibility {
  source: string;
  target: string;
  bidirectional: boolean;
  important: boolean;
  specificity: Puzzle3dCompatSpecificity;
}
