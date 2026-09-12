/** 🎟️ The TypeScript half of the shared resident-refresh-budget law — same fixture as the Rust twin
 * (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧫️fixtures/🔣️.json`, Rust twin
 * `…/🎟️resident/🔄️refresh/🧪️tests/🔄️refresh/🦀️.rs`).
 *
 * 🏁️ What this encodes: the retained-UI resident aggregate is ONE fixed, process-wide ledger, and a
 * refresh that replaces N surfaces' documents is only admissible if it never needs a SECOND full set
 * of them resident at once. The wgpu shell used to move every live document into its retirement
 * registry and only then ask the guest for the replacements, so the aggregate had to carry 2N roots;
 * `UiResidentPermit::try_reserve` refused the second set on every surface with a bare `Capacity` and
 * the shell kept the stale documents forever (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
 * `📓️wgpu-dock-layout-world3d-2026-09-12.md` §6.2). React never had the defect because
 * `UiDocumentStore` retires a surface's previous document as it publishes its replacement — one root
 * at a time — which is exactly the order the wgpu shell now runs.
 *
 * ⚖️ The ledger below is an INDEPENDENT re-implementation of the contract's own arithmetic
 * (`🎟️resident/🦀️.rs`: a fixed slot table, an item ceiling, a byte ceiling, and a fixed backing
 * charge that is committed before any dynamic root), driven from the same fixture, so neither side
 * can drift alone. */

import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, it } from "vitest";

//#region 🧫️Fixture
type FixtureSurface = { readonly id: string; readonly nodes: number };
type FixtureMeasurement = {
  readonly target: string;
  readonly run: string;
  readonly seconds: number;
  readonly surfaces: number;
  readonly residentRoots: number;
  readonly residentBytes: number;
  readonly fixedBackingBytes: number;
  readonly capacityFaults: number;
};
type Fixture = {
  readonly version: 1;
  readonly aggregateBytes: number;
  readonly surfaceBytes: number;
  readonly surfaceItems: number;
  readonly documentNodes: number;
  readonly nodeRecordBytes: number;
  readonly openBytes: number;
  readonly refreshes: number;
  readonly retirementGrantItems: number;
  readonly retirementGrantBytes: number;
  readonly surfaces: readonly FixtureSurface[];
  readonly peakResidentRoots: number;
  readonly peakResidentBytes: number;
  readonly permitFaults: number;
  readonly doubleBufferedPeakRoots: number;
  readonly doubleBufferedPeakBytes: number;
  readonly ceilingSizedRoots: number;
  readonly measured: FixtureMeasurement;
  readonly laws: readonly string[];
};

