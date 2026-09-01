/** 🧬️ SemioImageMutation schema — real facet mirror of the Rust `🦀️component.rs` sibling.
 * Discriminated union on the `mutation` tag, matching the serde `#[serde(tag = "mutation")]`
 * shape. */
import type { SemioColorspace, SemioImageFrame, SemioImageSnapshot } from "../📸️snapshot/🟦️component.ts";

export type SemioImageMutation =
  | { mutation: "setSnapshot"; snapshot: SemioImageSnapshot }
  | { mutation: "setDimensions"; width: number; height: number }
  | { mutation: "setColorspace"; colorspace: SemioColorspace }
  | { mutation: "setBitDepth"; bitDepth: number }
  | { mutation: "setIcc"; icc: string | null }
  | { mutation: "insertFrame"; index: number; frame: SemioImageFrame }
  | { mutation: "removeFrame"; index: number }
  | { mutation: "moveFrame"; from: number; to: number }
  | { mutation: "setFrameDelay"; index: number; delayMs: number }
  | { mutation: "setFramePixels"; index: number; rgba8: string }
  | { mutation: "setMetadataEntry"; key: string; value: string }
  | { mutation: "removeMetadataEntry"; key: string };
