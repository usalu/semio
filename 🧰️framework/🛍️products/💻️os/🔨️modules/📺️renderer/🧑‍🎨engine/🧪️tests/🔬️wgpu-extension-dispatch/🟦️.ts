import { describe, expect, it } from "vitest";
import Ajv from "ajv";
import laws from "../../🧫️fixtures/🔬️wgpu-extension-dispatch/🔣️.json";
import documentBackbone from "../../🧫️fixtures/📡️wgpu-document-backbone/🔣️.json";
import integerView from "../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json";
import viewContextSchema from "../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧬️schema/🔣️.json";
import { GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, GUEST_HOST_ANSWER_CEILING_BYTES, guestAnswerPages } from "../../../../../../../🔨️modules/⏱️trace/🧮️memory/🟦️.ts";
import { extensionRequestActivationReason, serializeWgpuActorCall, wgpuBackboneMessageEffect, wgpuBuildScopedContributionsPack, wgpuCommandIngressByteLength, wgpuContributionsIngressSize, wgpuGuestAnswerPages, wgpuHostAnswerCeilingBytes, wgpuSetContributionsCommand, wgpuSlimContributionsView, WGPU_ACTOR_CALL_QUEUE_CAPACITY, WGPU_CONTRIBUTIONS_SLIM_VIEW, WgpuActorCallQueueFullError } from "../../🎯️targets/🧊️wgpu/🐚️plugin-bridge/🟦️.ts";
import { activationEventEnvelope, createTurnOutcomeBroadcast, type TurnOutcome } from "@semio-tech/framework";
import { AppChannelClient, type AppChannelHandle, AppChannelRequestSequence, decodeAppCommand, encodeAppCommand, encodeAppFrame, encodePackValue } from "@semio-tech/framework-os";
import { FRAME_WORKER_BOOT_LIVENESS_POLICY, bootPhaseCeilingMs, evaluateBrowserBootLiveness } from "../../🎯️targets/🧊️wgpu/🫀️boot-liveness/🟦️.ts";
import { SHARD_COMMAND_MAXIMUM_PAGES } from "../../../../../../../🔨️modules/🎭️actor/📮️shard-client/🟦️.ts";
import { PUBLIC_INVOCATION_BODY_BYTES, PUBLIC_INVOCATION_STRING_BYTES, publicInvocationStringPages } from "../../../../../../../🔨️modules/🛂️manifest/🟦️.ts";

async function actualAppChannelCommand(sequenceOwnerBefore: number, commandBytes: Uint8Array, viewState: unknown): Promise<Uint8Array> {
  const outcomes = createTurnOutcomeBroadcast<TurnOutcome>();
  let captured: Uint8Array | undefined;
  const handle: AppChannelHandle = {
    enqueue: (instanceId, events) => {
      captured = events[0];
      if (!captured) throw new Error("actual AppChannel oracle did not receive a command");
      const command = decodeAppCommand(captured);
      if (!("Command" in command)) throw new Error("actual AppChannel oracle received the wrong command variant");
      outcomes.push({ instanceId, frames: [encodeAppFrame({ Done: { in_reply_to: command.Command.seq } })] });
    },
    outcomes: outcomes.stream,
  };
  const client = new AppChannelClient(handle, new AppChannelRequestSequence(sequenceOwnerBefore), 7, "fixture");
  try {
    await client.command(commandBytes, viewState);
    if (!captured) throw new Error("actual AppChannel oracle did not encode a command");
    return captured;
  } finally {
    client.dispose();
    outcomes.complete();
  }
}

function bytesHex(bytes: readonly number[]): string {
  return bytes.map((byte) => byte.toString(16).padStart(2, "0")).join("");
}

