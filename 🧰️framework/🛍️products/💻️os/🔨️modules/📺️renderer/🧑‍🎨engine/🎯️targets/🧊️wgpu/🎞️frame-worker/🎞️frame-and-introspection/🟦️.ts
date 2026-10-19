import { assetAbort, assetCancellation, assetPumping, bindings, closed, closing, diagnosticsStamp, failed, frameInput, frameSequence, frameTurns, INTROSPECTION_STEP_BUDGET_MS, lastStepOutcome, lastWorld3dAcceptedFrameDiagnostic, lifecycle, mediaRegistry, originalFrameSource, pageImageDecode, quarantined, runtime, setFailed, setFrameSequence, setLastFrame, setLastWorld3dAcceptedFrameDiagnostic, setPendingFault, setQuarantined } from "../🧵️state/🟦️.ts";
import { nextFrameSequence } from "../../🧵️frame-turn-scheduler/🟦️.ts";
import { ownedStep, post } from "../⏱️steps-and-reporting/🟦️.ts";
import { beginClose, fault, requestFault } from "../🧯️close-and-fault/🟦️.ts";
import type { BrowserFrameUiMessage, BrowserFrameWorkerMessage } from "../../🚚️browser-frame-transport/📋️contract/🟦️.ts";
import { cancelPageImageDecode } from "../🖼️page-image-decode/🟦️.ts";
import { scheduleAssetPump } from "../📦️assets-and-host-io/🟦️.ts";
import type { BrowserMediaCommand } from "../../../../🎬️media/🌐️browser/🟦️.ts";

export function runFrameTurn(): boolean {
  if (!runtime || !frameInput || closed || closing || failed || quarantined) return false;
  const startedAt = performance.now();
  try {
    setFrameSequence(nextFrameSequence(frameSequence));
    const input = frameInput;
    const originalFrameOutput=ownedStep("frame-step",()=>originalFrameSource!.tick(input.timestampMs,BigInt(frameSequence),BigInt(input.generation)));
    if(originalFrameOutput===undefined)return true;
    if(typeof originalFrameOutput!=="string"){setPendingFault({code:"original-frame-failure",detail:"original native failure retained",originalFailure:originalFrameOutput});setFailed(true);beginClose();return false;}
    const result = JSON.parse(originalFrameOutput) as Omit<Extract<BrowserFrameWorkerMessage, { kind: "frame" }>, "kind" | "lifecycle" | "frameSequence" | "generation" | "workerDurationMs"> & { readonly continueFrame: boolean;readonly presentationGeneration:string };
    const presentationGeneration=Number(BigInt(result.presentationGeneration));
    if(!Number.isSafeInteger(presentationGeneration)||presentationGeneration<0)throw Error("original-frame.presentation-generation-refused");
    let mediaSlots = mediaRegistry.accept(result.quarantined ? [] : result.mediaSlots);
    const outcome = lastStepOutcome;
    setLastFrame({ cursor: result.cursor, fullscreen: result.fullscreen });
    const assetCancellationStep = assetCancellation.step({
      hasFetch: () => assetAbort !== undefined,
      hasPageImageDecode: () => pageImageDecode !== undefined,
      responseCurrent: () => runtime!.assetResponseCurrent(),
      abortFetch: () => assetAbort?.abort(),
      cancelPageImageDecode,
      returnResponseOwner: () => runtime!.abortAssetResponse(),
    });
    if (result.quarantined) setQuarantined({ code: result.faultCode ?? "renderer-quarantine", detail: result.faultDetail ?? "renderer quarantined its own frame step" });
    const sustained = outcome?.verdict === "sustained-overrun";
    const degrade = quarantined ?? (sustained ? { code: "worker-step-overrun", detail: `frame step executed ${outcome!.executingMs.toFixed(3)} ms for ${outcome!.consecutive} consecutive steps` } : undefined);
    if (degrade) mediaSlots = mediaRegistry.accept([]);
    post({ kind: "frame", mediaSlots, lifecycle, frameSequence, generation: presentationGeneration, cursor: result.cursor, fullscreen: result.fullscreen, requestFrame: result.requestFrame, nextDeadlineDelayMs: result.nextDeadlineDelayMs, progress: result.progress, workerDurationMs: performance.now() - startedAt, workerExecutingMs: outcome?.executingMs ?? 0, workerStepVerdict: outcome?.verdict ?? "clock-fault", quarantined: degrade !== undefined, faultCode: degrade?.code, faultDetail: degrade?.detail });
    publishWorld3dAcceptedFrameDiagnostic(input.generation);
    if (quarantined) requestFault((quarantined as { readonly code: string; readonly detail: string }).code, (quarantined as { readonly code: string; readonly detail: string }).detail);
    else if (assetCancellationStep === "idle") scheduleAssetPump();
    else if (assetCancellationStep === "returned" && !assetPumping) {
      assetCancellation.releaseReturned();
      scheduleAssetPump();
    }
    return result.continueFrame;
  } catch (error) {
    fault("frame-runtime-fault", error instanceof Error ? error.message : String(error));
    return false;
  }
}