const here = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(here, "../../../../../../../🔨️modules/🖱️ui/🧬️contract/🎟️resident/🔄️refresh/🧫️fixtures/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
//#endregion 🧫️Fixture

//#region 🎟️Ledger
/** 🎟️ The contract's aggregate, re-derived: `UI_RESIDENT_SLOTS` fixed positions, one item ceiling,
 * one byte ceiling, and the fixed backing charge already committed at rest. A reservation is admitted
 * only if all three still fit; anything else is a typed `Capacity` refusal, never a silent skip. */
const RESIDENT_SLOTS = 64;

type Limits = { readonly items: number; readonly bytes: number };

class ResidentLedger {
  private readonly occupied: (Limits | null)[] = Array.from({ length: RESIDENT_SLOTS }, () => null);
  private items = 0;
  private bytes: number;

  constructor(fixedBackingBytes: number) {
    this.bytes = fixedBackingBytes;
  }

  get roots(): number {
    return this.occupied.filter((slot) => slot !== null).length;
  }

  get committedBytes(): number {
    return this.bytes;
  }

  get committedItems(): number {
    return this.items;
  }

  reserve(limits: Limits): number | "capacity" | "invalid-limits" {
    if (limits.items > fixture.surfaceItems || limits.bytes > fixture.surfaceBytes) return "invalid-limits";
    if (this.items + limits.items > fixture.surfaceItems * 32) return "capacity";
    if (this.bytes + limits.bytes > fixture.aggregateBytes) return "capacity";
    const slot = this.occupied.findIndex((entry) => entry === null);
    if (slot < 0) return "capacity";
    this.occupied[slot] = limits;
    this.items += limits.items;
    this.bytes += limits.bytes;
    return slot;
  }

  release(slot: number): void {
    const limits = this.occupied[slot];
    assert(limits, "a released resident slot was the exact owner");
    this.occupied[slot] = null;
    this.items -= limits.items;
    this.bytes -= limits.bytes;
  }
}

/** 📐️ The reservation one published document asks for — the arithmetic the wgpu ProgramBridge runs
 * before it opens a root, and the arithmetic the Rust twin asserts against `size_of::<UiNodeRecord>()`. */
function surfaceLimits(nodes: number): Limits {
  return {
    items: Math.min(nodes + 2, fixture.surfaceItems),
    bytes: Math.min((nodes + 2) * fixture.nodeRecordBytes + fixture.openBytes, fixture.surfaceBytes),
  };
}
//#endregion 🎟️Ledger

describe("retained resident refresh budget", () => {
  it("never needs a second full resident set", () => {
    const ledger = new ResidentLedger(fixture.measured.fixedBackingBytes);
    const live: (number | null)[] = fixture.surfaces.map(() => null);
    let peakRoots = 0;
    let peakBytes = 0;
    let faults = 0;
    for (let refresh = 0; refresh < fixture.refreshes; refresh += 1) {
      fixture.surfaces.forEach((surface, index) => {
        // ♻️ This surface's PREVIOUS root is released before its replacement asks for credit.
        const previous = live[index];
        if (previous !== null) {
          ledger.release(previous);
          live[index] = null;
        }
        const slot = ledger.reserve(surfaceLimits(surface.nodes));
        if (slot === "capacity") faults += 1;
        else {
          assert.notEqual(slot, "invalid-limits", `${surface.id} fits one surface ceiling`);
          live[index] = slot as number;
        }
        peakRoots = Math.max(peakRoots, ledger.roots);
        peakBytes = Math.max(peakBytes, ledger.committedBytes - fixture.measured.fixedBackingBytes);
      });
    }
    for (const slot of live) if (slot !== null) ledger.release(slot);
    console.log(`[DEBUG] ${JSON.stringify({ surfaces: fixture.surfaces.length, refreshes: fixture.refreshes, peakRoots, peakBytes, faults })}`);
    assert.equal(faults, fixture.permitFaults);
    assert.equal(peakRoots, fixture.peakResidentRoots);
    assert.equal(peakRoots, fixture.surfaces.length);
    assert.equal(peakBytes, fixture.peakResidentBytes);
    assert.equal(ledger.roots, 0);
    assert.equal(ledger.committedBytes, fixture.measured.fixedBackingBytes);
  });

  it("shows the double-buffered order needs a second full set", () => {
    const ledger = new ResidentLedger(fixture.measured.fixedBackingBytes);
    const first = fixture.surfaces.map((surface) => ledger.reserve(surfaceLimits(surface.nodes)));
    const singleRoots = ledger.roots;
    const singleBytes = ledger.committedBytes - fixture.measured.fixedBackingBytes;
    const second = fixture.surfaces.map((surface) => ledger.reserve(surfaceLimits(surface.nodes)));
    const doubledRoots = ledger.roots;
    const doubledBytes = ledger.committedBytes - fixture.measured.fixedBackingBytes;
    for (const slot of [...first, ...second]) {
      assert.equal(typeof slot, "number", "both full sets are admitted against an otherwise empty ledger");
      ledger.release(slot as number);
    }
    console.log(`[DEBUG] ${JSON.stringify({ singleRoots, singleBytes, doubledRoots, doubledBytes })}`);
    assert.equal(doubledRoots, fixture.doubleBufferedPeakRoots);
    assert.equal(doubledBytes, fixture.doubleBufferedPeakBytes);
    assert.equal(doubledRoots, 2 * singleRoots);
    assert.equal(doubledBytes, 2 * singleBytes);
  });

  it("admits only three ceiling-sized surfaces", () => {
    const ledger = new ResidentLedger(fixture.measured.fixedBackingBytes);
    const ceiling: Limits = { items: fixture.surfaceItems, bytes: fixture.surfaceBytes };
    const admitted: number[] = [];
    let refusal: string | null = null;
    for (let attempt = 0; attempt <= fixture.ceilingSizedRoots; attempt += 1) {
      const slot = ledger.reserve(ceiling);
      if (typeof slot === "number") admitted.push(slot);
      else refusal = slot;
    }
    for (const slot of admitted) ledger.release(slot);
    console.log(`[DEBUG] ${JSON.stringify({ admitted: admitted.length, refusal, surfaceBytes: fixture.surfaceBytes, aggregateBytes: fixture.aggregateBytes })}`);
    assert.equal(admitted.length, fixture.ceilingSizedRoots);
    assert.equal(refusal, "capacity");
  });

  it("carries the measured six-surface census the wgpu shell reported", () => {
    const { measured } = fixture;
    console.log(`[DEBUG] ${JSON.stringify(measured)}`);
    assert.equal(measured.capacityFaults, 0);
    assert.equal(measured.residentRoots, fixture.peakResidentRoots);
    assert.equal(measured.surfaces, fixture.surfaces.length);
    assert(measured.residentBytes > measured.fixedBackingBytes, "the live census includes the fixed backing charge");
    assert(measured.residentBytes < fixture.aggregateBytes / 2, "a settled six-surface refresh sits nowhere near the aggregate ceiling");
    assert.equal(fixture.laws.length, 4);
  });
});
