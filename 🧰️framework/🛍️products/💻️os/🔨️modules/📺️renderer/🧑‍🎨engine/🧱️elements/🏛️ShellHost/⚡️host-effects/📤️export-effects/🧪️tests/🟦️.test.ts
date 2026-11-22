import { expect, test } from "bun:test";
import { applyDownloadMediaExportEffect } from "../🟦️.ts";

type Download = Parameters<typeof applyDownloadMediaExportEffect>[0];

function download(signed: boolean, retireDuringRead = false): { readonly run: () => Promise<unknown>; readonly reads: string[] } {
  let current = true;
  const reads: string[] = [];
  const args = {
    segmentedDownloadAbortRef: { current: new AbortController() },
    isCurrentEffectOwner: () => current,
    effect: { downloadMediaExport: { filename: "terrain.asc", mimeType: "text/plain", encoding: "semio-segmented-handle-v1:identity", data: "91" } },
    baseSession: { pluginId: "gis", instanceId: 17 },
    effectOwner: {
      plugin: {
        takeSegmentedDownloadChunk: async (id: number, operation: bigint) => {
          reads.push(`local:${id}:${operation}`);
          return undefined;
        },
      },
      ...(signed ? {
        browserActor: {
          current: () => current,
          refresh: () => {},
          takeSegmentedDownloadChunk: async (operation: bigint) => {
            reads.push(`signed:${operation}`);
            if (retireDuringRead) current = false;
            return reads.length === 1 ? new Uint8Array([36, 37]) : undefined;
          },
        },
      } : {}),
    },
  } as unknown as Download;
  return { run: () => applyDownloadMediaExportEffect(args), reads };
}

test("the shell drains a signed export through the actor that published the effect", async () => {
  const signed = download(true);
  await signed.run();
  expect(signed.reads).toEqual(["signed:91", "signed:91"]);
  const local = download(false);
  await local.run();
  expect(local.reads).toEqual(["local:17:91"]);
});

test("owner retirement during a read aborts the download without falling back", async () => {
  const retired = download(true, true);
  await expect(retired.run()).rejects.toThrow("segmented-download-owner-retired");
  expect(retired.reads).toEqual(["signed:91"]);
});
