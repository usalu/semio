/** 📚️ Source values exceed scalar labels without losing UTF-8 bytes or their parent action owner. */
import { describe, expect, it } from "vitest";
import Ajv2020 from "ajv/dist/2020.js";
import { createHash } from "node:crypto";
import { textEditorSceneFromLanes } from "@semio-tech/framework";
import answer from "../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🧪️tests/🧫️fixtures/📝️answer.json";
import answerSchema from "../../../../../../../../../../✏️s/🔌️plugins/📋️forms/🗿️artifacts/📋️forms/🧪️tests/🧫️fixtures/📝️answer.schema.json";
import schema from "../../🧬️schema/📄️value-source/🔣️.json";
import fixture from "../🧫️fixtures/📄️value-source/🔣️.json";
import boundarySchema from "../🧫️fixtures/📄️value-source/encoding-boundary.schema.json";
import sourceRecipeSchema from "../🧫️fixtures/📄️value-source/source-recipe.schema.json";
import { admitTextEditorSourceDraftV1, changeTextEditorSourceDraftV1, parseTextEditorValueSourceV1, textEditorValueSourceActionV1, textEditorValueSourceEditIdentityV1 } from "../../📄️value-source/🟦️.ts";

const longAnswer = answer.answer.unit.repeat(answer.answer.repeat);

