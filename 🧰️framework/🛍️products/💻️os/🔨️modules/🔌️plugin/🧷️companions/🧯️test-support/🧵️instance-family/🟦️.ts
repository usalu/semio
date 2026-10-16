/** 🧵️ The run-time source text of the cut instance family: the module tree of the instance parent (its staged tier-1 slices, bundles and use-case slices, and the ladders it mounts) followed by the two feature turn files that are mounted from their feature roots. Source-text laws read it instead of the files the cut left. */
import { resolve } from "node:path";
import { readModuleTreeSource } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

const plugin = resolve(import.meta.dir, "../../..");
const INSTANCE = resolve(plugin, "🧵️instance/🦀️.rs");
const LADDERS = resolve(plugin, "🪜️maintenance-ladder/🦀️.rs");
const TURNS = [resolve(plugin, "⏪️time-travel/🧵️instance/🦀️.rs"), resolve(plugin, "⏯️tool-run/🧵️instance/🦀️.rs")];

/** 🧩️ The instance parent and every staged module it mounts. */
export function instanceText(): string {
  return readModuleTreeSource(INSTANCE);
}

/** 🪜️ The ladder root file (its staged slices are part of [`instanceText`]). */
export function ladderText(): string {
  return readModuleTreeSource(LADDERS);
}

/** ⏯️ The time-travel and tool-run turn files with their extension types. */
export function turnText(): string {
  return TURNS.map(path => readModuleTreeSource(path)).join("\n");
}

/** 🧵️ The whole family. */
export function familyText(): string {
  return [instanceText(), turnText()].join("\n");
}
