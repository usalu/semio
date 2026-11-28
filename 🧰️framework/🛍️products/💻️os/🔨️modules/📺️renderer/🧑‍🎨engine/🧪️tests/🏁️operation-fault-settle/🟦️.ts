/** 🏁️ Laws of the program route's settlement: two inputs sent back to back both reach a typed ledger outcome, even when the second one's operation faults after the call that dispatched it returned. */
import { describe, expect, test } from "bun:test";
import { SemioFaultError } from "@semio-tech/framework";
import { createInputLedgerV1 } from "../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import { dispatchProgramAction } from "../../🧱️elements/🏛️ShellHost/🖱️shell-actions/🧭️input-funnel/🚀️program-dispatch/🟦️.ts";

const stale = new SemioFaultError({ origin: "framework", code: "interactive-job.stale", severity: "error", message: "stale", scope: {}, retryable: true });

function harness() {
  const ledger = createInputLedgerV1();
  const settlers = new Map<string, (completion: { readonly operation: bigint; readonly revision: bigint; readonly fault?: SemioFaultError }) => void>();
  const notices: string[] = [];
  const plugin = {
    handleCommand: async (_instance: number, invocation: string) => ({ output: { operationId: JSON.parse(invocation).arguments.operation, generation: "1" }, historyPatch: undefined, requestedEffects: [], uiScope: undefined }),
    handleAction: async () => { throw new Error("commands only"); },
  };
  const dispatch = (operation: string) => {
    const action = { controllerId: "c", action: "setFilterModule", commandAddress: { owner: { app: { pluginId: "sourcing", appId: "editor" } }, commandId: "setFilterModule" }, args: { operation } } as const;
    const entry = ledger.issue(action, { windowId: null, origin: "user" });
    const outcomeOf = (outcome: Parameters<typeof ledger.settle>[0]) => { ledger.settle(outcome); return outcome; };
    return dispatchProgramAction({
      sessionWorkRef: { current: { begin: () => () => {} } },
      isCurrentEffectOwner: () => true,
      applyHistoryPatch: () => {},
      applyLeftoverInteractionView: () => {},
      historyStore: { get: () => ({ entries: {} }) },
      refreshHistorySnapshot: () => {},
      applyHostEffects: async () => {},
      observeLocalInteraction: () => {},
      awaitOperationSettle: (_instance: number, id: bigint | undefined) => new Promise(resolve => settlers.set(String(id), resolve)),
      uiTerminologyRef: { current: undefined },
      uiLocale: "en",
      dropForSealedInstance: () => false,
      dropForRetiredInstance: () => false,
      targetSession: { pluginId: "sourcing", instanceId: 1, app: { id: "editor", windowKinds: [] }, viewState: {} },
      plugin,
      dispatchViewState: {},
      action,
      dispatchExtraInstances: [],
      dispatchWindowId: undefined,
      entry,
      actionOwner: {},
      lifecycle: undefined,
      onAction: async () => { throw new Error("no follow-up"); },
      actionOrigin: undefined,
      hostWindowId: undefined,
      core: { localSourceOwnerRef: { current: null }, showTransientNotice: (text: string) => notices.push(text), isViewerReadOnlyFault: () => false, isMutationRejectedFault: () => false },
      applied: () => outcomeOf({ kind: "applied", inputSeq: entry.provenance.inputSeq }),
      refuse: (reason: string, detail?: string) => outcomeOf({ kind: "refused", inputSeq: entry.provenance.inputSeq, reason, retryable: true, detail } as never),
      refusalReasonForError: (error: unknown) => ({ reason: "dispatch-failed", detail: String(error) }),
      propagateFailure: false,
    } as never);
  };
  return { dispatch, settlers, ledger, notices };
}

describe("program route settlement", () => {
  test("two back-to-back inputs both settle, the second with the fault its operation ended in", async () => {
    const { dispatch, settlers, ledger } = harness();
    const first = dispatch("320");
    const second = dispatch("321");
    await Promise.resolve();
    await new Promise(resolve => setTimeout(resolve, 0));
    settlers.get("320")!({ operation: 320n, revision: 5n });
    settlers.get("321")!({ operation: 321n, revision: 0n, fault: stale });
    const [one, two] = await Promise.all([first, second]);
    expect(one.kind).toBe("applied");
    expect(two).toMatchObject({ kind: "refused", reason: "dispatch-failed", detail: "interactive-job.stale" });
    expect([1, 2].map(seq => ledger.outcome(seq)?.kind)).toEqual(["applied", "refused"]);
  });

  test("two back-to-back inputs that both complete settle both as applied", async () => {
    const { dispatch, settlers } = harness();
    const [first, second] = [dispatch("1"), dispatch("2")];
    await new Promise(resolve => setTimeout(resolve, 0));
    settlers.get("2")!({ operation: 2n, revision: 7n });
    settlers.get("1")!({ operation: 1n, revision: 6n });
    expect((await Promise.all([first, second])).map(outcome => outcome.kind)).toEqual(["applied", "applied"]);
  });
});
