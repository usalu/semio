/** 🧪️ Bun execution of the neutral live artifact ownership and cancellation contract. */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import Ajv2020 from "ajv/dist/2020";
import { encodeAppFrame, decodeAppFrame } from "@semio-tech/framework/app-channel";
import { shellAppFrameToJson } from "@semio-tech/framework/mcp/channel";
import type { ActionDispatchLifecycleV1 } from "@semio-tech/framework";
import type { InputOutcomeV1 } from "../../🎯️input-ledger/🟦️.ts";
import { createAgentArtifactRouteV1, decodeAgentDeferredPlanV1 } from "../🟦️.ts";
import { declaredProgramCommandV1, ownedCallerViewContextV1, invokeProgramInputV1, programInputResultV1, settledProgramOutputV1 } from "../../📨️program-invocation/🟦️.ts";
import type { ShellInputActionV1 } from "../../🎯️input-ledger/🟦️.ts";
import { startedTypedOperationV1, typedOperationCancellationActionV1 } from "../../🏁️operation-settle/🟦️.ts";
import { OS_MCP_SCHEMA_DOCUMENT, osMcpViolations, parseInvocationReport } from "../../../../../../🌉️mcp/🧬️schema/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧪️fixtures/🔣️.json", import.meta.url), "utf8"));

const harness = (dispatch: (action: unknown, origin?: unknown, propagate?: boolean, lifecycle?: ActionDispatchLifecycleV1) => Promise<InputOutcomeV1>, timing?: { now: () => number; yieldTurn: (delayMs: number) => Promise<void> }) => {
  const app = { id: "note", controllerId: "note-controller", commands: [{ id: "refresh" }], modes: [{ id: "review", commands: [{ id: "accept" }] }], actions: [{ id: "appendParagraph" }, { id: "append.paragraph" }], windowKinds: [{ id: "editor", bodyKey: "editor" }, { id: "details", bodyKey: "details", actions: [{ id: "inspectRow" }] }] };
  const ports = {
    agentDeferredPlansRef: { current: new Map<string, string>() },
    sessionRef: { current: { pluginId: "host", instanceId: 1, app: { ...app, id: "host", controllerId: "host-controller" } } },
    spawnedAppsRef: { current: [{ id: "note-a", pluginId: "note", appId: "note", instanceId: 2 }, { id: "note-b", pluginId: "note", appId: "note", instanceId: 3 }] },
    loadedPluginsRef: { current: [{ manifest: { apps: [app], commands: [{ id: "sync" }] }, handle: { pluginId: "note", readHistory: async () => ({ cursor: 8, upserts: [{ seq: 8, actionId: "edit-8" }] }) } }] },
    onActionRef: { current: dispatch },
    activeWindowIdRef: { current: "note-b::editor" },
  };
  const route = createAgentArtifactRouteV1(ports as unknown as Parameters<typeof createAgentArtifactRouteV1>[0], timing);
  const send = (instanceId: string, command: unknown) => route({ seq: 1n, instanceId, command } as Parameters<typeof route>[0]);
  return { send, ports };
};