describe("text editor value source", () => {
  it("scopes only semantic edit identity and keeps full baseline dispatch arguments", () => {
    const settings = { ...fixture.settings, commit: "change" as const, editIdentityArguments: { key: "question", windowId: "first" }, editArguments: { key: "question", windowId: "first", baseline: ["old"] } };
    const identity = textEditorValueSourceEditIdentityV1("child", "source", settings);
    expect(textEditorValueSourceEditIdentityV1("child", "source", { ...settings, label: "Flow-Dokument", editArguments: { ...settings.editArguments, baseline: ["new"] } })).toBe(identity);
    for (const changed of [{ ...settings, editControllerId: "other" }, { ...settings, editAction: "other" }, { ...settings, editArgument: "other" }, { ...settings, editIdentityArguments: { key: "question", windowId: "second" } }]) expect(textEditorValueSourceEditIdentityV1("child", "source", changed)).not.toBe(identity);
    const result = textEditorValueSourceActionV1(settings, { controllerId: "child", action: "textEdit", args: { text: fixture.source } });
    if (result.kind !== "dispatch") throw new Error("Expected source action");
    expect(result.action.args).toMatchObject({ baseline: ["old"], key: "question", windowId: "first" });
    expect(changeTextEditorSourceDraftV1(null, fixture.source, fixture.source, false, true)).toEqual({ text: fixture.source, refused: false, admitted: null });
  });

  it("retains refused local corrections until their own admission or a revert", () => {
    const law = fixture.encodingBoundaries, accepted = law.unit.repeat(law.repeat), refused = accepted + "x";
    let state = changeTextEditorSourceDraftV1(null, refused, fixture.source, true);
    expect(state).toEqual({ text: refused, refused: true, admitted: null });
    expect(admitTextEditorSourceDraftV1(state, accepted)).toBe(state);
    state = changeTextEditorSourceDraftV1(state, accepted, fixture.source, false);
    expect(state?.refused).toBe(true);
    state = admitTextEditorSourceDraftV1(state, accepted);
    expect(state).toEqual({ text: accepted, refused: false, admitted: accepted });
    state = changeTextEditorSourceDraftV1(state, refused, fixture.source, true);
    expect(admitTextEditorSourceDraftV1(state, accepted)).toBe(state);
    expect(changeTextEditorSourceDraftV1(state, fixture.source, fixture.source, false)).toBeNull();
  });

  it("agrees with the exact shared encoded command boundary", () => {
    const law = fixture.encodingBoundaries;
    expect(new Ajv2020({ strict: true }).validate(boundarySchema, law)).toBe(true);
    const settings = { ...fixture.settings, commit: "change" as const, editEncoding: "json-chunks" as const, editArgument: "valueJsonChunks", editMaximumBytes: law.maximumBytes };
    for (const row of law.cases) {
      const source = law.unit.repeat(law.repeat) + row.suffix, encoded = JSON.stringify(source);
      expect(Buffer.byteLength(encoded)).toBe(row.encodedBytes);
      expect(new TextEncoder().encode(encoded).byteLength <= law.maximumBytes).toBe(row.accepted);
      const commit = () => textEditorValueSourceActionV1(settings, { controllerId: "child", action: "textEdit", args: { text: source } });
      expect(new Ajv2020({ strict: true }).validate(schema.$defs.CommitResult, commit())).toBe(true);
      if (!row.accepted) { expect(commit()).toEqual({ kind: "refused", reason: "source-too-large" }); continue; }
      const chunks = ((commit() as { kind: "dispatch"; action: { args: unknown } }).action.args as { valueJsonChunks: string[] }).valueJsonChunks;
      expect(chunks).toHaveLength(Math.ceil(row.encodedBytes / 512));
      expect(chunks.every(chunk => Buffer.byteLength(chunk) <= 512)).toBe(true);
      expect(Buffer.concat(chunks.map(chunk => Buffer.from(chunk))).toString("utf8")).toBe(encoded);
      expect(JSON.parse(chunks.join(""))).toBe(source);
    }
  });

  it("encodes a near-bound escaped Unicode graph with the caller-owned byte budget", () => {
    for (const row of fixture.largeSources) {
      expect(new Ajv2020({ strict: true }).validate(sourceRecipeSchema, row.source)).toBe(true);
      const source = row.source.prefix + row.source.unit.repeat(row.source.repeat) + row.source.suffix;
      expect(createHash("sha256").update(source).digest("hex")).toBe(row.sourceSha256);
      expect(createHash("sha256").update(JSON.stringify(source)).digest("hex")).toBe(row.encodedSha256);
      expect(Buffer.byteLength(source)).toBe(row.rawBytes);
      expect(row.rawBytes).toBeLessThanOrEqual(16_384);
      expect(Buffer.byteLength(JSON.stringify(source))).toBe(row.encodedBytes);
      expect(row.encodedBytes).toBeGreaterThan(16_384);
      const settings = { ...fixture.settings, commit: "change" as const, editEncoding: "json-chunks" as const, editArgument: "valueJsonChunks", editMaximumBytes: row.maximumBytes };
      expect(parseTextEditorValueSourceV1(JSON.stringify(settings))).toEqual(settings);
      const result = textEditorValueSourceActionV1(settings, { controllerId: "child", action: "textEdit", args: { text: source } });
      expect(result.kind).toBe("dispatch");
      if (result.kind !== "dispatch") throw new Error("Expected valid source admission");
      const action = result.action;
      const chunks = (action.args as { valueJsonChunks: string[] }).valueJsonChunks;
      for (const chunk of chunks) expect(Buffer.byteLength(chunk)).toBeLessThanOrEqual(512);
      expect(JSON.parse(Buffer.concat(chunks.map(chunk => Buffer.from(chunk))).toString("utf8"))).toBe(source);
      expect(textEditorValueSourceActionV1({ ...settings, editMaximumBytes: undefined }, { controllerId: "child", action: "textEdit", args: { text: source } })).toEqual({ kind: "refused", reason: "source-too-large" });
    }
  });

  it("chunks a shared long answer as exact JSON for one Fill window", () => {
    expect(new Ajv2020({ strict: true }).validate(answerSchema, answer)).toBe(true);
    expect(Buffer.byteLength(longAnswer)).toBe(answer.answer.bytes);
    expect(createHash("sha256").update(longAnswer).digest("hex")).toBe(answer.answer.sha256);
    expect(Buffer.byteLength(longAnswer)).toBeGreaterThan(4096);
    for (const windowId of answer.windowIds) {
      const settings = { ...fixture.settings, commit: "change" as const, editAction: "setTryValue", editArgument: "valueJsonChunks", editEncoding: "json-chunks" as const, editArguments: { key: "long-answer", windowId, windowKindId: "forms-try" } };
      expect(parseTextEditorValueSourceV1(JSON.stringify(settings))).toEqual(settings);
      const result = textEditorValueSourceActionV1(settings, { controllerId: "child", action: "textEdit", args: { text: longAnswer } });
      if (result.kind !== "dispatch") throw new Error("Expected answer admission");
      const action = result.action;
      const args = action.args as { valueJsonChunks: string[]; windowId: string };
      expect(args.windowId).toBe(windowId);
      for (const chunk of args.valueJsonChunks) expect(Buffer.byteLength(chunk)).toBeLessThanOrEqual(512);
      expect(JSON.parse(Buffer.concat(args.valueJsonChunks.map(chunk => Buffer.from(chunk))).toString("utf8"))).toBe(longAnswer);
    }
  });

  it("agrees with the independent JSON Schema oracle", () => {
    const validate = new Ajv2020({ strict: true }).compile(schema);
    expect(parseTextEditorValueSourceV1(JSON.stringify({ readOnly: true, label: "Long Answer" }))).toEqual({ readOnly: true, label: "Long Answer" });
    expect(textEditorValueSourceActionV1({ readOnly: true, label: "Long Answer" }, { controllerId: "child", action: "textEdit", args: { text: longAnswer } })).toEqual({ kind: "local" });
    for (const value of [fixture.settings, ...fixture.invalid]) expect(parseTextEditorValueSourceV1(JSON.stringify(value)) !== null).toBe(Boolean(validate(value)));
  });

  it("reassembles bounded Unicode chunks through the existing scene buffer lane", () => {
    for (const chunk of fixture.chunks) expect(Buffer.byteLength(chunk)).toBeLessThanOrEqual(fixture.scalarMaximumBytes);
    expect(fixture.bytes).toBeGreaterThan(600);
    const expected = Buffer.concat(fixture.chunks.map(chunk => Buffer.from(chunk))).toString("utf8");
    const scene = textEditorSceneFromLanes({ buffer: "", lanes: [], language: "json" }, new Map([["framework.scene.text.buffer", fixture.chunks.join("")]]));
    expect(scene.buffer).toBe(expected);
    expect(scene.buffer).toBe(fixture.source);
    expect(new TextEncoder().encode(scene.buffer)).toEqual(new Uint8Array(Buffer.from(fixture.source)));
    expect(Buffer.byteLength(scene.buffer)).toBe(fixture.bytes);
  });

  it("delivers the complete source to the exact parent and preserves static arguments", () => {
    const settings = parseTextEditorValueSourceV1(JSON.stringify(fixture.settings))!;
    const before = structuredClone(settings);
    const action = textEditorValueSourceActionV1(settings, { controllerId: "contributor", action: "textEdit", args: { text: fixture.source, surfaceId: "source" } });
    expect(action).toEqual({ kind: "dispatch", action: { controllerId: fixture.settings.editControllerId, action: fixture.settings.editAction, args: { ...fixture.settings.editArguments, value: fixture.source } } });
    expect(settings).toEqual(before);
    expect(textEditorValueSourceActionV1(settings, { controllerId: "child", action: "textSelect", args: {} })).toEqual({ kind: "local" });
    expect(textEditorValueSourceActionV1({ ...settings, readOnly: true }, { controllerId: "child", action: "textEdit", args: { text: fixture.source } })).toEqual({ kind: "local" });
    expect(textEditorValueSourceActionV1(settings, { controllerId: "child", action: "undo" })).toMatchObject({ kind: "dispatch", action: { controllerId: fixture.settings.editControllerId, action: "undo" } });
  });
});
