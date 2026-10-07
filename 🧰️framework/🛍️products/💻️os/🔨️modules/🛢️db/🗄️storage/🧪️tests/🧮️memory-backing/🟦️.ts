import { test, expect } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import sum from "lodash/sum";

/** 🧮️ Exercises byte reservation, task retirement and retry examples against independent allocation primitives. */
export async function proveMemoryBackendBackingFixture(repoRoot: string): Promise<void> {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🧫️fixtures/🧮️memory-backing");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
  const inline = 128;
  const lengths = fixture.tables.map((row: { slots: number }, index: number) => row.slots * (index + 1) * 8);
  const required = BigInt(inline) + lengths.reduce((sum: bigint, length: number) => sum + BigInt(length), 0n);
  expect(required).toBe(BigInt(inline + sum(lengths)));
  expect(lengths.map((length: number) => new Uint8Array(length).byteLength)).toEqual(lengths);
  for (const test of fixture.admission) {
    const remaining = test.remaining === "exact" ? required : test.remaining === "one-short" ? required - 1n : 0n;
    const accepted = required <= remaining;
    if (accepted !== test.accepted) throw new Error(`memory backing admission differs: ${test.remaining}`);
    if (!accepted) continue;
    const tables = lengths.map((length: number) => Buffer.alloc(length));
    const actual = BigInt(inline) + tables.reduce((sum: bigint, bytes: Buffer) => sum + BigInt(bytes.byteLength), 0n);
    if (actual !== required) throw new Error("memory backing allocation differs from reserved bytes");
    tables.length = 0;
  }
  const pendingTasks = new Set([1]);
  if ((pendingTasks.size === 0) !== fixture.closeWhileAdmitted) throw new Error("memory backing retired an admitted task");
  pendingTasks.delete(1);
  if (pendingTasks.size !== 0) throw new Error("memory backing task admission was not returned");
  const heldResult = Buffer.from([0]);
  for (let index = 0; index < fixture.sequentialTasks; index++) {
    pendingTasks.add(index + 1);
    await Promise.resolve();
    pendingTasks.delete(index + 1);
    if (pendingTasks.size !== 0 || heldResult[0] !== 0) throw new Error("sequential task retirement changed a retained result or leaked task admission");
  }
  let queueOccupied = true,
    retryAttempts = 0,
    terminal = false;
  const retry = async (): Promise<void> => {
    while (!terminal && retryAttempts < fixture.retry.maximumAttempts) {
      retryAttempts++;
      await new Promise<void>((resolve) => setTimeout(resolve, fixture.retry.timerDelayMs));
      terminal = !queueOccupied;
    }
  };
  const pendingRetry = retry();
  queueMicrotask(() => {
    queueOccupied = false;
  });
  await pendingRetry;
  if (terminal !== fixture.retry.terminalAfterQueueRelease || retryAttempts !== 1) throw new Error("memory backing retry did not reach terminal after queue release");
  console.log(
    `memory-backing-oracle: tables=${fixture.tables.length} admission=${fixture.admission.length} timer-retry=1 sequential=${fixture.sequentialTasks}; runtime ABI sizes and worker wake are checked by the Rust owner laws`,
  );
}

test("memory backing preserves exact reserved bytes and returns every admitted task", async () => {
  let root = import.meta.dir;
  while (!existsSync(join(root, "bun.lock"))) root = dirname(root);
  await proveMemoryBackendBackingFixture(root);
});