describe("live artifact neutral contract", () => {
  test("matches AJV on refreshed public result envelopes including null and false", () => {
    const doc = OS_MCP_SCHEMA_DOCUMENT;
    const ajv = new Ajv({ strict: true, allErrors: true }).addSchema(doc);
    const report = { invocationId: "invoke-output", capabilityId: "note.note.cmd.refresh", status: "SUCCEEDED", affectedResources: [], postconditions: [], replayed: false, warnings: [], untrusted: null };
    const check = (name: string, value: unknown, valid: boolean) => {
      expect(Boolean(ajv.getSchema(`${doc.$id}#/$defs/${name}`)!(value))).toBe(valid);
      expect(osMcpViolations(name, value).length === 0).toBe(valid);
    };
    check("InvocationReport", report, true);
    expect(parseInvocationReport(report).untrusted).toBeNull();
    const envelope = (doc.$defs.InvocationReport as any).properties.untrusted.anyOf[0];
    const output = { schema: "semio.mcp.untrusted-content/v1", notice: envelope.properties.notice.const, content: false, provenance: { source: "capability-output", artifactId: "document-a", artifactKind: null, spaceId: null, authors: { kind: "unattributed" }, revision: { contentSha256: "a".repeat(64), headEditId: "edit-8", commitSeq: null } } };
    check("InvocationReport", { ...report, untrusted: output }, true);
    check("InvocationReport", { ...report, untrusted: { ...output, content: { value: 7 } } }, true);
    check("InvocationReport", { ...report, untrusted: false }, false);
    check("InvocationReport", { ...report, untrusted: undefined }, false);
    const prepared = { preparedHandle: "prep-output", capabilityId: report.capabilityId, expiresAtMs: 100, preview: { opsCount: { document: 0, config: 0, draft: 0, childOpBytes: 0 }, warnings: [], untrusted: output } };
    check("PreparedActionReport", prepared, true);
    check("PreparedActionReport", { ...prepared, preview: { ...prepared.preview, untrusted: null } }, true);
    check("PreparedActionReport", { ...prepared, preview: { ...prepared.preview, extra: "untrusted escape" } }, false);
  });
  test("matches the shared Rust command result corpus and schema without trusting its text", () => {
    const rows = JSON.parse(readFileSync(new URL("../../../../../../🌉️mcp/🐚️channel/🧪️tests/🧫️fixtures/📦️command-results/🔣️.json", import.meta.url), "utf8"));
    const schema = JSON.parse(readFileSync(new URL("../../../../../../../../../🔨️modules/🤖️mcp/🐚️channel/🧬️schema/🔣️.json", import.meta.url), "utf8"));
    const oracle = new Ajv().compile(schema);
    for (const row of rows) {
      expect(oracle(row.payload)).toBe(true);
      const frame = row.kind === "emit" ? { ...row.payload, ops: { document: [], config: [], draft: [] } } : row.payload;
      const { version, ...reply } = frame;
      expect(shellAppFrameToJson(reply)).toEqual(row.payload);
    }
    const applied = { kind: "applied" as const, inputSeq: 1 };
    expect(programInputResultV1(applied, rows[0].payload.output)).toEqual({ ...applied, output: rows[0].payload.output });
    expect(programInputResultV1(applied, null)).toEqual({ ...applied, output: null });
    expect(programInputResultV1(applied, undefined)).toBe(applied);
    const refused = { kind: "superseded" as const, inputSeq: 1, by: 2 };
    expect(programInputResultV1(refused, rows[0].payload.output)).toBe(refused);
    const admission = { operationId: "19", generation: "7" };
    expect(settledProgramOutputV1(admission, true, { terminalOutput: rows[0].payload.output })).toEqual(rows[0].payload.output);
    expect(settledProgramOutputV1(admission, true, { terminalOutput: null })).toBeNull();
    expect(settledProgramOutputV1(admission, true, null)).toBeUndefined();
    expect(settledProgramOutputV1(rows[0].payload.output, false, null)).toEqual(rows[0].payload.output);
  });
  test("preserves actual committed guest output without executing preparation", async () => {
    const output = { status: "guest-readonly", nested: { value: 7 }, text: "untrusted guest text" };
    let calls = 0;
    const { send } = harness(async () => { calls++; return { kind: "applied", inputSeq: 1, output }; });
    const preview = (await send("2", { kind: "pureCommand", capabilityId: fixture.commandCases[0].id, input: {} }))[0];
    if (preview?.kind !== "emit") throw new Error("command must prepare");
    expect(calls).toBe(0);
    expect("output" in preview).toBe(false);
    await send("2", { kind: "transactionPrepare", txnId: "guest-output", ops: preview.ops });
    expect(calls).toBe(0);
    const committed = (await send("2", { kind: "transactionCommit", txnId: "guest-output" }))[0];
    expect(calls).toBe(1);
    expect(committed && shellAppFrameToJson(committed)).toEqual({ version: 1, kind: "transactionCommitted", txnId: "guest-output", editId: "edit-8", output });
  });
  test("prepares and commits declared app, mode and plugin commands with their exact owner", async () => {
    const actions: any[] = [];
    const { send } = harness(async action => { actions.push(action); return { kind: "applied", inputSeq: 1 }; });
    for (const row of fixture.commandCases) {
      const frame = (await send("2", { kind: "pureCommand", capabilityId: row.id, input: { value: 7 } }))[0];
      expect(frame?.kind).toBe(row.kind === "error" ? "error" : "emit");
      if (frame?.kind !== "emit") continue;
      expect(new Ajv().compile(fixture.planSchema)(JSON.parse(new TextDecoder().decode(frame.ops.document[0])))).toBe(true);
      expect((await send("3", { kind: "transactionPrepare", txnId: row.id, ops: frame.ops }))[0]?.kind).toBe("error");
      expect((await send("2", { kind: "transactionPrepare", txnId: row.id, ops: frame.ops }))[0]?.kind).toBe("transactionPrepared");
      expect((await send("2", { kind: "transactionCommit", txnId: row.id }))[0]?.kind).toBe("transactionCommitted");
      expect(actions.at(-1)?.commandAddress).toEqual(row.address);
      expect(actions.at(-1)?.provenance.windowId).toBe("note-a::editor");
    }
    expect(actions).toHaveLength(3);
  });

  test("uses the actual typed command port with causal authority and unchanged response", async () => {
    const calls: unknown[] = [];
    const response = { historyPatch: { cursor: 9 }, requestedEffects: [{ kind: "refresh" }], output: { operation: 19 } };
    const plugin = { handleAction: async () => { throw new Error("command must never enter action port"); }, handleCommand: async (...args: unknown[]) => { calls.push(args); return response; } };
    for (const row of fixture.commandCases.filter((row: { kind: string }) => row.kind !== "error")) {
      const input: ShellInputActionV1 = { controllerId: "note-controller", action: `command:${row.address.commandId}`, args: { value: 7 }, commandAddress: row.address };
      const owner = { pluginId: "note", app: { id: "note", commands: [{ id: "refresh" }], modes: [{ id: "review", commands: [{ id: "accept" }] }] }, pluginCommands: [{ id: "sync" }] };
      expect(declaredProgramCommandV1(input, owner)?.id).toBe(row.address.commandId);
      expect(declaredProgramCommandV1(input, { ...owner, pluginId: "foreign" })).toBeNull();
      expect(await invokeProgramInputV1(plugin, 2, input, () => { throw new Error("no action encoding"); }, { locale: "de" }, { order: 11 })).toBe(response);
      expect(calls.at(-1)).toEqual([2, JSON.stringify({ address: row.address, arguments: { value: 7 } }), { locale: "de" }, { order: 11 }]);
    }
  });

  test("cancels a typed command through the action cancellation port", async () => {
    const started = startedTypedOperationV1({ operationId: "19", generation: "7" });
    if (!started) throw new Error("typed command must admit an operation");
    const original: ShellInputActionV1 = { controllerId: "note-controller", action: "command:refresh", commandAddress: fixture.commandCases[0].address, args: { value: 7 }, provenance: { origin: "agent", windowId: "note-a::editor", causedBy: null } };
    const cancellation = typedOperationCancellationActionV1(original, started);
    expect(cancellation.commandAddress).toBeUndefined();
    const calls: unknown[] = [];
    await invokeProgramInputV1({ handleAction: async (...args: unknown[]) => { calls.push(args); }, handleCommand: async () => { throw new Error("cancellation is an action"); } }, 2, cancellation, () => "cancel-action-wire", { locale: "en" }, { order: 11 });
    expect(calls).toEqual([[2, "cancel-action-wire", { locale: "en" }, { order: 11 }]]);
    expect(cancellation.provenance).toEqual(original.provenance);
  });
  test("preserves descriptor bytes in the shared Rust and TypeScript wire corpus", () => {
    const url = new URL("../../../../../../📡️spr/🧵️channel/🧪️tests/🧫️fixtures/🎬️media-export-wire/🔣️.json", import.meta.url);
    const wire = JSON.parse(readFileSync(url, "utf8"));
    const schema = JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json", url), "utf8"));
    expect(new Ajv2020().compile(schema)(wire)).toBe(true);
    const row = wire.frames.status.fields;
    const handle = { app_instance_id: row.handle.appInstanceId, parent_document_id: row.handle.parentDocumentId, operation_id: BigInt(row.handle.operationId), base_revision: BigInt(row.handle.baseRevision), generation: BigInt(row.handle.generation) };
    const frame = { MediaExportStatus: { in_reply_to: BigInt(row.inReplyTo), handle, state: "complete" as const, applied_progress: BigInt(row.appliedProgress), checkpoint_available: row.checkpointAvailable, mime_type: row.mimeType, total_bytes: BigInt(row.totalBytes), detail: row.detail, descriptor: row.descriptor } };
    const bytes = encodeAppFrame(frame);
    expect(Array.from(bytes)).toEqual(wire.frames.status.bytes);
    expect(Buffer.from(bytes).toString("hex")).toBe(wire.frames.status.hex);
    expect(decodeAppFrame(bytes)).toEqual(frame);
  });

  test("matches AJV on the plan corpus", () => {
    const ownedSchema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
    expect(ownedSchema.$defs.DeferredPlan).toEqual(fixture.planSchema);
    const oracle = new Ajv().compile(fixture.planSchema);
    for (const row of fixture.planCases) {
      expect(Boolean(oracle(row.value))).toBe(row.valid);
      expect(decodeAgentDeferredPlanV1(JSON.stringify(row.value)) !== null).toBe(row.valid);
    }
    expect(decodeAgentDeferredPlanV1("malformed")).toBeNull();
  });

  test("cancels an admitted command through its actual lifecycle control", async () => {
    let finish: (() => void) | undefined;
    let cancelled = 0;
    const { send } = harness(async (_action, _origin, _propagate, lifecycle) => {
      lifecycle?.started({ operationId: "command-19", generation: "7", cancel: () => { cancelled++; finish?.(); } });
      await new Promise<void>(resolve => { finish = resolve; });
      return { kind: "applied", inputSeq: 1 };
    });
    const emitted = (await send("2", { kind: "pureCommand", capabilityId: fixture.commandCases[0].id, input: {} }))[0];
    if (emitted?.kind !== "emit") throw new Error("command must prepare");
    await send("2", { kind: "transactionPrepare", txnId: "cancel-command", ops: emitted.ops });
    const executing = send("2", { kind: "transactionCommit", txnId: "cancel-command" });
    expect((await send("3", { kind: "cancel", cancelSeq: 1n }))[0]?.kind).toBe("error");
    expect(cancelled).toBe(0);
    await send("2", { kind: "cancel", cancelSeq: 1n });
    expect(cancelled).toBe(1);
    expect((await executing)[0]?.kind).toBe("error");
  });

  test("binds full capability ownership and preserves dotted declared action ids", async () => {
    const { send } = harness(async () => ({ kind: "applied", inputSeq: 1 }));
    for (const row of fixture.capabilityCases) {
      const frame = (await send("2", { kind: "pureCommand", capabilityId: row.id, input: {} }))[0];
      expect(frame?.kind).toBe(row.kind);
      if (frame?.kind === "emit") {
        const plan = decodeAgentDeferredPlanV1(new TextDecoder().decode(frame.ops.document[0]));
        expect(plan?.deferredAction).toBe(row.action);
        expect(plan?.windowKindId).toBe(row.windowKindId);
      }
    }
  });

  test("dispatches a window-scoped published action through its declared pane", async () => {
    const actions: unknown[] = [];
    const { send, ports } = harness(async action => { actions.push(action); return { kind: "applied", inputSeq: 1 }; });
    ports.activeWindowIdRef.current = "note-a::editor";
    const frame = (await send("2", { kind: "pureCommand", capabilityId: "note.note.inspectRow", input: {} }))[0];
    if (frame?.kind !== "emit") throw new Error("window action must prepare");
    expect((await send("2", { kind: "transactionPrepare", txnId: "window-action", ops: frame.ops }))[0]?.kind).toBe("transactionPrepared");
    expect((await send("2", { kind: "transactionCommit", txnId: "window-action" }))[0]?.kind).toBe("transactionCommitted");
    expect(actions).toEqual([{ controllerId: "note-controller", action: "inspectRow", args: {}, provenance: { origin: "agent", windowId: "note-a::details", causedBy: null } }]);
  });

  test("commits the addressed editor once despite foreign preparation, commit and rollback", async () => {
    const actions: unknown[] = [];
    const { send } = harness(async action => { actions.push(action); return { kind: "applied", inputSeq: 1 }; });
    let ops: unknown;
    for (const step of fixture.journey) {
      const command = step.kind === "pureCommand" ? { kind: step.kind, capabilityId: "note.note.appendParagraph", input: { text: "hello" } }
        : step.kind === "transactionPrepare" ? { kind: step.kind, txnId: "shared", ops, origin: { kind: "agent", principal: "agent:local", invocationId: "one" } }
        : { kind: step.kind, txnId: "shared" };
      const frames = await send(step.instance, command);
      expect(frames[0]?.kind).toBe(step.expect);
      if (frames[0]?.kind === "emit") ops = frames[0].ops;
    }
    expect(actions).toEqual([{ controllerId: "note-controller", action: "appendParagraph", args: { text: "hello" }, provenance: { origin: "agent", windowId: "note-a::editor", causedBy: null } }]);
  });

  test("rejects every invalid schema plan before preparing", async () => {
    let calls = 0;
    const { send } = harness(async () => { calls++; return { kind: "applied", inputSeq: 1 }; });
    for (const row of fixture.planCases.filter((row: { valid: boolean }) => !row.valid)) {
      const ops = { document: [new TextEncoder().encode(JSON.stringify(row.value))], config: [], draft: [] };
      expect((await send("2", { kind: "transactionPrepare", txnId: "invalid", ops }))[0]?.kind).toBe("error");
    }
    expect((await send("2", { kind: "transactionCommit", txnId: "invalid" }))[0]?.kind).toBe("error");
    expect(calls).toBe(0);
  });

  for (const beforeAdmission of [true, false]) test(`cancels an owned operation ${beforeAdmission ? "before" : "after"} admission`, async () => {
    const cancellation = fixture.cancellation;
    let lifecycle: ActionDispatchLifecycleV1 | undefined;
    let settle: (() => void) | undefined;
    let cancellations = 0;
    const cancel = () => { cancellations++; settle?.(); };
    const { send, ports } = harness(async (_action, _origin, _propagate, observer) => {
      lifecycle = observer;
      if (!beforeAdmission) lifecycle?.started({ operationId: cancellation.operationId, generation: cancellation.generation, cancel });
      await new Promise<void>(resolve => { settle = resolve; });
      return { kind: "applied", inputSeq: 1 };
    });
    const pending = send(cancellation.instance, { kind: "transactionUndo", groupId: "edit-8" });
    expect((await send(cancellation.instance, { kind: "transactionUndo", groupId: "edit-8" }))[0]?.kind).toBe("error");
    await send(cancellation.foreignInstance, { kind: "cancel", cancelSeq: BigInt(cancellation.seq) });
    expect(cancellations).toBe(0);
    ports.spawnedAppsRef.current = [];
    await send(cancellation.instance, { kind: "cancel", cancelSeq: BigInt(cancellation.seq) });
    if (beforeAdmission) lifecycle?.started({ operationId: cancellation.operationId, generation: cancellation.generation, cancel });
    expect(cancellations).toBe(cancellation.cancelCalls);
    expect((await pending)[0]?.kind).toBe(cancellation.terminal);
    await send(cancellation.instance, { kind: "cancel", cancelSeq: BigInt(cancellation.seq) });
    expect(cancellations).toBe(cancellation.cancelCalls);
  });

  test("exports bounded owner chunks and preserves the actual guest descriptor", async () => {
    const row = fixture.export;
    const handle = { ...row.handle, base_revision: BigInt(row.handle.base_revision), operation_id: BigInt(row.handle.operation_id), generation: BigInt(row.handle.generation) };
    const { send, ports } = harness(async () => ({ kind: "applied", inputSeq: 1 }));
    const submissions: unknown[] = [];
    let page = 0;
    Object.assign(ports.loadedPluginsRef.current[0]!.handle, {
      readAppDocumentIdentity: async () => ({ ...row.identity, base_revision: BigInt(row.identity.base_revision) }),
      submitMediaExport: async (...args: unknown[]) => { submissions.push(args); return handle; },
      pollMediaExport: async () => ({ handle, state: "complete", applied_progress: 3n, checkpoint_available: false, mime_type: row.mimeType, total_bytes: 3n, detail: "", descriptor: row.descriptor }),
      cancelMediaExport: async () => { throw new Error("successful export must not cancel"); },
      takeMediaExportChunk: async () => ({ handle, data: Uint8Array.from(row.chunks[page] ?? []), terminal: ++page === row.chunks.length }),
    });
    const frames = await send("2", { kind: "exportMedia", port: row.port, document: new Uint8Array(), documentSpr: new Uint8Array() });
    expect(submissions).toEqual([[2, row.port, row.identity.parent_document_id, BigInt(row.identity.base_revision)]]);
    const exported = frames[0];
    expect(exported?.kind).toBe("exported");
    if (exported?.kind !== "exported") throw new Error("bounded export must produce an exported frame");
    const actual = { descriptor: Array.from(exported.descriptor), data: Array.from(exported.data) };
    expect(new Ajv().compile(row.schema)(actual)).toBe(true);
    expect(actual).toEqual({ descriptor: row.descriptor, data: row.expected });
  });

  test("cancels a real export handle when cancellation arrives before submission settles", async () => {
    const row = fixture.export;
    const handle = { ...row.handle, base_revision: BigInt(row.handle.base_revision), operation_id: BigInt(row.handle.operation_id), generation: BigInt(row.handle.generation) };
    const { send, ports } = harness(async () => ({ kind: "applied", inputSeq: 1 }));
    let admit: ((value: typeof handle) => void) | undefined;
    let submitted: (() => void) | undefined;
    const submission = new Promise<void>(resolve => { submitted = resolve; });
    const cancelled: unknown[] = [];
    Object.assign(ports.loadedPluginsRef.current[0]!.handle, {
      readAppDocumentIdentity: async () => ({ ...row.identity, base_revision: BigInt(row.identity.base_revision) }),
      submitMediaExport: async () => { submitted?.(); return await new Promise<typeof handle>(resolve => { admit = resolve; }); },
      pollMediaExport: async () => ({ handle, state: "cancelled", applied_progress: 0n, checkpoint_available: false, mime_type: "", total_bytes: 0n, detail: "", descriptor: [] }),
      cancelMediaExport: async (...args: unknown[]) => { cancelled.push(args); },
      takeMediaExportChunk: async () => { throw new Error("cancelled export must not collect bytes"); },
    });
    const pending = send("2", { kind: "exportMedia", port: row.port, document: new Uint8Array(), documentSpr: new Uint8Array() });
    await submission;
    await send("3", { kind: "cancel", cancelSeq: 1n });
    expect(cancelled).toEqual([]);
    await send("2", { kind: "cancel", cancelSeq: 1n });
    admit?.(handle);
    expect((await pending)[0]?.kind).toBe("error");
    expect(cancelled).toEqual([[2, handle]]);
  });

  test("cancels an admitted running export through its exact port handle", async () => {
    const row = fixture.export;
    const handle = { ...row.handle, base_revision: BigInt(row.handle.base_revision), operation_id: BigInt(row.handle.operation_id), generation: BigInt(row.handle.generation) };
    const { send, ports } = harness(async () => ({ kind: "applied", inputSeq: 1 }));
    let entered: (() => void) | undefined;
    let release: (() => void) | undefined;
    const polling = new Promise<void>(resolve => { entered = resolve; });
    const cancelled: unknown[] = [];
    let polls = 0;
    Object.assign(ports.loadedPluginsRef.current[0]!.handle, {
      readAppDocumentIdentity: async () => ({ ...row.identity, base_revision: BigInt(row.identity.base_revision) }),
      submitMediaExport: async () => handle,
      pollMediaExport: async () => {
        if (++polls === 1) { entered?.(); await new Promise<void>(resolve => { release = resolve; }); }
        return { handle, state: polls === 1 ? "running" : "cancelled", applied_progress: 1n, checkpoint_available: false, mime_type: "", total_bytes: 0n, detail: "", descriptor: [] };
      },
      cancelMediaExport: async (...args: unknown[]) => { cancelled.push(args); release?.(); },
      takeMediaExportChunk: async () => { throw new Error("cancelled operation must not collect"); },
    });
    const pending = send("2", { kind: "exportMedia", port: row.port, document: new Uint8Array(), documentSpr: new Uint8Array() });
    await polling;
    await send("2", { kind: "cancel", cancelSeq: 1n });
    expect((await pending)[0]?.kind).toBe("error");
    expect(cancelled).toEqual([[2, handle]]);
    expect(polls).toBe(2);
  });

  test("refuses every foreign export authority and cancels only its own admitted generation", async () => {
    const row = fixture.export;
    const handle = { ...row.handle, base_revision: BigInt(row.handle.base_revision), operation_id: BigInt(row.handle.operation_id), generation: BigInt(row.handle.generation) };
    for (const field of ["app_instance_id", "parent_document_id", "operation_id", "base_revision", "generation"]) {
      const { send, ports } = harness(async () => ({ kind: "applied", inputSeq: 1 }));
      const value = handle[field], foreign = { ...handle, [field]: typeof value === "bigint" ? value + 1n : typeof value === "number" ? value + 1 : "foreign" };
      const cancelled: unknown[] = [];
      Object.assign(ports.loadedPluginsRef.current[0]!.handle, {
        readAppDocumentIdentity: async () => ({ ...row.identity, base_revision: BigInt(row.identity.base_revision) }),
        submitMediaExport: async () => handle,
        pollMediaExport: async () => ({ handle: foreign, state: "complete", applied_progress: 3n, checkpoint_available: false, mime_type: row.mimeType, total_bytes: 3n, detail: "", descriptor: row.descriptor }),
        cancelMediaExport: async (...args: unknown[]) => { cancelled.push(args); },
        takeMediaExportChunk: async () => { throw new Error("foreign completion must not collect bytes"); },
      });
      const frames = await send("2", { kind: "exportMedia", port: row.port, document: new Uint8Array(), documentSpr: new Uint8Array() });
      expect(frames[0]?.kind).toBe("error");
      expect(cancelled).toEqual([[2, handle]]);
    }
  });

  for (const cancelLate of [false, true]) test(`keeps explicit export time policy past the output page count${cancelLate ? " until cancellation" : " until completion"}`, async () => {
    const row = fixture.export;
    const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
    const validate = new Ajv().compile(schema.$defs.ExportOperationPolicy);
    expect(validate(row.operationPolicy)).toBe(true);
    expect(validate({ ...row.operationPolicy, maximumDurationMs: 0 })).toBe(false);
    const handle = { ...row.handle, base_revision: BigInt(row.handle.base_revision), operation_id: BigInt(row.handle.operation_id), generation: BigInt(row.handle.generation) };
    let now = 0, polls = 0, cancels = 0;
    const timing = { now: () => now, yieldTurn: async (delayMs: number) => {
      now += delayMs;
      if (cancelLate && polls === row.longRunningPolls) await send("2", { kind: "cancel", cancelSeq: 1n });
    } };
    const { send, ports } = harness(async () => ({ kind: "applied", inputSeq: 1 }), timing);
    Object.assign(ports.loadedPluginsRef.current[0]!.handle, {
      readAppDocumentIdentity: async () => ({ ...row.identity, base_revision: BigInt(row.identity.base_revision) }),
      submitMediaExport: async () => handle,
      pollMediaExport: async () => ({ handle, state: cancels ? "cancelled" : ++polls > row.longRunningPolls ? "complete" : "running", applied_progress: BigInt(polls), checkpoint_available: false, mime_type: row.mimeType, total_bytes: 0n, detail: "", descriptor: row.descriptor }),
      cancelMediaExport: async () => { cancels++; },
      takeMediaExportChunk: async () => ({ handle, data: new Uint8Array(), terminal: true }),
    });
    const frames = await send("2", { kind: "exportMedia", port: row.port, document: new Uint8Array(), documentSpr: new Uint8Array() });
    expect(frames[0]?.kind).toBe(cancelLate ? "error" : "exported");
    expect(polls).toBeGreaterThan(8192);
    expect(cancels).toBe(cancelLate ? 1 : 0);
    expect(now).toBe(row.longRunningPolls * row.operationPolicy.pollIntervalMs);
  });

  test("enforces the explicit export deadline and cancels the real admitted handle", async () => {
    const row = fixture.export;
    const handle = { ...row.handle, base_revision: BigInt(row.handle.base_revision), operation_id: BigInt(row.handle.operation_id), generation: BigInt(row.handle.generation) };
    let now = 0, polls = 0;
    const cancelled: unknown[] = [];
    const { send, ports } = harness(async () => ({ kind: "applied", inputSeq: 1 }), { now: () => now, yieldTurn: async () => { now = row.operationPolicy.maximumDurationMs; } });
    Object.assign(ports.loadedPluginsRef.current[0]!.handle, {
      readAppDocumentIdentity: async () => ({ ...row.identity, base_revision: BigInt(row.identity.base_revision) }),
      submitMediaExport: async () => handle,
      pollMediaExport: async () => { polls++; return { handle, state: "running", applied_progress: BigInt(polls), checkpoint_available: false, mime_type: "", total_bytes: 0n, detail: "", descriptor: [] }; },
      cancelMediaExport: async (...args: unknown[]) => { cancelled.push(args); },
      takeMediaExportChunk: async () => { throw new Error("expired operation must not collect bytes"); },
    });
    const frames = await send("2", { kind: "exportMedia", port: row.port, document: new Uint8Array(), documentSpr: new Uint8Array() });
    expect(frames[0]).toEqual(expect.objectContaining({ kind: "error", code: "budget.exceeded" }));
    expect(cancelled).toEqual([[2, handle]]);
    expect(polls).toBe(1);
  });

  test("keeps descriptor metadata capacity separate from output chunk capacity", async () => {
    const row = fixture.export;
    const handle = { ...row.handle, base_revision: BigInt(row.handle.base_revision), operation_id: BigInt(row.handle.operation_id), generation: BigInt(row.handle.generation) };
    const descriptor = Array.from({ length: row.largeDescriptorBytes }, (_, index) => index % 256);
    const { send, ports } = harness(async () => ({ kind: "applied", inputSeq: 1 }));
    Object.assign(ports.loadedPluginsRef.current[0]!.handle, {
      readAppDocumentIdentity: async () => ({ ...row.identity, base_revision: BigInt(row.identity.base_revision) }),
      submitMediaExport: async () => handle,
      pollMediaExport: async () => ({ handle, state: "complete", applied_progress: 0n, checkpoint_available: false, mime_type: row.mimeType, total_bytes: 0n, detail: "", descriptor }),
      cancelMediaExport: async () => { throw new Error("valid metadata must not cancel"); },
      takeMediaExportChunk: async () => ({ handle, data: new Uint8Array(), terminal: true }),
    });
    const frame = (await send("2", { kind: "exportMedia", port: row.port, document: new Uint8Array(), documentSpr: new Uint8Array() }))[0];
    expect(frame?.kind).toBe("exported");
    expect(frame?.kind === "exported" && Array.from(frame.descriptor)).toEqual(descriptor);
  });

  test("cancels completed native output ownership while chunk collection is in progress", async () => {
    const row = fixture.export;
    const handle = { ...row.handle, base_revision: BigInt(row.handle.base_revision), operation_id: BigInt(row.handle.operation_id), generation: BigInt(row.handle.generation) };
    let cancelled = false, reads = 0;
    const cancellations: unknown[] = [];
    const { send, ports } = harness(async () => ({ kind: "applied", inputSeq: 1 }), { now: () => 0, yieldTurn: async () => { await send("2", { kind: "cancel", cancelSeq: 1n }); } });
    Object.assign(ports.loadedPluginsRef.current[0]!.handle, {
      readAppDocumentIdentity: async () => ({ ...row.identity, base_revision: BigInt(row.identity.base_revision) }),
      submitMediaExport: async () => handle,
      pollMediaExport: async () => ({ handle, state: "complete", applied_progress: 3n, checkpoint_available: false, mime_type: row.mimeType, total_bytes: 3n, detail: "", descriptor: row.descriptor }),
      cancelMediaExport: async (...args: unknown[]) => { cancellations.push(args); cancelled = true; },
      takeMediaExportChunk: async () => { reads++; if (cancelled) throw new Error("native completed output was retired by exact-handle cancellation"); return { handle, data: Uint8Array.from(row.chunks[0]), terminal: false }; },
    });
    const frame = (await send("2", { kind: "exportMedia", port: row.port, document: new Uint8Array(), documentSpr: new Uint8Array() }))[0];
    expect(frame).toEqual(expect.objectContaining({ kind: "error", code: "budget.exceeded" }));
    expect(cancellations).toEqual([[2, handle]]);
    expect(reads).toBe(2);
  });
});

