/**
 * 🎮️ TypeScript twin of `🧪️tests/🎮️wgpu-runtime-mailbox-admission/🦀️.rs`.
 *
 * Re-derives the renderer mailbox's admission arithmetic from the SAME oracle
 * (`🧫️fixtures/🎮️wgpu-runtime-mailbox-admission/🔣️.json`) without looking at the Rust: a fixed bound
 * shared by ready and in-flight work with one reserve for the interaction return, keyed coalescing,
 * a first-applicable rule that lets work needing no interaction state past work that does, and a
 * checkout ledger whose verdict is whether an OWNER is still outstanding — a reservation in flight,
 * or a ready completion that restores the state — rather than how many blocked opportunities the
 * checkout has spanned.
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

type Completion = { key: string | null; revision: number; requiresInteraction: boolean; restoresInteraction: boolean };
type Step = Record<string, unknown>;
type Row = { id: string; why: string; capacity: number; steps: Step[]; expect: { length: number; readyRevisions: number[]; abandonedNotices: { site: string; opportunities: number }[] } };
type Fixture = { rows: Row[] };

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

  /** 🎟️ Anything that can still bring a checked-out interaction state home. */
  interactionOwnerOutstanding(): boolean {
    return this.inFlight > 0 || this.ready.some(completion => completion.restoresInteraction);
  }

  firstApplicable(interactionAvailable: boolean): number | null {
    if (interactionAvailable) return this.ready.length > 0 ? 0 : null;
    const index = this.ready.findIndex(completion => !completion.requiresInteraction);
    return index < 0 ? null : index;
  }

  firstInteractionRestoration(): number | null {
    const index = this.ready.findIndex(completion => completion.restoresInteraction);
    return index < 0 ? null : index;
  }

  takeAt(index: number): Completion | null {
    return this.ready.splice(index, 1)[0] ?? null;
  }

  restoreAt(index: number, completion: Completion): void {
    this.ready.splice(Math.min(index, this.ready.length), 0, completion);
  }
}

type Admission = "admitted" | "deferred" | "abandoned";

/** 🎟️ The interaction checkout ledger, re-derived. */
class InteractionCheckoutLedger {
  private site: string | null = null;
  private ageInOpportunities = 0;
  private abandoned = false;
  private notified = false;

  checkOut(site: string): boolean {
    if (this.site !== null) return false;
    this.site = site;
    this.ageInOpportunities = 0;
    this.abandoned = false;
    this.notified = false;
    return true;
  }

  checkIn(): void {
    this.site = null;
    this.ageInOpportunities = 0;
    this.abandoned = false;
    this.notified = false;
  }

  admit(headRequiresInteraction: boolean, interactionAvailable: boolean, ownerOutstanding: boolean): Admission {
    if (interactionAvailable || !headRequiresInteraction) return "admitted";
    this.ageInOpportunities += 1;
    if (ownerOutstanding) return "deferred";
    this.abandoned = true;
    return "abandoned";
  }

  takeAbandonedNotice(): { site: string; opportunities: number } | null {
    if (!this.abandoned || this.notified) return null;
    this.notified = true;
    return { site: this.site ?? "unknown", opportunities: this.ageInOpportunities };
  }

  get opportunities(): number {
    return this.ageInOpportunities;
  }
}

