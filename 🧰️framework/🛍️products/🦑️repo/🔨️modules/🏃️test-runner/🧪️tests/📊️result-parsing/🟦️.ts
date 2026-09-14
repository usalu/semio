//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.

//#endregion 🧲️Header

//#region 🔌️Adapters
import { defineTestAdapter } from "../../../🧪️test/📦️packages/🟦️typescript/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🔖️Second Parser
// 🔮️ An independently written second parser for the six runner dialects, and the only reference this
// capability has, registered as `repo-test-runner-second-parser`. It reads the same
// recorded transcripts as the Rust implementation and projects the same model — it deliberately
// shares no code with it, which is what makes the pairwise comparison evidence rather than an echo.

type TestStatus = "passed" | "failed" | "skipped";
type RunStatus = "passed" | "failed" | "cancelled" | "not-run";
type Case = { name: string; suite?: string; status: TestStatus; duration_ms?: number; message?: string };
type Outcome = { runner: string; status: RunStatus; tests: Case[]; totals: { passed: number; failed: number; skipped: number; total: number }; exit_status?: number };
type Output = { status: number; stdout?: string; stderr?: string };
type Vector = { id: string; runner: string; output: Output };

function makeCase(name: string, suite: string, status: TestStatus, durationMs: number | null, message: string): Case {
  const built: Case = { name, status };
  if (suite !== "") built.suite = suite;
  if (durationMs !== null) built.duration_ms = durationMs;
  if (message !== "") built.message = message;
  return built;
}

function finish(runner: string, tests: Case[], output: Output): Outcome {
  const totals = { passed: 0, failed: 0, skipped: 0, total: tests.length };
  for (const one of tests) totals[one.status] += 1;
  return { runner, status: totals.failed > 0 || output.status !== 0 ? "failed" : "passed", tests, totals, exit_status: output.status };
}

/** 🧹️ The JSON document inside output that may carry leading runner chatter. */
function jsonBody(text: string): string {
  const start = text.indexOf("{") < 0 ? 0 : text.indexOf("{");
  const end = text.lastIndexOf("}") < 0 ? text.length : text.lastIndexOf("}") + 1;
  return start < end ? text.slice(start, end) : text;
}

function lines(text: string): string[] {
  const split = text.split("\n");
  return split[split.length - 1] === "" ? split.slice(0, -1) : split;
}

function trimEnd(text: string): string {
  return text.replace(/\s+$/u, "");
}

function goTestJson(output: Output): Outcome {
  const messages = new Map<string, string>();
  const tests: Case[] = [];
  for (const line of lines(output.stdout ?? "")) {
    let event: Record<string, unknown>;
    try {
      event = JSON.parse(line.trim()) as Record<string, unknown>;
    } catch {
      continue;
    }
    if (typeof event !== "object" || event === null) continue;
    const action = typeof event.Action === "string" ? event.Action : "";
    const pkg = typeof event.Package === "string" ? event.Package : "";
    if (typeof event.Test !== "string") continue;
    const name = event.Test;
    const key = `${pkg}::${name}`;
    if (action === "output") {
      messages.set(key, (messages.get(key) ?? "") + (typeof event.Output === "string" ? event.Output : ""));
      continue;
    }
    const status: TestStatus | null = action === "pass" ? "passed" : action === "fail" ? "failed" : action === "skip" ? "skipped" : null;
    if (status === null) continue;
    const elapsed = typeof event.Elapsed === "number" ? event.Elapsed * 1000 : null;
    tests.push(makeCase(name, pkg, status, elapsed, status === "failed" ? trimEnd(messages.get(key) ?? "") : ""));
  }
  return finish("go", tests, output);
}

function vitestJson(runner: string, output: Output): Outcome {
  const tests: Case[] = [];
  let report: Record<string, unknown> | null = null;
  try {
    const parsed: unknown = JSON.parse(jsonBody(output.stdout ?? ""));
    report = typeof parsed === "object" && parsed !== null && !Array.isArray(parsed) ? (parsed as Record<string, unknown>) : null;
  } catch {
    report = null;
  }
  const files = Array.isArray(report?.testResults) ? (report.testResults as Record<string, unknown>[]) : [];
  for (const file of files) {
    const suite = typeof file.name === "string" ? file.name : "";
    const assertions = Array.isArray(file.assertionResults) ? (file.assertionResults as Record<string, unknown>[]) : [];
    for (const assertion of assertions) {
      const name = typeof assertion.fullName === "string" ? assertion.fullName : typeof assertion.title === "string" ? assertion.title : "";
      const raw = typeof assertion.status === "string" ? assertion.status : "";
      const status: TestStatus = raw === "passed" ? "passed" : raw === "failed" ? "failed" : "skipped";
      const failures = Array.isArray(assertion.failureMessages) ? (assertion.failureMessages as unknown[]).filter((value): value is string => typeof value === "string") : [];
      tests.push(makeCase(name, suite, status, typeof assertion.duration === "number" ? assertion.duration : null, failures.join("\n")));
    }
  }
  return finish(runner, tests, output);
}

