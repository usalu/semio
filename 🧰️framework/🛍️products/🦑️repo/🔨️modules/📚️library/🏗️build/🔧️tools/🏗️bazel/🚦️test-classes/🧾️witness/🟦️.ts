/** 🧾️ The input witness: one small source file per witnessed source group whose content is the digest of every declared file's path, size and content sha256, so Bazel keys and links one input where it would link every source as a runfile; the publisher writes it from the capture manifest after every capture, so it changes exactly when a planned byte changes, and no Bazel action re-reads the sources. */
import { createHash } from "node:crypto";
import { readFile } from "node:fs/promises";
import { join } from "node:path";
import { validateJsonSchemaSubset } from "../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const policy = schema.definitions.Policy.const;
/** 📛️ The plan file the planner writes and the suffix of a witness file. */
export const bazelWitnessFilesV1 = Object.freeze({ plan: policy.plan, suffix: policy.suffix });
/** 🗒️ Per witness name the repository paths of its declarable files. */
export type BazelWitnessPlanV1 = Readonly<{ version: 1; witnesses: Readonly<Record<string, readonly string[]>> }>;
/** 📄️ The content of one witness file. */
export type BazelWitnessV1 = { version: 1; files: number; sha256: string };
/** 📇️ What the publisher knows of one captured file. */
export type BazelWitnessRowV1 = Readonly<{ path: string; bytes: number; sha256: string }>;

const order = (left: string, right: string) => (left < right ? -1 : left > right ? 1 : 0);

/** 🧮️ The witness of captured rows: `path NUL size NUL content-sha256 LF` over the paths in UTF-16 code-unit order. */
export function bazelWitnessOfRowsV1(rows: readonly BazelWitnessRowV1[]): BazelWitnessV1 {
  const outer = createHash("sha256");
  for (const row of [...rows].sort((left, right) => order(left.path, right.path))) outer.update(`${row.path}\0${row.bytes}\0${row.sha256}\n`);
  return { version: 1, files: rows.length, sha256: outer.digest("hex") };
}

/** 🧪️ The reference digest of files given by their bytes, for tests and tools that must reproduce a witness from the files themselves. */
export function bazelWitnessDigestV1(files: readonly Readonly<{ path: string; bytes: Uint8Array }>[]): BazelWitnessV1 {
  return bazelWitnessOfRowsV1(files.map((file) => ({ path: file.path, bytes: file.bytes.length, sha256: createHash("sha256").update(file.bytes).digest("hex") })));
}

/** 📝️ The plan text the planner writes: names sorted, paths sorted and unique. */
export function bazelWitnessPlanTextV1(witnesses: ReadonlyMap<string, Iterable<string>>): string {
  const plan: BazelWitnessPlanV1 = { version: 1, witnesses: Object.fromEntries([...witnesses].sort(([left], [right]) => order(left, right)).map(([name, paths]) => [name, [...new Set(paths)].sort(order)])) };
  if (validateJsonSchemaSubset(schema.definitions.Plan, plan, schema).length) throw Error("bazel-witness:plan-closed-required");
  return JSON.stringify(plan);
}

/** 📖️ Reads the witness plan of a workspace; a workspace without witnessed tests has none. */
export async function readBazelWitnessPlanV1(workspace: string): Promise<BazelWitnessPlanV1 | undefined> {
  let text: string;
  try { text = await readFile(join(workspace, policy.plan), "utf8"); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return undefined; throw error; }
  if (Buffer.byteLength(text) > policy.maximumPlanBytes) throw Error("bazel-witness:plan-bounded-required");
  const plan = JSON.parse(text) as BazelWitnessPlanV1;
  if (validateJsonSchemaSubset(schema.definitions.Plan, plan, schema).length) throw Error("bazel-witness:plan-closed-required");
  return plan;
}

/** ✍️ The witness files of a plan from the captured rows, as `[file name, text]`; a planned path the capture does not hold fails closed (the capture runs first and refuses vanished files itself). */
export function bazelWitnessFilesOfPlanV1(plan: BazelWitnessPlanV1, rows: ReadonlyMap<string, BazelWitnessRowV1>): [string, string][] {
  return Object.entries(plan.witnesses).map(([name, paths]) => {
    const found = paths.map((path) => { const row = rows.get(path); if (!row) throw Error(`bazel-witness:input-required:${path}`); return row; });
    return [name + policy.suffix, JSON.stringify(bazelWitnessOfRowsV1(found)) + "\n"];
  });
}