test("retains exact explicit caller context through deferred preparation and committed dispatch", async () => {
  const seen: ShellInputActionV1[] = [];
  const {send}=harness(async action => {seen.push(action as ShellInputActionV1);return {kind:"applied"} as InputOutcomeV1;});
  const context={locale:"de",terminology:"reuse",windowId:"note-a::editor"} as const;
  const prepared=await send("2",{kind:"pureCommand",capabilityId:"note.note.cmd.refresh",input:{},context});
  expect(prepared[0].kind).toBe("emit");
  if(prepared[0].kind!=="emit")throw new Error("expected emit");
  const plan=JSON.parse(new TextDecoder().decode(prepared[0].ops.document[0]));
  expect(plan.context).toEqual(context);
  expect(seen).toHaveLength(0);
  await send("2",{kind:"transactionPrepare",txnId:"context",ops:prepared[0].ops,origin:{kind:"agent",principal:"agent:test",invocationId:"ctx"}});
  await send("2",{kind:"transactionCommit",txnId:"context"});
  expect(seen[0].callerViewState).toEqual(context);
  expect(seen[0].provenance?.windowId).toBe("note-a::editor");
  const foreign=await send("2",{kind:"pureCommand",capabilityId:"note.note.cmd.refresh",input:{},context:{...context,windowId:"note-b::editor"}});
  expect(foreign[0].kind).toBe("error");
  const foreignMode=await send("2",{kind:"pureCommand",capabilityId:"note.note.cmd.refresh",input:{},context:{...context,activeModeId:"foreign"}});
  expect(foreignMode[0].kind).toBe("error");
});

test("projects explicit caller windows into genuine retained guest surface authority",()=>{
 const actual={locale:"en",terminology:"native",activeModeId:"review",windowId:"editor",activeWindowKindId:"editor",windowInstances:[{id:"editor",windowKindId:"editor"}],sessionIdentity:{userId:"user-a",displayName:"A"}} as const;
 const caller={locale:"de",terminology:"reuse",activeModeId:"review",windowId:"note-a::editor",windowInstances:[{id:"note-a::editor",windowKindId:"editor"}]} as const;
 const projected=ownedCallerViewContextV1(caller,actual,"note-a",["note-a","note-b"]);
 expect(projected).toEqual({...actual,...caller,windowId:"editor",windowInstances:actual.windowInstances});
 expect(ownedCallerViewContextV1({...caller,windowId:"note-b::editor"},actual,"note-a",["note-a","note-b"])).toBeNull();
 expect(ownedCallerViewContextV1({...caller,activeModeId:"other"},actual,"note-a",["note-a","note-b"])).toBeNull();
 expect(ownedCallerViewContextV1({...caller,sessionIdentity:{userId:"user-b",displayName:"B"}},actual,"note-a",["note-a","note-b"])).toBeNull();
});
