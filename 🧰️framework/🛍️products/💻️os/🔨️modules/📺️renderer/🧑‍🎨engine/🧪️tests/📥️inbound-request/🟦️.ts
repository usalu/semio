/** 📥️ The HOST half of the ABI's one call-into-an-actor seam: `Event::Request` out, `respond`
 * effect back, over the SAME language-neutral fixture the guest law
 * (`⚛️reactor/🔄️turn/🧪️tests/📥️inbound-request/🦀️.rs`) drives through the real reactor turn, with
 * strict Ajv as the independent oracle for the fixture's own shape.
 *
 * 🧨️ Regression this pins: `PluginWasmHandle` had no `invoke` at all, so `runCapturedExtensionEffect`
 * refused every evaluation with `extension.invoke-unavailable` before a guest was ever entered —
 * eight boots of `meshes: 0` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). The rows of
 * `🔁️extension-invocation-wire` are therefore driven END TO END here: through the real door, onto a
 * guest emulation that answers the same way `evaluate_invoke_json` does, and back out as the
 * completion `captureExtensionCompletion` submits. */

import Ajv from "ajv";
import { describe, expect, it, vi } from "vitest";
import { decodeFaultFromWire, decodePackValue, encodePackValue } from "@semio-tech/framework-os";
// 🪪️ From `@semio-tech/framework`, NOT `@semio-tech/framework-os`: `runCapturedExtensionEffect`'s
// `error instanceof SemioFaultError` branch is typed against that module instance, and a door double
// that throws the other package's class degrades to a generic `extension.invoke-failed`.
import { SemioFaultError } from "@semio-tech/framework";
import { GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, GUEST_HOST_ANSWER_CEILING_BYTES } from "../../../../../../../🔨️modules/⏱️trace/🧮️memory/🟦️.ts";
import { driveInboundRequest, INBOUND_REQUEST_TURN_BUDGET, wireEffectToFriendly, wireRespondAnswer, type WireVariant } from "../../../../../../../🔨️modules/🎭️actor/🖼️wire-turn/🟦️.ts";
import { EXTENSION_WORKER_LOST_FAULT, runInvokeExtensionEffect } from "../../🧱️elements/🏛️ShellHost/🟦️.tsx";
import { abortExtensionRequestsForActor, declareSurfaceCancelAction, inFlightExtensionRequestCount, isDeclaredSurfaceCancelAction } from "../../🧱️elements/🛠️ShellHelpers/🟦️.tsx";
import type { LoadedProgramState } from "../../🧱️elements/🐚️Shell/🟦️.tsx";
import reactorSchema from "../../../../🔌️plugin/⚛️reactor/🧬️schema/🔣️.json";
import inboundRequestFixture from "../../../../🔌️plugin/⚛️reactor/🧫️fixtures/📥️inbound-request/🔣️.json";
import extensionInvocationWireFixture from "../../../../🌊️flow/🧩️extensions/🕸️wasm/🧫️fixtures/🔁️extension-invocation-wire/🔣️.json";

type SubmittedEvent = { readonly kind: string; readonly payload: unknown };
type RequestEvent = { readonly req: bigint; readonly params: { readonly origin: WireVariant; readonly capability: string; readonly payload: readonly number[] } };

/** 🧩️ The guest the fixture describes: one installed capability that echoes a decodable request and
 * refuses everything else by the same typed codes `extension_invoke` raises. Emits the `respond`
 * effect exactly as `⚛️reactor/🦀️.rs` lowers it — `{tag, val:{req, outcome:{tag, val}}}`. */
function emulateGuest(options: { readonly capability: string | null; readonly turnsBeforeAnswer?: number }) {
  let pending: { readonly req: bigint; readonly capability: string; readonly request: string } | null = null;
  let delay = options.turnsBeforeAnswer ?? 0;
  const fault = (code: string, message: string): WireVariant =>
    ({ tag: "fault", val: Array.from(encodePackValue({ origin: "plugin", code, severity: "error", message, scope: {}, retryable: false })) });
  return async (events: readonly SubmittedEvent[]): Promise<{ readonly effects: readonly WireVariant[]; readonly status?: unknown }> => {
    for (const event of events) {
      if (event.kind !== "request") continue;
      const value = event.payload as RequestEvent;
      pending = { req: value.req, capability: value.params.capability, request: new TextDecoder().decode(Uint8Array.from(value.params.payload)) };
    }
    if (!pending) return { effects: [], status: { tag: "idle" } };
    if (delay > 0) {
      delay -= 1;
      return { effects: [], status: { tag: "more-work" } };
    }
    const request = pending;
    pending = null;
    let outcome: WireVariant;
    if (options.capability === null) outcome = fault("extension.inactive", "extension not activated");
    else if (request.capability !== options.capability) outcome = fault("extension.unknown-capability", `unknown extension capability '${request.capability}'`);
    else {
      try {
        outcome = { tag: "ok", val: Array.from(new TextEncoder().encode(JSON.stringify({ echo: JSON.parse(request.request) }))) };
      } catch (error) {
        outcome = fault("extension.evaluate.bad-request", error instanceof Error ? error.message : String(error));
      }
    }
    return { effects: [{ tag: "respond", val: { req: request.req, outcome } }], status: { tag: "idle" } };
  };
}

