/** 🗄️ File open import: slicing one opened file into wire chunks and the exact argument envelope each chunk is dispatched with.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES } from "@semio-tech/framework/trace/memory";

// #region 🎠️Kernel
//#region InvocationResponse
//#region 📤️FileOpenImport
/** 📥️ Bytes ONE import chunk may carry to the guest — TS twin of Rust `kernel::IMPORT_CHUNK_BYTES`.
 *
 * 🧊️ Derived from the guest's per-request contiguous ceiling, never a literal: an import's `payload`
 * crosses as ONE string and every guest hop that carries it asks for one contiguous block, so a whole
 * document sent as a single invocation asks the fixed guest heap for a block several times that
 * ceiling. Half the ceiling leaves the other half for the invocation envelope the chunk rides in. */
export const IMPORT_CHUNK_BYTES = GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2;

/** 📥️ The argument names one import chunk is dispatched with — TS twins of Rust
 * `kernel::IMPORT_ARGUMENT_*`. Declared beside the effect so no shell invents a second spelling of
 * the same envelope: the wgpu shell used to send `{json, payload}` in one unchunked invocation while
 * React sent `{payload, name, chunk, chunkCount}` per chunk, and a plugin could satisfy only one
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export const IMPORT_ARGUMENT_PAYLOAD = "payload";
export const IMPORT_ARGUMENT_NAME = "name";
export const IMPORT_ARGUMENT_CHUNK = "chunk";
export const IMPORT_ARGUMENT_CHUNK_COUNT = "chunkCount";
export const IMPORT_ARGUMENT_INDEX = "index";
export const IMPORT_ARGUMENT_TOTAL = "total";

/** 📥️ One chunk of one opened file, positioned in its own run — TS twin of Rust `kernel::ImportChunk`. */
export type ImportChunk = { readonly payload: string; readonly chunk: number; readonly chunkCount: number };

/** 📥️ Slices one opened file's contents into the chunks a shell dispatches, so no single import
 * invocation asks the guest for a contiguous block above its own per-request ceiling.
 *
 * 🔤️ Sliced by UTF-8 EXTENT, not by code units: the guest measures `text.len()` in bytes, so a slice
 * counted in UTF-16 units would overrun the cap by up to 3× on non-ASCII text. No slice ever splits a
 * code point, and an empty payload still yields exactly one chunk — a picked empty file is a real pick
 * the guest must be told about.
 *
 * Twin of Rust `kernel::import_payload_chunks`; both drive `🧫️fixtures/📤️file-open-import/🔣️.json`. */
export function importPayloadChunks(payload: string): readonly ImportChunk[] {
  const pages: string[] = [];
  let page = "";
  let pageBytes = 0;
  for (const character of payload) {
    const code = character.codePointAt(0) ?? 0;
    const characterBytes = code < 0x80 ? 1 : code < 0x800 ? 2 : code < 0x10000 ? 3 : 4;
    if (pageBytes + characterBytes > IMPORT_CHUNK_BYTES) {
      pages.push(page);
      page = "";
      pageBytes = 0;
    }
    page += character;
    pageBytes += characterBytes;
  }
  if (page.length > 0 || pages.length === 0) pages.push(page);
  return pages.map((text, chunk) => ({ payload: text, chunk, chunkCount: pages.length }));
}

/** 📥️ The arguments ONE import chunk is dispatched with — `fanOut` is present only when the picker
 * was opened with `multiple`, so a single-file pick's args stay byte-for-byte the pre-fan-out shape.
 * Twin of Rust `kernel::import_chunk_arguments`.
 *
 * 🔢️ `chunk`/`chunkCount`/`index`/`total` are exact integers: the guest decodes them as `u32` and
 * `FromValue`'s unsigned arm refuses a float outright. */
export function importChunkArguments(name: string, chunk: ImportChunk, fanOut?: { readonly index: number; readonly total: number }): Record<string, string | number> {
  const args: Record<string, string | number> = {
    [IMPORT_ARGUMENT_PAYLOAD]: chunk.payload,
    [IMPORT_ARGUMENT_NAME]: name,
    [IMPORT_ARGUMENT_CHUNK]: chunk.chunk,
    [IMPORT_ARGUMENT_CHUNK_COUNT]: chunk.chunkCount,
  };
  if (fanOut) {
    args[IMPORT_ARGUMENT_INDEX] = fanOut.index;
    args[IMPORT_ARGUMENT_TOTAL] = fanOut.total;
  }
  return args;
}
//#endregion 📤️FileOpenImport
//#endregion InvocationResponse
// #endregion 🎠️Kernel
