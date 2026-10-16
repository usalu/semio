/** 📤️ Test results
 *
 * The result record of one test execution and its reader.
 */

//#region 🔌️Adapters
import { type Implementation, type MutationOutcomeClass, type PlatformId, type TestRole, IMPLEMENTATIONS, TEST_ROLES } from "../../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { existsSync, readFileSync } from "node:fs";
import { TEST_LEVELS, type TestLevel } from "../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { isPlainObject } from "../🟰️comparison/🟦️.ts";
import { type ResultArtifact } from "../🔬️probes/🟦️.ts";
//#endregion 🔌️Adapters

//#region 📤️Results
/** 📤️ One executed `(scenario, implementation, role)` — the single shape every native host emits. */
export type TestResult = Readonly<{
  schemaVersion?: 2;
  testId: string;
  baselineSha?: string;
  owner: string;
  case: string;
  scenario: string;
  implementation: Implementation;
  role: TestRole;
  level: TestLevel;
  platform?: PlatformId;
  status: "passed" | "failed" | "errored";
  durationMs: number;
  seed?: string;
  featureHash?: string;
  fixtureHash?: string;
  sourceHash?: string;
  dependencyFingerprint?: string;
  runKey?: string;
  mutation?: string;
  outcome?: MutationOutcomeClass;
  /** 🏭️ Proof this execution reached PRODUCTION dispatch. A subject result without it may never count toward production-bridge coverage — replaying a committed vector is exactly what it exists to distinguish. */
  productionDispatch?: { invoked: true; operation: string; bridgeVersion: number };
  /** 📦️ Every file this execution produced, addressed by ROLE so a comparison stage never names a path. */
  artifacts?: readonly ResultArtifact[];
  output: { rawHash: string; projectionHash: string; rawPath?: string; projectionPath?: string; projection?: unknown };
  diagnostics: readonly { severity: "info" | "warning" | "error"; message: string; detail?: string }[];
}>;

/** 📤️ Validates one host-emitted record against the owned result schema, returning its problems. */
export function validateResult(value: unknown): string[] {
  const problems: string[] = [];
  if (!isPlainObject(value)) return ["result is not an object"];
  for (const key of ["testId", "owner", "case", "scenario", "implementation", "role", "level", "status", "durationMs", "output"]) if (value[key] === undefined) problems.push(`missing field ${key}`);
  if (value.implementation !== undefined && !(IMPLEMENTATIONS as readonly string[]).includes(String(value.implementation))) problems.push(`unknown implementation ${String(value.implementation)}`);
  if (value.role !== undefined && !(TEST_ROLES as readonly string[]).includes(String(value.role))) problems.push(`unknown role ${String(value.role)}`);
  if (value.level !== undefined && !(TEST_LEVELS as readonly string[]).includes(String(value.level))) problems.push(`unknown level ${String(value.level)}`);
  if (value.status !== undefined && !["passed", "failed", "errored"].includes(String(value.status))) problems.push(`unknown status ${String(value.status)}`);
  if (value.output !== undefined && !isPlainObject(value.output)) problems.push("output is not an object");
  return problems;
}

/** 📤️ Reads a host's JSONL result stream, rejecting malformed records loudly rather than skipping them. */
export function readResults(absPath: string): { results: TestResult[]; problems: string[] } {
  if (!existsSync(absPath)) return { results: [], problems: [`no result stream at ${absPath}`] };
  const results: TestResult[] = [];
  const problems: string[] = [];
  for (const [index, line] of readFileSync(absPath, "utf8").split(/\r?\n/).entries()) {
    if (line.trim() === "") continue;
    let parsed: unknown;
    try {
      parsed = JSON.parse(line);
    } catch (error) {
      problems.push(`line ${index + 1}: not JSON (${(error as Error).message})`);
      continue;
    }
    const issues = validateResult(parsed);
    if (issues.length > 0) problems.push(`line ${index + 1}: ${issues.join("; ")}`);
    else results.push(parsed as TestResult);
  }
  return { results, problems };
}
//#endregion 📤️Results
