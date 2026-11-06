/** 📤️ Checked exports bind the saved source and drain their exact operation before delivery. */
import { describe, expect, it } from "vitest";
import Ajv2020 from "ajv/dist/2020";
import { createActor, createMachine } from "xstate";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import declarations from "./📜️declarations.json";
import declarationSchema from "./🧬️schema/📜️declarations.json";
import operationPolicySchema from "../../🎬️media/🚚️lifecycle/🧬️schema/🔣️.json";
import agentRouteSchema from "../../🧱️elements/🏛️ShellHost/🌉️agent-artifact-route/🧬️schema/🔣️.json";
import { EXPORT_MEDIA_FILE_ACTION_ID, EXPORT_SNAPSHOT_FILE_ACTION_ID, type AppDefinition } from "@semio-tech/framework";
import { declaredMediaFileExportV1, downloadDeclaredMediaFileV1, mediaFileDescriptorMatchesV1 } from "../../🎬️media/📤️file/🟦️.ts";
import { encodePackValue } from "@semio-tech/framework/packs";
import { collectMediaExportBytes, MEDIA_EXPORT_OPERATION_POLICY, type MediaExportBytes, type MediaTransportPort } from "../../🎬️media/🚚️lifecycle/🟦️.ts";
import type { MediaExportHandle, MediaExportStatus } from "@semio-tech/framework-os";