function replay(row: Row) {
  const queue = new BoundedCompletionQueue(row.capacity);
  const ledger = new InteractionCheckoutLedger();
  const notices: { site: string; opportunities: number }[] = [];
  let available = true;

  row.steps.forEach((step, index) => {
    const repeat = typeof step.repeat === "number" ? step.repeat : 1;
    for (let iteration = 0; iteration < repeat; iteration += 1) {
      const last = iteration + 1 === repeat;
      const where = `${row.id} step ${index}`;
      switch (step.op) {
        case "enqueue": {
          const admitted = queue.enqueue({ key: (step.key as string | null) ?? null, revision: step.revision as number, requiresInteraction: step.requiresInteraction as boolean, restoresInteraction: step.restoresInteraction as boolean });
          expect(admitted, `${where}: enqueue admission`).toBe(step.admitted);
          break;
        }
        case "reserveInteraction": {
          expect(queue.reserveInteraction(), `${where}: interaction reserve`).toBe(step.admitted);
          break;
        }
        case "finish": {
          queue.finish({ key: (step.key as string | null) ?? null, revision: step.revision as number, requiresInteraction: step.requiresInteraction as boolean, restoresInteraction: step.restoresInteraction as boolean });
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
        case "presenterRestore": {
          const at = queue.firstInteractionRestoration();
          expect(at, `${where}: presenter restoration owner`).not.toBeNull();
          const completion = queue.takeAt(at ?? -1);
          expect(completion?.revision, `${where}: exact return owner`).toBe(step.restoredRevision);
          if (!completion) throw new Error(`${where}: typed restoration completion`);
          completion.restoresInteraction = false;
          queue.restoreAt(at ?? 0, completion);
          ledger.checkIn();
          available = true;
          break;
        }
        case "apply": {
          const admission = ledger.admit(queue.headRequiresInteraction(), available, queue.interactionOwnerOutstanding());
          const at = queue.firstApplicable(available);
          const applied = at === null ? null : (queue.takeAt(at)?.revision ?? null);
          const notice = ledger.takeAbandonedNotice();
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
  expect(notices, `${row.id}: abandoned checkout diagnostics`).toEqual(row.expect.abandonedNotices);
}

describe("wgpu runtime mailbox admission", () => {
  it("declares rows", () => {
    expect(fixture.rows.length).toBeGreaterThan(0);
  });

  for (const row of fixture.rows) {
    it(`${row.id} — ${row.why}`, () => replay(row));
  }

  it("never lets a completion that needs no interaction state wait behind one that does", () => {
    const queue = new BoundedCompletionQueue(8);
    queue.enqueue({ key: null, revision: 1, requiresInteraction: true, restoresInteraction: false });
    queue.enqueue({ key: null, revision: 2, requiresInteraction: true, restoresInteraction: false });
    queue.enqueue({ key: null, revision: 3, requiresInteraction: false, restoresInteraction: false });
    expect(queue.firstApplicable(false)).toBe(2);
    expect(queue.firstApplicable(true)).toBe(0);
  });

  it("preserves the order of the completions that do need the interaction state", () => {
    const queue = new BoundedCompletionQueue(8);
    queue.enqueue({ key: null, revision: 1, requiresInteraction: true, restoresInteraction: false });
    queue.enqueue({ key: null, revision: 2, requiresInteraction: true, restoresInteraction: false });
    const applied: number[] = [];
    for (;;) {
      const at = queue.firstApplicable(true);
      if (at === null) break;
      applied.push(queue.takeAt(at)?.revision ?? -1);
    }
    expect(applied).toEqual([1, 2]);
  });

  it("ages a checkout only on the opportunities its own head actually blocked", () => {
    const ledger = new InteractionCheckoutLedger();
    expect(ledger.checkOut("dispatch-event")).toBe(true);
    expect(ledger.admit(false, false, true)).toBe("admitted");
    expect(ledger.opportunities).toBe(0);
    expect(ledger.admit(true, true, true)).toBe("admitted");
    expect(ledger.opportunities).toBe(0);
    expect(ledger.admit(true, false, true)).toBe("deferred");
    expect(ledger.opportunities).toBe(1);
  });

  it("never ages a checkout with a live owner into a fault", () => {
    const ledger = new InteractionCheckoutLedger();
    ledger.checkOut("frame-deferred");
    for (let opportunity = 0; opportunity < 100_000; opportunity += 1) expect(ledger.admit(true, false, true)).toBe("deferred");
    expect(ledger.takeAbandonedNotice()).toBeNull();
    expect(ledger.opportunities).toBe(100_000);
  });

  it("reports an abandoned checkout once and clears the episode on check-in", () => {
    const ledger = new InteractionCheckoutLedger();
    ledger.checkOut("frame-deferred");
    expect(ledger.admit(true, false, false)).toBe("abandoned");
    expect(ledger.takeAbandonedNotice()).toEqual({ site: "frame-deferred", opportunities: 1 });
    expect(ledger.admit(true, false, false)).toBe("abandoned");
    expect(ledger.takeAbandonedNotice()).toBeNull();
    ledger.checkIn();
    ledger.checkOut("dispatch-event");
    expect(ledger.admit(true, false, true)).toBe("deferred");
    expect(ledger.takeAbandonedNotice()).toBeNull();
  });

  it("counts a ready completion that restores the state as an outstanding owner", () => {
    const queue = new BoundedCompletionQueue(8);
    expect(queue.interactionOwnerOutstanding()).toBe(false);
    queue.enqueue({ key: null, revision: 1, requiresInteraction: true, restoresInteraction: false });
    expect(queue.interactionOwnerOutstanding()).toBe(false);
    queue.enqueue({ key: null, revision: 2, requiresInteraction: false, restoresInteraction: true });
    expect(queue.interactionOwnerOutstanding()).toBe(true);
    queue.takeAt(1);
    expect(queue.interactionOwnerOutstanding()).toBe(false);
    expect(queue.reserveInteraction()).toBe(true);
    expect(queue.interactionOwnerOutstanding()).toBe(true);
  });
});