function pytestReport(output: Output): Outcome {
  const tests: Case[] = [];
  for (const line of lines(output.stdout ?? "")) {
    const trimmed = trimEnd(line);
    const space = trimmed.indexOf(" ");
    if (space < 0) continue;
    const node = trimmed.slice(0, space);
    if (!node.includes("::")) continue;
    const verdict = trimmed.slice(space + 1).split(/\s+/u)[0] ?? "";
    const status: TestStatus | null =
      verdict === "PASSED" || verdict === "XPASS" ? "passed" : verdict === "FAILED" || verdict === "ERROR" ? "failed" : verdict === "SKIPPED" || verdict === "XFAIL" ? "skipped" : null;
    if (status === null) continue;
    const marker = node.indexOf("::");
    tests.push(makeCase(node.slice(marker + 2), node.slice(0, marker), status, null, ""));
  }
  return finish("pytest", tests, output);
}

function cargoFailureBody(stdout: string, name: string): string {
  const header = `---- ${name} stdout ----`;
  const index = stdout.indexOf(header);
  if (index < 0) return "";
  const rest = stdout.slice(index + header.length);
  const end = rest.indexOf("\n----") < 0 ? rest.length : rest.indexOf("\n----");
  return rest.slice(0, end).trim();
}

function cargoLibtest(output: Output): Outcome {
  const stdout = output.stdout ?? "";
  const tests: Case[] = [];
  for (const line of lines(stdout)) {
    const trimmed = line.trim();
    if (!trimmed.startsWith("test ")) continue;
    const rest = trimmed.slice("test ".length);
    const marker = rest.indexOf(" ... ");
    if (marker < 0) continue;
    const name = rest.slice(0, marker).trim();
    const verdict = rest.slice(marker + " ... ".length).trim();
    const status: TestStatus | null = verdict === "ok" ? "passed" : verdict === "FAILED" ? "failed" : verdict.startsWith("ignored") ? "skipped" : null;
    if (status === null) continue;
    tests.push(makeCase(name, "", status, null, status === "failed" ? cargoFailureBody(stdout, name) : ""));
  }
  return finish("cargo", tests, output);
}

function cargoNextest(output: Output): Outcome {
  const tests: Case[] = [];
  for (const line of [...lines(output.stdout ?? ""), ...lines(output.stderr ?? "")]) {
    const trimmed = line.trim();
    const space = trimmed.indexOf(" ");
    if (space < 0) continue;
    const verdict = trimmed.slice(0, space);
    const status: TestStatus | null = verdict === "PASS" ? "passed" : verdict === "FAIL" || verdict === "TRY" ? "failed" : verdict === "SKIP" ? "skipped" : null;
    if (status === null) continue;
    const rest = trimmed.slice(space + 1).trim();
    const close = rest.indexOf("]");
    if (close < 0) continue;
    const seconds = Number(rest.slice(0, close).replace(/^\[/u, "").trim().replace(/s$/u, "").trim());
    const identifier = rest.slice(close + 1).trim();
    const gap = identifier.indexOf(" ");
    const suite = gap < 0 ? "" : identifier.slice(0, gap).trim();
    const name = gap < 0 ? identifier : identifier.slice(gap + 1).trim();
    tests.push(makeCase(name, suite, status, Number.isFinite(seconds) ? seconds * 1000 : null, ""));
  }
  return finish("cargo-nextest", tests, output);
}

function dotnetDuration(value: string): number | null {
  const trimmed = value.trim();
  const space = trimmed.indexOf(" ");
  if (space < 0) return null;
  const amount = Number(trimmed.slice(0, space));
  if (!Number.isFinite(amount)) return null;
  const unit = trimmed.slice(space + 1);
  return unit === "ms" ? amount : unit === "s" ? amount * 1000 : unit === "us" ? amount / 1000 : null;
}

function dotnetConsole(output: Output): Outcome {
  const tests: Case[] = [];
  for (const line of lines(output.stdout ?? "")) {
    const trimmed = line.trim();
    const space = trimmed.indexOf(" ");
    if (space < 0) continue;
    const verdict = trimmed.slice(0, space);
    const status: TestStatus | null = verdict === "Passed" ? "passed" : verdict === "Failed" ? "failed" : verdict === "Skipped" ? "skipped" : null;
    if (status === null) continue;
    const rest = trimmed.slice(space + 1).trim();
    if (rest === "" || rest.startsWith("!") || rest.startsWith("-")) continue;
    const marker = rest.lastIndexOf(" [");
    const name = marker < 0 ? rest : rest.slice(0, marker).trim();
    const duration = marker < 0 ? null : dotnetDuration(rest.slice(marker + 2).replace(/\]$/u, ""));
    tests.push(makeCase(name, "", status, duration, ""));
  }
  return finish("dotnet", tests, output);
}

