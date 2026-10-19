import type { ScriptInvocation } from "@semio-tech/framework/process/routing";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { runArtifactRustTests } from "@semio-tech/framework/process/cargo-test";
import { runOwnedCommand } from "@semio-tech/framework/process/execution";
import { resolve as scriptDirectoryResolve } from "node:path";

const SCRIPT_DIR = scriptDirectoryResolve(import.meta.dir, "../../📦️packages/🦀️rust");

const familyRoot = resolve(SCRIPT_DIR, "../..");
export const familyPackages = (Bun.TOML.parse(readFileSync(join(familyRoot, "Cargo.toml"), "utf8")) as { workspace: { members: string[] } }).workspace.members.map((member) => (Bun.TOML.parse(readFileSync(join(familyRoot, member, "Cargo.toml"), "utf8")) as { package: { name: string } }).package.name);

/** 🏃️ Runs the `ControlledMetadataTest` command under the caller's roots and invocation. */
export async function runControlledMetadataTest(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  const mode = args.shift();
  if (args.length || (mode !== undefined && mode !== "source" && mode !== "native")) throw new Error("Expected test controlled-metadata [source|native]");
  if (mode !== "native") await runOwnedCommand(process.execPath, ["test", resolve(context.root, "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🎛️metadata/🟦️.ts")], context.repoRoot, "dwg:controlled-metadata:source", 15_000);
  if (mode !== "source") await runArtifactRustTests("semio-s-artifact-stdio-dwg-snapshot", context.repoRoot, ["--lib", "dwg_controlled_metadata_"], context.invocation);
}

/** 🏃️ Runs the `GrammarShapeTest` command under the caller's roots and invocation. */
export async function runGrammarShapeTest(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, args: string[]): Promise<void> {
  const mode = args.shift();
  if (args.length || (mode !== undefined && mode !== "source" && mode !== "native")) throw new Error("Expected test grammar-shape [source|native]");
  if (mode !== "native") await runOwnedCommand(process.execPath, ["test", resolve(context.root, "../../🏅️standards/🔟ac1024/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🏗️grammar/🟦️.ts")], context.repoRoot, "dwg:grammar-shape:source", 15_000);
  if (mode !== "source") await runArtifactRustTests("semio-s-artifact-stdio-dwg-snapshot", context.repoRoot, ["--lib", "dwg_grammar_shape_"], context.invocation);
}