describe("inbound request seam", () => {
  it("accepts the shared fixture under a strict independent schema oracle", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).addSchema(reactorSchema).compile({ $ref: `${reactorSchema.$id}#/definitions/InboundRequestFixture` });
    expect(validate(inboundRequestFixture), JSON.stringify(validate.errors)).toBe(true);
  });

  it("declares the same budgets and arms the host enforces", () => {
    expect(inboundRequestFixture.seam.turnBudget).toBe(INBOUND_REQUEST_TURN_BUDGET);
    expect(inboundRequestFixture.seam.contiguousRequestCeilingBytes).toBe(GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);
    expect(inboundRequestFixture.seam.hostAnswerCeilingBytes).toBe(GUEST_HOST_ANSWER_CEILING_BYTES);
  });

  it.each(inboundRequestFixture.rows)("answers the $name row on the turn it arrives", async (row) => {
    const submit = vi.fn(emulateGuest({ capability: row.bundle ? inboundRequestFixture.capability : null }));
    const answer = await driveInboundRequest({ req: 7n, capability: row.capability, payload: new TextEncoder().encode(row.requestText), originInstanceId: 3, submit });
    expect(answer.status).toBe("answered");
    if (answer.status !== "answered") return;
    expect(answer.turns).toBe(inboundRequestFixture.seam.answeredOnTurn);
    expect(submit).toHaveBeenCalledOnce();
    const [events] = submit.mock.calls[0] as unknown as [readonly SubmittedEvent[]];
    expect(events).toHaveLength(1);
    expect(events[0]!.kind).toBe(inboundRequestFixture.seam.eventKind);
    const sent = events[0]!.payload as RequestEvent;
    expect(Object.keys(sent).sort()).toEqual([...inboundRequestFixture.seam.eventFields].sort());
    expect(Object.keys(sent.params).sort()).toEqual([...inboundRequestFixture.seam.paramFields].sort());
    expect(sent.params.origin).toEqual({ tag: inboundRequestFixture.seam.originTag, val: 3 });
    if (row.outcome === "ok") {
      expect("ok" in answer.result).toBe(true);
      if (!("ok" in answer.result)) return;
      expect(JSON.parse(new TextDecoder().decode(answer.result.ok))).toEqual(JSON.parse(row.answerText!));
    } else {
      expect("fault" in answer.result).toBe(true);
      if (!("fault" in answer.result)) return;
      expect(decodeFaultFromWire(Array.from(answer.result.fault), decodePackValue)?.code).toBe(row.faultCode);
    }
  });

  it("drains empty turns while the guest still reports more-work, and reports progress per turn", async () => {
    const progress: number[] = [];
    const answer = await driveInboundRequest({
      req: 9n,
      capability: inboundRequestFixture.capability,
      payload: new TextEncoder().encode("{}"),
      originInstanceId: 0,
      submit: emulateGuest({ capability: inboundRequestFixture.capability, turnsBeforeAnswer: 3 }),
      onProgress: ({ turns, budget }) => { progress.push(turns); expect(budget).toBe(INBOUND_REQUEST_TURN_BUDGET); },
    });
    expect(answer.status).toBe("answered");
    expect(answer.turns).toBe(4);
    expect(progress).toEqual([1, 2, 3]);
  });

  it("stops at the caller's cancellation instead of draining the whole budget", async () => {
    const controller = new AbortController();
    const submit = vi.fn(emulateGuest({ capability: inboundRequestFixture.capability, turnsBeforeAnswer: INBOUND_REQUEST_TURN_BUDGET }));
    const answer = await driveInboundRequest({
      req: 11n,
      capability: inboundRequestFixture.capability,
      payload: new TextEncoder().encode("{}"),
      originInstanceId: 0,
      submit,
      signal: controller.signal,
      onProgress: ({ turns }) => { if (turns === 2) controller.abort(); },
    });
    expect(answer).toEqual({ status: "cancelled", turns: 3 });
    expect(submit).toHaveBeenCalledTimes(3);
  });

  it("gives up on a guest that goes quiet without answering", async () => {
    const answer = await driveInboundRequest({
      req: 13n,
      capability: inboundRequestFixture.capability,
      payload: new TextEncoder().encode("{}"),
      originInstanceId: 0,
      submit: async () => ({ effects: [], status: { tag: "idle" } }),
    });
    expect(answer).toEqual({ status: "unanswered", turns: 1 });
  });

  it("ignores a respond that answers someone else's request", async () => {
    const answer = await driveInboundRequest({
      req: 17n,
      capability: inboundRequestFixture.capability,
      payload: new TextEncoder().encode("{}"),
      originInstanceId: 0,
      submit: async () => ({ effects: [{ tag: "respond", val: { req: 18n, outcome: { tag: "ok", val: [1, 2, 3] } } }], status: { tag: "idle" } }),
    });
    expect(answer).toEqual({ status: "unanswered", turns: 1 });
  });

  it("maps the respond effect onto the friendly union by both arms", () => {
    const ok = wireEffectToFriendly({ tag: "respond", val: { req: 4n, outcome: { tag: "ok", val: [7] } } }, decodePackValue);
    expect(ok).toEqual({ respond: { req: 4n, result: { ok: Uint8Array.from([7]) } } });
    const faulted = wireRespondAnswer({ tag: "respond", val: { req: 5n, outcome: { tag: "fault", val: [8] } } });
    expect(faulted).toEqual({ respond: { req: 5n, result: { fault: Uint8Array.from([8]) } } });
    expect(() => wireRespondAnswer({ tag: "respond", val: { req: 0n, outcome: { tag: "ok", val: [] } } })).toThrow("respond.request-id-invalid");
    expect(() => wireRespondAnswer({ tag: "respond", val: { req: 1n, outcome: { tag: "pending", val: [] } } })).toThrow("respond.outcome-invalid");
  });
});

