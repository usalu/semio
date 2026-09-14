import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";
import {
  IMPORT_ARGUMENT_CHUNK,
  IMPORT_ARGUMENT_CHUNK_COUNT,
  IMPORT_ARGUMENT_INDEX,
  IMPORT_ARGUMENT_TOTAL,
  IMPORT_CHUNK_BYTES,
  importChunkArguments,
  importPayloadChunks,
  type ImportChunk,
} from "../../🟦️.ts";
import { GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES } from "../../../⏱️trace/🧮️memory/🟦️.ts";
import { wireEffectToFriendly } from "../../../🎭️actor/🖼️wire-turn/🟦️.ts";

/** 📤️ TypeScript twin of `🧪️tests/📤️file-open-import/🦀️.rs`, driven from the SAME fixture
 * (`🧫️fixtures/📤️file-open-import/🔣️.json`): what `Effect::RequestFileOpen` looks like on the wire,
 * how one opened file is sliced, and the exact argument envelope one chunk is dispatched with.
 *
 * 🐛️ The defects this pins, measured on 6118 for ticket 26/09/09/PROCEDURAL-3D-END-TO-END:
 * `wireEffectToFriendly` had NO `request-file-open` case at all, so `Import Document…` reached the
 * guest, produced its effect, and the wgpu door printed `unmapped effect "request-file-open"
 * dropped`; and the two renderers disagreed about what one import invocation carries.
 *
 * 🔍️ Independent where it counts: the byte-extent oracle is Node's own `Buffer.byteLength`/
 * `TextEncoder`, not the hand-rolled per-character width this module's slicer uses, so a slicer that
 * mismeasured UTF-8 fails against a decoder that shares no line with it.
 */

interface PayloadRepeat {
  readonly character: string;
  readonly count: number;
}

interface ChunkCase {
  readonly id: string;
  readonly payload?: string;
  readonly payloadRepeat?: PayloadRepeat;
  readonly chunks?: readonly string[];
  readonly chunkLengths?: readonly number[];
  readonly chunkByteLengths?: readonly number[];
}

interface WireCase {
  readonly id: string;
  readonly effect: { readonly tag: string; readonly val: unknown };
  readonly friendly: { readonly requestFileOpen: Record<string, unknown> };
  readonly warns: boolean;
}

interface ArgumentCase {
  readonly id: string;
  readonly name: string;
  readonly chunk: ImportChunk;
  readonly fanOut: { readonly index: number; readonly total: number } | null;
  readonly arguments: Record<string, string | number>;
}

interface FileOpenImportFixture {
  readonly importChunkBytes: number;
  readonly wireCases: readonly WireCase[];
  readonly chunkCases: readonly ChunkCase[];
  readonly argumentCases: readonly ArgumentCase[];
}

function loadFixture(): FileOpenImportFixture {
  const here = dirname(fileURLToPath(import.meta.url));
  return JSON.parse(readFileSync(join(here, "../../🧫️fixtures/📤️file-open-import/🔣️.json"), "utf8")) as FileOpenImportFixture;
}

function payloadOf(row: ChunkCase): string {
  if (typeof row.payload === "string") return row.payload;
  assert.ok(row.payloadRepeat, `${row.id} states neither payload nor payloadRepeat`);
  return row.payloadRepeat.character.repeat(row.payloadRepeat.count);
}

