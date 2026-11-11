import { expect, test } from "bun:test";
import Ajv from "ajv";
import { LanguageTextErrorV1, bindCatalogV1 } from "../../📖️catalog/🟦️.ts";
import { CargoLockRefusedV1 } from "../../../../🦀️cargo-workspace/🔒️lock/🟦️.ts";
import { PROGRESS_LEVELS_V1, createCliSinkV1, emitProgressV1, progressV1, renderProgressV1, reportBoundaryErrorV1, reportRepoErrorV1, sayV1, withProgressSinkV1, type ProgressEventV1, type ProgressLevelV1 } from "../🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };
import { isRepoErrorV1 } from "../../🚨️error/🟦️.ts";
import { OwnedCommandFailure, isOwnedCommandFailureV1, ownedCommandExitCodeV1, runOwnedCommand } from "../../../../🎛️owned-execution/🟦️.ts";
import outcomeSchema from "../../../../🎛️owned-execution/📪️outcome/🧬️schema/🔣️.json" with { type: "json" };
import { refuseProcessV1 } from "../../../../📖️texts/🟦️.ts";

type EventRow = Readonly<{ name: string; key: string; values: Readonly<Record<string, string | number>>; level: ProgressLevelV1; en: string; de: string }>;

const steps = bindCatalogV1("steps", corpus.catalog);
const events = corpus.events as unknown as readonly EventRow[];

test("the corpus wire forms agree with the strict schema validator", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  for (const row of corpus.wire) expect(ajv.validate(schema, row.data), `${row.name} ${JSON.stringify(ajv.errors)}`).toBe(row.valid);
  expect([...PROGRESS_LEVELS_V1]).toEqual(schema.properties.level.enum);
});

for (const row of events) {
  test(`event: ${row.name}`, () => {
    const event = progressV1(steps, row.key, row.values, row.level);
    expect(event).toEqual({ version: 1, key: `steps.${row.key}`, values: row.values, level: row.level } as never);
    expect(renderProgressV1(event, "en")).toBe(row.en);
    expect(renderProgressV1(event, "de")).toBe(row.de);
    const lines: string[] = [];
    createCliSinkV1({ environment: { SEMIO_LANGUAGE: "de" }, osLanguages: () => [], write: (line) => lines.push(line) })(event);
    expect(lines).toEqual([row.de]);
  });
}

test("an event carries data and is checked when it is built", () => {
  expect(() => progressV1(steps, "start", {})).toThrow(LanguageTextErrorV1);
  expect(() => progressV1(steps, "absent")).toThrow(LanguageTextErrorV1);
  expect(Object.isFrozen(progressV1(steps, "done"))).toBe(true);
});

test("a scope receives every event emitted inside it, across awaits, and nothing leaks outside", async () => {
  const seen: ProgressEventV1[] = [];
  await withProgressSinkV1((event) => seen.push(event), async () => {
    sayV1(steps, "done");
    await Promise.resolve();
    emitProgressV1(progressV1(steps, "start", { count: 2 }));
  });
  expect(seen.map((event) => event.key)).toEqual(["steps.done", "steps.start"]);
  const inner: string[] = [];
  withProgressSinkV1(() => undefined, () => withProgressSinkV1((event) => inner.push(event.key), () => sayV1(steps, "done")));
  expect(inner).toEqual(["steps.done"]);
});

test("the CLI sink renders in the language the shared resolver chooses and refuses bilingually when none is chosen", () => {
  const lines: string[] = [];
  const event = progressV1(steps, "done");
  createCliSinkV1({ environment: { LANG: "en_GB.UTF-8" }, osLanguages: () => [], write: (line) => lines.push(line) })(event);
  createCliSinkV1({ args: ["--lang=de"], environment: { LANG: "en_GB.UTF-8" }, osLanguages: () => [], write: (line) => lines.push(line) })(event);
  expect(lines).toEqual(["[steps] done.", "[steps] fertig."]);
  expect(() => createCliSinkV1({ environment: {}, osLanguages: () => [], write: () => undefined })(event)).toThrow(/--lang=en\|de[\s\S]*--lang=en\|de/u);
  expect(() => createCliSinkV1({ environment: { SEMIO_LANGUAGE: "fr" }, osLanguages: () => [], write: () => undefined })(event)).toThrow(/Unbekannte|nicht angebotene/u);
});

type ChildRow = Readonly<{ name: string; label: string; status: number | null; signal?: string; exitCode: number; en: string; de: string }>;

const children = corpus.children as unknown as readonly ChildRow[];
const signals = corpus.signals as unknown as readonly ChildRow[];

type Expected = Readonly<{ en: string; de: string; exitCode: number }>;