describe("extension invocation across the door", () => {
  /** 🚪️ A real door: the handle's `invoke` is `driveInboundRequest` over the guest emulation, so this
   * exercises the SAME protocol the adapted `PluginWasmHandle` uses — not a stubbed `invoke`. */
  const doorEntry = (answerFor: (request: string) => { readonly ok: string } | { readonly faultCode: string }): LoadedProgramState => {
    let seq = 0n;
    const submit = async (events: readonly SubmittedEvent[]) => {
      const request = events.find((event) => event.kind === "request")!.payload as RequestEvent;
      const answer = answerFor(new TextDecoder().decode(Uint8Array.from(request.params.payload)));
      const outcome: WireVariant = "ok" in answer
        ? { tag: "ok", val: Array.from(new TextEncoder().encode(answer.ok)) }
        : { tag: "fault", val: Array.from(encodePackValue({ origin: "plugin", code: answer.faultCode, severity: "error", message: answer.faultCode, scope: {}, retryable: false })) };
      return { effects: [{ tag: "respond", val: { req: request.req, outcome } }], status: { tag: "idle" } };
    };
    const invoke = async (capability: string, payload: Uint8Array | string): Promise<Uint8Array> => {
      seq += 1n;
      const driven = await driveInboundRequest({ req: seq, capability, payload: typeof payload === "string" ? new TextEncoder().encode(payload) : payload, originInstanceId: 1, submit });
      if (driven.status !== "answered") throw new Error(`extension.request-${driven.status}`);
      if ("fault" in driven.result) throw new Error(decodeFaultFromWire(Array.from(driven.result.fault), decodePackValue)?.code ?? "extension.answer-not-a-fault");
      return driven.result.ok;
    };
    return { handle: { pluginId: "flow-extension-test", invoke }, manifest: {} } as unknown as LoadedProgramState;
  };

  it.each(extensionInvocationWireFixture.rows)("carries the $name evaluate row through the door to one completion", async (row) => {
    const answer = row.outcome === "ok"
      ? Object.fromEntries((row.outputKeys ?? []).map((key) => [key, row.outputEmpty ? {} : { value: 1 }]))
      : undefined;
    const complete = vi.fn(async () => ({}));
    const requester = {
      handle: {
        pluginId: "requester",
        captureExtensionCompletion: (instanceId: number, req: bigint) => ({ instanceId, req, assertActive: () => {}, complete }),
      },
      manifest: {},
    } as unknown as LoadedProgramState;
    const extension = doorEntry(() => (answer === undefined ? { faultCode: "extension.evaluate.bad-request" } : { ok: JSON.stringify(answer) }));
    await runInvokeExtensionEffect(requester, extension, 1, "flow-extension-test", extensionInvocationWireFixture.capability, row.requestJson, 21n);
    expect(complete).toHaveBeenCalledOnce();
    const [outcome] = complete.mock.calls[0] as unknown as [{ ok?: Uint8Array; fault?: Uint8Array }];
    expect("ok" in outcome ? "ok" : "fault").toBe(row.outcome);
    if (row.outcome === "ok") expect(decodePackValue(outcome.ok!)).toEqual(answer);
    else expect(decodeFaultFromWire(Array.from(outcome.fault!), decodePackValue)?.message).toBe("extension.evaluate.bad-request");
  });
});

