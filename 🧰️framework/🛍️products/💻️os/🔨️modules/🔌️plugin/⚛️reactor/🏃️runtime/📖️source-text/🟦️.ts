import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { readModuleTreeSource } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";

const CRATE_ROOTS = [
  "../🧰️ops/🦀️.rs",
  "../🔄️exchange/🦀️.rs",
  "../📦️instances/🦀️.rs",
  "../../📨️pending/🦀️.rs",
] as const;

const LIFTED_AND_MACRO_FILES = [
  "../../../🧩️extension/🛬️lifted/🏃️runtime/🦀️.rs",
  "../../../🏗️builder/🔌️program/🛬️lifted/🏃️runtime/🦀️.rs",
  "../../../📣️macros/⚛️wp4-reactor/🦀️.rs",
] as const;

/** 📖️ The text the unsplit plugin runtime file held: its module tree, the module trees of the crates the pieces moved into (runtime-ops, runtime-exchange, runtime-instances and the lifecycle pieces of reactor-lifecycle), then the three files that left its directory (the extension and plugin-program lifts and the facade macro mount), so source-text laws keep seeing every cut item. */
export function readPluginRuntimeSource(): string {
  const runtime = fileURLToPath(new URL("../🦀️.rs", import.meta.url));
  const left = LIFTED_AND_MACRO_FILES.map(file => readFileSync(fileURLToPath(new URL(file, import.meta.url)), "utf8"));
  const crates = CRATE_ROOTS.map(file => readModuleTreeSource(fileURLToPath(new URL(file, import.meta.url))));
  return readModuleTreeSource(runtime) + crates.map(text => (text.endsWith("\n") ? text : text + "\n")).join("") + left.map(text => (text.endsWith("\n") ? text : text + "\n")).join("");
}