const reportInBoth = (error: unknown, expected: Expected) => {
  for (const [language, text] of [["en", expected.en], ["de", expected.de]] as const) {
    const lines: string[] = [];
    let status = -1;
    expect(reportRepoErrorV1(error, { environment: { SEMIO_LANGUAGE: language }, osLanguages: () => [], write: (line) => lines.push(line), setExitCode: (code) => { status = code; } })).toBe(true);
    expect(lines).toEqual([text]);
    expect(status).toBe(expected.exitCode);
  }
};

for (const row of children) {
  test(`a failing child becomes a clean refusal with its exit code: ${row.name}`, async () => {
    let failure: unknown;
    try {
      await runOwnedCommand(process.execPath, ["-e", `process.exit(${row.status})`], process.cwd(), row.label, 30_000, { stdout: "ignore" });
    } catch (error) {
      failure = error;
    }
    expect(failure).toBeInstanceOf(OwnedCommandFailure);
    expect(isOwnedCommandFailureV1(failure)).toBe(true);
    expect(new Ajv({ strict: true }).validate(outcomeSchema, (failure as OwnedCommandFailure).outcome)).toBe(true);
    expect((failure as OwnedCommandFailure).outcome).toEqual({ version: 1, status: row.status, signal: null });
    reportInBoth(failure, row);
  }, 30_000);
}

for (const row of signals) {
  test(`a child outcome is rendered with its exit code: ${row.name}`, () => {
    const failure = new OwnedCommandFailure(row.label, { version: 1, status: row.status, signal: row.signal ?? null });
    expect(ownedCommandExitCodeV1(failure.outcome)).toBe(row.exitCode);
    reportInBoth(failure, row);
  });
}

type StopRow = Readonly<{ name: string; label: string; kind: "pre-cancelled" | "cancel-after" | "timeout" | "output-limit"; abortAfterMs?: number; timeoutMs?: number; exitCode: number; en: string; de: string }>;

const stops = corpus.stops as unknown as readonly StopRow[];
const stopScripts = { "pre-cancelled": "process.exit(0)", "cancel-after": "setTimeout(() => {}, 10000)", timeout: "setTimeout(() => {}, 10000)", "output-limit": "process.stdout.write('x'.repeat(2000))" } as const;

for (const row of stops) {
  test(`a stopped command becomes a typed refusal with its exit code: ${row.name}`, async () => {
    const controller = new AbortController();
    if (row.kind === "pre-cancelled") controller.abort();
    const timer = row.kind === "cancel-after" ? setTimeout(() => controller.abort(), row.abortAfterMs) : undefined;
    let failure: unknown;
    try {
      await runOwnedCommand(process.execPath, ["-e", stopScripts[row.kind]], process.cwd(), row.label, row.kind === "timeout" ? row.timeoutMs! : 30_000, { stdout: "ignore", signal: controller.signal, ...(row.kind === "output-limit" ? { output: { maximumBytes: 100, maximumLines: 10 } } : {}) });
    } catch (error) {
      failure = error;
    } finally {
      clearTimeout(timer);
    }
    expect(isRepoErrorV1(failure)).toBe(true);
    expect(isOwnedCommandFailureV1(failure)).toBe(false);
    reportInBoth(failure, row);
  }, 30_000);
}

test("a plain error is not a child failure and stays unhandled at the boundary", () => {
  expect(isOwnedCommandFailureV1(new Error("plain"))).toBe(false);
  expect(reportRepoErrorV1(new Error("plain"), { write: () => { throw new Error("must not write"); } })).toBe(false);
});

type BoundaryRow = Readonly<{ name: string; kind: "cargo-lock" | "error" | "string" | "refusal"; message: string; values?: Readonly<Record<string, string | number>>; en: readonly string[]; de: readonly string[]; exitCode: number }>;

for (const row of corpus.boundary as unknown as readonly BoundaryRow[]) {
  test(`boundary: ${row.name}`, () => {
    const thrown = row.kind === "cargo-lock" ? new CargoLockRefusedV1("en", null, { en: "Short resolver refusal", de: "Kurze Resolver-Ablehnung" }, { version: 1, status: 101, stderr: "original resolver output", stdout: "", truncated: false }) : row.kind === "refusal" ? refuseProcessV1("refused", row.message as "source.read-bytes-exceeded", row.values) : row.kind === "error" ? new Error(row.message) : row.message;
    for (const language of ["en", "de"] as const) {
      const lines: string[] = [];
      let status = -1;
      reportBoundaryErrorV1(thrown, { environment: { SEMIO_LANGUAGE: language }, osLanguages: () => [], write: (line) => lines.push(line), setExitCode: (code) => { status = code; } });
      expect(lines).toEqual(row[language]);
      expect(status).toBe(row.exitCode);
      expect(lines.join("\n")).not.toMatch(/\n\s+at |Bun v/u);
    }
  });
}
