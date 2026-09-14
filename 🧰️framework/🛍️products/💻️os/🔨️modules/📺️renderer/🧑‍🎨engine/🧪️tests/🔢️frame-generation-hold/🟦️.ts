/**
 * 🔢️ TypeScript twin of `🧪️tests/🔢️frame-generation-hold/🦀️.rs`.
 *
 * Re-derives the host loop's generation arithmetic from the SAME oracle
 * (`🧫️fixtures/🔢️frame-generation-hold/🔣️.json`) without looking at the Rust: a frame generation that
 * names the input state a build is answering and therefore may not move underneath a live build, an
 * input that is delivered whether or not its number was held, and ONE presentation authority whose
 * ADMITTED pair — written by the build, read by the presenter — decides admission, while its LIVE
 * pair keeps moving underneath both.
 *
 * ⚖️ An independent derivation is the point: if the two implementations agree with the oracle they
 * agree with each other, and the browser frame loop cannot start cancelling its own builds (or
 * quarantining its own surface) while the Rust unit tests stay green.
 *
 * 🐛️ Both halves were live defects on 6118 (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-edit-convergence-perf-2026-09-14.md` §6): every pointer move renumbered the generation and
 * cancelled the in-flight build from phase 0 — 28 supersessions per converging edit, 26 % of the wall
 * clock — and holding the number alone let the build finish into a presenter that re-read a moving
 * authority and answered `prepared render revision is stale: live=35, packet=27`.
 */

import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(here, "../../🧫️fixtures/🔢️frame-generation-hold/🔣️.json");

type Step = { readonly op: string; readonly repeat?: number };
type Expectation = { readonly generation: number; readonly delivered: number; readonly supersessions: number; readonly framesPresented: number; readonly quarantines: number };
type Row = { readonly id: string; readonly why: string; readonly steps: readonly Step[]; readonly expect: Expectation; readonly quarantinesIfPresenterRereadsLiveAuthority?: number };
type Fixture = { readonly rows: readonly Row[]; readonly laws: readonly string[] };

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

type Witness = { readonly sceneRevision: number; readonly inputGeneration: number };

/** 🎟️ The one presentation authority, re-derived: a live pair that moves, and an admitted pair one build writes. */
class PresentationAuthority {
  private sceneRevision = 1;
  private inputGeneration = 0;
  private admittedPair: Witness | undefined;

  markSceneChanged(): void {
    this.sceneRevision += 1;
  }

  observeInputGeneration(generation: number): void {
    this.inputGeneration = generation;
  }

  current(): Witness {
    return { sceneRevision: this.sceneRevision, inputGeneration: this.inputGeneration };
  }

  witnessFor(generation: number): Witness | undefined {
    const witness = this.current();
    return witness.inputGeneration === generation ? witness : undefined;
  }

  admitBuild(witness: Witness): void {
    this.admittedPair = witness;
  }

  admitted(): Witness {
    return this.admittedPair ?? this.current();
  }
}

function same(first: Witness, second: Witness): boolean {
  return first.sceneRevision === second.sceneRevision && first.inputGeneration === second.inputGeneration;
}

type Tally = { delivered: number; supersessions: number; framesPresented: number; quarantines: number; quarantinesAgainstLive: number };

/** 🎬️ One host loop, replayed from the transcript. */
function replay(row: Row): { generation: number; tally: Tally } {
  const authority = new PresentationAuthority();
  const tally: Tally = { delivered: 0, supersessions: 0, framesPresented: 0, quarantines: 0, quarantinesAgainstLive: 0 };
  let generation = 0;
  let liveBuild: { readonly witness: Witness; readonly generation: number } | undefined;
  let pendingPacket: Witness | undefined;
  for (const step of row.steps) {
    for (let index = 0; index < (step.repeat ?? 1); index += 1) {
      switch (step.op) {
        case "input":
        case "metrics":
          // 🔢️ Delivered either way; the NUMBER is held only while a build is live.
          if (liveBuild === undefined) generation += 1;
          tally.delivered += 1;
          break;
        case "sceneChange":
          authority.markSceneChanged();
          break;
        case "redraw":
          if (liveBuild === undefined) generation += 1;
          authority.observeInputGeneration(generation);
          break;
        case "forceRenumber":
          generation += 1;
          authority.observeInputGeneration(generation);
          break;
        case "buildAdmit": {
          expect(liveBuild, `${row.id}: two builds admitted at once`).toBeUndefined();
          authority.observeInputGeneration(generation);
          const witness = authority.witnessFor(generation);
          expect(witness, `${row.id}: the authority must publish the generation a build is admitted at`).toBeDefined();
          authority.admitBuild(witness!);
          liveBuild = { witness: witness!, generation };
          break;
        }
        case "buildComplete": {
          expect(liveBuild, `${row.id}: no build to complete`).toBeDefined();
          const finished = liveBuild!;
          liveBuild = undefined;
          if (finished.generation === generation) pendingPacket = finished.witness;
          else tally.supersessions += 1;
          break;
        }
        case "present": {
          if (pendingPacket === undefined) break;
          const packet = pendingPacket;
          pendingPacket = undefined;
          if (same(packet, authority.admitted())) tally.framesPresented += 1;
          else tally.quarantines += 1;
          if (!same(packet, authority.current())) tally.quarantinesAgainstLive += 1;
          break;
        }
        default:
          throw new Error(`${row.id}: unknown transcript operation ${step.op}`);
      }
    }
  }
  return { generation, tally };
}

describe("wgpu frame generation hold and shared presentation authority", () => {
  it("declares its laws", () => {
    expect(fixture.laws.length).toBe(5);
    expect(fixture.laws).toContain("an-input-event-never-renumbers-the-frame-generation-underneath-a-live-build");
    expect(fixture.laws).toContain("the-presenter-admits-a-packet-against-the-pair-its-build-was-admitted-under");
  });

  for (const row of fixture.rows) {
    it(row.id, () => {
      const { generation, tally } = replay(row);
      expect({ generation, delivered: tally.delivered, supersessions: tally.supersessions, framesPresented: tally.framesPresented, quarantines: tally.quarantines }).toEqual({
        generation: row.expect.generation,
        delivered: row.expect.delivered,
        supersessions: row.expect.supersessions,
        framesPresented: row.expect.framesPresented,
        quarantines: row.expect.quarantines,
      });
      // 🩸️ The counterfactual: a presenter that re-read the LIVE authority refuses a correctly-built
      // packet and quarantines the surface, which is exactly what was measured.
      expect(tally.quarantinesAgainstLive).toBe(row.quarantinesIfPresenterRereadsLiveAuthority ?? 0);
    });
  }
});
