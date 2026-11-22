import { requestFileSelectionV1 } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import type { PickedFileFormatCatalogPortV1 } from "../../../🧱️elements/🏛️ShellHost/📂️picked-file-opening/🟦️.ts";
import type { BrowserFramePickedFile } from "../🚚️browser-frame-transport/📋️contract/🟦️.ts";

/** 🛬️ How many OS files the page holds for the Worker at once; the oldest is released first. */
export const WGPU_PICKED_FILE_CAPACITY = 64;

/** 📏️ The widest range one read answers, so a base64 reply stays a small host-io frame. */
export const WGPU_PICKED_FILE_RANGE_MAX_BYTES = 1024 * 1024;

/** 🗂️ The page's table of OS files taken from a drop or a pick: the Worker names a file by token, the `File` never crosses. */
export type WgpuPickedFiles = Readonly<{
  hold: (file: File) => BrowserFramePickedFile;
  pick: (accept: string) => Promise<BrowserFramePickedFile | null>;
  read: (token: number, start: number, end: number) => Promise<Uint8Array>;
  release: (token: number) => void;
}>;

/** 🛬️ Opens the page's table of picked files; a token is never reused, and a read of a released or unknown token is refused. */
export function createWgpuPickedFiles(): WgpuPickedFiles {
  const held = new Map<number, File>();
  let next = 1;
  const hold = (file: File): BrowserFramePickedFile => {
    const token = next++;
    held.set(token, file);
    if (held.size > WGPU_PICKED_FILE_CAPACITY) held.delete(held.keys().next().value as number);
    return { token, name: file.name, size: file.size, mime: file.type };
  };
  return {
    hold,
    pick: async (accept) => {
      const [file] = await requestFileSelectionV1(accept);
      return file === undefined ? null : hold(file);
    },
    read: async (token, start, end) => {
      const file = held.get(token);
      if (file === undefined) throw new Error("wgpu-picked-files.unknown-token");
      if (!Number.isSafeInteger(start) || !Number.isSafeInteger(end) || start < 0 || end < start || end - start > WGPU_PICKED_FILE_RANGE_MAX_BYTES) throw new Error("wgpu-picked-files.range");
      return new Uint8Array(await file.slice(start, end).arrayBuffer());
    },
    release: (token) => void held.delete(token),
  };
}

/** 🧭️ The Open file format catalog as the Worker reads it: every claim with its owner dialect, each revision's content check and every package claim, the product port's answers flattened into one JSON document. */
export function wgpuFormatClaimsJson(catalog: PickedFileFormatCatalogPortV1 | undefined): string {
  const owned = (kindId: string) => ({ kindId, artifactRef: catalog?.artifactRefOf(kindId) ?? null, package: catalog?.packageOf?.(kindId) ?? null });
  const formats = (catalog?.descriptors() ?? []).map((claim) => ({
    ...claim,
    ...owned(claim.kindId),
    revisions: (catalog?.revisionsOf?.(claim.kindId) ?? []).map((revision) => ({ ...owned(revision.kindId), window: revision.window, contains: revision.contains })),
  }));
  return JSON.stringify({ formats });
}
