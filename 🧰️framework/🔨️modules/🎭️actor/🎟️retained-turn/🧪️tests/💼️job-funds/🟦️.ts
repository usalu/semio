import { expect, test } from "bun:test";
import { KernelOriginalOperationIssuer, KernelOriginalResourcePool, OriginalJobFunds, OriginalOperationSource, OriginalSelfIssuedTurns, originalSourceJob, type OriginalGuestExports, type OriginalOperationReceipt } from "../../🟦️.ts";

/** 🧪️ Laws of job funding: output paging (pending is a retry, held output resumes), cancellation, progress, the Kernel-issued source, and the self-issued child. Native reference: `🔌️plugin/🖥️host/🎭️guest-runtime/💼️job-crossing/🦀️.rs` (`JobFunds`). */
const zero = { copiedItems: 0n, copiedBytes: 0n, retainedCapacityBytes: 0n, releasedBytes: 0n };
const none = { tag: "none" };
const some = (val: unknown) => ({ tag: "some", val });
const reply = (budget: { input: unknown }, spent = zero, extra: object = {}) => ({ input: budget.input, spent, workUnitsSpent: 1n, refusal: none, ...extra });

function guest(pages: (Uint8Array | undefined)[], closeAfter = pages.length, start = 0) {
  const log: string[] = [];
  let read = 0;
  let offset = start;
  const total = start + pages.reduce((sum, page) => sum + (page?.length ?? 0), 0);
  const jobs = {
    startJob: async (_source: unknown, budget: { input: unknown }) => (log.push("start"), reply(budget, zero, { admitted: true })),
    stepJob: async (_job: bigint, budget: { input: unknown }) => (log.push("step"), reply(budget, zero, { outcome: some({ state: "done", byteLength: BigInt(total) }) })),
    readJobOutput: async (_job: bigint, budget: { input: unknown }) => {
      log.push("read");
      const index = read++;
      const page = pages[index];
      if (index >= closeAfter) return reply(budget, zero, { chunk: some({ offset: BigInt(offset), bytes: none, outputClosed: true }) });
      const chunk = { offset: BigInt(offset), bytes: page === undefined ? none : some(page), outputClosed: index === pages.length - 1 };
      offset += page?.length ?? 0;
      return reply(budget, { ...zero, copiedBytes: BigInt(page?.length ?? 0), copiedItems: 1n }, { chunk: some(chunk) });
    },
    acknowledgeJobOutput: async (_job: bigint, budget: { input: unknown }) => (log.push("ack"), reply(budget, zero, { acknowledged: true })),
    cancelJob: async (_job: bigint, budget: { input: unknown }) => (log.push("cancel"), reply(budget, zero, { admitted: true })),
  };
  return { jobs: jobs as never, log };
}

const clock = (start = 0n) => { let now = start; return () => now++; };

test("a pending page is retried until the output closes, not a fatal stall", async () => {
  const { jobs, log } = guest([new Uint8Array([1, 2]), undefined, undefined, new Uint8Array([3])]);
  const funds = new OriginalJobFunds(1n, 0n, 1_000_000n, clock());
  expect(await funds.step(jobs, 9n)).toEqual({ status: "done", value: new Uint8Array([1, 2, 3]) });
  expect(log.filter((entry) => entry === "read").length).toBe(4);
});

test("an output that never offers a page stalls once the pending bound is spent", async () => {
  const { jobs } = guest(Array.from({ length: 5000 }, () => undefined));
  await expect(new OriginalJobFunds(1n, 0n, 1_000_000_000n, clock()).step(jobs, 9n)).rejects.toThrow("original-job.output-stalled");
});

test("a pending page past the wall deadline stalls without waiting for the bound", async () => {
  const { jobs, log } = guest(Array.from({ length: 5000 }, () => undefined));
  await expect(new OriginalJobFunds(1n, 0n, 3n, clock(1n)).step(jobs, 9n)).rejects.toThrow("original-job.output-stalled");
  expect(log.filter((entry) => entry === "read").length).toBeLessThan(10);
});

