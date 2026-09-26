/** 🔁️ The execution-target retry policy (`🏪️store/👷️worker/🔁️execution-target-retry/🔣️.json`) on its own: which answers are asked
 * again, how often, after which waits, and that a cancel ends the wait at once. The npm `retry` package's exponential schedule
 * (`retry.timeouts`, randomize off) is the independent oracle for every backoff delay. */
import { describe, expect, it } from "vitest";
import { createRequire } from "node:module";
import { EXECUTION_TARGET_RETRY_V1, executionTargetRetryDelayV1, executionTargetRetrySleepV1, executionTargetTransientStatusV1, requestExecutionTargetAssetV1 } from "../../🔨️modules/🏪️store/👷️worker/🔁️execution-target-retry/🟦️.ts";

const retryOracle = createRequire(import.meta.url)("retry") as { timeouts(options: { retries: number; factor: number; minTimeout: number; maxTimeout: number; randomize: boolean }): number[] };
const policy = EXECUTION_TARGET_RETRY_V1;

describe("🔁️ execution-target retry policy", () => {
  it("waits exactly the exponential schedule the retry package computes", () => {
    const oracle = retryOracle.timeouts({ retries: 12, factor: policy.backoffFactor, minTimeout: policy.backoffInitialMs, maxTimeout: policy.backoffMaxMs, randomize: false });
    expect(Array.from({ length: 12 }, (_unused, index) => executionTargetRetryDelayV1(index + 1))).toEqual(oracle);
  });

  it("declares only the hub's and a proxy's temporary answers transient", () => {
    for (const status of [200, 206, 400, 401, 403, 404, 408, 409, 410, 429, 500, 501, 505]) expect(executionTargetTransientStatusV1(status), String(status)).toBe(false);
    for (const status of [502, 503, 504]) expect(executionTargetTransientStatusV1(status), String(status)).toBe(true);
  });

  it("asks again after each transient answer up to the attempt bound, announcing every retry and releasing each dropped body", async () => {
    for (const failures of [0, 1, policy.maxAttempts - 1, policy.maxAttempts + 2]) {
      const answers: { status: number; cancelled: boolean }[] = [];
      const announced: [number, number][] = [];
      const waited: number[] = [];
      const answer = await requestExecutionTargetAssetV1(
        async () => {
          const cancelled = { value: false };
          const status = answers.length < failures ? 503 : 200;
          const body = new ReadableStream<Uint8Array>({ cancel: () => void (cancelled.value = true) });
          answers.push({ status, get cancelled() { return cancelled.value; } });
          return { status, body };
        },
        { signal: new AbortController().signal, onRetry: (attempt, of) => announced.push([attempt, of]), sleep: async (ms) => void waited.push(ms) },
      );
      const requests = Math.min(failures + 1, policy.maxAttempts);
      expect(answers.map((row) => row.status), String(failures)).toEqual([...Array(Math.min(failures, requests)).fill(503), ...(failures < policy.maxAttempts ? [200] : [])].slice(0, requests));
      expect(answer.status).toBe(failures < policy.maxAttempts ? 200 : 503);
      expect(announced).toEqual(Array.from({ length: requests - 1 }, (_unused, index) => [index + 2, policy.maxAttempts]));
      expect(waited).toEqual(Array.from({ length: requests - 1 }, (_unused, index) => executionTargetRetryDelayV1(index + 1)));
      expect(answers.slice(0, requests - 1).every((row) => row.cancelled)).toBe(true);
      expect(answers.at(-1)!.cancelled).toBe(false);
    }
  });

  it("returns a non-transient answer at once and ends a backoff wait on cancel", async () => {
    let requests = 0;
    const refused = await requestExecutionTargetAssetV1(async () => ((requests += 1), { status: 404 }), { signal: new AbortController().signal, onRetry: () => undefined, sleep: executionTargetRetrySleepV1 });
    expect([refused.status, requests]).toEqual([404, 1]);
    const abort = new AbortController();
    const started = performance.now();
    const pending = requestExecutionTargetAssetV1(async () => ({ status: 503 }), { signal: abort.signal, onRetry: () => setTimeout(() => abort.abort(new Error("cancelled by the person")), 20), sleep: executionTargetRetrySleepV1 });
    await expect(pending).rejects.toThrow("cancelled by the person");
    expect(performance.now() - started).toBeLessThan(policy.backoffInitialMs);
  });
});
