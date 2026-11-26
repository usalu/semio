import { decodeAppFrame } from "../../../../../../🧰️framework/🔨️modules/🧾️app-channel/🎞️frame-codec/🟦️.ts";
import { encodeAppCommand } from "../../../../../../🧰️framework/🔨️modules/🧾️app-channel/🎛️command-codec/🟦️.ts";
import { decodeActorInstanceLifecycle, type ActorInstanceLifetime } from "../../../../../../🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🟦️.ts";
import { createShardCommandIngressPages } from "../../../../../../🧰️framework/🔨️modules/🎭️actor/📮️shard-client/📜️wire/🟦️.ts";
import { shellFrameBytes } from "../../../../../../🧰️framework/🔨️modules/🎭️actor/🖼️wire-turn/🔖️turn-result/🟦️.ts";

export type GuestCheck = {
  moduleUrl: string;
  applicationIds: readonly string[];
  guestReadConfigSequence: number;
  browserCaller: {
    lifetimeUs: number;
    fuel: number;
    maximumTurns: number;
    grant: { maximumItems: number; maximumCopyBytes: number; maximumCapacityBytes: number; maximumReleaseBytes: number; maximumDepth: number };
  };
};

async function exercise(input: GuestCheck): Promise<void> {
  const { createActorApi } = await import(input.moduleUrl);
  const { browserCaller: policy, guestReadConfigSequence: seq } = input;
  const instance = 1;
  const activationGeneration = 1n;
  const now = () => BigInt(Math.floor(performance.now() * 1000));
  const loadCore = async (url: string) => {
    const response = await fetch(url);
    if (!response.ok) throw Error(`Missing browser guest module ${url}`);
    return WebAssembly.compileStreaming(response);
  };
  const budget = { maxEffects: 64, maxPatchBytes: 1 << 20, maxFrames: 64 };
  const [page] = createShardCommandIngressPages({
    owner: 1n, generation: 1n, commandIndex: 0, commandCount: 1, instance, seq: BigInt(seq), command: encodeAppCommand({ ReadConfig: { seq } }),
  });
  if (!page || page.cursor.pageCount !== 1) throw Error("Expected one ReadConfig page");
  for (const appId of input.applicationIds) {
    const actor = appId.split("#").at(-1);
    const api = await createActorApi(actor, activationGeneration, {
      operation: 1n, generation: 1n, grant: policy.grant, fuel: policy.fuel, maximumElapsedUs: BigInt(policy.lifetimeUs),
    }, now, { receipt() {}, hold(refusal: unknown) { throw Error("BIM guest refused its original call", { cause: refusal }); } }, loadCore);
    let event: { kind: string; payload: unknown } | undefined = {
      kind: "instance-open",
      payload: { instance, purpose: { tag: "ordinary" }, activationGeneration, requestSequence: 1, appId, actor, config: new Uint8Array(), assets: [], capabilities: [], quotas: new Uint8Array() },
    };
    let phase: "open" | "read" | "close" = "open";
    let lifetime: ActorInstanceLifetime | undefined;
    let closeGeneration: bigint | undefined;
    let commandAdmitted = false, commandComplete = false, configReceived = false, retired = false, settled = false;
    for (let turn = 0; turn < policy.maximumTurns; turn++) {
      const result = await api.poll(event ? [event] : [], phase === "read" && !commandAdmitted ? page : undefined, undefined, budget);
      event = undefined;
      if (result.status.tag === "faulted" || result.commandIngress?.tag === "fault") throw Error(`${appId} faulted during ${phase}`);
      if (result.uiPatches.length || result.presence.length || result.uiPatchReceipt) throw Error(`${appId} issued an unexpected renderer update`);
      for (const effect of result.effects) {
        const bytes = shellFrameBytes(effect, instance);
        const frame = bytes && decodeAppFrame(bytes);
        if (phase !== "read" || !frame || !("Config" in frame) || frame.Config.in_reply_to !== seq || configReceived) {
          throw Error(`${appId} sent an unexpected ReadConfig reply`);
        }
        configReceived = true;
      }
      if (result.lifecycleReceipt) {
        const receipt = decodeActorInstanceLifecycle(result.lifecycleReceipt);
        if (receipt.kind === "open" || receipt.kind === "close" || receipt.kind === "ack" || phase === "read") throw Error("Unexpected lifecycle receipt");
        if (receipt.lifetime.instanceId !== instance || receipt.lifetime.activationGeneration !== activationGeneration || receipt.requestSequence !== (phase === "open" ? 1 : 2)) {
          throw Error(`${appId} acknowledged another lifecycle request`);
        }
        if (receipt.kind === "captured" && phase === "open") lifetime = receipt.lifetime;
        else if (receipt.kind !== "captured" && phase === "close" && receipt.lifetime.guestLifetime === lifetime?.guestLifetime) {
          if (closeGeneration !== undefined && receipt.closeGeneration !== closeGeneration) throw Error("BIM close generation changed");
          closeGeneration = receipt.closeGeneration;
          retired ||= receipt.kind === "retired";
        } else throw Error("BIM lifecycle receipt did not match its current phase");
        event = { kind: "instance-lifecycle-ack", payload: { kind: "ack", receipt } };
      }
      if (phase === "read") {
        const ingress = result.commandIngress?.tag;
        commandAdmitted ||= ingress === "page-accepted" || ingress === "command-pending" || ingress === "command-complete";
        commandComplete ||= ingress === "command-complete";
      }
      if (event || result.status.tag !== "idle" || result.nextWake != null) continue;
      if (phase === "open") {
        if (!lifetime) throw Error(`${appId} settled without capturing its original instance`);
        phase = "read";
      } else if (phase === "read" && commandComplete) {
        if (!configReceived) throw Error(`${appId} completed ReadConfig without a reply`);
        phase = "close";
        event = { kind: "instance-close", payload: { kind: "close", lifetime, requestSequence: 2 } };
      } else if (phase === "close" && retired) {
        settled = true;
        break;
      }
    }
    if (!settled || !(await api.closeOriginal()).sourceClosed) throw Error(`${appId} did not reach terminal close`);
  }
}

self.onmessage = async (message: MessageEvent<GuestCheck>) => {
  try {
    const wasm = WebAssembly as typeof WebAssembly & { Suspending?: Function; promising?: Function };
    if (typeof wasm.Suspending !== "function" || typeof wasm.promising !== "function") throw Error("Chromium worker does not support WebAssembly JSPI");
    await exercise(message.data);
    self.postMessage({ applicationIds: message.data.applicationIds });
  } catch (error) {
    self.postMessage({ error: error instanceof Error ? error.stack ?? error.message : String(error) });
  }
};