test("an aborted collection keeps the held pages so the retried step resumes at the guest's offset", async () => {
  const { jobs } = guest([new Uint8Array([1]), new Uint8Array([2]), new Uint8Array([3])]);
  const held = new Map();
  const controller = new AbortController();
  const seen: bigint[] = [];
  await expect(new OriginalJobFunds(1n, 0n, 1_000_000n, clock()).step(jobs, 9n, { held, signal: controller.signal, onProgress: ({ bytes }) => { seen.push(bytes); if (bytes === 2n) controller.abort(); } })).rejects.toThrow("original-job.aborted");
  expect(held.get(9n)!.length).toBe(2n);
  expect(seen).toEqual([1n, 2n]);
  const resumed = guest([new Uint8Array([3])], 1, 2);
  const final = await new OriginalJobFunds(1n, 0n, 1_000_000n, clock()).step(resumed.jobs, 9n, { held });
  expect(final).toEqual({ status: "done", value: new Uint8Array([1, 2, 3]) });
  expect(held.has(9n)).toBe(false);
});

test("cancel drops the held output of its job", async () => {
  const { jobs, log } = guest([new Uint8Array([1])]);
  const held = new Map([[4n, { parts: [new Uint8Array([1])], length: 1n }]]);
  await new OriginalJobFunds(1n, 0n, 1_000_000n, clock()).cancel(jobs, 4n, { held });
  expect(held.size).toBe(0);
  expect(log).toEqual(["cancel"]);
});

function kernelSource() {
  const pool = new KernelOriginalResourcePool({ grant: { maximumItems: 100, maximumCopyBytes: 1000, maximumCapacityBytes: 1000, maximumReleaseBytes: 1000, maximumDepth: 8 }, fuel: 1000, maximumElapsedUs: 1_000_000n });
  const admission = new KernelOriginalOperationIssuer((request) => pool.admit(request)).admit({ grant: { maximumItems: 100, maximumCopyBytes: 1000, maximumCapacityBytes: 1000, maximumReleaseBytes: 1000, maximumDepth: 8 }, fuel: 1000, maximumElapsedUs: 1_000_000n });
  return new OriginalOperationSource(admission, clock(10n));
}

test("a Kernel-issued job request spends the actor's source and reports one aggregate receipt", async () => {
  const source = kernelSource();
  const receipts: OriginalOperationReceipt[] = [];
  const { jobs } = guest([new Uint8Array([1, 2, 3])]);
  const value = await originalSourceJob(source, (receipt) => receipts.push(receipt), 1_000n, (funds) => funds.step(jobs, 5n));
  expect(value).toEqual({ status: "done", value: new Uint8Array([1, 2, 3]) });
  expect(receipts).toHaveLength(1);
  expect(receipts[0]!.spent.copiedBytes).toBe(3);
  expect(receipts[0]!.workUnitsSpent).toBe(3);
  expect(source.retainedRemaining.maximumCopyBytes).toBe(997);
  expect(source.fuelRemaining).toBe(997);
  expect(source.outstanding).toBe(false);
});

test("a failing Kernel-issued request still reports its spend and throws its own failure", async () => {
  const source = kernelSource();
  const receipts: OriginalOperationReceipt[] = [];
  const { jobs } = guest([new Uint8Array([1])]);
  await expect(originalSourceJob(source, (receipt) => receipts.push(receipt), 1_000n, async (funds) => { await funds.step(jobs, 5n); throw new Error("first"); })).rejects.toThrow("first");
  expect(receipts).toHaveLength(1);
  expect(source.outstanding).toBe(false);
});

test("a request larger than the source's remaining stock is refused and cannot spend beyond it", async () => {
  const source = kernelSource();
  const { jobs } = guest([new Uint8Array(2000)]);
  await expect(originalSourceJob(source, () => {}, 1_000n, (funds) => funds.step(jobs, 5n))).rejects.toThrow();
  expect(source.retainedRemaining.maximumCopyBytes).toBeLessThanOrEqual(1000);
});