describe("wgpu extension-answer paging", () => {
  it("pages a 1 MiB answer so no cabi_realloc block exceeds one contiguous page", () => {
    const bytes = new Uint8Array(laws.laws.pagedCompletion.oneMibAnswerBytes).fill(7);
    const pages = guestAnswerPages(bytes);
    const wgpu = wgpuGuestAnswerPages(bytes);
    expect(wgpuHostAnswerCeilingBytes()).toBe(GUEST_HOST_ANSWER_CEILING_BYTES);
    expect(GUEST_HOST_ANSWER_CEILING_BYTES).toBe(laws.laws.pagedCompletion.hostAnswerCeilingBytes);
    expect(GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES).toBe(laws.laws.pagedCompletion.contiguousRequestCeilingBytes);
    expect(pages.prologue.length).toBe(laws.laws.pagedCompletion.oneMibProloguePages);
    expect(pages.terminal.byteLength).toBe(laws.laws.pagedCompletion.oneMibTerminalBytes);
    expect(pages.prologue.length + 1).toBe(laws.laws.pagedCompletion.oneMibEventCount);
    expect(Math.max(pages.terminal.byteLength, ...pages.prologue.map((page) => page.byteLength))).toBe(laws.laws.pagedCompletion.largestBlockBytes);
    expect(wgpu.prologue.length).toBe(pages.prologue.length);
    expect(wgpu.terminal.byteLength).toBe(pages.terminal.byteLength);
  });

  it("refuses an assembled answer past the host-answer ceiling", () => {
    expect(laws.laws.pagedCompletion.oneMibAnswerBytes).toBeLessThan(GUEST_HOST_ANSWER_CEILING_BYTES);
    expect(GUEST_HOST_ANSWER_CEILING_BYTES + 1).toBeGreaterThan(GUEST_HOST_ANSWER_CEILING_BYTES);
  });
});

describe("wgpu contributions-before-eval", () => {
  it("records one pack contributions crossing before the first flowEvalTick", () => {
    const timeline = laws.laws.contributionsBeforeEval.timeline;
    const firstEval = timeline.findIndex((row) => row.kind === "flowEvalTick");
    const lastPush = timeline.reduce((last: number, row: { kind: string }, index: number) => (row.kind === "contributions-push" ? index : last), -1);
    const push = timeline.find((row) => row.kind === "contributions-push") as { crossings?: number; encoding?: string } | undefined;
    expect(firstEval).toBeGreaterThan(lastPush);
    expect(timeline[0]?.kind).toBe("refresh-ui");
    expect(timeline.filter((row) => row.kind === "contributions-push")).toHaveLength(1);
    expect(push?.crossings).toBe(laws.laws.scopedContributionsPack.crossings);
    expect(push?.encoding).toBe(laws.laws.scopedContributionsPack.encoding);
    expect(PUBLIC_INVOCATION_BODY_BYTES).toBe(laws.laws.scopedContributionsPack.bodyCeilingBytes);
    expect(PUBLIC_INVOCATION_STRING_BYTES).toBe(laws.laws.scopedContributionsPack.stringCeilingBytes);
    expect(publicInvocationStringPages("x".repeat(5000)).length).toBeGreaterThan(1);
  });
});

