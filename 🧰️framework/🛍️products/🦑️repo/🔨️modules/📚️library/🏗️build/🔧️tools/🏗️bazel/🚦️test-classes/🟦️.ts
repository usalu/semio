/** 🚥️ Bazel test classes: the size, timeout, shard count and retry policy of a generated test, how a source set of any size enters it, and how Bazel's own test environment becomes the neutral test protocol. */
import { existsSync, mkdirSync, writeFileSync } from "node:fs";
import { dirname, isAbsolute } from "node:path";
import { validateJsonSchemaSubset } from "../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import { renderTestReportV1, syntheticTestSuiteV1, TEST_PROTOCOL_NAMES_V1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🚦️protocol/🟦️.ts";
import table from "./🔣️.json" with { type: "json" };
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

export type BazelTestSizeV1 = "small" | "medium" | "large" | "enormous";
export type BazelTestTimeoutV1 = "short" | "moderate" | "long" | "eternal";
export type BazelTestRuleV1 = Readonly<{ id: string; evidence: "measured" | "declared"; reason: string; targets: readonly string[]; projects?: readonly string[]; size?: BazelTestSizeV1; shards?: number; flaky?: boolean }>;
export type BazelTestTableV1 = Readonly<{ version: 1; sizes: Readonly<Record<BazelTestSizeV1, Readonly<{ timeout: BazelTestTimeoutV1; seconds: number }>>>; levels: Readonly<Record<"fundamental" | "quick" | "long" | "exhaustive", BazelTestSizeV1>>; witness: Readonly<{ from: number }>; measure: Readonly<{ headroom: number; minimumRuns: number; shardAboveSeconds: number }>; default: Readonly<{ size: BazelTestSizeV1; shards: 1; flaky: false }>; rules: readonly BazelTestRuleV1[]; messages: Readonly<{ shardNotHonoured: Readonly<{ en: string; de: string }> }> }>;
export type BazelTestInputV1 = Readonly<{ project: string; target: string; files: number }>;
export type BazelTestPlanV1 = Readonly<{ size: BazelTestSizeV1; timeout: BazelTestTimeoutV1; shards: number; flaky: boolean; witness: boolean; rule: string | null }>;

/** 🔐️ Admits a class table against its schema; the authored table and every fixture table go through it. */
export const admitBazelTestClassesV1 = (value: unknown): BazelTestTableV1 => {
  if (validateJsonSchemaSubset(schema.definitions.Table, value, schema).length) throw Error("Closed Bazel test class table required");
  return value as BazelTestTableV1;
};

/** 📚️ The authored table. */
export const BAZEL_TEST_CLASSES_V1: BazelTestTableV1 = admitBazelTestClassesV1(table);

const globs = new Map<string, RegExp>();
const matches = (glob: string, text: string): boolean => {
  let row = globs.get(glob);
  if (!row) {
    row = new RegExp("^" + glob.replace(/[.+^$()|]/gu, "\\$&").replace(/\*/gu, ".*").replace(/\?/gu, ".") + "$", "su");
    globs.set(glob, row);
  }
  return row.test(text);
};

/** 🧮️ Tells whether a test's declared source set enters it as one input witness: Bazel links every runfile of a test before every run, cache hits included (measured: 60,000 files cost 25 s cold and 23 s after a one-file edit against 4 s and 6 s for a witness; at 2,000 files the difference is below the noise). */
export function bazelTestWitnessV1(files: number, classes: BazelTestTableV1 = BAZEL_TEST_CLASSES_V1): boolean {
  if (!Number.isSafeInteger(files) || files < 0) throw Error("Finite source file count required");
  return files > classes.witness.from;
}

/** 🏷️ Chooses the class of one test: the first rule that names its target (and project, when it names any) decides size, shards and retry, everything else is the default; the witness follows the source count alone. */
export function bazelTestPlanV1(input: BazelTestInputV1, classes: BazelTestTableV1 = BAZEL_TEST_CLASSES_V1): BazelTestPlanV1 {
  if (validateJsonSchemaSubset(schema.definitions.Input, input, schema).length) throw Error("Closed Bazel test input required");
  const chosen = classes.rules.find((row) => row.targets.some((glob) => matches(glob, input.target)) && (row.projects === undefined || row.projects.some((glob) => matches(glob, input.project))));
  const size = chosen?.size ?? classes.default.size;
  return Object.freeze({ size, timeout: classes.sizes[size].timeout, shards: chosen?.shards ?? classes.default.shards, flaky: chosen?.flaky ?? classes.default.flaky, witness: bazelTestWitnessV1(input.files, classes), rule: chosen?.id ?? null });
}

