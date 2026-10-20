/** 🧾️ App channel codec: channel version admission, app commands and frames, window config, media export wire.
 *
 * @see ../🛠️backbone-wire-helpers/🟦️.ts — the shared helpers
 * @see ../../🟦️.ts — the package entry the suite reads fixtures relative to
 */
import { describe, expect, it } from "vitest";
import { AppChannelClient, AppChannelRequestSequence, decodeAppCommand, decodeAppFrame, encodeAppCommand, encodeAppFrame } from "@semio-tech/framework/app-channel";
import { createTurnOutcomeBroadcast } from "@semio-tech/framework";
import { source, type AppChannelHandle, type AppCommandValue, type AppFrameValue, type TurnOutcome } from "../🛠️backbone-wire-helpers/🟦️.ts";
import { registerRoundTripsAppTypedTests } from "./🔶️round-trips-app-typed/🟦️.ts";
import { registerTagsAppframeVariantPerTests } from "./🔸️tags-appframe-variant-per/🟦️.ts";
import { registerPinsAppChannelVersionTests } from "./🟠️pins-app-channel-version/🟦️.ts";

describe("@semio-tech/framework-os AppChannelCodec", () => {
  registerRoundTripsAppTypedTests();
  registerTagsAppframeVariantPerTests();
  registerPinsAppChannelVersionTests();
});

