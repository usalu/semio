import { createElement } from "react";
import { createRoot } from "react-dom/client";
import { mediaTransportLabels } from "../../../../../../🔌️plugin/🪟️window-kits/🎬️media/🟦️.ts";
import { MediaTransportHost, MediaTransportOwnerContext, type MediaTransportPort } from "../../🟦️.tsx";

type BrowserInput = {
  readonly identity: { readonly instanceId: number; readonly controllerId: string; readonly parentDocumentId: string; readonly windowId: string; readonly revision: string; readonly generation: string; readonly outputPort: "playback:out" };
  readonly media: { readonly kind: "audio" | "video"; readonly mediaType: string; readonly durationMs: number; readonly positionMs: number; readonly selection: readonly [number, number] };
  readonly running: { readonly progress: readonly number[]; readonly detail: readonly string[] };
  readonly output: { readonly chunks: readonly (readonly number[])[]; readonly totalBytes: number };
};

declare global {
  interface Window {
    mountMediaTransportOracle(input: BrowserInput): Promise<void>;
    mediaTransportCalls: readonly string[];
  }
}

window.mountMediaTransportOracle = async (input) => {
  const calls: string[] = [];
  const handle = { app_instance_id: input.identity.instanceId, parent_document_id: input.identity.parentDocumentId, operation_id: 9n, base_revision: BigInt(input.identity.revision), generation: BigInt(input.identity.generation) };
  let poll = 0;
  let chunk = 0;
  const port: MediaTransportPort = {
    submitMediaExport: async (instanceId, outputPort, parentDocumentId, revision) => {
      calls.push(`submit:${instanceId}:${outputPort}:${parentDocumentId}:${revision}`);
      return handle;
    },
    pollMediaExport: async () => {
      const running = poll < input.running.progress.length;
      const index = poll++;
      calls.push(`poll:${index}`);
      return { handle, state: running ? "running" : "complete", applied_progress: BigInt(input.running.progress[Math.min(index, input.running.progress.length - 1)] ?? 0), checkpoint_available: false, mime_type: input.media.mediaType, total_bytes: BigInt(input.output.totalBytes), detail: running ? input.running.detail[index] ?? "" : "complete" };
    },
    cancelMediaExport: async () => { calls.push("cancel"); },
    takeMediaExportChunk: async () => {
      const index = chunk++;
      calls.push(`chunk:${index}`);
      return { handle, data: Uint8Array.from(input.output.chunks[index] ?? []), terminal: index === input.output.chunks.length - 1 };
    },
  };
  const labels = mediaTransportLabels("en");
  const value = { schemaVersion: 1, kind: input.media.kind, mediaType: input.media.mediaType, revision: input.identity.revision, durationMs: input.media.durationMs, positionMs: input.media.positionMs, selectionStartMs: input.media.selection[0], selectionEndMs: input.media.selection[1], locale: "en", labels, resource: { kind: "artifact-media-export", controllerId: input.identity.controllerId, appInstanceId: input.identity.instanceId, parentDocumentId: input.identity.parentDocumentId, outputPort: input.identity.outputPort, revision: input.identity.revision, generation: input.identity.generation }, capability: { status: "ready", reason: null }, hostContentHeight: 240 };
  const root = createRoot(document.getElementById("root")!);
  root.render(createElement(MediaTransportOwnerContext.Provider, { value: { instanceId: input.identity.instanceId, controllerId: input.identity.controllerId, windowId: input.identity.windowId, port } }, createElement(MediaTransportHost, { value, nodeId: 7, nodeKey: "media.transport" })));
  await new Promise<void>((resolve, reject) => {
    const timeout = window.setTimeout(() => reject(new Error("media-browser-timeout")), 5_000);
    const observer = new MutationObserver(() => {
      if (!document.querySelector('[data-media-state="ready"]')) return;
      window.clearTimeout(timeout);
      observer.disconnect();
      resolve();
    });
    observer.observe(document.getElementById("root")!, { childList: true, subtree: true, attributes: true });
  });
  window.mediaTransportCalls = calls;
};
