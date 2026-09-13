/**
 * 🎮️ TypeScript twin of `🧪️tests/🎮️wgpu-runtime-mailbox-admission/🦀️.rs`.
 *
 * Re-derives the renderer mailbox's admission arithmetic from the SAME oracle
 * (`🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json`) without looking at the Rust: a fixed bound
 * shared by ready and in-flight work with one reserve for the interaction return, keyed coalescing,
 * a first-applicable rule that lets work needing no interaction state past work that does, and a
 * checkout ledger that ages one bounded step and turns an overrun into a named diagnostic exactly once.
 *
 * ⚖️ An independent derivation is the point: if the two implementations agree with the fixture they
 * agree with each other, and a browser frame cannot start swallowing input while the Rust unit tests
 * stay green.
 */

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(here, "../../🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json");

type Completion = { key: string | null; revision: number; requiresInteraction: boolean };
type Step = Record<string, unknown>;
type Row = { id: string; why: string; capacity: number; steps: Step[]; expect: { length: number; readyRevisions: number[]; staleNotices: { site: string; opportunities: number }[] } };
type Fixture = { credits: number; rows: Row[] };

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

/** 📮️ The bounded completion queue, re-derived. */
class BoundedCompletionQueue {
  readonly ready: Completion[] = [];
  private inFlight = 0;

  constructor(private readonly capacity: number) {
    if (capacity <= 1) throw new Error("completion mailbox needs an interaction reserve");
  }

  get length(): number {
    return this.ready.length + this.inFlight;
  }

  private makeRoomFor(key: string | null, limit: number): boolean {
    if (this.length < limit) return true;
    if (key === null) return false;
    const index = this.ready.findIndex(queued => queued.key === key);
    if (index < 0) return false;
    this.ready.splice(index, 1);
    return true;
  }

  enqueue(completion: Completion): boolean {
    if (!this.makeRoomFor(completion.key, this.capacity - 1)) return false;
    this.ready.push(completion);
    return true;
  }

  reserveInteraction(): boolean {
    if (this.length === this.capacity) return false;
    this.inFlight += 1;
    return true;
  }

  finish(completion: Completion): void {
    if (this.inFlight === 0) throw new Error("runtime completion without reservation");
    this.inFlight -= 1;
    this.ready.unshift(completion);
    if (this.length > this.capacity) throw new Error("runtime completion mailbox capacity exceeded");
  }

  headRequiresInteraction(): boolean {
    return this.ready.length > 0 && this.ready[0].requiresInteraction;
  }

  firstApplicable(interactionAvailable: boolean): number | null {
    if (interactionAvailable) return this.ready.length > 0 ? 0 : null;
    const index = this.ready.findIndex(completion => !completion.requiresInteraction);
    return index < 0 ? null : index;
  }

  takeAt(index: number): Completion | null {
    return this.ready.splice(index, 1)[0] ?? null;
  }
}

type Admission = "admitted" | "deferred" | "stale";

/** 🎟️ The interaction checkout ledger, re-derived. */
class InteractionCheckoutLedger {
  private site: string | null = null;
  private ageInOpportunities = 0;
  private stale = false;
  private notified = false;

  constructor(private readonly credits: number) {}

  checkOut(site: string): boolean {
    if (this.site !== null) return false;
    this.site = site;
    this.ageInOpportunities = 0;
    this.stale = false;
    this.notified = false;
    return true;
  }

  checkIn(): void {
    this.site = null;
    this.ageInOpportunities = 0;
    this.stale = false;
    this.notified = false;
  }

  admit(headRequiresInteraction: boolean, interactionAvailable: boolean): Admission {
    if (interactionAvailable || !headRequiresInteraction) return "admitted";
    this.ageInOpportunities += 1;
    if (this.ageInOpportunities <= this.credits) return "deferred";
    this.stale = true;
    return "stale";
  }

  takeStaleNotice(): { site: string; opportunities: number } | null {
    if (!this.stale || this.notified) return null;
    this.notified = true;
    return { site: this.site ?? "unknown", opportunities: this.ageInOpportunities };
  }

  get opportunities(): number {
    return this.ageInOpportunities;
  }
}

