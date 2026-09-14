/** 🚪️ The ingress-generation gate: a refresh request whose scope the pass in flight already covers,
 * and for which NO guest-mutating ingress was submitted since that pass submitted its own turn, is
 * already answered by it and joins it instead of owing a second guest turn.
 *
 * 🩺️ Why it exists, measured: one `flowEvalTick` hop on the React door cost three serialized guest
 * turns — one dispatch and two full refresh passes at ~380-420 ms each — because a converging preview
 * arms one chain per flow window and each chain's completion demands a pass of its own. Both
 * completions are host-side callbacks of turns that had ALREADY settled, and the second request
 * arrives a few milliseconds after the first pass submitted its own turn, so nothing crossed into the
 * guest in between (`📓️react-hop-latency-2026-09-14.md` §3.2/§4,
 * `📓️react-guest-turn-cost-2026-09-14.md`).
 *
 * ⚖️ Both directions are law, and the second one is the dangerous one: a join that should not have
 * happened is a stale pane with no fault anywhere. So every refusal is asserted by name — a dispatch
 * that crossed since, a host-owned render input, a wider scope, a body replacement the pass in flight
 * is not doing, a different instance, and a pass that has not submitted its turn at all.
 */
import { describe, expect, it } from "vitest";
import fixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🚪️ingress-generation-gate.json" with { type: "json" };
import { resolvePluginCanvasStatus } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import { createUiRefreshCoalescerV1, uiDirtyScopeCoveredByV1, uiRefreshAlreadyAnsweredV1 } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import { uiIntakeOwesYieldV1 } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import type { UiDirtyScope } from "@semio-tech/framework";

// 🔗️ `🐚️Shell` is imported FIRST on purpose: `ShellHelpers` and `Shell` are mutually recursive, and a
// suite that enters the cycle through `ShellHelpers` alone evaluates `shellLabel` before `Shell`'s
// own bindings exist (`Cannot access '__vite_ssr_import_13__' before initialization`). Entering it
// through `Shell`, the way `🔬️engine-contract` does, is what makes the module graph resolve.
void resolvePluginCanvasStatus;

type LaneRequest = { readonly instanceId: number; readonly scope: UiDirtyScope; readonly replaceBodies: boolean; readonly hostInputs: boolean };