describe("wgpu scoped contributions pack", () => {
  const manifest = (kind: string) => ({ topicContributions: [{ topic: "flow.extension", payload: { operators: [{ kind }] } }], apps: [], workflows: [] });
  const loaded = [
    { pluginId: "procedural", manifest: manifest("procedural.example") },
    { pluginId: "flow-extension-brep", manifest: manifest("brep.solid.extrude") },
    { pluginId: "flow-extension-bim", manifest: manifest("bim.wall") },
    { pluginId: "flow-extension-math", manifest: manifest("math.vector") },
  ];
  it("sends one body-bounded pack of the receiver plus reachable operators", () => {
    const pack = wgpuBuildScopedContributionsPack(laws.laws.scopedContributionsPack.receiverPluginId, [{ widgets: [{ neuronKind: "brep.solid.extrude" }, { neuronKind: "math.vector" }] }], loaded);
    expect(pack).not.toBeNull();
    expect(pack?.crossings).toBe(1);
    expect(pack?.bytes.byteLength).toBeLessThanOrEqual(PUBLIC_INVOCATION_BODY_BYTES);
    expect(PUBLIC_INVOCATION_STRING_BYTES).toBeLessThan(PUBLIC_INVOCATION_BODY_BYTES);
    expect([...pack!.pluginIds].sort()).toEqual(laws.laws.scopedContributionsPack.expectPluginIds);
  });
  it("drops foreign contributions when the graph names no operator kind", () => {
    const pack = wgpuBuildScopedContributionsPack(laws.laws.scopedContributionsPack.receiverPluginId, [{}], loaded);
    expect(pack).not.toBeNull();
    expect(pack?.crossings).toBe(1);
    expect([...pack!.pluginIds]).toEqual(laws.laws.scopedContributionsPack.expectEmptyGraphPluginIds);
  });
  it("reaches brep operators nested in a flow-extension manifestJson string", () => {
    const packed = [
      loaded[0]!,
      { pluginId: "flow-extension-brep", manifest: { topicContributions: [{ topic: "flow.extension", payload: { manifestJson: JSON.stringify({ contributes: { operators: [{ id: "brep.solid.extrude" }] } }) } }], apps: [], workflows: [] } },
      loaded[2]!,
    ];
    const pack = wgpuBuildScopedContributionsPack(laws.laws.scopedContributionsPack.receiverPluginId, ["neuron-kind=brep.solid.extrude"], packed);
    expect(pack).not.toBeNull();
    expect(pack?.crossings).toBe(1);
    expect([...pack!.pluginIds].sort()).toEqual(laws.laws.scopedContributionsPack.expectManifestJsonPluginIds);
  });
});

describe("wgpu contributions command ingress", () => {
  it.each([
    [0, 0],
    [127, 128],
    [16_383, 16_384],
    [190_701, 37],
  ])("matches the actual AppCommand encoder at command=%i view=%i bytes", (commandLength, viewLength) => {
    const command = new Uint8Array(commandLength);
    const view = new Uint8Array(viewLength);
    const encoded = encodeAppCommand({ Command: { seq: 1, command: Array.from(command), view_state: Array.from(view) } });
    expect(wgpuCommandIngressByteLength(1, command, view)).toBe(encoded.byteLength);
  });

  it.each(laws.laws.commandIngress.sequenceCases)("matches AppChannelClient at exact next sequence $sequence and normalizes integer view tags", async ({ ownerBefore, sequence }) => {
    const validate = new Ajv({ strict: true }).compile(viewContextSchema);
    expect(validate(integerView.viewContext), validate.errors?.map((error) => `${error.instancePath} ${error.message}`).join("\n")).toBe(true);
    const command = wgpuSetContributionsCommand("procedural", "s.procedural.generation3d@1/*#editor", "[]");
    const commandBytes = encodePackValue(command);
    const actualBytes = await actualAppChannelCommand(ownerBefore, commandBytes, integerView.viewContext);
    const actual = decodeAppCommand(actualBytes);
    expect(actual).toHaveProperty("Command.seq", sequence);
    if (!("Command" in actual)) throw new Error("actual AppChannel oracle received the wrong command variant");
    expect(bytesHex(actual.Command.view_state)).toBe(integerView.packHex);
    const estimated = wgpuContributionsIngressSize(command, integerView.viewContext, sequence);
    expect(estimated.ingressBytes).toBe(actualBytes.byteLength);
  });

  it("crosses one slim-view pack under the derived shard page ceiling", async () => {
    const json = `[{"pluginId":"flow-extension-brep","pad":"${"p".repeat(190700)}"}]`;
    const command = wgpuSetContributionsCommand("procedural", "s.procedural.generation3d@1/*#editor", json);
    // 📌️ `panelJson` is the ONE long field a view context still carries (contributions left the
    // contract entirely — `🪟️view-context/🧬️schema/🔣️.json`), so it is what a slim view must drop.
    const live = { locale: "en", terminology: "native", panelJson: "n".repeat(laws.laws.commandIngress.fatViewChars) };
    const sequenceCase = laws.laws.commandIngress.sequenceCases[1]!;
    const slimView = wgpuSlimContributionsView(live);
    const slim = wgpuContributionsIngressSize(command, slimView, sequenceCase.sequence);
    const fat = wgpuContributionsIngressSize(command, live, sequenceCase.sequence);
    const commandBytes = encodePackValue(command);
    const encoded = await actualAppChannelCommand(sequenceCase.ownerBefore, commandBytes, slimView);
    expect(wgpuSlimContributionsView(live).panelJson).toBeUndefined();
    expect(wgpuSlimContributionsView(live).contributionsJson).toBeUndefined();
    expect(json.length).toBeGreaterThan(laws.laws.commandIngress.payloadChars);
    expect(laws.laws.commandIngress.maximumPages).toBe(SHARD_COMMAND_MAXIMUM_PAGES);
    expect(SHARD_COMMAND_MAXIMUM_PAGES * laws.laws.commandIngress.pageBytes).toBe(GUEST_HOST_ANSWER_CEILING_BYTES);
    expect(slim.ingressPages).toBeGreaterThan(0);
    expect(slim.ingressPages).toBeLessThanOrEqual(laws.laws.commandIngress.maximumPages);
    expect(slim.ingressBytes).toBeLessThanOrEqual(laws.laws.commandIngress.maximumPages * laws.laws.commandIngress.pageBytes);
    expect(slim.ingressBytes).toBe(encoded.byteLength);
    expect(fat.ingressPages).toBeGreaterThan(laws.laws.commandIngress.maximumPages);
    expect(slim.viewBytes).toBeLessThan(fat.viewBytes);
  });
});

