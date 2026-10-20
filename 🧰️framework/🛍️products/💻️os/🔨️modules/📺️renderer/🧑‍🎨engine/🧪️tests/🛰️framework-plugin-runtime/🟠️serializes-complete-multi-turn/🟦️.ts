/** 🟠️ The serializes complete multi turn tests of the framework plugin runtime suite. */
import { expect, it } from "vitest";
import { adaptPluginHandle, serializeCommandIngressForActor } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { exchangeStyleChannel } from "../🟣️shared/🟦️.ts";

/** 🟡 Registers the serializes complete multi turn tests of the framework plugin runtime suite. */
export function registerSerializesCompleteMultiTurnTests(): void {
  it("serializes complete multi-turn command ingress sequences for one actor", async () => {
    const order: string[] = [];
    let releaseFirst!: () => void;
    let markFirstStarted!: () => void;
    const firstStarted = new Promise<void>((resolve) => {
      markFirstStarted = resolve;
    });
    const firstGate = new Promise<void>((resolve) => {
      releaseFirst = resolve;
    });
    const first = serializeCommandIngressForActor("actor-ingress", async () => {
      order.push("first-page");
      markFirstStarted();
      await firstGate;
      order.push("first-terminal");
    });
    await firstStarted;
    const second = serializeCommandIngressForActor("actor-ingress", async () => {
      order.push("second-page");
      await Promise.resolve();
      order.push("second-terminal");
    });
    await Promise.resolve();
    expect(order).toEqual(["first-page"]);
    releaseFirst();
    await Promise.all([first, second]);
    expect(order).toEqual(["first-page", "first-terminal", "second-page", "second-terminal"]);
  });
  it("serializeCommandIngressForActor dequeues queued calls by ascending order — a later call with a smaller order overtakes a larger one behind the held call", async () => {
    const ran: string[] = [];
    let releaseHeld!: () => void;
    let markHeldStarted!: () => void;
    const heldStarted = new Promise<void>((resolve) => {
      markHeldStarted = resolve;
    });
    const heldGate = new Promise<void>((resolve) => {
      releaseHeld = resolve;
    });
    const held = serializeCommandIngressForActor("actor-causal-order", async () => {
      ran.push("held");
      markHeldStarted();
      await heldGate;
    });
    await heldStarted;
    const callA = serializeCommandIngressForActor(
      "actor-causal-order",
      async () => {
        ran.push("A");
      },
      "Interactive",
      5,
    );
    const callB = serializeCommandIngressForActor(
      "actor-causal-order",
      async () => {
        ran.push("B");
      },
      "Interactive",
      7,
    );
    const callC = serializeCommandIngressForActor(
      "actor-causal-order",
      async () => {
        ran.push("C");
      },
      "Interactive",
      6,
    );
    await Promise.resolve();
    expect(ran).toEqual(["held"]);
    releaseHeld();
    await Promise.all([held, callA, callB, callC]);
    expect(ran).toEqual(["held", "A", "C", "B"]);
  });
  it("serializeCommandIngressForActor keeps plain FIFO for calls without order", async () => {
    const ran: string[] = [];
    let releaseHeld!: () => void;
    let markHeldStarted!: () => void;
    const heldStarted = new Promise<void>((resolve) => {
      markHeldStarted = resolve;
    });
    const heldGate = new Promise<void>((resolve) => {
      releaseHeld = resolve;
    });
    const held = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("held");
      markHeldStarted();
      await heldGate;
    });
    await heldStarted;
    const first = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("first");
    });
    const second = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("second");
    });
    const third = serializeCommandIngressForActor("actor-fifo-order", async () => {
      ran.push("third");
    });
    await Promise.resolve();
    expect(ran).toEqual(["held"]);
    releaseHeld();
    await Promise.all([held, first, second, third]);
    expect(ran).toEqual(["held", "first", "second", "third"]);
  });
  it("adaptPluginHandle.handleAction round-trips an action's output/uiScope/historyPatch from AppFrame::Invocation; requestedEffects is honestly empty for a bare command-only handle", async () => {
    const { encodeAppFrame, decodeAppCommand, encodePackValue, decodePackValue } = await import("@semio-tech/framework-os");
    const fakeHandle = {
      manifest: { pluginId: "mock-action", label: "Mock Action", version: "0", apps: [], programs: [], examples: [] } as unknown as import("@semio-tech/framework").PluginManifest,
      createApp: async () => 3,
      destroyApp: async () => {},
      takeSegmentedDownloadChunk: async () => undefined,
      ...exchangeStyleChannel((_instanceId, frames) => {
        const [command] = frames.map(decodeAppCommand);
        if (!command || typeof command !== "object" || !("Command" in command)) throw new Error("expected a Command");
        const invocation = decodePackValue(new Uint8Array(command.Command.command));
        return [
          encodeAppFrame({
            Invocation: {
              in_reply_to: command.Command.seq,
              output: Array.from(encodePackValue({ echo: invocation })),
              diagnostics: Array.from(encodePackValue([])),
              ui_scope: Array.from(encodePackValue({ kind: "partial", windowBodies: ["graph"], utilities: false })),
              history_patch: Array.from(encodePackValue({ cursor: 1, upserts: [] })),
              messages: [],
              mutations: Array.from(encodePackValue([])),
              inverse_group: Array.from(encodePackValue({ invocationId: "", mutations: [], inverseMutations: [] })),
            },
          }),
        ];
      }),
      dispose: () => {},
    };
    const handle = await adaptPluginHandle("mock-action", { handle: fakeHandle, release: () => {} } as unknown as Parameters<typeof adaptPluginHandle>[1]);
    const instanceId = await handle.createApp("main");
    const invocation = { address: { pluginId: "mock-action", appId: "main", modeId: "edit", windowKindId: "main", windowInstanceId: "main", actionId: "addShot" }, arguments: { format: "png" } };
    const response = await handle.handleAction(instanceId, JSON.stringify(invocation), { locale: "en", terminology: "native" });
    expect(response.output).toEqual({ echo: invocation });
    expect(response.requestedEffects).toEqual([]);
    expect(response.uiScope).toEqual({ kind: "partial", windowBodies: ["graph"], utilities: false });
    expect(response.historyPatch).toEqual({ cursor: 1, upserts: [] });
  });
}