/** 📤️ The whole law, callable without vitest so the nx lane runs it as a plain oracle. */
export function testFileOpenImportContract(): void {
  const fixture = loadFixture();
  assert.equal(IMPORT_CHUNK_BYTES, GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2, "the chunk extent is derived from the guest ceiling, never a literal");
  assert.equal(fixture.importChunkBytes, IMPORT_CHUNK_BYTES, "the fixture records the extent both twins compute");
  assert.ok(fixture.wireCases.length >= 4 && fixture.chunkCases.length >= 6 && fixture.argumentCases.length >= 3, "the fixture must keep driving all three halves");

  // 1️⃣ The wire hop the defect lived on: every recorded `request-file-open` shape decodes, and none
  //    of them warns. A door with no case at all returns `null` and prints an `unmapped effect` line.
  const warnings: string[] = [];
  const consoleWarn = console.warn;
  console.warn = (...parts: readonly unknown[]): void => {
    warnings.push(parts.map(String).join(" "));
  };
  try {
    for (const row of fixture.wireCases) {
      const before = warnings.length;
      const friendly = wireEffectToFriendly(row.effect, (bytes) => [...bytes]);
      assert.ok(friendly !== null, `${row.id}: the decoder must map request-file-open`);
      assert.deepEqual(friendly, row.friendly, `${row.id}: friendly effect`);
      assert.equal(warnings.length > before, row.warns, `${row.id}: warning expectation`);
    }
  } finally {
    console.warn = consoleWarn;
  }

  // 2️⃣ Every fixture payload slices into exactly the chunks it declares, each naming its position.
  for (const row of fixture.chunkCases) {
    const payload = payloadOf(row);
    const chunks = importPayloadChunks(payload);
    if (row.chunks) assert.deepEqual(chunks.map((chunk) => chunk.payload), [...row.chunks], `${row.id}: chunk payloads`);
    if (row.chunkLengths) assert.deepEqual(chunks.map((chunk) => [...chunk.payload].length), [...row.chunkLengths], `${row.id}: chunk character lengths`);
    if (row.chunkByteLengths) assert.deepEqual(chunks.map((chunk) => Buffer.byteLength(chunk.payload, "utf8")), [...row.chunkByteLengths], `${row.id}: chunk byte lengths`);
    chunks.forEach((chunk, position) => {
      assert.equal(chunk.chunk, position, `${row.id}: chunk ${position} names its position`);
      assert.equal(chunk.chunkCount, chunks.length, `${row.id}: chunk ${position} names its run length`);
    });
  }

  // 3️⃣ No chunk exceeds the extent, and the run reassembles the payload exactly — measured with a
  //    third-party byte counter, never the slicer's own arithmetic.
  for (const row of fixture.chunkCases) {
    const payload = payloadOf(row);
    const chunks = importPayloadChunks(payload);
    assert.ok(chunks.length > 0, `${row.id}: a pick is always at least one chunk`);
    for (const chunk of chunks) {
      assert.ok(Buffer.byteLength(chunk.payload, "utf8") <= IMPORT_CHUNK_BYTES, `${row.id}: chunk ${chunk.chunk} is ${Buffer.byteLength(chunk.payload, "utf8")} B`);
      assert.equal(new TextDecoder("utf-8", { fatal: true }).decode(new TextEncoder().encode(chunk.payload)), chunk.payload, `${row.id}: chunk ${chunk.chunk} split a code point`);
    }
    assert.equal(chunks.map((chunk) => chunk.payload).join(""), payload, `${row.id}: the run must reassemble`);
  }

  // 4️⃣ One chunk's dispatch arguments are exactly the fixture's, the fan-out only for a multi pick.
  for (const row of fixture.argumentCases) {
    const args = importChunkArguments(row.name, row.chunk, row.fanOut ?? undefined);
    assert.deepEqual(args, row.arguments, `${row.id}: dispatch arguments`);
    assert.equal(IMPORT_ARGUMENT_INDEX in args, row.fanOut !== null, `${row.id}: fan-out presence`);
    for (const key of [IMPORT_ARGUMENT_CHUNK, IMPORT_ARGUMENT_CHUNK_COUNT, IMPORT_ARGUMENT_INDEX, IMPORT_ARGUMENT_TOTAL]) {
      if (!(key in args)) continue;
      assert.ok(Number.isInteger(args[key]), `${row.id}: ${key} must stay an exact integer — the guest decodes it as u32 and refuses a float`);
    }
  }

  // 5️⃣ A run long enough to need several chunks really does, and the LAST chunk is the only short
  //    one — the shape the guest's staging admits without a gap.
  const long = importPayloadChunks("x".repeat(IMPORT_CHUNK_BYTES * 2 + 7));
  assert.equal(long.length, 3, "two full chunks and a remainder");
  assert.deepEqual(long.map((chunk) => chunk.payload.length), [IMPORT_CHUNK_BYTES, IMPORT_CHUNK_BYTES, 7]);

  console.log(`file-open-import wire=${fixture.wireCases.length} chunks=${fixture.chunkCases.length} arguments=${fixture.argumentCases.length} extent=${IMPORT_CHUNK_BYTES} oracle=Buffer.byteLength`);
}
