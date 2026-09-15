/** 🕹️ The wgpu host's half of the SELECTION round trip: a framework-reserved interaction verb answers
 * on a frame that answers no command sequence, and this target must still fold it.
 *
 * ⚖️ The defect pinned here, measured on `http://127.0.0.1:6118/?plugin=generation3d` across 7 of 8
 * generation3d examples in both roles: the pick resolved the right target, the reserved job settled
 * `outcome=ok`, the bridge settled with frames — and the shell traced
 * `wgpu-shell dispatch action=interactionSelect scope=none` / `refresh scope=none rendered=0`, so no
 * surface re-rendered and the guest's `World3dScene.selectionJson` stayed `[]` for the whole run.
 * The job's settled `InvocationResult` is published as `AppFrame::Invocation { in_reply_to: 0 }`
 * (`plugin_complete_reserved_spawned_job`), which `AppChannelClient` can correlate to no waiter; on
 * the frame lane it fell into the `onOperationProgress` lane this target subscribes to nowhere.
 *
 * Oracle: `🎭️actor/🧫️fixtures/🕹️reserved-verb-answer/🔣️.json`, the same file the lane rule in
 * `🖼️wire-turn/🟦️.ts` (`leftoverShellInvocationFrames`) is written for.
 * Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-selection-roundtrip-2026-09-15.md`. */
import { describe, expect, it } from "vitest";
import fixture from "../../../../../../../🔨️modules/🎭️actor/🧫️fixtures/🕹️reserved-verb-answer/🔣️.json";
import { encodeAppFrame, encodePackValue } from "../../../../../🟦️.ts";
import type { WireVariant } from "../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { shellFrameAnswersACaller, wgpuInvocationFromFrames } from "../../🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts";

type InvocationLike = { readonly replySequence: number; readonly output: unknown; readonly uiScope: unknown };

const invocationFrame = (published: InvocationLike) => ({
  Invocation: {
    in_reply_to: published.replySequence,
    output: published.output === null ? [] : [...encodePackValue(published.output)],
    diagnostics: [],
    ui_scope: [...encodePackValue(published.uiScope)],
    history_patch: [],
    messages: [],
    mutations: [],
    inverse_group: [],
  },
});

const shellMessage = (payload: Uint8Array): WireVariant => ({ tag: "send-message", val: { target: { tag: "shell", val: fixture.instanceId }, payload: [...payload] } });

const selectedIdsOf = (output: unknown): readonly string[] => {
  const view = (output as { readonly interactionView?: { readonly selectedIds?: unknown } } | null)?.interactionView;
  return Array.isArray(view?.selectedIds) ? (view.selectedIds as string[]) : [];
};

describe("🕹️ wgpu selection round trip: a reserved verb's answer answers no command sequence", () => {
  it("routes the admission onto the frame lane and the settled answer onto the leftover lane", () => {
    const admission = encodeAppFrame(invocationFrame(fixture.admission) as never);
    const settled = encodeAppFrame(invocationFrame(fixture.settled) as never);
    expect(fixture.admission.replySequence).toBe(fixture.commandSequence);
    expect(fixture.settled.replySequence).toBe(fixture.uncorrelatedReplySequence);
    expect(shellFrameAnswersACaller(admission)).toBe(fixture.expect.admissionAnswersACaller);
    expect(shellFrameAnswersACaller(settled)).toBe(fixture.expect.settledAnswersACaller);
  });

  it("folds the settled answer's interaction view and refresh scope out of the leftover lane", () => {
    const admission = encodeAppFrame(invocationFrame(fixture.admission) as never);
    const settled = encodeAppFrame(invocationFrame(fixture.settled) as never);
    const response = wgpuInvocationFromFrames([invocationFrame(fixture.admission) as never], [shellMessage(settled)]);
    expect(selectedIdsOf(response.output)).toEqual(fixture.expect.foldedSelectedIds);
    expect((response.uiScope as { kind?: string } | undefined)?.kind).toBe(fixture.expect.foldedUiScopeKind);
    expect((response.uiScope as { windowBodies?: readonly string[] } | undefined)?.windowBodies).toEqual(fixture.expect.foldedWindowBodies);
    expect(shellFrameAnswersACaller(admission)).toBe(true);
  });

  it("is discriminating: the frame lane alone answers the admission's empty scope, which refreshes nothing", () => {
    const response = wgpuInvocationFromFrames([invocationFrame(fixture.admission) as never], []);
    expect(selectedIdsOf(response.output)).toEqual(fixture.expect.frameLaneOnlySelectedIds);
    expect((response.uiScope as { kind?: string } | undefined)?.kind).toBe(fixture.expect.frameLaneOnlyUiScopeKind);
  });

  it("keeps an admission that carried its own answer when a completion frame carries none", () => {
    const carrying = { replySequence: fixture.commandSequence, output: { interactionView: { selectedIds: ["extrude@solid"] } }, uiScope: { kind: "full" } };
    const silent = encodeAppFrame(invocationFrame({ replySequence: fixture.uncorrelatedReplySequence, output: null, uiScope: { kind: "none" } }) as never);
    const response = wgpuInvocationFromFrames([invocationFrame(carrying) as never], [shellMessage(silent)]);
    expect(selectedIdsOf(response.output)).toEqual(["extrude@solid"]);
    expect((response.uiScope as { kind?: string } | undefined)?.kind).toBe("none");
  });
});
