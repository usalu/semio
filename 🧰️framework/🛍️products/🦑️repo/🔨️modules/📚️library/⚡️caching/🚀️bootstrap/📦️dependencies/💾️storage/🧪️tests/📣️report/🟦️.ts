import { expect, test } from "bun:test";
import { concurrentEventV1 } from "../../../🚦️concurrent/🟦️.ts";
import { javascriptDependencyConcurrencyReportV1 } from "../../🟦️.ts";

const control = Object.freeze({ signal: new AbortController().signal, remainingMilliseconds: () => null, publish: () => undefined, yieldContinuation: async () => undefined });

/** 📣️ Records every write of both process streams while the body runs. */
async function record(body: () => Promise<void>): Promise<Readonly<{ stdout: string[]; stderr: string[] }>> {
  const out: string[] = [], err: string[] = [], writeOut = process.stdout.write, writeErr = process.stderr.write;
  const capture = (rows: string[]) => ((chunk: unknown, ...rest: unknown[]) => { rows.push(String(chunk)); const done = rest.find((value) => typeof value === "function") as (() => void) | undefined; done?.(); return true; }) as typeof process.stdout.write;
  process.stdout.write = capture(out);
  process.stderr.write = capture(err);
  try { await body(); } finally { process.stdout.write = writeOut; process.stderr.write = writeErr; }
  return { stdout: out, stderr: err };
}

for (const [resource, language, text] of [["build", "en", "Another process is building the dependencies"], ["build", "de", "Ein anderer Prozess"], ["writer", "en", "Waiting for another"]] as const) {
  test(`the ${resource} wait is reported on the diagnostic stream in ${language} and never on the result stream`, async () => {
    const report = javascriptDependencyConcurrencyReportV1(control, language), written = await record(() => report(concurrentEventV1({ event: "waiting", resource, elapsedMs: 10_000, scope: "scope", epoch: 1, count: 1 })));
    expect(written.stdout).toEqual([]);
    expect(written.stderr.some((line) => line.startsWith("[DATA] Store dependency concurrency "))).toBe(true);
    expect(written.stderr.some((line) => line.includes(text))).toBe(true);
  });
}
