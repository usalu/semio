/** 📤️ The host effects that hand a document out: media downloads, video render and icon render exports.
 *
 * 🧱️ Child module of {@link ../🟦️.ts | useApplyHostEffects}: each function is one effect arm of the `applyHostEffects` pass, moved verbatim. */
import { renderIconRequest } from "../../../🖼️IconRenderHost/🚚️request/🟦️.ts";
import { type Effect } from "@semio-tech/framework";
import { type ActiveSession } from "../../../🐚️Shell/📦️program-sessions/🟦️.ts";
import { downloadMediaExportBytes, downloadDataUrl, downloadMediaExport, mediaExportEncodingText, shellSegmentedDownloadSinkFactory } from "../../../🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { drainSegmentedMediaExport, SEGMENTED_DOWNLOAD_MARKER_PREFIX } from "../../../📤️SegmentedDownload/🟦️.ts";
import { browserVideoRenderEnvironmentV1, runVideoRenderExportV1 } from "../../../🎥️VideoRenderHost/🟦️.ts";
import type { ShellCoreAfterSpawnedSession } from "../../🛫️spawned-session/🟦️.ts";

/** 💾️ Delivers an inline or segmented media export as a download. */
export async function applyDownloadMediaExportEffect({ loadedPlugins, segmentedDownloadAbortRef, effect, baseSession }: Pick<ShellCoreAfterSpawnedSession, "loadedPlugins" | "segmentedDownloadAbortRef"> & {
  readonly effect: Extract<Effect, Readonly<Record<"downloadMediaExport", unknown>>>;
  readonly baseSession: ActiveSession;
}) {
  const { filename, mimeType, data, encoding } = effect.downloadMediaExport;
  const encodingText = mediaExportEncodingText(encoding);
  if (encodingText?.startsWith(SEGMENTED_DOWNLOAD_MARKER_PREFIX)) {
    const pluginEntry = loadedPlugins.find((entry) => entry.handle.pluginId === baseSession.pluginId);
    if (!pluginEntry) throw new Error(`segmented-download-plugin-missing:${baseSession.pluginId}`);
    await drainSegmentedMediaExport(filename, mimeType, data, encodingText, (operationId) => pluginEntry.handle.takeSegmentedDownloadChunk(baseSession.instanceId, operationId), {
      signal: segmentedDownloadAbortRef.current.signal,
      sinkFactory: shellSegmentedDownloadSinkFactory,
    });
  } else {
    downloadMediaExport(filename, mimeType, data, encodingText);
  }
}

/** 🎞️ Runs a video render export and announces its outcome. */
export function applyVideoRenderExportEffect({ catalog, uiLocaleRef, showTransientNoticeRef, effect, baseSession }: Pick<ShellCoreAfterSpawnedSession, "catalog" | "uiLocaleRef" | "showTransientNoticeRef"> & {
  readonly effect: Extract<Effect, Readonly<Record<"videoRenderExport", unknown>>>;
  readonly baseSession: ActiveSession;
}) {
  const { filename, program } = effect.videoRenderExport;
  const capabilities = catalog.plugins.find((target) => target.pluginId === baseSession.pluginId)?.capabilities ?? [];
  void runVideoRenderExportV1(
    { filename, owner: baseSession.pluginId, program, capabilities },
    {
      environment: browserVideoRenderEnvironmentV1(),
      deliver: (name, bytes) => downloadMediaExportBytes(name, "video/mp4", bytes),
      locale: () => uiLocaleRef.current,
      announce: (text) => showTransientNoticeRef.current(text, "info", "video-render-export.started"),
    },
  ).then(({ outcome, text }) => showTransientNoticeRef.current(text, outcome.status === "done" || outcome.status === "cancelled" ? "info" : "warning", `video-render-export.${outcome.status}`));
}

/** 🖼️ Renders each requested icon and downloads the data URL. */
export async function applyIconRenderExportEffect({ effect }: {
  readonly effect: Extract<Effect, Readonly<Record<"iconRenderExport", unknown>>>;
}) {
  for (const item of effect.iconRenderExport.items) {
    try {
      const result = await renderIconRequest(item.request as Parameters<typeof renderIconRequest>[0]);
      downloadDataUrl(item.filename, result.dataUrl);
    } catch (error) {
      console.error(`icon render export failed for ${item.filename}`, error);
    }
  }
}
