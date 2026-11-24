import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import { KernelOriginalBootstrapStock, RootOriginalBootstrapOwner } from "../../🏛️bootstrap/🟦️.ts";
import { OriginalBootstrapRootPort, RemoteOriginalBootstrapOwner } from "../../🏛️bootstrap/📮️port/🟦️.ts";
import { OriginalOperationSource, OriginalStockExhausted, OriginalStockSuccession, originalJobWallUs, originalSourceJob, originalTurnWit, type OriginalContextBudgetWit, type OriginalOperationAdmission, type OriginalOperationReceipt, type OriginalRetainedGrant, type OriginalRetainedProgress } from "../../🟦️.ts";
import { decodePackValue } from "../../../../🎁️packs/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import policy from "../📋️policy.json";
import fixture from "./🧫️fixtures/🔣️.json";

/** 🧪️ Laws of the successor operation: a Source outlives its finite stocks only through kernel-issued, receipted succession, its identity never changes, and an exhausted Source is a typed fault. Native twin: `🎟️context/🧪️tests/🔁️succession/🦀️.rs`, driven by the same fixture. */
const axes = [["maximumItems", "copiedItems"], ["maximumCopyBytes", "copiedBytes"], ["maximumCapacityBytes", "retainedCapacityBytes"], ["maximumReleaseBytes", "releasedBytes"]] as const;
const none = { copiedItems: 0, copiedBytes: 0, retainedCapacityBytes: 0, releasedBytes: 0 };
const operation = { grant: fixture.stock.grant, fuel: fixture.stock.fuel, maximumElapsedUs: BigInt(fixture.source.deadlineUs) };
const kernelStock = (operations: number) => new KernelOriginalBootstrapStock({ version: 1, operation, stock: { grant: { ...Object.fromEntries(Object.entries(operation.grant).map(([axis, amount]) => [axis, amount * operations])), maximumDepth: operation.grant.maximumDepth } as OriginalRetainedGrant, fuel: operation.fuel * operations, maximumElapsedUs: operation.maximumElapsedUs } });
const command = (): OriginalRetainedProgress => { const { work: _, ...spent } = fixture.run.command; return spent; };

/** 📥️ The pack library, not the encoder under test, reads the wire a guest would read. */
const decoded = (value: any): any => value !== null && typeof value === "object" ? "value" in value && typeof value.value === "bigint" ? Number(value.value) : Object.fromEntries(Object.entries(value).map(([key, item]) => [key, decoded(item)])) : typeof value === "bigint" ? Number(value) : value;