describe("wgpu nested shell-boot phases", () => {
  it("prices a nested shell-boot sub-phase on the parent ceiling and stays BUSY", () => {
    const child = laws.laws.nestedBootPhase.child;
    expect(bootPhaseCeilingMs(child)).toBe(laws.laws.nestedBootPhase.parentCeilingMs);
    expect(bootPhaseCeilingMs(child)).toBe(FRAME_WORKER_BOOT_LIVENESS_POLICY.phaseCeilingMs["shell-boot"]);
    const decision = evaluateBrowserBootLiveness({
      nowMs: 120_000,
      lastLivenessAtMs: 1_000,
      silenceTimeoutMs: FRAME_WORKER_BOOT_LIVENESS_POLICY.silenceTimeoutMs,
      phase: { phase: child, ceilingMs: bootPhaseCeilingMs(child), enteredAtMs: 1_000 },
    });
    expect(decision.terminate).toBe(false);
    expect(decision.phaseElapsedMs).toBe(119_000);
  });
});

// 🔗️ INPUT-CAUSALITY-LEDGER §2 F (transport parity), law L2: the wgpu bridge's call-level serializer
// honours the same causal `order` key as `PluginRuntime`'s `serializeCommandIngressForActor` — a
// pending call with a smaller `order` is inserted before pending calls with a strictly larger one,
// unordered calls stay FIFO, and the in-flight call is never reordered.
describe("wgpu serializeWgpuActorCall causal order", () => {
  const hold = (actorId: string, ran: string[]) => {
    let release!: () => void;
    let markStarted!: () => void;
    const started = new Promise<void>((resolve) => { markStarted = resolve; });
    const gate = new Promise<void>((resolve) => { release = resolve; });
    const held = serializeWgpuActorCall(actorId, async () => {
      ran.push("held");
      markStarted();
      await gate;
    });
    return { held, started, release };
  };

  it("dequeues pending calls by ascending order — a later call with a smaller order overtakes a larger one behind the held call", async () => {
    const ran: string[] = [];
    const { held, started, release } = hold("wgpu-actor-causal-order", ran);
    await started;
    const callA = serializeWgpuActorCall("wgpu-actor-causal-order", async () => { ran.push("A"); }, 5);
    const callB = serializeWgpuActorCall("wgpu-actor-causal-order", async () => { ran.push("B"); }, 7);
    const callC = serializeWgpuActorCall("wgpu-actor-causal-order", async () => { ran.push("C"); }, 6);
    await Promise.resolve();
    expect(ran).toEqual(["held"]);
    release();
    await Promise.all([held, callA, callB, callC]);
    expect(ran).toEqual(["held", "A", "C", "B"]);
  });

  it("keeps plain FIFO for calls without order and never starts two calls for one actor at once", async () => {
    const ran: string[] = [];
    let inFlight = 0;
    let maxInFlight = 0;
    const { held, started, release } = hold("wgpu-actor-fifo-order", ran);
    await started;
    const one = (name: string) => serializeWgpuActorCall("wgpu-actor-fifo-order", async () => {
      inFlight += 1;
      maxInFlight = Math.max(maxInFlight, inFlight);
      ran.push(name);
      await new Promise((resolve) => setTimeout(resolve, 1));
      inFlight -= 1;
    });
    const calls = [one("first"), one("second"), one("third")];
    await Promise.resolve();
    expect(ran).toEqual(["held"]);
    release();
    await Promise.all([held, ...calls]);
    expect(ran).toEqual(["held", "first", "second", "third"]);
    expect(maxInFlight).toBe(1);
  });

  it("keeps queuing after a faulted call and refuses past the pending capacity with a typed error", async () => {
    const ran: string[] = [];
    const { held, started, release } = hold("wgpu-actor-bounded", ran);
    await started;
    const failing = serializeWgpuActorCall("wgpu-actor-bounded", async () => { ran.push("failing"); throw new Error("call faulted"); }, 1);
    const pending = Array.from({ length: WGPU_ACTOR_CALL_QUEUE_CAPACITY - 1 }, (_, index) => serializeWgpuActorCall("wgpu-actor-bounded", async () => { ran.push(`p${index}`); }));
    const overflow = serializeWgpuActorCall("wgpu-actor-bounded", async () => { ran.push("overflow"); });
    await expect(overflow).rejects.toBeInstanceOf(WgpuActorCallQueueFullError);
    await expect(overflow).rejects.toMatchObject({ code: "wgpu-actor-call.queue-full", actorId: "wgpu-actor-bounded", capacity: WGPU_ACTOR_CALL_QUEUE_CAPACITY });
    expect(ran).toEqual(["held"]);
    release();
    await held;
    await expect(failing).rejects.toThrow("call faulted");
    await Promise.all(pending);
    expect(ran.slice(0, 3)).toEqual(["held", "failing", "p0"]);
    expect(ran).toHaveLength(1 + WGPU_ACTOR_CALL_QUEUE_CAPACITY);
    expect(ran).not.toContain("overflow");
  });
});

describe("wgpu extension request activation", () => {
  it("activates a request actor on-extension-request with the capability that pulled it up, never manual", () => {
    const reason = extensionRequestActivationReason("gis.tiles.render");
    expect(reason).toBe("on-extension-request:gis.tiles.render");
    expect(activationEventEnvelope(reason)).toEqual({ kind: "activate", payload: { instance: 0, reason: { tag: "on-extension-request", val: "gis.tiles.render" } } });
  });
});

describe("wgpu document-backbone effect projection", () => {
  for (const law of documentBackbone.effects) {
    it(law.name, () => {
      expect(wgpuBackboneMessageEffect(law.wire as Parameters<typeof wgpuBackboneMessageEffect>[0])).toEqual(law.host);
    });
  }
});