describe("a surface cancel aborts the in-flight extension request", () => {
  /** 🛑️ A door over a guest that PARKS: it reports `more-work` forever and never responds, which is
   * the only shape an abort can be observed on — `driveInboundRequest` reads its signal at turn
   * boundaries, so a guest that answers on turn 1 is never cancellable by construction. */
  const parkingDoorEntry = (observed: { signal?: AbortSignal }): LoadedProgramState => {
    // ⏳️ A real macrotask per turn, so the drain SPANS the event loop the way a wasm worker turn
    // does. A synchronous `more-work` loop burns the whole turn budget inside one microtask flush
    // and no cancellation could ever be observed between two turns.
    const submit = async () => {
      await new Promise((resolve) => setTimeout(resolve, 1));
      return { effects: [] as readonly WireVariant[], status: { tag: "more-work" } };
    };
    const invoke = async (capability: string, payload: Uint8Array | string, context?: { readonly signal?: AbortSignal }): Promise<Uint8Array> => {
      observed.signal = context?.signal;
      const driven = await driveInboundRequest({
        req: 1n,
        capability,
        payload: typeof payload === "string" ? new TextEncoder().encode(payload) : payload,
        originInstanceId: 1,
        signal: context?.signal,
        submit,
      });
      if (driven.status === "cancelled") throw new SemioFaultError({ origin: "os", code: "extension.request-cancelled", severity: "error", message: `cancelled after ${driven.turns} turn(s)`, scope: {}, retryable: false });
      if (driven.status !== "answered") throw new Error(`extension.request-${driven.status}`);
      if ("fault" in driven.result) throw new Error("unexpected fault");
      return driven.result.ok;
    };
    return { handle: { pluginId: "flow-extension-brep", invoke }, manifest: {} } as unknown as LoadedProgramState;
  };

  it("hands the door a signal and retires the parked request when the requester's actor is aborted", async () => {
    const observed: { signal?: AbortSignal } = {};
    const complete = vi.fn(async () => ({}));
    const requester = {
      handle: {
        pluginId: "s.procedural",
        captureExtensionCompletion: (instanceId: number, req: bigint) => ({ instanceId, req, assertActive: () => {}, complete }),
      },
      manifest: {},
    } as unknown as LoadedProgramState;
    const actorKey = "s.procedural:1";
    expect(inFlightExtensionRequestCount(actorKey)).toBe(0);
    const settled = runInvokeExtensionEffect(requester, parkingDoorEntry(observed), 1, "flow-extension-brep", "tessellate", "{\"handle\":\"brep:solid-1\"}", 42n);
    // ⏳️ One macrotask so the request actually reaches the door before the cancel gesture lands.
    await new Promise((resolve) => setTimeout(resolve, 5));
    expect(observed.signal, "the shell must hand `invoke` an AbortSignal — it never did, so a cancel reached the guest's own bookkeeping and never the parked request").toBeInstanceOf(AbortSignal);
    expect(observed.signal?.aborted).toBe(false);
    expect(inFlightExtensionRequestCount(actorKey)).toBe(1);

    expect(abortExtensionRequestsForActor(actorKey, "cancelled by cancelPreviewEval")).toBe(1);
    expect(observed.signal?.aborted).toBe(true);

    await settled;
    expect(complete).toHaveBeenCalledOnce();
    const [outcome] = complete.mock.calls[0] as unknown as [{ ok?: Uint8Array; fault?: Uint8Array }];
    expect("fault" in outcome).toBe(true);
    expect(decodeFaultFromWire(Array.from(outcome.fault!), decodePackValue)?.code).toBe("extension.request-cancelled");
    expect(inFlightExtensionRequestCount(actorKey), "a settled request leaves no controller behind").toBe(0);
  });

  /** ⚖️ LAW: a callee whose WORKER was taken down is not an anonymous invocation failure. The host's
   * own watchdog killed it, `ShardClient.rebuild` has already replaced it, and re-issuing the same
   * request is the correct response — so the fault the requester receives is the typed, RETRYABLE
   * `extension.worker-lost`, which its surface can name. Calling it `extension.invoke-failed`
   * alongside a genuine guest refusal left the preview unable to distinguish "the geometry kernel
   * refused this" from "the geometry kernel's worker died"
   * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️extension-evaluate-budget-2026-09-12.md`). */
  it("answers a shard-lost invocation with the typed, retryable extension.worker-lost fault", async () => {
    const complete = vi.fn(async () => ({}));
    const requester = {
      handle: {
        pluginId: "s.procedural",
        captureExtensionCompletion: (instanceId: number, req: bigint) => ({ instanceId, req, assertActive: () => {}, complete }),
      },
      manifest: {},
    } as unknown as LoadedProgramState;
    // 🩺️ The exact message `ShardClient.terminate` raises — `isShardLostError` matches on that
    // prefix, so a test that invented its own wording would prove nothing.
    const shardLost = new Error("shard 0 terminated by the host watchdog: the worker was silent for 16271 ms; outstanding: turn flow-extension-brep#request started 16299 ms ago.");
    const dyingDoor = { handle: { pluginId: "flow-extension-brep", invoke: async () => { throw shardLost; } }, manifest: {} } as unknown as LoadedProgramState;
    await runInvokeExtensionEffect(requester, dyingDoor, 1, "flow-extension-brep", "evaluate", "{\"operatorId\":\"brep.bool.cut\",\"inputJson\":\"{}\"}", 9n);
    expect(complete).toHaveBeenCalledOnce();
    const [outcome] = complete.mock.calls[0] as unknown as [{ ok?: Uint8Array; fault?: Uint8Array }];
    expect("fault" in outcome).toBe(true);
    const fault = decodeFaultFromWire(Array.from(outcome.fault!), decodePackValue);
    expect(fault?.code).toBe(EXTENSION_WORKER_LOST_FAULT);
    expect(fault?.retryable, "a rebuilt shard means the identical request is worth re-issuing").toBe(true);
  });

  it("still answers a genuine guest refusal with extension.invoke-failed, not a worker loss", async () => {
    const complete = vi.fn(async () => ({}));
    const requester = {
      handle: {
        pluginId: "s.procedural",
        captureExtensionCompletion: (instanceId: number, req: bigint) => ({ instanceId, req, assertActive: () => {}, complete }),
      },
      manifest: {},
    } as unknown as LoadedProgramState;
    const refusingDoor = { handle: { pluginId: "flow-extension-brep", invoke: async () => { throw new Error("the guest could not decode that request"); } }, manifest: {} } as unknown as LoadedProgramState;
    await runInvokeExtensionEffect(requester, refusingDoor, 1, "flow-extension-brep", "evaluate", "{}", 10n);
    const [outcome] = complete.mock.calls[0] as unknown as [{ ok?: Uint8Array; fault?: Uint8Array }];
    const fault = decodeFaultFromWire(Array.from(outcome.fault!), decodePackValue);
    expect(fault?.code).toBe("extension.invoke-failed");
    expect(fault?.retryable).toBe(false);
  });

  it("offers the affordance only for an action a mounted surface declared", () => {
    expect(isDeclaredSurfaceCancelAction("cancelPreviewEval")).toBe(false);
    const retire = declareSurfaceCancelAction("cancelPreviewEval");
    expect(isDeclaredSurfaceCancelAction("cancelPreviewEval")).toBe(true);
    const retireSecondWindow = declareSurfaceCancelAction("cancelPreviewEval");
    retire();
    expect(isDeclaredSurfaceCancelAction("cancelPreviewEval"), "two preview windows may offer the same verb — the last one out clears it").toBe(true);
    retireSecondWindow();
    expect(isDeclaredSurfaceCancelAction("cancelPreviewEval")).toBe(false);
    expect(abortExtensionRequestsForActor("nobody:0", "no-op")).toBe(0);
  });
});