function replay(row: Row, credits: number) {
  const queue = new BoundedCompletionQueue(row.capacity);
  const ledger = new InteractionCheckoutLedger(credits);
  const notices: { site: string; opportunities: number }[] = [];
  let available = true;

  row.steps.forEach((step, index) => {
    const repeat = typeof step.repeat === "number" ? step.repeat : 1;
    for (let iteration = 0; iteration < repeat; iteration += 1) {
      const last = iteration + 1 === repeat;
      const where = `${row.id} step ${index}`;
      switch (step.op) {
        case "enqueue": {
          const admitted = queue.enqueue({ key: (step.key as string | null) ?? null, revision: step.revision as number, requiresInteraction: step.requiresInteraction as boolean });
          expect(admitted, `${where}: enqueue admission`).toBe(step.admitted);
          break;
        }
        case "reserveInteraction": {
          expect(queue.reserveInteraction(), `${where}: interaction reserve`).toBe(step.admitted);
          break;
        }
        case "finish": {
          queue.finish({ key: (step.key as string | null) ?? null, revision: step.revision as number, requiresInteraction: step.requiresInteraction as boolean });
          break;
        }
        case "checkOut": {
          const admitted = ledger.checkOut(step.site as string);
          expect(admitted, `${where}: checkout admission`).toBe(step.admitted);
          if (admitted) available = false;
          break;
        }
        case "checkIn": {
          ledger.checkIn();
          available = true;
          break;
        }
        case "apply": {
          const admission = ledger.admit(queue.headRequiresInteraction(), available);
          const at = queue.firstApplicable(available);
          const applied = at === null ? null : (queue.takeAt(at)?.revision ?? null);
          const notice = ledger.takeStaleNotice();
          if (notice) notices.push(notice);
          if (!last) break;
          expect(admission, `${where}: admission verdict`).toBe(step.admission);
          expect(applied, `${where}: applied revision`).toBe(step.appliedRevision ?? null);
          expect(ledger.opportunities, `${where}: checkout age`).toBe(step.opportunities);
          break;
        }
        default:
          throw new Error(`${where}: unknown operation ${String(step.op)}`);
      }
    }
  });

  expect(queue.length, `${row.id}: final mailbox length`).toBe(row.expect.length);
  expect(queue.ready.map(completion => completion.revision), `${row.id}: ready order`).toEqual(row.expect.readyRevisions);
  expect(notices, `${row.id}: stale checkout diagnostics`).toEqual(row.expect.staleNotices);
}

describe("wgpu runtime mailbox admission", () => {
  it("declares rows", () => {
    expect(fixture.rows.length).toBeGreaterThan(0);
    expect(fixture.credits).toBeGreaterThan(0);
  });

  for (const row of fixture.rows) {
    it(`${row.id} — ${row.why}`, () => replay(row, fixture.credits));
  }

  it("never lets a completion that needs no interaction state wait behind one that does", () => {
    const queue = new BoundedCompletionQueue(8);
    queue.enqueue({ key: null, revision: 1, requiresInteraction: true });
    queue.enqueue({ key: null, revision: 2, requiresInteraction: true });
    queue.enqueue({ key: null, revision: 3, requiresInteraction: false });
    expect(queue.firstApplicable(false)).toBe(2);
    expect(queue.firstApplicable(true)).toBe(0);
  });

  it("preserves the order of the completions that do need the interaction state", () => {
    const queue = new BoundedCompletionQueue(8);
    queue.enqueue({ key: null, revision: 1, requiresInteraction: true });
    queue.enqueue({ key: null, revision: 2, requiresInteraction: true });
    const applied: number[] = [];
    for (;;) {
      const at = queue.firstApplicable(true);
      if (at === null) break;
      applied.push(queue.takeAt(at)?.revision ?? -1);
    }
    expect(applied).toEqual([1, 2]);
  });

  it("ages a checkout only on the opportunities its own head actually blocked", () => {
    const ledger = new InteractionCheckoutLedger(fixture.credits);
    expect(ledger.checkOut("dispatch-event")).toBe(true);
    expect(ledger.admit(false, false)).toBe("admitted");
    expect(ledger.opportunities).toBe(0);
    expect(ledger.admit(true, true)).toBe("admitted");
    expect(ledger.opportunities).toBe(0);
    expect(ledger.admit(true, false)).toBe("deferred");
    expect(ledger.opportunities).toBe(1);
  });

  it("reports a stale checkout once and clears the episode on check-in", () => {
    const ledger = new InteractionCheckoutLedger(2);
    ledger.checkOut("frame-deferred");
    expect(ledger.admit(true, false)).toBe("deferred");
    expect(ledger.admit(true, false)).toBe("deferred");
    expect(ledger.admit(true, false)).toBe("stale");
    expect(ledger.takeStaleNotice()).toEqual({ site: "frame-deferred", opportunities: 3 });
    expect(ledger.admit(true, false)).toBe("stale");
    expect(ledger.takeStaleNotice()).toBeNull();
    ledger.checkIn();
    ledger.checkOut("dispatch-event");
    expect(ledger.admit(true, false)).toBe("deferred");
    expect(ledger.takeStaleNotice()).toBeNull();
  });
});