function childGuest(poll: () => Promise<unknown>, copiedBytes = 0n) {
  const calls: string[] = [];
  const retained = (turn: { retained: { operation: bigint; generation: bigint; epoch: bigint; grant: unknown } }) => ({ input: { operation: turn.retained.operation, generation: turn.retained.generation, epoch: turn.retained.epoch, grant: turn.retained.grant }, spent: { ...zero, copiedBytes }, workUnitsSpent: 1n });
  const guest = {
    reactor: {
      admitActorContext: async (input: { operation: bigint; generation: bigint; epoch: bigint; grant: unknown }) => (calls.push("admit"), { input: { operation: input.operation, generation: input.generation, epoch: input.epoch, grant: input.grant }, spent: { ...zero, copiedItems: 1n }, workUnitsSpent: 1n, ready: true, refusal: none }),
      poll: async (_events: unknown[], budget: never) => (calls.push("poll"), (await poll(), { retainedReceipt: retained(budget) })),
    },
    jobs: {},
  } as unknown as OriginalGuestExports;
  return { guest, calls };
}

const shape = { maxEffects: 1, maxPatchBytes: 1, maxFrames: 1 };

test("a poll that fails without a receipt rethrows its own failure and later polls name it as the cause", async () => {
  const { guest: child } = childGuest(async () => { throw new TypeError("lift-failed"); });
  const turns = new OriginalSelfIssuedTurns(child, clock(1n));
  await expect(turns.poll([], shape)).rejects.toThrow("lift-failed");
  const later = await turns.poll([], shape).then(() => undefined, (error: Error & { cause?: unknown }) => error);
  expect(later?.message).toBe("original-turn.source-not-ready");
  expect((later?.cause as Error).message).toBe("lift-failed");
});

test("an exhausted self-issued stock is a typed fault, not an endless more-work spin", async () => {
  const { guest: child, calls } = childGuest(async () => {}, 21n << 20n);
  const turns = new OriginalSelfIssuedTurns(child, clock(1n));
  for (let turn = 0; turn < 3; turn += 1) await turns.poll([], shape);
  const fault = await turns.poll([], shape).then(() => undefined, (error: Error & { remaining?: { maximumCopyBytes: number } }) => error);
  expect(fault?.message).toBe("original-turn.stock-exhausted");
  expect(fault?.remaining!.maximumCopyBytes).toBeLessThan(4 << 20);
  expect(calls.filter((call) => call === "poll")).toHaveLength(3);
});

test("output of N bytes completes in ceil(N/page) crossings, each bounded by the schema page", async () => {
  const schema = (await import("../../💼️job/📄️output-page/🧬️schema/🔣️.json", { with: { type: "json" } })).default;
  const page = schema.properties.outputPageBytes.const;
  const total = page * 3 + 17;
  const body = Uint8Array.from({ length: total }, (_, index) => index % 251);
  let at = 0;
  const reads: bigint[] = [];
  const jobs = {
    stepJob: async (_job: bigint, budget: { input: unknown }) => reply(budget, zero, { outcome: some({ state: "done", byteLength: BigInt(total) }) }),
    readJobOutput: async (_job: bigint, budget: { input: unknown; retainedPolicy: { maximumCopyBytes: bigint } }) => {
      reads.push(budget.retainedPolicy.maximumCopyBytes);
      const count = Math.min(total - at, Number(budget.retainedPolicy.maximumCopyBytes));
      const chunk = { offset: BigInt(at), bytes: some(body.slice(at, at + count)), outputClosed: at + count === total };
      at += count;
      return reply(budget, { ...zero, copiedBytes: BigInt(count), copiedItems: 1n }, { chunk: some(chunk) });
    },
    acknowledgeJobOutput: async (_job: bigint, budget: { input: unknown }) => reply(budget, zero, { acknowledged: true }),
  };
  const result = await new OriginalJobFunds(1n, 0n, 1_000_000n, clock()).step(jobs as never, 9n);
  expect(result).toEqual({ status: "done", value: body });
  expect(reads.length).toBe(Math.ceil(total / page));
  expect(reads.every((copy) => copy <= BigInt(page))).toBe(true);
});