/** 🧫️ The guest's own accounting: it refuses any grant above what it knows is left, except the one succession it verifies. */
class GuestModel {
  remaining: OriginalRetainedGrant; fuel: number; epoch = 0n; tenure: bigint; document: number[] = []; held = new Map<bigint, number>(); done = new Set<bigint>(); input: any; outputHeld = false; successions = 0; refusals: string[] = [];
  constructor(readonly admission: OriginalOperationAdmission) { this.remaining = admission.grant; this.fuel = admission.fuel; this.tenure = admission.operation; }
  #begin(budget: OriginalContextBudgetWit): void {
    const { input } = budget;
    if (input.operation !== this.admission.operation || input.generation !== this.admission.generation || input.epoch <= this.epoch) throw new Error("guest.identity");
    for (const [axis] of axes) if (Number(input.grant[axis]) > this.remaining[axis]) throw new Error("guest.renewed-authority");
    if (Number(input.work.fuel) > this.fuel) throw new Error("guest.renewed-work");
    this.epoch = input.epoch;
  }
  #spend(budget: OriginalContextBudgetWit, spent: OriginalRetainedProgress, work: number) {
    const next = { ...this.remaining };
    for (const [axis, key] of axes) { next[axis] -= spent[key]; if (next[axis] < 0) throw new Error("guest.overspent"); }
    if (work > this.fuel) throw new Error("guest.overspent");
    this.remaining = next; this.fuel -= work;
    return { input: budget.input, spent: Object.fromEntries(Object.entries(spent).map(([key, amount]) => [key, BigInt(amount)])), workUnitsSpent: BigInt(work), refusal: undefined as unknown };
  }
  readonly reactor = {
    admitActorInput: async (wire: Uint8Array, budget: OriginalContextBudgetWit) => {
      this.#begin(budget);
      if (this.input !== undefined) return { ...this.#spend(budget, none, 0), ready: false, disposition: "returned", refusal: "invariant-violated" };
      this.input = decoded(decodePackValue(wire));
      return { ...this.#spend(budget, { ...none, copiedItems: 1, copiedBytes: wire.length, retainedCapacityBytes: wire.length }, 0), ready: true, disposition: "retained" };
    },
    admitActorContext: async (budget: OriginalContextBudgetWit) => {
      const succession = this.input?.succession; this.input = undefined;
      const refuse = (refusal: string) => { this.refusals.push(refusal); return { ...this.#spend(budget, none, 0), ready: false, refusal }; };
      if (this.held.size > 0 || this.outputHeld) { this.epoch = budget.input.epoch; return refuse("ownership-limit"); }
      this.#begin(budget);
      if (BigInt(succession.operation) !== this.admission.operation || BigInt(succession.epoch) !== budget.input.epoch || BigInt(succession.successor) <= this.tenure) return refuse("invariant-violated");
      const reply = { ...this.#spend(budget, { ...none, copiedItems: 1, copiedBytes: 96 }, 1), ready: true };
      this.tenure = BigInt(succession.successor); this.remaining = succession.grant; this.fuel = succession.fuel; this.successions += 1;
      return reply;
    },
    poll: async () => { throw new Error("unused"); },
  };
  apply(budget: OriginalContextBudgetWit, edit: number) { this.#begin(budget); const reply = this.#spend(budget, command(), fixture.run.command.work); this.document.push(edit); return reply; }
  start(budget: OriginalContextBudgetWit, job: bigint) { this.#begin(budget); const reply = this.#spend(budget, command(), 1); this.held.set(job, 3); return { ...reply, admitted: true }; }
  step(budget: OriginalContextBudgetWit, job: bigint) { this.#begin(budget); const left = this.held.get(job)!; if (left === 1) { this.held.delete(job); this.done.add(job); } else this.held.set(job, left - 1); return { ...this.#spend(budget, command(), fixture.run.command.work), outcome: undefined }; }
  cancel(budget: OriginalContextBudgetWit, job: bigint) { this.#begin(budget); this.held.delete(job); return { ...this.#spend(budget, { ...none, copiedItems: 1 }, 1), admitted: true }; }
}

/** 🧵️ One actor over the real kernel stock, the real Source and the real succession driver; only the guest is a model. */
function actor(operations: number) {
  const stock = kernelStock(operations), admission = stock.admit(), guest = new GuestModel(admission);
  let now = 0n, draws = 0; const receipts: OriginalOperationReceipt[] = [], refusals: unknown[] = [];
  const source = new OriginalOperationSource(admission, () => now++), held = new Map();
  const reactor = { ...guest.reactor };
  const kernel = { receipt: (receipt: OriginalOperationReceipt) => { receipts.push(receipt); stock.receive(admission, receipt); }, hold: (refusal: unknown) => { refusals.push(refusal); }, succeed: async () => { draws += 1; return stock.succeed(admission); } };
  const succession = new OriginalStockSuccession(source, reactor, kernel, held);
  const settle = (reply: any) => { const receipt = { input: turn!.input, spent: Object.fromEntries(Object.entries(reply.spent).map(([key, amount]) => [key, Number(amount)])) as unknown as OriginalRetainedProgress, workUnitsSpent: Number(reply.workUnitsSpent) }; source.receive(receipt); kernel.receipt(receipt); return reply; };
  let turn: ReturnType<typeof source.issue> | undefined;
  const cross = async (run: (budget: OriginalContextBudgetWit) => any, cleanup = false) => { await succession.advance(); if (!cleanup) source.requireStock(succession.refusal); turn = source.issue(); const input = originalTurnWit(turn); return settle(run({ input, retainedPolicy: input.grant, fuel: input.work.fuel, deadlineUs: turn.deadlineUs, cancelled: false })); };
  return { stock, admission, guest, source, succession, receipts, refusals, held, reactor, kernel, get draws() { return draws; }, advanceClock: (to: bigint) => { now = to; }, edit: (value: number) => cross((budget) => guest.apply(budget, value)), start: async (job: bigint) => { const reply = await cross((budget) => guest.start(budget, job)); succession.started(job); return reply; }, step: async (job: bigint) => { const reply = await cross((budget) => guest.step(budget, job)); if (guest.done.has(job)) succession.settled(job); return reply; }, cancel: async (job: bigint) => { const reply = await cross((budget) => guest.cancel(budget, job), true); succession.settled(job); return reply; } };
}

test("the fixture and the policy satisfy the succession schema and hostile rows are refused", () => {
  const ajv = new Ajv({ strict: true, allErrors: true }), law = ajv.compile(schema);
  expect(law(fixture), JSON.stringify(law.errors)).toBe(true);
  expect(law({ ...fixture, run: { ...fixture.run, stocks: 1 } })).toBe(false);
  expect(ajv.compile({ ...schema.$defs.policy, $defs: schema.$defs })(policy)).toBe(true);
  expect(ajv.compile({ ...schema.$defs.succession, $defs: schema.$defs })({ operation: 31, generation: 1, epoch: 2, successor: 32, grant: fixture.stock.grant, fuel: 1, refill: true })).toBe(false);
});

test("low water and the floor follow the policy fractions an independent SQL engine computes", () => {
  const db = new Database(":memory:");
  try {
    for (const row of fixture.thresholds) {
      const oracle = db.query("SELECT ?1 < (?2 + ?3 - 1) / ?3 AS low, ?1 < (?2 + ?4 - 1) / ?4 AS gone").get(row.remaining, row.granted, policy.lowWaterDivisor, policy.floorDivisor) as { low: number; gone: number };
      expect([Boolean(oracle.low), Boolean(oracle.gone)]).toEqual([row.lowWater, row.exhausted]);
      const grant = { ...operation.grant, maximumItems: row.granted };
      const source = new OriginalOperationSource({ operation: 1n, generation: 1n, grant, fuel: operation.fuel, maximumElapsedUs: 10n }, () => 0n);
      const turn = source.issue();
      source.receive({ input: turn.input, spent: { ...none, copiedItems: row.granted - row.remaining }, workUnitsSpent: 0 });
      expect(source.lowWater).toBe(row.lowWater);
      if (row.exhausted) expect(() => source.requireStock()).toThrow(OriginalStockExhausted); else expect(() => source.requireStock()).not.toThrow();
    }
  } finally { db.close(); }
});

test("an actor outlives several exhausted stocks without losing a job or an edit, and every stock is conserved", async () => {
  const host = actor(fixture.run.stocks), edits: number[] = [], jobs: bigint[] = [];
  const identity = [host.source.operation, host.source.generation, host.source.deadlineUs];
  let live: bigint | undefined, tenures = new Set([host.source.tenure]);
  for (let turn = 0; host.guest.successions < fixture.run.stocks - 1; turn += 1) {
    if (turn > 200_000) throw new Error("the run never exhausted its stocks");
    if (live === undefined && turn % fixture.run.jobEvery === 0) { live = BigInt(turn + 1); await host.start(live); jobs.push(live); }
    else if (live !== undefined && turn % 2 === 0) { await host.step(live); if (host.guest.done.has(live)) live = undefined; }
    else { await host.edit(turn); edits.push(turn); }
    tenures.add(host.source.tenure);
  }
  while (live !== undefined) { await host.step(live); if (host.guest.done.has(live)) live = undefined; }
  expect(host.guest.document).toEqual(edits);
  expect([...host.guest.done]).toEqual(jobs);
  expect(jobs.length).toBeGreaterThan(fixture.run.stocks);
  expect([...tenures]).toEqual(Array.from({ length: fixture.run.stocks }, (_, index) => BigInt(index + 1)));
  expect([host.source.operation, host.source.generation, host.source.deadlineUs]).toEqual(identity);
  expect(host.draws).toBe(fixture.run.stocks - 1);
  expect(host.refusals).toEqual([]);
  expect(host.receipts.every((receipt, index) => receipt.input.epoch === BigInt(index + 1))).toBe(true);
  expect([host.source.retainedRemaining, host.source.fuelRemaining]).toEqual([host.guest.remaining, host.guest.fuel]);
  const db = new Database(":memory:");
  try {
    db.run("CREATE TABLE receipt(epoch INTEGER PRIMARY KEY, successor INTEGER, items INTEGER, copied INTEGER, capacity INTEGER, released INTEGER, work INTEGER)");
    const insert = db.query("INSERT INTO receipt VALUES(?,?,?,?,?,?,?)");
    for (const { input, spent, workUnitsSpent, successor } of host.receipts) insert.run(Number(input.epoch), successor === undefined ? null : Number(successor), spent.copiedItems, spent.copiedBytes, spent.retainedCapacityBytes, spent.releasedBytes, workUnitsSpent);
    const spans = db.query("SELECT tenure, sum(items) items, sum(copied) copied, sum(capacity) capacity, sum(released) released, sum(work) work FROM (SELECT *, 1 + (SELECT count(*) FROM receipt earlier WHERE earlier.successor IS NOT NULL AND earlier.epoch < receipt.epoch) AS tenure FROM receipt) GROUP BY tenure ORDER BY tenure").all() as Record<string, number>[];
    expect(spans.length).toBe(fixture.run.stocks);
    for (const span of spans) expect([span.items! <= operation.grant.maximumItems, span.copied! <= operation.grant.maximumCopyBytes, span.capacity! <= operation.grant.maximumCapacityBytes, span.released! <= operation.grant.maximumReleaseBytes, span.work! <= operation.fuel]).toEqual([true, true, true, true, true]);
    const total = db.query("SELECT sum(copied) AS copied FROM receipt").get() as { copied: number };
    expect(total.copied).toBeGreaterThan(operation.grant.maximumCopyBytes * (fixture.run.stocks - 1) / 2);
    expect(host.source.physicalSpent.copiedBytes).toBe(total.copied);
  } finally { db.close(); }
  expect(host.stock.retainedRemaining.maximumItems).toBe(0);
  expect(host.stock.fuelRemaining).toBe(0);
});

test("without a successor the exhausted actor answers one typed fault that names its axis and the kernel denial", async () => {
  const host = actor(1);
  const fault = await (async () => { for (let edit = 0; edit < 100_000; edit += 1) try { await host.edit(edit); } catch (error) { return error as OriginalStockExhausted; } throw new Error("never exhausted"); })();
  expect(fault).toBeInstanceOf(OriginalStockExhausted);
  expect(fault.fault).toMatchObject({ origin: "framework", code: "plugin.original-stock-exhausted", severity: "error", retryable: true, params: { axis: "maximumCopyBytes" } });
  expect(structuredClone(fault.fault)).toEqual(fault.fault);
  expect((fault.cause as Error).message).toContain("kernel-resource-denied");
  await expect(host.edit(-1)).rejects.toBeInstanceOf(OriginalStockExhausted);
  expect(host.guest.document).not.toContain(-1);
});

test("a job that spans the whole stock faults typed, stays cancellable on the floor, and its release renews the actor", async () => {
  const host = actor(3);
  await host.start(7n);
  host.guest.held.set(7n, Number.MAX_SAFE_INTEGER);
  const before = await (async () => { for (let step = 0; step < 100_000; step += 1) try { await host.step(7n); } catch (error) { return error; } throw new Error("never exhausted"); })();
  expect(before).toBeInstanceOf(OriginalStockExhausted);
  expect(host.guest.successions).toBe(0);
  expect(host.draws).toBe(0);
  await host.cancel(7n);
  await host.edit(1);
  expect([host.guest.successions, host.guest.document]).toEqual([1, [1]]);
});

test("a quiescent host attempts nothing the guest must refuse, and a guest refusal defers without drawing twice", async () => {
  for (const row of fixture.quiescence) {
    const host = actor(2);
    while (!host.source.lowWater) await host.edit(0);
    const before = host.guest.epoch;
    if (row.jobSlot) host.guest.held.set(9n, 1);
    if (row.heldOutput) host.held.set(9n, { parts: [], length: 0n });
    if (row.admittedInput) host.guest.input = { operation: "poll" };
    if (row.outstanding) host.source.issue();
    await host.succession.advance();
    expect([row.name, host.guest.successions]).toEqual([row.name, row.accepted ? 1 : 0]);
    if (row.heldOutput || row.outstanding) expect(host.guest.epoch).toBe(before);
  }
  const host = actor(2);
  while (!host.source.lowWater) await host.edit(0);
  host.guest.held.set(9n, 1);
  for (let attempt = 0; attempt < 64; attempt += 1) await host.succession.advance();
  expect(host.guest.refusals.length).toBeLessThan(8);
  expect([host.draws, host.succession.refusal]).toEqual([1, "ownership-limit"]);
  host.guest.held.delete(9n);
  host.succession.settled(9n);
  await host.succession.advance();
  expect([host.guest.successions, host.draws, host.succession.refusal]).toEqual([1, 1, undefined]);
});

test("a Source accepts only its own unused successor and never a renewed receipt, and its deadline is no stock", () => {
  const stock = kernelStock(3), admission = stock.admit(), successor = stock.succeed(admission);
  expect(stock.succeed(admission)).toBe(successor);
  let now = 5n;
  const source = new OriginalOperationSource(admission, () => now);
  const succession = source.quoteSuccession(successor);
  expect(succession).toMatchObject({ operation: admission.operation, generation: 1n, epoch: 2n, successor: successor.operation, fuel: operation.fuel });
  expect(() => source.quoteSuccession(admission)).toThrow("succession-refused");
  const turn = source.issue(), receipt = { input: turn.input, spent: { ...none, copiedItems: 1 }, workUnitsSpent: 1 };
  expect(() => source.succeed(receipt, successor)).toThrow("succession-refused");
  expect(() => source.succeed({ ...receipt, successor: admission.operation }, admission)).toThrow("succession-refused");
  expect(() => stock.receive(admission, { ...receipt, successor: 99n })).toThrow("foreign-receipt");
  source.succeed({ ...receipt, successor: successor.operation }, successor);
  stock.receive(admission, { ...receipt, successor: successor.operation });
  expect([source.tenure, source.fuelRemaining, source.issue().input.epoch]).toEqual([successor.operation, operation.fuel, 2n]);
  expect(() => stock.receive(admission, { ...receipt, successor: successor.operation })).toThrow("foreign-receipt");
  expect(stock.succeed(admission).operation).toBe(successor.operation + 1n);
  now = source.deadlineUs;
  expect(() => source.requireStock()).toThrow("maximumElapsedUs");
});

test("a cancelled source is never renewed and a guest that traps the succession holds its cause", async () => {
  const cancelled = actor(2);
  while (!cancelled.source.lowWater) await cancelled.edit(0);
  cancelled.source.cancel();
  await cancelled.succession.advance();
  expect(cancelled.draws).toBe(0);
  const trapped = actor(2), cause = new Error("trap");
  while (!trapped.source.lowWater) await trapped.edit(0);
  trapped.reactor.admitActorContext = async () => { throw cause; };
  await expect(trapped.succession.advance()).rejects.toBe(cause);
  expect([trapped.source.nativeRefusal, trapped.refusals]).toEqual([cause, [cause]]);
});

test("a guest that answers ready without the one paid succession receipt is held, never renewed", async () => {
  const host = actor(2);
  while (!host.source.lowWater) await host.edit(0);
  const before = host.source.tenure;
  host.reactor.admitActorContext = async (budget: OriginalContextBudgetWit) => ({ input: budget.input, spent: { copiedItems: 9n, copiedBytes: 6064n, retainedCapacityBytes: 0n, releasedBytes: 0n }, workUnitsSpent: 3n, ready: true, refusal: undefined }) as never;
  await expect(host.succession.advance()).rejects.toThrow("succession-unproven");
  expect([host.source.tenure, host.source.lowWater, (host.source.nativeRefusal as Error).message.includes("rebuild the plugin"), host.refusals.length]).toEqual([before, true, true, 1]);
});

test("job crossings stop typed at the floor while a cleanup crossing still runs", async () => {
  const source = new OriginalOperationSource({ operation: 1n, generation: 1n, grant: operation.grant, fuel: 16, maximumElapsedUs: 1n << 40n }, () => 0n);
  const turn = source.issue();
  source.receive({ input: turn.input, spent: none, workUnitsSpent: 16 });
  const jobs = { cancelJob: async (_: bigint, budget: any) => ({ input: budget.input, spent: { copiedItems: 0n, copiedBytes: 0n, retainedCapacityBytes: 0n, releasedBytes: 0n }, workUnitsSpent: 0n, admitted: true }) } as any;
  await expect(originalSourceJob(source, () => {}, originalJobWallUs(), (funds) => funds.cancel(jobs, 1n))).rejects.toThrow("plugin.original-stock-exhausted: fuel");
  const cleanupSource = new OriginalOperationSource({ operation: 1n, generation: 1n, grant: operation.grant, fuel: 32, maximumElapsedUs: 1n << 40n }, () => 0n);
  const spent = cleanupSource.issue();
  cleanupSource.receive({ input: spent.input, spent: none, workUnitsSpent: 31 });
  await originalSourceJob(cleanupSource, () => {}, originalJobWallUs(), (funds) => funds.cancel(jobs, 1n), true);
});

test("the frame port draws the same root successor across realms", async () => {
  const stock = kernelStock(2);
  let parent!: OriginalBootstrapRootPort;
  const remote = new RemoteOriginalBootstrapOwner((request) => { void parent.receive(structuredClone(request)); });
  parent = new OriginalBootstrapRootPort(new RootOriginalBootstrapOwner(stock), (response) => remote.receiveResponse(structuredClone(response)));
  const admission = await remote.admit(), successor = await remote.succeed(admission);
  expect([successor.operation, (await remote.succeed(admission)).operation, stock.liveSources]).toEqual([2n, 2n, 1]);
  const source = new OriginalOperationSource(admission, () => 1n), turn = source.issue(), receipt = { input: turn.input, spent: none, workUnitsSpent: 0, successor: successor.operation };
  source.succeed(receipt, successor);
  await remote.receive(admission, receipt);
  await expect(remote.succeed(admission)).rejects.toThrow("kernel-resource-denied");
  await expect(remote.succeed({ ...admission })).rejects.toThrow("foreign-admission");
});
