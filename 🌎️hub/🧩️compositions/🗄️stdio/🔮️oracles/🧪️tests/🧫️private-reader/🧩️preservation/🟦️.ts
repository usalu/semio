import { createHash } from "node:crypto";
import { lstatSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, relative, resolve, sep } from "node:path";
import contract from "../../../🧫️fixtures/🧫️private-reader/🔣️.json";
import { inspectRustCompileReferences } from "../../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../../..");
const digest = (source: string) => createHash("sha256").update(source).digest("hex");

/** 🧫️ Current source inputs required for an exact private reader ownership proof. */
export interface PrivateReaderSources {
  read(path: string): string;
  assertFile(path: string): void;
}

/** 🪨️ Requires a physically owned regular file inside the declared workspace. */
export function assertPrivateReaderInput(path: string, workspace: string = root): void {
  const owner = resolve(workspace), target = resolve(owner, path), local = relative(owner, target);
  if (local === ".." || local.startsWith(`..${sep}`) || isAbsolute(local)) throw new Error(`private reader input escapes workspace: ${path}`);
  if (!lstatSync(target).isFile()) throw new Error(`private reader input is not a regular file: ${path}`);
  for (let current = target; ; current = dirname(current)) {
    if (lstatSync(current).isSymbolicLink()) throw new Error(`private reader input has a linked ancestor: ${path}`);
    if (relative(owner, current) === "") break;
    if (dirname(current) === current) throw new Error(`private reader input never reaches workspace: ${path}`);
  }
}

const currentSources: PrivateReaderSources = {
  read: (path) => readFileSync(resolve(root, path), "utf8"),
  assertFile: assertPrivateReaderInput,
};

/** 🪞️ Restores a declared caller after proving every privately owned literal input. */
export function originalPrivateReaderSource(path: string, source: string, sources: PrivateReaderSources = currentSources): string {
  const witness = contract.cases.find((row) => row.source === path);
  if (!witness) return source;
  sources.assertFile(witness.source);
  if (source.split(witness.currentRegion).length !== 2 || source.split(witness.importAnchor).length !== 2 || source.includes(witness.originalImport)) throw new Error(`changed private reader caller bindings: ${path}`);
  sources.assertFile(witness.helper);
  const helper = sources.read(witness.helper);
  if (digest(helper) !== witness.helperSha256) throw new Error(`changed private reader helper: ${witness.helper}`);
  const references = inspectRustCompileReferences(helper);
  if (references.length !== witness.inputs.length) throw new Error(`changed private reader input count: ${witness.helper}`);
  for (const [index, input] of witness.inputs.entries()) {
    const reference = references[index];
    const target = relative(root, resolve(root, dirname(witness.helper), input.include)).split(sep).join("/");
    if (reference?.kind !== "include_str" || reference.path !== input.include || target !== input.path) throw new Error(`changed private reader input binding: ${input.path}`);
    sources.assertFile(input.path);
    if (digest(sources.read(input.path)) !== input.currentSha256) throw new Error(`changed private reader input bytes: ${input.path}`);
  }
  const original = source.replace(witness.currentRegion, witness.originalRegion).replace(witness.importAnchor, witness.importAnchor + witness.originalImport);
  if (original !== witness.originalSource) throw new Error(`changed retained private reader caller bytes: ${path}`);
  return original;
}
