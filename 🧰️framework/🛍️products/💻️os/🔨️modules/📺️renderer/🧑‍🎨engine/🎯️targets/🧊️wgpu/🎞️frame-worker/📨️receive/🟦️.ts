import { bindings, closed, closing, failed, frameTurns, interactiveJobs, lifecycle, mediaRegistry, quarantined, runtime, setFrameInput } from "../🧵️state/🟦️.ts";
import type { BrowserFrameUiMessage } from "../../🚚️browser-frame-transport/📋️contract/🟦️.ts";
import { boot } from "../🔌️boot-and-plugins/🟦️.ts";
import { beginClose, fault } from "../🧯️close-and-fault/🟦️.ts";
import { answerIntrospection, answerMediaCommand } from "../🎞️frame-and-introspection/🟦️.ts";
import { settleHostIo } from "../📦️assets-and-host-io/🟦️.ts";
import { settlePageImageDecode } from "../🖼️page-image-decode/🟦️.ts";
import { ownedStep, post } from "../⏱️steps-and-reporting/🟦️.ts";

export async function receive(message: BrowserFrameUiMessage): Promise<void> {
  if (message.kind === "original-stock-result") return;
  if (message.kind === "boot") {
    await boot(message);
    return;
  }
  if (message.kind === "shard-port") return;
  if (message.kind === "image-decode-result" && message.lifecycle !== lifecycle) {
    message.bitmap?.close();
    return;
  }
  if (message.lifecycle !== lifecycle) return;
  if (message.kind === "close") {
    if (closed || closing) return;
    beginClose();
    return;
  }
  if (message.kind === "media-release") {
    mediaRegistry.release(message.slotToken);
    return;
  }
  if (message.kind === "media-command") {
    await answerMediaCommand(message);
    return;
  }
  if (message.kind === "introspect") {
    answerIntrospection(message);
    return;
  }
  if (message.kind === "host-io-result") {
    settleHostIo(message);
    return;
  }
  if (message.kind === "image-decode-result") {
    settlePageImageDecode(message);
    return;
  }
  if (message.kind === "host-appearance") {
    ownedStep("host-appearance", () => bindings?.semioWgpuSetHostAppearance?.(message.appearance.preference, message.appearance.systemDark));
    return;
  }
  if (message.kind === "host-introduction-policy") {
    ownedStep("host-introduction-policy", () => bindings?.semioWgpuSetHostIntroductionSuppressed?.(message.suppressed));
    return;
  }
  if (message.kind === "host-storage") {
    ownedStep("host-storage", () => bindings?.semioWgpuSetHostStorage?.(JSON.stringify(message.storage)));
    return;
  }
  if (message.kind === "host-agent-bridge") {
    ownedStep("host-agent-bridge", () => bindings?.semioWgpuSetAgentBridgeConfig?.(message.offer?.url ?? "", message.offer?.admissionProof ?? ""));
    return;
  }
  if (closed || closing || failed || quarantined) return;
  if (message.kind === "host-page-hidden") {
    ownedStep("host-page-hidden", () => bindings?.semioWgpuHostPageHidden?.());
    frameTurns?.request();
    return;
  }
  if (message.kind === "host-window-blur") {
    ownedStep("host-window-blur", () => bindings?.semioWgpuHostWindowBlur?.());
    frameTurns?.request();
    return;
  }
  if (message.kind === "job-submit" || message.kind === "job-input-page" || message.kind === "job-cancel") {
    if (!interactiveJobs) {
      fault("interactive-job-not-ready", "interactive job arrived before Worker boot completed");
      return;
    }
    ownedStep(`interactive-job:${message.kind}`, () => interactiveJobs!.receive(message));
    return;
  }
  if (!runtime) {
    fault("worker-not-booted", "frame batch arrived before renderer boot completed");
    return;
  }
  try {
    ownedStep("frame-ingress", () => runtime!.enqueueBatch(JSON.stringify({ replaceable: message.replaceable, lossless: message.lossless }), BigInt(message.generation)));
    setFrameInput({ timestampMs: message.timestampMs, generation: message.generation });
    post({ kind: "batch-accepted", lifecycle, inputSequence: message.inputSequence, generation: message.generation });
    frameTurns?.request();
  } catch (error) {
    fault("frame-runtime-fault", error instanceof Error ? error.message : String(error));
  }
}