/** 🩺️ Projects the accepted World3d visual inputs into a changed-only page-visible receipt. */
function publishWorld3dAcceptedFrameDiagnostic(generation: number): void {
  if (diagnosticsStamp !== true || !bindings?.dumpMeshStats) return;
  try {
    const dump = JSON.parse(bindings.dumpMeshStats()) as { readonly surfaces?: readonly Record<string, unknown>[] };
    const surfaces = (dump.surfaces ?? []).map((surface) => ({
      surfaceId: surface.surfaceId,
      paneId: surface.paneId,
      rect: surface.rect,
      bboxMin: surface.bboxMin,
      bboxMax: surface.bboxMax,
      camera: surface.camera,
      liveCamera: surface.liveCamera,
    }));
    const json = JSON.stringify({ surfaces });
    if (json === lastWorld3dAcceptedFrameDiagnostic) return;
    setLastWorld3dAcceptedFrameDiagnostic(json);
    post({ kind: "diagnostic", lifecycle, channel: "world3d-accepted-frame", generation, frameSequence, json });
  } catch {}
}

export function runAssetDecodeTurn(): boolean {
  if (!runtime || closed || closing || failed || quarantined) return false;
  try {
    const result = ownedStep("asset-decode-step", () => originalFrameSource!.assetDecode());
    if (result === undefined) return true;
    if (!(["pending", "published", "cancelled", "idle", "fault"] as const).includes(result.kind) || typeof result.pending !== "boolean") {
      throw new Error("asset-decode-protocol: invalid step result");
    }
    if (result.kind === "published") post({ kind: "wake", lifecycle });
    if (result.kind === "fault") {
      fault("asset-decode-fault", "renderer asset decode rejected its retained response");
      return false;
    }
    return result.pending;
  } catch (error) {
    fault("asset-decode-fault", error instanceof Error ? error.message : String(error));
    return false;
  }
}

/** 🔬️ Answers one read-only introspection request from the renderer's own thread-local, which only
 * exists in this isolate. It never faults the Worker: a missing export, a throwing export, or a turn wider
 * than `INTROSPECTION_STEP_BUDGET_MS` all come back as `json: null` plus a `detail`, so a probe can tell
 * "no hooks" from "empty dump" while the surface keeps running. */
export function answerIntrospection(message: Extract<BrowserFrameUiMessage, { kind: "introspect" }>): void {
  const respond = (json: string | null, detail?: string) => post({ kind: "introspection", lifecycle, requestId: message.requestId, probe: message.probe, json, ...(detail === undefined ? {} : { detail }) });
  if (closed) return;
  if (!runtime || !bindings) {
    respond(null, "renderer bindings are not mounted in this Worker");
    return;
  }
  const hook = message.probe === "structure" ? bindings.dumpStructure : message.probe === "accessibility" ? bindings.dumpAccessibility : message.probe === "mesh-stats" ? bindings.dumpMeshStats : message.probe === "board2d" ? bindings.dumpBoard2d : message.probe === "chrome" ? bindings.dumpChrome : message.probe === "agent-bridge-scope" ? bindings.dumpAgentBridgeOfferScope : bindings.dumpFrameStats;
  if (!hook) {
    respond(null, `renderer bindings expose no ${message.probe} introspection export`);
    return;
  }
  const startedAt = performance.now();
  try {
    const json = hook(message.windowId);
    const duration = performance.now() - startedAt;
    respond(json, duration >= INTROSPECTION_STEP_BUDGET_MS ? `${message.probe} introspection took ${duration.toFixed(3)} ms` : undefined);
  } catch (error) {
    respond(null, error instanceof Error ? error.message : String(error));
  }
}

/** 📮️ Answers one accepted slot command with transferable bounded chunk storage. */
export async function answerMediaCommand(message: BrowserMediaCommand): Promise<void> {
  try {
    if (closed || closing || failed || quarantined || !runtime) throw new Error("browser-media.closed");
    const result = await mediaRegistry.execute(message);
    const current = closed || closing || failed || quarantined;
    if (current) throw new Error("browser-media.retired");
    const transfer: Transferable[] = result && "data" in result ? [result.data.buffer as ArrayBuffer] : [];
    post({ kind: "media-result", lifecycle, requestId: message.requestId, slotToken: message.slotToken, result }, transfer);
    frameTurns?.requestRuntimeWake();
  } catch (error) {
    post({ kind: "media-result", lifecycle, requestId: message.requestId, slotToken: message.slotToken, fault: error instanceof Error ? error.message : String(error) });
  }
}