describe("🚪️ refresh ingress-generation gate", () => {
  it("declares the contract the lane answers to", () => {
    expect(fixture.contract.refreshTurnDoesNotBumpIngress).toBe(true);
    expect(fixture.contract.dispatchBumpsIngress).toBe(true);
    expect(fixture.contract.extensionCompletionPublicationBumpsIngress).toBe(true);
    expect(fixture.contract.hostInputsNeverJoin).toBe(true);
    expect(fixture.contract.unknownCoverageRefuses).toBe(true);
    expect(fixture.contract.joinerInheritsThePassVerdict).toBe(true);
  });

  for (const scenario of fixture.coverage) {
    it(`coverage: ${scenario.name}`, () => {
      expect(uiDirtyScopeCoveredByV1(scenario.inner as UiDirtyScope, scenario.outer as UiDirtyScope)).toBe(scenario.covered);
    });
  }

  for (const scenario of fixture.cases) {
    it(scenario.name, () => {
      expect(uiRefreshAlreadyAnsweredV1(scenario.running as never, scenario.next as never, scenario.ingressNow)).toBe(scenario.answered);
    });
  }

  it("joins the pass in flight rather than running a second one, and the joiner settles with it", async () => {
    let ingress = 4;
    let release: (() => void) | null = null;
    const ran: UiDirtyScope[] = [];
    const lane = createUiRefreshCoalescerV1<LaneRequest>(
      async (request) => {
        ran.push(request.scope);
        await new Promise<void>((resolve) => {
          release = resolve;
        });
      },
      (owed, next) => ({ ...next, hostInputs: owed.hostInputs || next.hostInputs }),
      undefined,
      (running, next) => uiRefreshAlreadyAnsweredV1({ ...running, ingressAtSubmit: ingress }, next, ingress),
    );
    const first = lane.request({ instanceId: 1, scope: { kind: "full" }, replaceBodies: false, hostInputs: true });
    const joined = lane.request({ instanceId: 1, scope: { kind: "full" }, replaceBodies: false, hostInputs: false });
    expect(lane.answered()).toBe(1);
    expect(lane.owedScope()).toBeNull();
    release!();
    await Promise.all([first, joined]);
    expect(ran).toHaveLength(1);
    expect(lane.passes()).toBe(1);
  });

  it("owes a second pass when ingress crossed while the first was in flight", async () => {
    let ingress = 4;
    const releases: (() => void)[] = [];
    const ran: number[] = [];
    const lane = createUiRefreshCoalescerV1<LaneRequest>(
      async () => {
        ran.push(ingress);
        await new Promise<void>((resolve) => releases.push(resolve));
      },
      (owed, next) => ({ ...next, hostInputs: owed.hostInputs || next.hostInputs }),
      undefined,
      (running, next) => uiRefreshAlreadyAnsweredV1({ ...running, ingressAtSubmit: 4 }, next, ingress),
    );
    const first = lane.request({ instanceId: 1, scope: { kind: "full" }, replaceBodies: false, hostInputs: true });
    ingress = 5;
    const second = lane.request({ instanceId: 1, scope: { kind: "full" }, replaceBodies: false, hostInputs: false });
    expect(lane.answered()).toBe(0);
    expect(lane.owedScope()).toEqual({ kind: "full" });
    releases[0]!();
    await first;
    await Promise.resolve();
    releases[1]?.();
    await second;
    expect(ran).toHaveLength(2);
    expect(lane.passes()).toBe(2);
  });

  it("hands a joiner the rejection of the pass it joined, so a failed pass is never silently reported as covered", async () => {
    const lane = createUiRefreshCoalescerV1<LaneRequest>(
      async () => {
        await Promise.resolve();
        throw new Error("pass failed");
      },
      (owed, next) => ({ ...next, hostInputs: owed.hostInputs || next.hostInputs }),
      undefined,
      (running, next) => uiRefreshAlreadyAnsweredV1({ ...running, ingressAtSubmit: 1 }, next, 1),
    );
    const first = lane.request({ instanceId: 1, scope: { kind: "full" }, replaceBodies: false, hostInputs: true });
    const joined = lane.request({ instanceId: 1, scope: { kind: "full" }, replaceBodies: false, hostInputs: false });
    expect(lane.answered()).toBe(1);
    await expect(first).rejects.toThrow("pass failed");
    await expect(joined).rejects.toThrow("pass failed");
    expect(lane.passes()).toBe(1);
  });

  it("asks for the intake yield on the same stride it always did, without a promise per step", () => {
    // 🪃️ The cadence is the law, not the call shape: exactly one yield per stride, and the predicate
    // itself must be synchronous — an `await` per intake step is what this replaced.
    const owed: number[] = [];
    for (let step = 1; step <= 4_096; step += 1) if (uiIntakeOwesYieldV1(step)) owed.push(step);
    expect(owed).toEqual([1_024, 2_048, 3_072, 4_096]);
    expect(uiIntakeOwesYieldV1(1)).toBe(false);
    expect(uiIntakeOwesYieldV1(1_023)).toBe(false);
    expect(uiIntakeOwesYieldV1(0)).toBe(true);
    expect(uiIntakeOwesYieldV1(1) as unknown).not.toBeInstanceOf(Promise);
  });

  it("never joins without a gate, so the lane's behaviour is unchanged where no gate is supplied", async () => {
    let release: (() => void) | null = null;
    const lane = createUiRefreshCoalescerV1<LaneRequest>(
      async () => {
        await new Promise<void>((resolve) => {
          release = resolve;
        });
      },
      (owed, next) => ({ ...next, hostInputs: owed.hostInputs || next.hostInputs }),
    );
    void lane.request({ instanceId: 1, scope: { kind: "full" }, replaceBodies: false, hostInputs: false });
    void lane.request({ instanceId: 1, scope: { kind: "full" }, replaceBodies: false, hostInputs: false });
    expect(lane.answered()).toBe(0);
    expect(lane.owedScope()).toEqual({ kind: "full" });
    release!();
  });
});