function rspecJson(output: Output): Outcome {
  const tests: Case[] = [];
  let report: Record<string, unknown> | null = null;
  try {
    const parsed: unknown = JSON.parse(jsonBody(output.stdout ?? ""));
    report = typeof parsed === "object" && parsed !== null && !Array.isArray(parsed) ? (parsed as Record<string, unknown>) : null;
  } catch {
    report = null;
  }
  const examples = Array.isArray(report?.examples) ? (report.examples as Record<string, unknown>[]) : [];
  for (const example of examples) {
    const name = typeof example.full_description === "string" ? example.full_description : typeof example.description === "string" ? example.description : "";
    const raw = typeof example.status === "string" ? example.status : "";
    const status: TestStatus = raw === "passed" ? "passed" : raw === "failed" ? "failed" : "skipped";
    const exception = typeof example.exception === "object" && example.exception !== null ? (example.exception as Record<string, unknown>) : {};
    const suite = typeof example.file_path === "string" ? example.file_path : "";
    const runTime = typeof example.run_time === "number" ? example.run_time * 1000 : null;
    tests.push(makeCase(name, suite, status, runTime, typeof exception.message === "string" ? exception.message : ""));
  }
  return finish("rspec", tests, output);
}

function parseOutcome(runner: string, output: Output): Outcome {
  switch (runner) {
    case "go":
      return goTestJson(output);
    case "cargo":
      return cargoLibtest(output);
    case "cargo-nextest":
      return cargoNextest(output);
    case "dotnet":
      return dotnetConsole(output);
    case "npx":
    case "npm":
      return vitestJson(runner, output);
    case "uv":
    case "pytest":
      return pytestReport(output);
    case "rspec":
      return rspecJson(output);
    default:
      return finish(runner, [], output);
  }
}
//#endregion 🔖️Second Parser

//#region 🧭️Adapter
function vectors(ctx: { fixtureBytes(uri: string): Uint8Array }): Vector[] {
  const text = new TextDecoder().decode(ctx.fixtureBytes("shared://📜️runner-transcripts.json"));
  return (JSON.parse(text) as { vectors: Vector[] }).vectors;
}

/** 🟦️ TypeScript second parser for the runner transcript case. */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "every-dialect-folds-into-one-model": {
      oracle: (ctx) => ({ projection: { parsed: vectors(ctx).map((vector) => ({ id: vector.id, outcome: parseOutcome(vector.runner, vector.output) })) } }),
    },
    "totals-are-recomputed-not-trusted": {
      oracle: (ctx) => ({
        projection: {
          totals: vectors(ctx).map((vector) => {
            const parsed = parseOutcome(vector.runner, vector.output);
            const passed = parsed.tests.filter((one) => one.status === "passed").length;
            const failed = parsed.tests.filter((one) => one.status === "failed").length;
            const skipped = parsed.tests.filter((one) => one.status === "skipped").length;
            return {
              id: vector.id,
              total: parsed.totals.total,
              passed: parsed.totals.passed,
              failed: parsed.totals.failed,
              skipped: parsed.totals.skipped,
              recountAgrees: passed === parsed.totals.passed && failed === parsed.totals.failed && skipped === parsed.totals.skipped && parsed.tests.length === parsed.totals.total,
            };
          }),
        },
      }),
    },
    "unparseable-output-is-an-empty-run": {
      oracle: (ctx) => {
        const vector = vectors(ctx).find((entry) => entry.id === "unparseable-output-yields-no-tests");
        if (vector === undefined) throw new Error("the transcript fixture has no unparseable vector");
        const parsed = parseOutcome(vector.runner, vector.output);
        return { projection: { tests: parsed.tests.length, status: parsed.status === "failed" ? "failed" : "not-failed", total: parsed.totals.total, exitStatus: parsed.exit_status ?? 0 } };
      },
    },
  },
});
//#endregion 🧭️Adapter
