/** 🏁 Job completion ownership, the whole-instance UI close ladder budget and isolated job admission batches. */
import { encodeAppFrame } from "@semio-tech/framework-os";
import { type PluginRuntimeVitest, type PluginRuntimeDependencies } from "../🛠️plugin-runtime-helpers/🟦️.tsx";

export async function registerUiCloseAndJobAdmissionTests(vitest: PluginRuntimeVitest, dependencies: PluginRuntimeDependencies): Promise<void> {
  const { describe, expect, it, vi } = vitest;
  const { leftoverShellInvocationFrames } = dependencies;
  it("validates independent job completion ownership and latest render authority", async () => {
    const { default: fixture } = await import("../../../../🔌️plugin/⚛️reactor/🧪️tests/🧫️fixtures/🏁️job-completion-ownership/🔣️.json");
    for (const row of fixture.cases) {
      const owners = new Map<string, number>();
      const current = new Map<number, string>();
      for (const step of row.steps) {
        if (step.action === "bind") {
          owners.set(step.job, step.instance);
          current.set(step.instance, step.job);
        } else if (step.action === "complete") {
          owners.delete(step.job);
          if (current.get(step.instance) === step.job) current.delete(step.instance);
        } else {
          for (const [job, instance] of owners) if (instance === step.instance) owners.delete(job);
          current.delete(step.instance);
        }
        expect([...owners.keys()].sort()).toEqual(step.owned);
        expect([...current.values()].sort()).toEqual(step.accepted);
      }
    }
    console.info("[DEBUG] Job ownership oracle: five independent histories preserve all terminals and latest repaint");
  });

  /** 🚪️ The whole-instance UI close ladder's own budget, priced off what a close RETIRES.
   *
   * 🐛️ It used to spend `PLUGIN_UI_CONTINUATION_LIMIT` — a SETTLE bound, host round trips for ONE
   * turn — and threw `plugin-ui.owner-close-budget-exhausted` with no phase and no count on a
   * converged generation3d editor instance (`📓️role-switch-regression-2026-09-14.md` §6). The ceiling
   * is a backstop; the progress GUARANTEE is byte-aware, the same rule `OwnedUiPatchIntake` applies to
   * a mint. */
  describe("plugin ui close ladder budget", () => {
    it("prices a close by the surfaces it has to retire, never by a settle's round trips", () => {
      const { retainedUiCloseStepCeilingV1, PLUGIN_UI_CLOSE_STEPS_PER_NODE, PLUGIN_UI_CONTINUATION_LIMIT } = dependencies;
      const one = retainedUiCloseStepCeilingV1(1);
      expect(retainedUiCloseStepCeilingV1(0)).toBe(one);
      expect(retainedUiCloseStepCeilingV1(11)).toBe(one * 11);
      expect(one % PLUGIN_UI_CLOSE_STEPS_PER_NODE).toBe(0);
      expect(one).toBeGreaterThan(PLUGIN_UI_CONTINUATION_LIMIT);
    });

    it("keeps the stall rule far tighter than the backstop, so a stuck ladder is NAMED rather than ground through", () => {
      const { retainedUiCloseStepCeilingV1, PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS, PLUGIN_UI_CONTINUATION_LIMIT } = dependencies;
      expect(PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS).toBeGreaterThan(0);
      expect(PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS).toBeLessThan(PLUGIN_UI_CONTINUATION_LIMIT);
      expect(PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS).toBeLessThan(retainedUiCloseStepCeilingV1(1));
    });

    it("counts one turn per retained item and never reads the bytes under it", () => {
      const { createRetainedUiCloseLadderV1 } = dependencies;
      const items = ["instance-wire-release", "instance-surface-release", "instance-close"];
      const payload = ["surface-root-close", "node-field-retire", "typed-object-retire", "prepared-scene-source-close"];
      const census = [1, 64, 4_096].map((payloadPerItem) => {
        const ladder = createRetainedUiCloseLadderV1(2);
        for (const item of items) {
          for(let step=0;step<payloadPerItem;step++){const phase=payload[step%payload.length];if(phase===undefined)throw Error("Actual owned close phase required");expect(ladder.admit({phase,bytes:1})).toBeNull();}
          expect(ladder.admit({ phase: item, bytes: 128 })).toBeNull();
        }
        return ladder.turns;
      });
      expect(census).toEqual([items.length, items.length, items.length]);
    });

    it("cannot exhaust its budget on a converged instance, at any payload", () => {
      const { createRetainedUiCloseLadderV1 } = dependencies;
      const ladder = createRetainedUiCloseLadderV1(1);
      const faults: string[] = [];
      for (let item = 0; item < ladder.ceiling; item += 1) {
        const fault = ladder.admit({ phase: "instance-surface-release", bytes: 64 });
        if (fault) faults.push(fault);
      }
      expect(faults).toEqual([]);
      expect(ladder.turns).toBe(ladder.ceiling);
      expect(ladder.admit({ phase: "instance-close", bytes: 64 })).toMatch(/^plugin-ui\.owner-close-budget-exhausted:instance-close after \d+ turns over 1 retained surfaces$/);
    });

    it("spends no turn on the descendant phases closeChild forwards, however many arrive", () => {
      const { createRetainedUiCloseLadderV1 } = dependencies;
      const ladder = createRetainedUiCloseLadderV1(14);
      for (let step = 0; step < 100_000; step += 1) expect(ladder.admit({ phase: step % 2 === 0 ? "prepared-scene-source-close" : "typed-retire", bytes: 8 })).toBeNull();
      expect(ladder.turns).toBe(0);
      expect(ladder.admit({ phase: "instance-surface-release", bytes: 1152 })).toBeNull();
      expect(ladder.turns).toBe(1);
    });

    it("names the phase of a ladder that releases nothing, long before the backstop", () => {
      const { createRetainedUiCloseLadderV1, PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS } = dependencies;
      const ladder = createRetainedUiCloseLadderV1(4);
      for (let step = 0; step < PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS; step += 1) expect(ladder.admit({ phase: "node-field-retire", bytes: 0 })).toBeNull();
      expect(ladder.admit({ phase: "node-field-retire", bytes: 0 })).toBe(`plugin-ui.owner-close-stalled:node-field-retire released nothing for ${PLUGIN_UI_CLOSE_ZERO_PROGRESS_STEPS + 1} steps`);
      expect(ladder.turns).toBe(0);
    });
  });

  describe("isolated job admission batch", () => {
    it("fails-before one slice per admission; passes-after a yield-sized batch", () => {
      const { isolatedJobStepsPerSerializedAdmission } = dependencies;
      expect(isolatedJobStepsPerSerializedAdmission(1)).toBe(1);
      expect(isolatedJobStepsPerSerializedAdmission(32)).toBe(32);
      expect(isolatedJobStepsPerSerializedAdmission(0)).toBe(1);
    });
    it("reads history_patch from a leftover job-completed Invocation send-message", () => {
      const { invocationFromFrames, encodePackValue } = dependencies;
      const patch = { cursor: 3, canUndo: true, canRedo: true, upserts: [{ seq: 2, actionId: "os.resizeWindow", label: "Resize Window", kind: "shell", timestamp: "0", revertible: false }] };
      const leftover = [{
        tag: "send-message",
        val: {
          target: { tag: "shell", val: "1" },
          payload: Array.from(encodeAppFrame({
            Invocation: {
              in_reply_to: 0,
              output: [],
              diagnostics: [],
              ui_scope: [],
              history_patch: Array.from(encodePackValue(patch)),
              messages: [],
              mutations: [],
              inverse_group: [],
            },
          })),
        },
      }, { tag: "replay-shell-command", val: { actionId: "os.resizeWindow", args: encodePackValue({ width: 800, height: 600 }) } }];
      const response = invocationFromFrames([{ Invocation: { in_reply_to: 1, output: [], diagnostics: [], ui_scope: [], history_patch: [], messages: [], mutations: [], inverse_group: [] } }], leftover, "action");
      expect(response.historyPatch).toEqual(patch);
      expect(response.requestedEffects).toEqual([{ replayShellCommand: { actionId: "os.resizeWindow", args: { width: 800, height: 600 } } }]);
      expect(leftoverShellInvocationFrames(leftover)).toHaveLength(1);
    });

    // 📋️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END (`📓️selection-dedupe-2026-09-12.md`): a reserved job
    // publishes its result frame on `send-message` whatever it did, so warning "clipboard-write missing"
    // whenever a leftover carried one fired on EVERY reserved-job completion — measured on every
    // `interactionSelect`/`interactionHover` of the generation3d preview. Nothing in the leftover list
    // declares that a clipboard write was expected, so there is no predicate to warn on.
    it("never warns about a missing clipboard-write on a job completion that only carries send-message effects", () => {
      const { promoteShellSendMessages } = dependencies;
      const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
      try {
        const leftover = [
          { tag: "send-message", val: { target: { tag: "shell", val: "1" }, payload: [1, 2, 3] } },
          { tag: "send-message", val: { target: { tag: "shell", val: "1" }, payload: [4, 5, 6] } },
        ];
        expect(promoteShellSendMessages(1, leftover, [])).toEqual(leftover);
        expect(warn.mock.calls.flat().filter((entry) => String(entry).includes("clipboard"))).toEqual([]);
      } finally {
        warn.mockRestore();
      }
    });

    it("fails-before every step polls UI; passes-after a 128-step stride", () => {
      const { isolatedJobUiPollEverySteps } = dependencies;
      expect(isolatedJobUiPollEverySteps(1, 1), "fails-before: stride 1 polls every step").toBe(true);
      expect(isolatedJobUiPollEverySteps(0)).toBe(false);
      expect(isolatedJobUiPollEverySteps(127)).toBe(false);
      expect(isolatedJobUiPollEverySteps(128)).toBe(true);
      expect(isolatedJobUiPollEverySteps(256)).toBe(true);
    });
  });
}