describe("checked media export lifecycle", () => {
  it("validates neutral vectors independently", () => {
    expect(new Ajv2020({ strict: true }).compile(schema)(fixture)).toBe(true);
    expect(new Ajv2020({ strict: true }).compile(declarationSchema)(declarations)).toBe(true);
    expect(new Ajv2020({ strict: true }).compile(operationPolicySchema)(MEDIA_EXPORT_OPERATION_POLICY)).toBe(true);
    expect(MEDIA_EXPORT_OPERATION_POLICY).toEqual(agentRouteSchema.$defs.ExportOperationPolicy.const);
  });
  it("binds static action metadata to one declared output and binary descriptor", () => {
    expect(EXPORT_MEDIA_FILE_ACTION_ID).toBe(declarations.actionId);
    const app = { actions: [{ id: declarations.actionId, args: Object.entries(declarations.defaults).map(([id, value]) => ({ id, default: value })) }], windowKinds: [], io: { ports: [declarations.port] } } as unknown as AppDefinition;
    const declaration = declaredMediaFileExportV1(app, EXPORT_MEDIA_FILE_ACTION_ID);
    expect(declaration).toEqual({ ...declarations.defaults, portMediaType: declarations.port.mediaType });
    expect(mediaFileDescriptorMatchesV1(declarations.descriptor, declaration!)).toBe(true);
    for (const descriptor of [{ ...declarations.descriptor, portId: "foreign" }, { ...declarations.descriptor, wire: { kind: "document", schema: "3d.mesh" } }, { ...declarations.descriptor, mediaType: { class: "twoD", form: "raster" } }]) expect(mediaFileDescriptorMatchesV1(descriptor, declaration!)).toBe(false);
    for (const filename of declarations.rejectedFilenames) {
      const invalid = structuredClone(app);
      invalid.actions[0]!.args.find(arg => arg.id === "filename")!.default = filename;
      expect(declaredMediaFileExportV1(invalid, EXPORT_MEDIA_FILE_ACTION_ID)).toBeNull();
    }
    expect(declaredMediaFileExportV1({ ...app, actions: [...app.actions, ...app.actions] }, EXPORT_MEDIA_FILE_ACTION_ID)).toEqual(declaration);
    const conflicting = structuredClone(app.actions[0]!);
    conflicting.args.find(arg => arg.id === "outputPort")!.default = "other:out";
    expect(declaredMediaFileExportV1({ ...app, actions: [...app.actions, conflicting] }, EXPORT_MEDIA_FILE_ACTION_ID)).toBeNull();
    expect(declaredMediaFileExportV1({ ...app, io: { ...app.io!, ports: [{ ...app.io!.ports[0]!, direction: "in" }] } }, EXPORT_MEDIA_FILE_ACTION_ID)).toBeNull();
    expect(declaredMediaFileExportV1({ ...app, io: { ...app.io!, ports: [] } }, EXPORT_MEDIA_FILE_ACTION_ID)).toBeNull();
  });
  it("resolves mesh and current snapshot actions independently on the same app", () => {
    expect(EXPORT_SNAPSHOT_FILE_ACTION_ID).toBe(declarations.snapshot.actionId);
    const rows = [[EXPORT_MEDIA_FILE_ACTION_ID, declarations], [EXPORT_SNAPSHOT_FILE_ACTION_ID, declarations.snapshot]] as const;
    const app = { actions: rows.map(([id, row]) => ({ id, args: Object.entries(row.defaults).map(([id, value]) => ({ id, default: value })) })), windowKinds: [], io: { ports: rows.map(([, row]) => row.port) } } as unknown as AppDefinition;
    for (const [id, row] of rows) {
      const result = declaredMediaFileExportV1(app, id);
      expect(result).toEqual({ ...row.defaults, portMediaType: row.port.mediaType });
      expect(mediaFileDescriptorMatchesV1(row.descriptor, result!)).toBe(true);
    }
    expect(declaredMediaFileExportV1({ ...app, actions: [app.actions[0]!] }, EXPORT_SNAPSHOT_FILE_ACTION_ID)).toBeNull();
  });
  for (const row of declarations.deliveryCases) it(`delivery ${row.id}`, async () => {
    const declaration = { ...declarations.defaults, portMediaType: { class: "threeD" as const, form: "mesh" as const } };
    const identity = { app_instance_id: fixture.instanceId, parent_document_id: fixture.documentId, base_revision: BigInt(fixture.revision) };
    const handle: MediaExportHandle = { ...identity, operation_id: 9n, generation: BigInt(fixture.generation) };
    const abort = new AbortController(), sink: string[] = [], names: unknown[][] = [], written: Uint8Array[] = [];
    let reads = 0, pages = 0, current = true;
    const descriptor = row.id === "wrong-format" ? { ...declarations.descriptor, wire: { kind: "binary", format_kind: "png" } } : declarations.descriptor;
    const port = {
      readAppDocumentIdentity: async () => {
        reads += 1;
        return { ...identity, base_revision: identity.base_revision + (row.id === "source-changed-after-drain" && reads >= 2 || row.id === "source-changed-before-delivery" && reads >= 3 ? 1n : 0n) };
      },
      submitMediaExport: async () => handle,
      pollMediaExport: async (): Promise<MediaExportStatus> => ({ handle, state: "complete", applied_progress: 4n, checkpoint_available: false, mime_type: fixture.mediaType, total_bytes: 4n, detail: "", descriptor: [...encodePackValue(descriptor)] }),
      cancelMediaExport: async () => {},
      takeMediaExportChunk: async () => ({ handle, data: Uint8Array.from(fixture.chunks[pages++] ?? []), terminal: pages > fixture.chunks.length }),
    };
    let delivered = false, failure: unknown;
    try {
      delivered = await downloadDeclaredMediaFileV1({ instanceId: fixture.instanceId, declaration, port, signal: abort.signal, current: () => current, progress: () => {}, yieldTurn: async () => {}, sinkFactory: async (filename, mediaType) => {
        names.push([filename, mediaType]); sink.push("open");
        return {
          write: async chunk => { sink.push("write"); written.push(chunk.slice()); if (row.id === "cancelled-during-copy") abort.abort(); if (row.id === "owner-retired-during-copy") current = false; },
          close: async () => { sink.push("close"); },
          abort: async () => { sink.push("abort"); },
        };
      } });
    } catch (error) { failure = error; }
    expect(delivered).toBe(row.delivered);
    expect(failure instanceof Error ? failure.message : null).toBe(row.error);
    expect(sink).toEqual(row.sink);
    for (const name of names) expect(name).toEqual([declarations.defaults.filename, declarations.defaults.mediaType]);
    if (delivered) expect([...Buffer.concat(written)]).toEqual(fixture.chunks.flat());
  });
  for (const row of fixture.cases) it(row.id, async () => {
    const handle: MediaExportHandle = { app_instance_id: fixture.instanceId, parent_document_id: fixture.documentId, base_revision: BigInt(fixture.revision), generation: BigInt(fixture.generation), operation_id: 9n };
    const calls: string[] = [];
    const transitions: boolean[] = [], authorities: MediaExportHandle[] = [], submissions: unknown[][] = [];
    let polls = 0, pages = 0, clock = 0, cancelled = row.fault === "cancel-before-submit";
    const model = createActor(createMachine({ initial: "ready", states: {
      ready: { on: { submit: "running" } },
      running: { on: { poll: "running", take: "draining", cancel: "retired" } },
      draining: { on: { take: "draining", cancel: "retired" } },
      retired: { type: "final" },
    } }));
    model.start();
    const record = (event: string): void => { transitions.push(model.getSnapshot().can({ type: event })); model.send({ type: event }); calls.push(event); };
    const port: MediaTransportPort = {
      submitMediaExport: async (instanceId, outputPort, documentId, revision) => {
        record("submit");
        submissions.push([instanceId, outputPort, documentId, revision]);
        return row.fault === "foreign-submit" ? { ...handle, parent_document_id: "foreign" } : handle;
      },
      pollMediaExport: async (_instanceId, authority) => {
        record("poll"); authorities.push(authority); polls += 1;
        if (row.fault === "poll-fault") throw new Error("fixture-fault");
        if (row.fault === "cancel-running") cancelled = true;
        if (row.fault === "deadline") clock = MEDIA_EXPORT_OPERATION_POLICY.maximumDurationMs;
        const status: MediaExportStatus = { handle: row.fault === "foreign-status" ? { ...handle, operation_id: 10n } : handle, state: polls === 1 || row.fault === "progress-regression" ? "running" : "complete", applied_progress: row.fault === "progress-regression" && polls === 2 ? 0n : BigInt(polls), checkpoint_available: false, mime_type: fixture.mediaType, total_bytes: row.fault === "wrong-length" ? 5n : 4n, detail: "", descriptor: row.fault === "descriptor-empty" ? [] : row.fault === "descriptor-too-large" ? Array(MEDIA_EXPORT_OPERATION_POLICY.maximumDescriptorBytes + 1).fill(0) : [1, 2] };
        return status;
      },
      cancelMediaExport: async (_instanceId, authority) => { record("cancel"); authorities.push(authority); },
      takeMediaExportChunk: async (_instanceId, authority) => {
        record("take"); authorities.push(authority);
        if (row.fault === "take-fault") throw new Error("fixture-take-fault");
        const data = Uint8Array.from(fixture.chunks[pages++] ?? []);
        return { handle: row.fault === "foreign-chunk" ? { ...handle, generation: 72n } : handle, data, terminal: pages > fixture.chunks.length };
      },
    };
    let result: MediaExportBytes | null | undefined, failure: unknown;
    try {
      result = await collectMediaExportBytes({ instanceId: fixture.instanceId, parentDocumentId: fixture.documentId, revision: handle.base_revision, outputPort: fixture.outputPort, mediaType: fixture.mediaType, port, cancelled: () => cancelled, progress: () => {}, yieldTurn: async () => {}, now: () => clock });
    } catch (error) { failure = error; }
    model.stop();
    const outcome = failure === undefined ? result === null ? "cancelled" : "complete" : "refused";
    if (result != null) {
      expect([...Buffer.concat(result.chunks)]).toEqual(fixture.chunks.flat());
      expect(result.descriptor).toEqual([1, 2]);
    }
    expect(failure instanceof Error ? failure.message : null).toBe(row.error);
    expect(transitions.every(Boolean)).toBe(true);
    for (const authority of authorities) expect(authority).toEqual(handle);
    for (const submission of submissions) expect(submission).toEqual([fixture.instanceId, fixture.outputPort, fixture.documentId, handle.base_revision]);
    expect(calls).toEqual(row.calls);
    expect(outcome).toBe(row.outcome);
  });
});