describe("resumable media export wire", () => {
  it("routes media requests through exact receipts without admitting unrelated large sequences", async () => {
    const handle = { app_instance_id: 7, parent_document_id: "doc-α", operation_id: 9007199254740993n, base_revision: 9007199254740995n, generation: 18446744073709551615n };
    const broadcast = createTurnOutcomeBroadcast<TurnOutcome>();
    const sent: AppCommandValue[] = [];
    const port: AppChannelHandle = { outcomes: broadcast.stream, enqueue: (_instance, packets) => sent.push(...packets.map(decodeAppCommand)) };
    const client = new AppChannelClient(port, new AppChannelRequestSequence(), 7, "fixture");
    try {
      const submitted = client.submitMediaExport("playback:out", handle.parent_document_id, handle.base_revision);
      expect(sent.at(-1)).toEqual({ SubmitMediaExport: { seq: 1n, port: "playback:out", expected_parent_document_id: handle.parent_document_id, expected_base_revision: handle.base_revision } });
      const valid: AppFrameValue = { MediaExportSubmitted: { in_reply_to: 1n, handle } };
      broadcast.push({ instanceId: 7, frames: [encodeAppFrame({ MediaExportSubmitted: { in_reply_to: 9007199254740993n, handle } }), encodeAppFrame(valid)] });
      expect(await submitted).toEqual([valid]);
      for (const [index, operation] of ["poll", "cancel", "chunk"].entries()) {
        const seq = BigInt(index + 2);
        const pending = operation === "poll" ? client.pollMediaExport(handle) : operation === "cancel" ? client.cancelMediaExport(handle) : client.takeMediaExportChunk(handle);
        expect(sent.at(-1)).toEqual({ [operation === "poll" ? "PollMediaExport" : operation === "cancel" ? "CancelMediaExport" : "TakeMediaExportChunk"]: { seq, handle } });
        const frame: AppFrameValue = operation === "chunk" ? { MediaExportChunk: { in_reply_to: seq, handle, data: [], terminal: true } } : operation === "cancel" ? { Done: { in_reply_to: Number(seq) } } : { MediaExportStatus: { in_reply_to: seq, handle, state: "running", applied_progress: 2n, checkpoint_available: false, mime_type: "", total_bytes: 0n, detail: "" } };
        broadcast.push({ instanceId: 7, frames: [encodeAppFrame(frame)] });
        expect(await pending).toEqual([frame]);
      }
    } finally { client.dispose(); }
  });

  it("matches the shared Rust vectors and the independent LEB128 encoder without rounding", async () => {
    const { readFileSync } = await import("node:fs");
    const directory = "./🔨️modules/📡️spr/🧵️channel/🧪️tests/🧫️fixtures/🎬️media-export-wire/";
    const fixture = JSON.parse(readFileSync(new URL(directory + "🔣️.json", source.url), "utf8"));
    const oracleModule = "@webassemblyjs/leb128/lib/leb.js";
    const imported = await import(oracleModule);
    const u64 = (value: bigint): number[] => {
      const input = Buffer.alloc(8); input.writeBigUInt64LE(value);
      return Array.from(imported.default.encodeUIntBuffer(input));
    };
    const string = (value: string): number[] => { const bytes = Buffer.from(value); return [...u64(BigInt(bytes.length)), ...bytes]; };
    const f = fixture.handle;
    const handle = { app_instance_id: f.appInstanceId, parent_document_id: f.parentDocumentId, operation_id: BigInt(f.operationId), base_revision: BigInt(f.baseRevision), generation: BigInt(f.generation) };
    const handleBytes = [...u64(BigInt(handle.app_instance_id)), ...string(handle.parent_document_id), ...u64(handle.operation_id), ...u64(handle.base_revision), ...u64(handle.generation)];
    for (const [index, name] of ["submit", "poll", "cancel", "takeChunk"].entries()) {
      const row = fixture.commands[name], value = row.fields, seq = BigInt(value.seq);
      const key = ["SubmitMediaExport", "PollMediaExport", "CancelMediaExport", "TakeMediaExportChunk"][index]!;
      const command = { [key]: index === 0 ? { seq, port: value.port, expected_parent_document_id: value.expectedParentDocumentId, expected_base_revision: BigInt(value.expectedBaseRevision) } : { seq, handle } } as AppCommandValue;
      const oracle = [37 + index, ...u64(seq), ...(index === 0 ? [...string(value.port), ...string(value.expectedParentDocumentId), ...u64(BigInt(value.expectedBaseRevision))] : handleBytes)];
      expect(oracle).toEqual(row.bytes);
      expect(Buffer.from(row.bytes).toString("hex")).toBe(row.hex);
      expect([...encodeAppCommand(command)]).toEqual(row.bytes);
      expect(decodeAppCommand(Uint8Array.from(row.bytes))).toEqual(command);
    }
    for (const name of ["submitted", "status", "chunk", "terminalChunk"]) {
      const row = fixture.frames[name], value = row.fields, in_reply_to = BigInt(value.inReplyTo);
      const state = ["running", "complete", "cancelled", "failed"].indexOf(value.state);
      const frame = name === "submitted" ? { MediaExportSubmitted: { in_reply_to, handle } } : name === "status" ? { MediaExportStatus: { in_reply_to, handle, state: value.state, applied_progress: BigInt(value.appliedProgress), checkpoint_available: value.checkpointAvailable, mime_type: value.mimeType, total_bytes: BigInt(value.totalBytes), detail: value.detail } } : { MediaExportChunk: { in_reply_to, handle, data: value.data, terminal: value.terminal } };
      const tail = name === "submitted" ? [] : name === "status" ? [state, ...u64(BigInt(value.appliedProgress)), Number(value.checkpointAvailable), ...string(value.mimeType), ...u64(BigInt(value.totalBytes)), ...string(value.detail)] : [...u64(BigInt(value.data.length)), ...value.data, Number(value.terminal)];
      expect([name === "submitted" ? 28 : name === "status" ? 29 : 30, ...u64(in_reply_to), ...handleBytes, ...tail]).toEqual(row.bytes);
      expect([...encodeAppFrame(frame as AppFrameValue)]).toEqual(row.bytes);
      expect(decodeAppFrame(Uint8Array.from(row.bytes))).toEqual(frame);
    }
  });

  it("preserves exact large operation authority and rejects incomplete or extra bytes", () => {
    const handle = { app_instance_id: 7, parent_document_id: "Überblick", operation_id: 9007199254740993n, base_revision: 18446744073709551615n, generation: 9007199254740995n };
    const commands = [
      { SubmitMediaExport: { seq: 1n, port: "playback:out", expected_parent_document_id: handle.parent_document_id, expected_base_revision: handle.base_revision } },
      { PollMediaExport: { seq: 2n, handle } }, { CancelMediaExport: { seq: 3n, handle } }, { TakeMediaExportChunk: { seq: 4n, handle } },
    ];
    const frames = [
      { MediaExportSubmitted: { in_reply_to: 1n, handle } },
      ...(["running", "complete", "cancelled", "failed"] as const).map((state) => ({ MediaExportStatus: { in_reply_to: 2n, handle, state, applied_progress: 9007199254740997n, checkpoint_available: true, mime_type: "audio/mpeg", total_bytes: 9007199254740999n, detail: "Status" } })),
      { MediaExportChunk: { in_reply_to: 4n, handle, data: [0, 128, 255], terminal: false } },
      { MediaExportChunk: { in_reply_to: 5n, handle, data: [], terminal: true } },
    ];
    for (const [index, command] of commands.entries()) {
      const bytes = encodeAppCommand(command as AppCommandValue);
      expect(bytes[0]).toBe(37 + index);
      expect(decodeAppCommand(bytes)).toEqual(command);
      expect(() => decodeAppCommand(bytes.subarray(0, bytes.length - 1))).toThrow();
      expect(() => decodeAppCommand(Uint8Array.from([...bytes, 0]))).toThrow();
    }
    for (const frame of frames) {
      const bytes = encodeAppFrame(frame as AppFrameValue);
      expect(decodeAppFrame(bytes)).toEqual(frame);
      expect(() => decodeAppFrame(bytes.subarray(0, bytes.length - 1))).toThrow();
      expect(() => decodeAppFrame(Uint8Array.from([...bytes, 0]))).toThrow();
    }
  });
});