/** 🔁️ Names the Bazel attributes of a plan, in the order a generated rule lists them; a shard count of one and a stable test list nothing. */
export function bazelTestAttributesV1(plan: BazelTestPlanV1): readonly (readonly [string, string])[] {
  return [["size", JSON.stringify(plan.size)], ["timeout", JSON.stringify(plan.timeout)], ...(plan.shards > 1 ? [["shard_count", String(plan.shards)] as const] : []), ...(plan.flaky ? [["flaky", "True"] as const] : [])];
}

const names = TEST_PROTOCOL_NAMES_V1;

/** 🌉️ Maps the variables Bazel sets for one test run to the neutral protocol: shard index and total become `index/total` (1-based), the status file and the XML file keep their paths. An unsharded run maps no shard. */
export function bazelTestEnvironmentV1(environment: Readonly<Record<string, string | undefined>>): Record<string, string> {
  const row: Record<string, string> = {};
  for (const key of ["TEST_SHARD_INDEX", "TEST_TOTAL_SHARDS", "TEST_SHARD_STATUS_FILE", "XML_OUTPUT_FILE"]) if (environment[key] !== undefined && environment[key] !== "") row[key] = environment[key]!;
  if (validateJsonSchemaSubset(schema.definitions.BazelEnvironment, row, schema).length) throw Error("Closed Bazel test environment required");
  const result: Record<string, string> = {};
  const total = row.TEST_TOTAL_SHARDS === undefined ? 1 : Number(row.TEST_TOTAL_SHARDS);
  if (total > 1) {
    const index = Number(row.TEST_SHARD_INDEX ?? "x");
    if (!Number.isSafeInteger(index) || index >= total) throw Error("Bazel shard index inside its total required");
    result[names.shard] = `${index + 1}/${total}`;
    if (row.TEST_SHARD_STATUS_FILE !== undefined) result[names.shardStatusFile] = row.TEST_SHARD_STATUS_FILE;
  }
  if (row.XML_OUTPUT_FILE !== undefined) result[names.xmlOutputFile] = row.XML_OUTPUT_FILE;
  for (const key of [names.shardStatusFile, names.xmlOutputFile]) if (result[key] !== undefined && !isAbsolute(result[key]!)) throw Error("Absolute Bazel test paths required");
  return result;
}

/** 🧾️ Renders the result Bazel is given when no command of the test wrote one: one suite, one case, the failure message when the test failed. */
export function bazelTestFallbackReportV1(input: { label: string; durationMs: number; failure?: string }): string {
  return renderTestReportV1([syntheticTestSuiteV1({ name: input.label, project: "bazel", durationMs: input.durationMs, ...(input.failure === undefined ? {} : { failure: input.failure }) })]);
}

/** 🏁️ Settles what Bazel expects of a finished test and the protocol could not deliver: a shard no command honoured still advertises support (with a visible warning that the shard ran the whole task, which is correct and only slower), and a test no command reported gets one fallback result. */
export function settleBazelTestV1(environment: Readonly<Record<string, string | undefined>>, input: { label: string; durationMs: number; failure?: string }, classes: BazelTestTableV1 = BAZEL_TEST_CLASSES_V1): void {
  const status = environment.TEST_SHARD_STATUS_FILE;
  if (status && !existsSync(status)) {
    writeFileSync(status, "");
    const text = (language: "en" | "de"): string => classes.messages.shardNotHonoured[language].replace("${label}", input.label).replace("${index}", String(Number(environment.TEST_SHARD_INDEX) + 1)).replace("${total}", environment.TEST_TOTAL_SHARDS ?? "?");
    process.stderr.write(`[shard] ${text("en")} | ${text("de")}\n`);
  }
  const xml = environment.XML_OUTPUT_FILE;
  if (xml && !existsSync(xml)) {
    mkdirSync(dirname(xml), { recursive: true });
    writeFileSync(xml, bazelTestFallbackReportV1(input));
  }
}
