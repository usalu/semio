/**
 * 🛍️ The TypeScript twin of `🐚️Shell/🧪️tests/🛍️app-catalogue-attempt/🦀️.rs`. Both read the SAME
 * neutral fixture (`🧑‍🎨engine/🧫️fixtures/🛍️app-catalogue-attempt/🔣️.json`): the Rust law drives the
 * production predicate `claim_app_catalogue_fetch`, and this one re-derives the rule from the
 * fixture's own statement with a second, independent implementation.
 *
 * The defect both sides pin: the shell recorded the catalogue's app instance only after a successful
 * reassembly, so a catalogue that failed was re-fetched by every refresh — and a refresh runs on
 * every settled command. The SECOND fetch of `framework.section.catalogue` never returned from its
 * own turn, so `boot_shell` never returned, the canvas never bound a pointer listener, and no
 * pointer, wheel or key event reached the shell at all
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-input-hit-runtime-2026-09-13.md`).
 */
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const fixturePath = resolve(suiteRoot, "../../🧫️fixtures/🛍️app-catalogue-attempt/🔣️.json");

type Refresh = { readonly instanceId: number; readonly outcome?: string };
type Case = {
  readonly name: string;
  readonly refreshes: readonly Refresh[];
  readonly expectedFetchIndices: readonly number[];
  readonly baselineFetchIndices: readonly number[];
  readonly discriminates: boolean;
  readonly recordedAfter: number;
};
type Fixture = { readonly rule: { readonly recordedIsAttemptNotSuccess: boolean }; readonly outcomes: readonly string[]; readonly cases: readonly Case[] };

const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;

/** 🛍️ The rule, re-derived: claiming records the instance immediately, so the fetch's outcome can
 * never change how many fetches happen. */
const claim = (recorded: number | null, instanceId: number): { readonly claimed: boolean; readonly recorded: number } =>
  recorded === instanceId ? { claimed: false, recorded } : { claimed: true, recorded: instanceId };

/** 🛍️ The PRE-FIX rule: the instance was recorded only when that fetch's payload reassembled. */
const baselineClaim = (recorded: number | null, instanceId: number, outcome: string | undefined): { readonly claimed: boolean; readonly recorded: number | null } =>
  recorded === instanceId ? { claimed: false, recorded } : { claimed: true, recorded: outcome === "succeeded" ? instanceId : recorded };

const replay = (refreshes: readonly Refresh[], step: (recorded: number | null, refresh: Refresh) => { readonly claimed: boolean; readonly recorded: number | null }) => {
  let recorded: number | null = null;
  const fetched: number[] = [];
  refreshes.forEach((refresh, index) => {
    const next = step(recorded, refresh);
    recorded = next.recorded;
    if (next.claimed) fetched.push(index);
  });
  return { fetched, recorded };
};

describe("🛍️ app catalogue attempt", () => {
  it("declares the cache key as the ATTEMPTED instance, not the succeeded one", () => {
    expect(fixture.rule.recordedIsAttemptNotSuccess).toBe(true);
    expect(fixture.outcomes).toContain("reassembly-failed");
    expect(fixture.cases.length).toBeGreaterThan(0);
  });

  for (const testCase of fixture.cases) {
    it(`fetches once per app instance — ${testCase.name}`, () => {
      const { fetched, recorded } = replay(testCase.refreshes, (current, refresh) => claim(current, refresh.instanceId));
      expect(fetched).toEqual([...testCase.expectedFetchIndices]);
      expect(recorded).toBe(testCase.recordedAfter);
    });
  }

  for (const testCase of fixture.cases) {
    it(`tells the pre-fix rule apart as the fixture declares — ${testCase.name}`, () => {
      const { fetched } = replay(testCase.refreshes, (current, refresh) => baselineClaim(current, refresh.instanceId, refresh.outcome));
      expect(fetched).toEqual([...testCase.baselineFetchIndices]);
      expect(fetched.join(",") !== testCase.expectedFetchIndices.join(",")).toBe(testCase.discriminates);
    });
  }

  it("carries cases that the pre-fix rule fails", () => {
    expect(fixture.cases.filter((entry) => entry.discriminates).length).toBeGreaterThanOrEqual(3);
  });
});
