/**
 * 🧾️ TypeScript twin of `🧪️tests/🧾️frame-action-ledger/🦀️.rs`.
 *
 * Re-derives, from the SAME oracle (`🧫️fixtures/🧾️frame-action-ledger/🔣️.json`) and without reading the
 * Rust that implements it, who owns an action between the frame authority that mints it and the
 * deferred owner that hands it to the shell: a FIFO ledger the RUNTIME keeps, which a discarded frame
 * candidate cannot touch, which only a completing frame takes and only when no previous owner is still
 * draining.
 *
 * ⚖️ An independent derivation is the point. The defect this closes was invisible to every existing
 * law because they all drove an UNINTERRUPTED frame, where the two ownership models are
 * indistinguishable — the oracle keeps both models side by side so the difference is what is scored.
 */

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(here, "../../🧫️fixtures/🧾️frame-action-ledger/🔣️.json");
const rendererPath = resolve(here, "../../🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs");
const frameJobPath = resolve(here, "../../🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs");

type Step = { op: string; action?: string | null; installs?: boolean };
type Row = { id: string; why: string; owner: "runtime" | "candidate"; steps: Step[]; expect: { dispatched: string[]; lost: string[]; ledgerDepth: number } };
type Fixture = { rows: Row[] };

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
const rendererSource = readFileSync(rendererPath, "utf8");
const frameJobSource = readFileSync(frameJobPath, "utf8");

/** 🧾️ The frame runtime, re-derived: one ledger, one deferred owner, and whatever the shell was handed. */
class FrameRuntime {
  readonly dispatched: string[] = [];
  readonly lost: string[] = [];
  private ledger: string[] = [];
  private candidate: string[] = [];
  private owner: string[] | null = null;

  constructor(private readonly runtimeOwned: boolean) {}

  private queue(): string[] {
    return this.runtimeOwned ? this.ledger : this.candidate;
  }

  mint(action: string): void {
    this.queue().push(action);
  }

  /** 🌀️ A candidate that is dropped (superseded) or closed (cancelled) takes its own queue with it. */
  discardCandidate(): void {
    if (this.runtimeOwned) return;
    this.lost.push(...this.candidate);
    this.candidate = [];
  }

  complete(): boolean {
    if (this.owner !== null || this.queue().length === 0) return false;
    this.owner = this.queue();
    if (this.runtimeOwned) this.ledger = [];
    else this.candidate = [];
    return true;
  }

  dispatch(): string | null {
    if (this.owner === null) return null;
    const action = this.owner.shift();
    if (action === undefined) {
      this.owner = null;
      return null;
    }
    this.dispatched.push(action);
    return action;
  }

  get depth(): number {
    return this.ledger.length + this.candidate.length;
  }
}

/** 🧱️ The body of one Rust struct declaration, for the ownership assertions below. */
const structBody = (source: string, declaration: string): string => {
  const start = source.indexOf(declaration);
  expect(start, `${declaration} still exists`).toBeGreaterThanOrEqual(0);
  return source.slice(start, start + source.slice(start).indexOf("\n}"));
};

describe("frame action ledger", () => {
  it("keeps a commit across every frame candidate discarded before it dispatched", () => {
    expect(fixture.rows.length).toBeGreaterThanOrEqual(9);
    for (const row of fixture.rows) {
      const runtime = new FrameRuntime(row.owner === "runtime");
      row.steps.forEach((step, index) => {
        const where = `${row.id} step ${index}`;
        switch (step.op) {
          case "mint":
            runtime.mint(String(step.action));
            break;
          case "supersede":
          case "retire":
            runtime.discardCandidate();
            break;
          case "complete":
            expect(runtime.complete(), `${where}: installs`).toBe(step.installs);
            break;
          case "dispatch":
            expect(runtime.dispatch(), `${where}: the action the owner hands the shell`).toBe(step.action ?? null);
            break;
          default:
            throw new Error(`${where}: the fixture names operation ${step.op}, which the renderer never performs`);
        }
      });
      expect(runtime.dispatched, `${row.id}: dispatched`).toEqual(row.expect.dispatched);
      expect(runtime.lost, `${row.id}: lost`).toEqual(row.expect.lost);
      expect(runtime.depth, `${row.id}: ledger depth`).toBe(row.expect.ledgerDepth);
    }
  });

  it("states the ledger once, on the runtime, and leaves no frame candidate owning one", () => {
    expect(rendererSource).toContain("    frame_actions: FrameActionOwners,");
    expect(rendererSource).toContain("app.frame_actions.try_push(action)");
    expect(rendererSource).toContain("std::mem::take(&mut self.frame_actions)");
    for (const declaration of ["struct FrameTransaction {", "struct FrameBuildCursor {", "struct AppFrameAfterChrome {", "struct FrameFinishCursor {"]) {
      expect(structBody(rendererSource, declaration), `${declaration} owns no action queue`).not.toContain("deferred_actions");
    }
    expect(rendererSource, "no close ladder retires a user's commit as a retirement unit").not.toContain("deferred_actions.pop_front()");
  });

  it("keeps the superseded candidate a plain drop, which is what makes the ledger's owner load-bearing", () => {
    expect(frameJobSource).toContain("AppFrameTransactionStep::Superseded => {\n                        self.phase = ActiveFramePhase::Terminal;");
  });
});
