/** 🧵️ The run-time source text of the instance family: the module trees of the composer crate and of every use-case and component crate below it (tier 1, extension, render, dispatch, typed operations, commands), the ladder crate and the two turn crates. Source-text laws read it instead of the files the cut left. */
import { resolve } from "node:path";
import { readModuleTreeSource } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

const plugin = resolve(import.meta.dir, "../../..");
const INSTANCE_ROOTS = ["🧵️instance/🦀️.rs", ...["🏪️stores", "📜️history", "🪆️children", "👷️jobs", "📄️docs", "🧩️extensions", "🖼️render", "📨️dispatch", "🧾️typed-operations", "🧷️command-routes"].map(name => `🧵️instance/${name}/🦀️.rs`)].map(path => resolve(plugin, path));
const LADDERS = resolve(plugin, "🪜️maintenance-ladder/🦀️.rs");
const TURNS = [resolve(plugin, "⏪️time-travel/🧵️instance/🦀️.rs"), resolve(plugin, "⏯️tool-run/🧵️instance/🦀️.rs")];

/** 🧩️ The composer crate and the module trees of the component and use-case crates below it. */
export function instanceText(): string {
  return INSTANCE_ROOTS.map(path => readModuleTreeSource(path)).join("\n");
}

/** 🪜️ The ladder root file (its staged slices belong to the ladder crate). */
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
