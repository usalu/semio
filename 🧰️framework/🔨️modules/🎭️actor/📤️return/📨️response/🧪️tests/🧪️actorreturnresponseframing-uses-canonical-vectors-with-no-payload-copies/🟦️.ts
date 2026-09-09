type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, testSource: TestSource): Promise<void> {
  const { ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES, ActorReturnResponseFraming, createActorBytePage, decodeActorReturnResponse, encodeActorReturnResponse, fault, readActorReturnResponseHeader, uint } = dependencies;

  const { it, expect, vi } = vitest;
  const hydrate = (value: unknown): any => JSON.parse(JSON.stringify(value), (key, item) => ["activationGeneration", "returnSequence", "pageSequence"].includes(key) ? BigInt(item) : item);
  const backing = (hex: string): ArrayBuffer => Uint8Array.from(Buffer.from(hex, "hex")).buffer;
  const oracle = async () => {
    const name = "@webassemblyjs/leb128/lib/leb.js"; const lib = await import(name); const encode = (lib.default ?? lib).encodeUIntBuffer;
    return (value: number | bigint): Buffer => { const bytes = Buffer.alloc(8); bytes.writeBigUInt64LE(BigInt(value)); return Buffer.from(encode(bytes)); };
  };
  it("ActorReturnResponseFraming uses canonical vectors with no payload copies or backing escape", async () => {
    const api = await import("../../🟦️.ts");
    const { default: schema } = await import("../../🌿️framing/🧬️schema/🔣️.json"); const { default: framing } = await import("../../🌿️framing/🧫️fixtures/🔣️.json");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json"); const { default: returned } = await import("../../../🧫️fixtures/🔣️.json");
    const { default: returnedSchema } = await import("../../../🧬️schema/🔣️.json"); const { default: value } = await import("../../../../../🌱️value/🧬️schema/🔣️.json"); const { default: lifetime } = await import("../../../../🚪️lifetime/🧬️schema/🔣️.json"); const { default: page } = await import("../../../../📃️page/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv"); const uint = await oracle();
    const ajv = new Ajv({ strict: true }).addSchema(value).addSchema(lifetime).addSchema(page).addSchema(returnedSchema).addSchema(schema);
    expect(ajv.validate(schema, framing)).toBe(true);
    const cases = fixture.vectors.map(row => ({ bytes: Buffer.from(row.hex, "hex"), expected: hydrate(row.value) }));
    for (const row of returned.pageResultVectors) {
      const origin = row.receipt.identity.origin;
      const header = Buffer.concat([Buffer.from(fixture.magicHex, "hex"), Buffer.of(0), uint(BigInt(origin.activationGeneration)), uint(origin.requestSequence)]);
      const payload = Buffer.alloc(4096); for (let index = 0; index < row.pageLength; index++) payload[index] = (index * 37 + 11) % 256;
      const prefix = Buffer.from(row.prefixHex, "hex"); const result = { kind: "page", receipt: row.receipt, payloadOffset: header.length + prefix.length };
      expect(ajv.validate({ $ref: schema.$id + "#/definitions/pageProjection" }, result)).toBe(true);
      cases.push({ bytes: Buffer.concat([header, prefix, payload]), expected: hydrate({ kind: "result", activationGeneration: origin.activationGeneration, transportRequestSequence: origin.requestSequence, result }) });
    }
    for (const row of cases) {
      const parser = new api.ActorReturnResponseFraming(); let allocations = 0; let header = null; let value;
      expect(parser.header).toBeNull(); expect(parser.value).toBeNull();
      try {
        for (const name of ["ArrayBuffer", "Uint8Array", "BigUint64Array"] as const) vi.stubGlobal(name, new Proxy(globalThis[name], { construct(target, args) { allocations++; return Reflect.construct(target, args); } }));
        for (const byte of row.bytes) { parser.push(byte); if (parser.header !== null) { if (header !== null && header !== parser.header) throw new Error("Framing replaced stable header"); header = parser.header; } }
        value = parser.finish();
      } finally { vi.unstubAllGlobals(); }
      expect(allocations).toBe(framing.maximumPayloadCopies); expect(value).toEqual(row.expected); expect(parser.finish()).toBe(value); expect(parser.value).toBe(value);
      expect(Object.isFrozen(value)).toBe(true); expect(Object.isFrozen(header)).toBe(true); expect(Object.keys(parser)).toEqual([]);
      const { kind, activationGeneration, transportRequestSequence } = row.expected;
      expect(header).toEqual({ kind, activationGeneration, transportRequestSequence });
      if (value.kind === "result" && value.result.kind === "page") {
        expect(Object.keys(value.result).sort()).toEqual(["kind", "payloadOffset", "receipt"]);
        expect(row.bytes.length - value.result.payloadOffset).toBe(4096); expect(Object.isFrozen(value.result)).toBe(true);
      }
    }
  });
  it("ActorReturnResponseFraming keeps malformed bodies and incomplete authority failed", async () => {
    const api = await import("../../🟦️.ts"); const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const { default: returned } = await import("../../../🧫️fixtures/🔣️.json");
    const reject = (bytes: readonly number[]) => {
      const parser = new api.ActorReturnResponseFraming();
      expect(() => { for (const byte of bytes) parser.push(byte); parser.finish(); }).toThrow();
      expect(parser.value).toBeNull(); expect(() => parser.finish()).toThrow(); expect(() => parser.push(0)).toThrow();
    };
    for (const row of fixture.vectors) {
      const bytes = Buffer.from(row.hex, "hex");
      for (let length = 0; length < bytes.length; length++) reject([...bytes.subarray(0, length)]);
      reject([...bytes, 0]);
    }
    for (const hex of fixture.invalidHex) reject([...Buffer.from(hex, "hex")]);
    for (const value of [-1, 256, 1.5, NaN, Infinity]) reject([value]);
    const emptyPage = Buffer.concat([Buffer.from(fixture.vectors[0]!.hex.slice(0, 14) + returned.pageResultVectors[0]!.prefixHex, "hex"), Buffer.alloc(4096)]);
    emptyPage[emptyPage.length - 1] = 1; reject([...emptyPage]);
    const finished = new api.ActorReturnResponseFraming(); for (const byte of Buffer.from(fixture.vectors[0]!.hex, "hex")) finished.push(byte);
    finished.finish(); expect(() => finished.push(0)).toThrow(); expect(finished.value).toBeNull(); expect(() => finished.finish()).toThrow();
    for (const row of fixture.headerCases) {
      const parser = new api.ActorReturnResponseFraming(); const bytes = Buffer.from(row.hex, "hex"); let offset = 0;
      while (parser.header === null && offset < bytes.length) parser.push(bytes[offset++]!);
      expect(parser.header).toEqual(hydrate(row.header)); expect(offset).toBeLessThanOrEqual(23); expect(parser.value).toBeNull();
      expect(() => { while (offset < bytes.length) parser.push(bytes[offset++]!); parser.finish(); }).toThrow(); expect(parser.value).toBeNull();
    }
  });
  it("ActorReturnResponseFraming actual module has no strict TypeScript diagnostics", async () => {
    const { default: ts } = await import("typescript"); const { fileURLToPath } = await import("node:url"); const path = fileURLToPath(testSource.url);
    const program = ts.createProgram([path], { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.ESNext, moduleResolution: ts.ModuleResolutionKind.Bundler, strict: true, noEmit: true, skipLibCheck: true, allowImportingTsExtensions: true, resolveJsonModule: true, esModuleInterop: true, types: ["node", "vitest/importMeta"] });
    const source = program.getSourceFile(path); expect(source).toBeDefined();
    expect([...program.getSyntacticDiagnostics(source), ...program.getSemanticDiagnostics(source)].map(item => ts.flattenDiagnosticMessageText(item.messageText, "\n"))).toEqual([]);
  });
  it("ActorWorkerInboxInventory binds mixed message kinds and logical shells to current source", async () => {
    const { default: schema } = await import("../../🎟️credit/📋️metadata/📥️inbox/🧬️schema/🔣️.json"); const { default: fixture } = await import("../../🎟️credit/📋️metadata/📥️inbox/🧫️fixtures/🔣️.json");
    const { default: Ajv } = await import("ajv"); const { default: ts } = await import("typescript"); const { readFileSync } = await import("node:fs");
    const validate = new Ajv({ strict: true }).compile(schema); expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const shardPath = new URL("../../📮️shard-client/🟦️.ts", testSource.url);
    const shard = ts.createSourceFile(shardPath.pathname, readFileSync(shardPath, "utf8"), ts.ScriptTarget.Latest, true);
    const totals = new Map<string, { bytes: bigint; slots: bigint; owners: bigint }>();
    for (const row of fixture.layouts) {
      const declaration = shard.statements.find(node => (ts.isClassDeclaration(node) || ts.isTypeAliasDeclaration(node)) && node.name?.text === row.declaration);
      const members = declaration && ts.isClassDeclaration(declaration) ? declaration.members.filter(ts.isPropertyDeclaration) : declaration && ts.isTypeAliasDeclaration(declaration) && ts.isTypeLiteralNode(declaration.type) ? declaration.type.members : null;
      if (!members) throw new Error("Missing inbox source " + row.declaration);
      const fields = members.map(member => member.name?.getText(shard).replace(/^#/, "")); expect(fields, row.declaration).toEqual(row.fields);
      const previous = totals.get(row.group) ?? { bytes: 0n, slots: 0n, owners: 0n };
      totals.set(row.group, { bytes: previous.bytes + BigInt(fixture.model.recordBytes) + BigInt(fields.length) * BigInt(fixture.model.fieldBytes), slots: previous.slots + 1n, owners: previous.owners + 1n });
    }
    for (const [group, expected] of Object.entries(fixture.minimumShells)) expect(Object.fromEntries(Object.entries(totals.get(group)!).map(([axis, count]) => [axis, Number(count)]))).toEqual(expected);
    const variants = (name: string) => {
      const declaration = shard.statements.find(node => ts.isTypeAliasDeclaration(node) && node.name.text === name);
      if (!declaration || !ts.isTypeAliasDeclaration(declaration) || !ts.isUnionTypeNode(declaration.type)) throw new Error("Missing inbox union " + name);
      return declaration.type.types.map(type => {
        if (!ts.isTypeLiteralNode(type)) throw new Error("Nonliteral inbox variant");
        const properties = type.members.filter(ts.isPropertySignature); const kind = properties.find(item => item.name.getText(shard) === "kind")?.type; const ok = properties.find(item => item.name.getText(shard) === "ok")?.type;
        if (!kind || !ts.isLiteralTypeNode(kind) || !ts.isStringLiteral(kind.literal)) throw new Error("Missing inbox tag");
        return { kind: kind.literal.text, ok: ok && ts.isLiteralTypeNode(ok) ? ok.literal.kind === ts.SyntaxKind.TrueKeyword : null, fields: properties.map(item => item.name.getText(shard)) };
      });
    };
    expect(variants("OutboundMessage")).toEqual(fixture.outbound); expect(variants("InboundMessage")).toEqual(fixture.inbound);
    const producerPath = new URL("../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts", testSource.url);
    const producer = ts.createSourceFile(producerPath.pathname, readFileSync(producerPath, "utf8"), ts.ScriptTarget.Latest, true);
    const producers = await import("../../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
    // 🫀️ Inventoried from the EMITTED bytes, not the template's raw text: `shardWorkerSource` now
    // interpolates the schema-owned liveness policy, so its return is a template EXPRESSION and its
    // literal text no longer is the worker. The AST check still fences the shape (one returned
    // template, nothing else), the inventory reads what the browser actually gets.
    const generated = (name: "shardWorkerSource" | "hostShimSource") => {
      const declaration = producer.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === name);
      const returned = declaration && ts.isFunctionDeclaration(declaration) ? declaration.body?.statements.find(ts.isReturnStatement)?.expression : null;
      if (!returned || !(ts.isNoSubstitutionTemplateLiteral(returned) || ts.isTemplateExpression(returned))) throw new Error("Changed generated inbox source " + name);
      return ts.createSourceFile(name + ".js", producers[name](), ts.ScriptTarget.Latest, true, ts.ScriptKind.JS);
    };
    const worker = generated("shardWorkerSource"); const shim = generated("hostShimSource"); const requests: string[] = []; const replies: string[] = []; const effects: string[] = []; const effectReplies: string[] = [];
    const visitWorker = (node: import("typescript").Node): void => {
      if (ts.isFunctionDeclaration(node) && node.name?.text === "deliverEffectResult") {
        const visitReply = (child: import("typescript").Node): void => { if (ts.isBinaryExpression(child) && child.left.getText(worker) === "kind" && ts.isStringLiteral(child.right)) effectReplies.push(child.right.text); ts.forEachChild(child, visitReply); };
        ts.forEachChild(node, visitReply); return;
      }
      if (ts.isBinaryExpression(node) && node.left.getText(worker) === "kind" && ts.isStringLiteral(node.right) && !requests.includes(node.right.text)) requests.push(node.right.text);
      if (ts.isSwitchStatement(node) && node.expression.getText(worker) === "kind") for (const clause of node.caseBlock.clauses) if (ts.isCaseClause(clause) && ts.isStringLiteral(clause.expression) && !requests.includes(clause.expression.text)) requests.push(clause.expression.text);
      if (ts.isCallExpression(node) && node.expression.getText(worker) === "self.postMessage" && node.arguments[0] && ts.isObjectLiteralExpression(node.arguments[0])) {
        const tag = node.arguments[0].properties.find(item => ts.isPropertyAssignment(item) && item.name.getText(worker) === "kind");
        if (!tag || !ts.isPropertyAssignment(tag) || !ts.isStringLiteral(tag.initializer)) throw new Error("Uninventoried worker post"); replies.push(tag.initializer.text);
      }
      ts.forEachChild(node, visitWorker);
    };
    const visitShim = (node: import("typescript").Node): void => {
      if (ts.isCallExpression(node) && ["call", "effectRequest"].includes(node.expression.getText(shim)) && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) effects.push(node.arguments[0].text);
      ts.forEachChild(node, visitShim);
    };
    visitWorker(worker); visitShim(shim); expect(requests).toEqual(fixture.workerRequestKinds); expect(replies).toEqual(fixture.workerReplyKinds); expect(effects).toEqual(fixture.hostEffectKinds); expect(effectReplies).toEqual(fixture.hostReplyKinds);
    const client = shard.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "ShardClient");
    const handler = client && ts.isClassDeclaration(client) ? client.members.find(node => ts.isMethodDeclaration(node) && node.name.getText(shard) === "handleMessage")?.getText(shard) : null;
    expect(handler).toBeTruthy(); expect(handler!.indexOf("message.kind")).toBeLessThan(handler!.indexOf("captureResponse(message)")); expect(handler!.indexOf("this.pending.get(message.requestId)")).toBeLessThan(handler!.indexOf("captureResponse(message)"));
    expect(worker.text.includes("msg.returnDrive")).toBe(false);
  });
  it("ActorWorkerInboxInventory executes generated heartbeat, ordinary reply and awaited effect traffic together", async () => {
    const { default: fixture } = await import("../../🎟️credit/📋️metadata/📥️inbox/🧫️fixtures/🔣️.json"); const { default: ts } = await import("typescript"); const { readFileSync } = await import("node:fs"); const vm = await import("node:vm");
    const path = new URL("../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts", testSource.url);
    const source = ts.createSourceFile(path.pathname, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true);
    const producers = await import("../../../../../../🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts");
    const generated = (name: "shardWorkerSource" | "hostShimSource") => {
      const declaration = source.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === name);
      const returned = declaration && ts.isFunctionDeclaration(declaration) ? declaration.body?.statements.find(ts.isReturnStatement)?.expression : null;
      if (!returned || !(ts.isNoSubstitutionTemplateLiteral(returned) || ts.isTemplateExpression(returned))) throw new Error("Changed generated inbox source " + name); return producers[name]();
    };
    const messages: any[] = []; const receive = (message: unknown) => messages.push(message);
    const shim = vm.createContext({ exports: {}, URL, self: { postMessage: receive } });
    const shimCode = generated("hostShimSource").replace("import.meta.url", JSON.stringify("https://fixture.invalid/host.js?actor=a&activation=1"));
    new vm.Script(ts.transpileModule(shimCode, { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.CommonJS } }).outputText).runInContext(shim);
    let dispatch: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
    const checkpoint = Object.freeze({ ordinary: "checkpoint" }); const effectResult = Object.freeze({ effect: "completed" });
    const context = vm.createContext({ WebAssembly: { Suspending: class {}, promising: (value: unknown) => value }, self: { postMessage: receive, addEventListener: (kind: string, callback: typeof dispatch) => { if (kind === "message") dispatch = callback; } }, apiA: { poll: () => shim.exports.storageRead({ key: "awaited" }), resolveEffect: shim.exports.__resolveEffect, rejectEffect: shim.exports.__rejectEffect }, apiB: { checkpoint: async () => checkpoint } });
    new vm.Script(generated("shardWorkerSource")).runInContext(context);
    new vm.Script('actors.set("a", { api: apiA, activationGeneration: 1n, pendingAssets: [] }); actors.set("b", { api: apiB, activationGeneration: 2n, pendingAssets: [] });').runInContext(context);
    if (!dispatch) throw new Error("Missing generated worker dispatcher");
    const send = dispatch as (event: { data: Record<string, unknown> }) => Promise<void>;
    const pending = send({ data: { kind: "turn", requestId: "r1", actorId: "a", activationGeneration: 1n, events: [], budget: {} } });
    expect(messages.map(message => message.kind)).toEqual(["heartbeat", "frame"]);
    const effectRequest = messages[1].frame.envelope.payload.payload; expect(effectRequest.effect).toBe("storage-read");
    await send({ data: { kind: "checkpoint", requestId: "r2", actorId: "b" } });
    expect(messages.find(message => message.requestId === "r2")?.value).toBe(checkpoint); expect(messages.some(message => message.requestId === "r1")).toBe(false);
    shim.exports.emit({ notification: true }); shim.exports.emitPatch({ patch: true });
    await send({ data: { kind: "frame", actorId: "a", activationGeneration: 1n, frame: { kind: "Envelope", envelope: { to: "a", from: { kind: "kernel" }, payload: { kind: "effect-complete", payload: { requestId: effectRequest.requestId, value: effectResult } } } } } });
    await pending;
    const trace = messages.map(message => message.kind === "heartbeat" ? `heartbeat:${message.turnSeq}` : message.kind === "frame" ? `frame:${message.frame.envelope.payload.kind}` : `result:${message.requestId}:${message.ok ? "success" : "fault"}`);
    expect(trace).toEqual(fixture.mixedTrace); expect(messages.at(-1).value).toBe(effectResult);
    expect(messages.filter(message => message.kind === "frame").map(message => message.frame.envelope.payload.kind)).toEqual(fixture.hostFramePayloadKinds);
    await send({ data: { kind: "unrecognized", requestId: "r3", actorId: "b" } });
    expect(messages.at(-3)).toMatchObject({ kind: "heartbeat", turnSeq: 3 }); expect(messages.at(-2)).toMatchObject({ kind: "worker-fault", source: "handler", phase: "unrecognized", actorId: "b" }); expect(messages.at(-1)).toMatchObject({ kind: "result", requestId: "r3", ok: false });
    const traps: unknown[] = []; const failed = vm.createContext({ WebAssembly: {}, self: { postMessage: (message: unknown) => traps.push(message) } });
    expect(() => new vm.Script(generated("shardWorkerSource")).runInContext(failed)).toThrow(/JSPI/); expect(traps).toHaveLength(1); expect(traps[0]).toMatchObject({ kind: "trap", actorId: "*", activationGeneration: null });
    const postFault = new Error("post-after-observation"); const normalizationFault = Object.freeze({ normalization: "failed" });
    const guestFault = Object.defineProperty({}, "payload", { get() { throw normalizationFault; } });
    for (const mode of ["postThenThrow", "errorNormalizationThrows"] as const) {
      const captured: any[] = []; let callback: ((event: { data: Record<string, unknown> }) => Promise<void>) | null = null;
      const broken = vm.createContext({ WebAssembly: { Suspending: class {}, promising: (value: unknown) => value }, api: { checkpoint: async () => { if (mode === "errorNormalizationThrows") throw guestFault; return checkpoint; } }, self: { addEventListener: (kind: string, handler: typeof callback) => { if (kind === "message") callback = handler; }, postMessage: (message: any) => { captured.push(message); if (mode === "postThenThrow" && message.kind === "result" && message.ok) throw postFault; } } });
      new vm.Script(generated("shardWorkerSource")).runInContext(broken); new vm.Script('actors.set("a", { api, activationGeneration: 1n, pendingAssets: [] });').runInContext(broken);
      if (!callback) throw new Error("Missing fault-probe dispatcher");
      const run = callback as (event: { data: Record<string, unknown> }) => Promise<void>;
      const completed = run({ data: { kind: "checkpoint", requestId: "r4", actorId: "a" } });
      if (mode === "errorNormalizationThrows") await expect(completed).rejects.toBe(normalizationFault); else await completed;
      expect(captured.map(message => message.kind === "result" ? `result-${message.ok ? "success" : "fault"}` : message.kind)).toEqual(fixture.currentFaultTraces[mode]);
      if (mode === "postThenThrow") expect(captured.at(-1)).toMatchObject({ requestId: "r4", error: String(postFault), ok: false });
    }
    expect(fixture.currentFaultTraces.semanticallyAccepted).toBe(false);
  });
  it("ActorReturnResponseMetadata matches actual fixed source records and separately prices the projection graph", async () => {
    const { default: schema } = await import("../../🎟️credit/📋️metadata/🧬️schema/🔣️.json"); const { default: fixture } = await import("../../🎟️credit/📋️metadata/🧫️fixtures/🔣️.json");
    const { default: resident } = await import("../../../../../🌱️value/💾️resident/🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv"); const { default: ts } = await import("typescript"); const { readFileSync } = await import("node:fs");
    const ajv = new Ajv({ strict: true }).addSchema(resident); const validate = ajv.compile(schema); expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    const paths = { output: "../../🪪️activation/🚪️instance/📥️output/🟦️.ts", shard: "../../📮️shard-client/🟦️.ts", response: "./🟦️.ts", result: "../🟦️.ts" };
    const totals = new Map<string, { bytes: bigint; slots: bigint; owners: bigint }>();
    for (const layout of fixture.layouts) {
      const path = paths[layout.source as keyof typeof paths]; const parsed = ts.createSourceFile(path, readFileSync(new URL(path, testSource.url), "utf8"), ts.ScriptTarget.Latest, true);
      const declaration = parsed.statements.find(node => (ts.isClassDeclaration(node) || ts.isTypeAliasDeclaration(node)) && node.name?.text === layout.declaration);
      const members = declaration && ts.isClassDeclaration(declaration) ? declaration.members.filter(ts.isPropertyDeclaration) : declaration && ts.isTypeAliasDeclaration(declaration) && ts.isTypeLiteralNode(declaration.type) ? declaration.type.members : null;
      if (!members) throw new Error("Missing declared metadata source " + layout.declaration);
      const fields = members.map(member => member.name?.getText(parsed).replace(/^#/, ""));
      expect(fields, layout.declaration).toEqual(layout.fields);
      const previous = totals.get(layout.group) ?? { bytes: 0n, slots: 0n, owners: 0n };
      totals.set(layout.group, { bytes: previous.bytes + BigInt(fixture.model.recordBytes) + BigInt(fields.length) * BigInt(fixture.model.fieldBytes), slots: previous.slots + 1n, owners: previous.owners + 1n });
    }
    const graph = fixture.projectionMaximum; const framing = totals.get("framing")!;
    totals.set("framing", { bytes: framing.bytes + BigInt(graph.records * fixture.model.recordBytes + graph.fields * fixture.model.fieldBytes), slots: framing.slots + BigInt(graph.records), owners: framing.owners + BigInt(graph.records) });
    for (const [group, expected] of Object.entries(fixture.minimumEnvelopes)) expect(Object.fromEntries(Object.entries(totals.get(group)!).map(([axis, count]) => [axis, Number(count)]))).toEqual(expected);
    expect(fixture.boundaries.finalMountEnvelope).toBe(false); expect(fixture.boundaries.neutralIntrinsicRecordsIncluded).toBe(false); expect(fixture.boundaries.rawBytesIncluded).toBe(false);
    const { default: responses } = await import("../../🧫️fixtures/🔣️.json"); let maximumRecords = 0; let maximumFields = 0;
    for (const row of responses.vectors) {
      const bytes = Buffer.from(row.hex, "hex"); const parser = new ActorReturnResponseFraming(); const freeze = Object.freeze; const records: object[] = [];
      const spy = vi.spyOn(Object, "freeze").mockImplementation(value => { if (value !== null && typeof value === "object") records.push(value); return freeze(value); });
      try { for (const byte of bytes) parser.push(byte); parser.finish(); } finally { spy.mockRestore(); }
      maximumRecords = Math.max(maximumRecords, records.length); maximumFields = Math.max(maximumFields, records.reduce((count, value) => count + Object.keys(value).length, 0));
    }
    expect({ records: maximumRecords, fields: maximumFields }).toEqual(fixture.projectionMaximum);
  });
  it("ActorReturnResponse declaration matches strict schemas and independent envelope encoding", async () => {
    const { default: schema } = await import("../../🧬️schema/🔣️.json");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const fixtureSchema = schema;
    const { default: value } = await import("../../../../../🌱️value/🧬️schema/🔣️.json"); const { default: lifetime } = await import("../../../../🚪️lifetime/🧬️schema/🔣️.json");
    const { default: page } = await import("../../../../📃️page/🧬️schema/🔣️.json");
    const { default: returned } = await import("../../../🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv");
    const ajv = new Ajv({ strict: true }).addSchema(value).addSchema(lifetime).addSchema(page).addSchema(returned).addSchema(schema);
    expect(ajv.getSchema(`${schema.$id}#/$defs/ResponseFixture`)!(fixture)).toBe(true);
    const uint = await oracle();
    for (const row of fixture.vectors) {
      const value = row.value;
      const body = value.kind === "result" ? Buffer.from(row.resultHex, "hex") : Buffer.of(fixture.faultTags[value.fault as keyof typeof fixture.faultTags]);
      const encoded = Buffer.concat([Buffer.from(fixture.magicHex, "hex"), Buffer.of(fixture.tags[value.kind as keyof typeof fixture.tags]), uint(BigInt(value.activationGeneration)), uint(value.transportRequestSequence), body]);
      expect(encoded.toString("hex")).toBe(row.hex);
      expect(ajv.getSchema(`${schema.$id}#/$defs/Response`)!(value)).toBe(true);
    }
    expect(fixture.vectors[1]!.value.transportRequestSequence).not.toBe(fixture.vectors[1]!.value.result!.control!.receipt!.identity.origin.requestSequence);
    for (const value of [0, -1, 9007199254740992, 1.5]) expect(ajv.getSchema(`${schema.$id}#/$defs/Response`)!({ ...fixture.vectors[0]!.value, transportRequestSequence: value })).toBe(false);
  });
  it("ActorReturnResponseCredit declaration validates exact one-reply and retained-fault transitions", async () => {
    const { default: schema } = await import("../../🎟️credit/🧬️schema/🔣️.json");
    const { default: fixture } = await import("../../🎟️credit/🧫️fixtures/🔣️.json");
    const fixtureSchema = schema;
    const { default: response } = await import("../../🧬️schema/🔣️.json");
    const { default: value } = await import("../../../../../🌱️value/🧬️schema/🔣️.json"); const { default: lifetime } = await import("../../../../🚪️lifetime/🧬️schema/🔣️.json");
    const { default: page } = await import("../../../../📃️page/🧬️schema/🔣️.json");
    const { default: returned } = await import("../../../🧬️schema/🔣️.json");
    const { default: Ajv } = await import("ajv"); const { produce } = await import("immer");
    const ajv = new Ajv({ strict: true }).addSchema(value).addSchema(lifetime).addSchema(page).addSchema(returned).addSchema(response).addSchema(schema);
    const validate = ajv.getSchema(`${schema.$id}#/$defs/CreditFixture`)!; expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    for (const row of fixture.receiverCases) {
      let state = fixture.receiverInitial;
      for (const action of row.actions) {
        state = produce(state, next => {
          if (action === "post") next.phase = "posted";
          else if (action === "cancelEmpty") { next.phase = "cancelled"; next.rawBackingBytes = 0; next.pendingRetained = false; }
          else if (action === "capture") { next.phase = "captured"; next.captures++; next.rawRetained = true; }
          else if (action === "settle") next.pendingRetained = false;
          else {
            next.phase = "heldFault"; next.sealed = true; next.faultRetained = true;
            if (["headerFault", "foreignWorker", "foreignActivation", "foreignRequest", "duplicate"].includes(action)) next.emergencyRoots++;
          }
        });
        expect(ajv.validate({ $ref: schema.$id + "#/definitions/receiver" }, state), JSON.stringify(ajv.errors)).toBe(true);
      }
      expect(state, row.name).toEqual(row.expected);
    }
    for (const row of fixture.workerCases) {
      let state = fixture.workerInitial;
      for (const action of row.actions) {
        state = produce(state, next => {
          if (action === "capture") { next.phase = "captured"; next.rawRetained = true; }
          else if (action === "normalize") { next.phase = "ready"; next.backingRetained = true; }
          else if (action === "post") { next.phase = "posted"; next.transferCreditsSpent++; next.backingRetained = false; }
          else if (action !== "duplicate") { next.phase = "heldFault"; next.faultRetained = true; if (action === "detachedPostFault") { next.transferCreditsSpent++; next.backingRetained = false; } }
        });
        expect(ajv.validate({ $ref: schema.$id + "#/definitions/worker" }, state), JSON.stringify(ajv.errors)).toBe(true);
      }
      expect(state, row.name).toEqual(row.expected);
    }
    for (const delta of [{ captures: 2 }, { emergencyRoots: 2 }, { rawBackingBytes: 0 }, { rawRetained: true }, { pendingRetained: false }]) expect(ajv.validate({ $ref: schema.$id + "#/definitions/receiver" }, { ...fixture.receiverInitial, ...delta })).toBe(false);
    expect(ajv.validate({ $ref: schema.$id + "#/definitions/worker" }, { ...fixture.workerCases[0]!.expected, transferCreditsSpent: 2 })).toBe(false);
    expect(ajv.validate({ $ref: schema.$id + "#/definitions/worker" }, { ...fixture.workerCases[0]!.expected, backingRetained: true })).toBe(false);
    expect(fixture.authority.transportRequestSequence).not.toBe(fixture.returnOriginRequestSequence);
    expect(fixture.maximumBackingBytes).toBe(ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES);
  });
  it("ActorReturnResponse header routing preserves exact authority without decoding malformed bodies", async () => {
    const api = await import("../../🟦️.ts");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const uint = await oracle();
    for (const row of fixture.headerCases) {
      const raw = backing(row.hex); const expected = hydrate(row.header);
      const header = api.readActorReturnResponseHeader(raw);
      expect(header).toEqual(expected); expect(Object.isFrozen(header)).toBe(true);
      const prefix = Buffer.concat([Buffer.from(fixture.magicHex, "hex"), Buffer.of(fixture.tags[row.header.kind as keyof typeof fixture.tags]), uint(BigInt(row.header.activationGeneration)), uint(row.header.transportRequestSequence)]);
      expect(Buffer.from(raw).subarray(0, prefix.length)).toEqual(prefix);
      expect(() => api.decodeActorReturnResponse(raw)).toThrow();
      expect(Object.keys(header).sort()).toEqual(["activationGeneration", "kind", "transportRequestSequence"]);
    }
    for (const row of fixture.vectors) {
      const { kind, activationGeneration, transportRequestSequence } = hydrate(row.value);
      expect(api.readActorReturnResponseHeader(backing(row.hex))).toEqual({ kind, activationGeneration, transportRequestSequence });
    }
    const maximum = new ArrayBuffer(fixture.maximumBytes); new Uint8Array(maximum).set(Buffer.from(fixture.maximumPage.headerHex, "hex"));
    expect(api.readActorReturnResponseHeader(maximum)).toEqual({ kind: "result", activationGeneration: 18446744073709551615n, transportRequestSequence: Number.MAX_SAFE_INTEGER });
    expect(() => api.decodeActorReturnResponse(maximum)).toThrow();
    for (const hex of fixture.invalidHex.slice(0, 7)) expect(() => api.readActorReturnResponseHeader(backing(hex))).toThrow();
    for (const invalid of [new Uint8Array(maximum), new SharedArrayBuffer(16), new Proxy(maximum, {}), new ArrayBuffer(fixture.maximumBytes + 1)]) expect(() => api.readActorReturnResponseHeader(invalid)).toThrow();
  });
  it("ActorReturnResponse codec preserves transport correlation and closed fault variants", async () => {
    const api = await import("../../🟦️.ts");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    for (const row of fixture.vectors) {
      const value = hydrate(row.value);
      const encoded = api.encodeActorReturnResponse(value);
      expect(Buffer.from(encoded).toString("hex")).toBe(row.hex);
      expect(Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "byteLength")!.get!.call(encoded)).toBe(encoded.byteLength);
      expect(api.decodeActorReturnResponse(encoded)).toEqual(value);
      expect(Object.isFrozen(api.decodeActorReturnResponse(encoded))).toBe(true);
      for (let length = 0; length < encoded.byteLength; length++) expect(() => api.decodeActorReturnResponse(encoded.slice(0, length))).toThrow();
    }
    for (const hex of fixture.invalidHex) expect(() => api.decodeActorReturnResponse(backing(hex))).toThrow();
    expect(() => api.encodeActorReturnResponse(hydrate({ kind: "fault", activationGeneration: "7", transportRequestSequence: 1, fault: "arbitraryError" }))).toThrow();
    expect(() => api.encodeActorReturnResponse({ kind: "result", activationGeneration: 8n, transportRequestSequence: 1, result: hydrate(fixture.vectors[0]!.value.result) })).toThrow();
  });
  it("ActorReturnResponse maximum page is exact and decoder rejects non-owning backings", async () => {
    const api = await import("../../🟦️.ts");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const { default: returned } = await import("../../../🧫️fixtures/🔣️.json");
    const { createActorBytePage } = await import("../../../../📃️page/🟦️.ts");
    const row = returned.pageResultVectors[fixture.maximumPage.sharedPageVector]!;
    const bytes = Uint8Array.from({ length: row.pageLength }, (_, index) => (index * 37 + 11) % 256);
    const result = { kind: "page" as const, receipt: hydrate(row.receipt), page: createActorBytePage(bytes) };
    const encoded = api.encodeActorReturnResponse({ kind: "result", activationGeneration: BigInt(row.receipt.identity.origin.activationGeneration), transportRequestSequence: row.receipt.identity.origin.requestSequence, result });
    expect(encoded.byteLength).toBe(fixture.maximumPage.wireBytes); expect(api.ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES).toBe(fixture.maximumBytes);
    expect(Buffer.from(encoded)).toEqual(Buffer.concat([Buffer.from(fixture.maximumPage.headerHex + row.prefixHex, "hex"), Buffer.from(bytes)]));
    const decoded = api.decodeActorReturnResponse(encoded); if (decoded.kind !== "result") throw new Error("expected result");
    expect(decoded.result).toEqual(result);
    const valid = backing(fixture.vectors[0]!.hex);
    for (const invalid of [new Uint8Array(valid), new Uint8Array(valid, 1), new SharedArrayBuffer(valid.byteLength), new ArrayBuffer(fixture.maximumBytes + 1), new Proxy(valid, {}), {}, null]) expect(() => api.decodeActorReturnResponse(invalid)).toThrow();
    let reads = 0;
    expect(() => api.decodeActorReturnResponse({ get byteLength() { reads++; return 12; } })).toThrow(); expect(reads).toBe(0);
  });
  it("ActorReturnResponse actual Node transfer detaches the whole fixed backing and preserves exact bytes", async () => {
    const { execFileSync } = await import("node:child_process");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const source = `
      import { MessageChannel } from "node:worker_threads";
      import { once } from "node:events";
      const bytes = Buffer.from(process.argv[1], "hex");
      const raw = Uint8Array.from(bytes).buffer;
      raw.unowned = { bytes: new Uint8Array(8192) };
      const { port1, port2 } = new MessageChannel();
      const incoming = once(port2, "message");
      port1.postMessage(raw, [raw]);
      const [received] = await incoming;
      const length = Object.getOwnPropertyDescriptor(ArrayBuffer.prototype, "byteLength").get.call(received);
      process.stdout.write(JSON.stringify({ senderDetached: raw.detached, receiverResizable: received.resizable, receiverWholeBacking: length === received.byteLength && !(received instanceof SharedArrayBuffer), customPropertyTransferred: Object.hasOwn(received, "unowned"), bytesEqual: Buffer.from(received).equals(bytes) }));
      port1.close(); port2.close();
    `;
    const actual = JSON.parse(execFileSync("node", ["--input-type=module", "--eval", source, fixture.vectors[0]!.hex], { encoding: "utf8", timeout: 10000 }));
    expect(actual).toEqual(fixture.transfer);
  });
  it("ActorReturnResponse actual Node codec rejects shared resizable detached and view backings", async () => {
    const { execFileSync } = await import("node:child_process");
    const { default: fixture } = await import("../../🧫️fixtures/🔣️.json");
    const source = `
      const api = await import(process.argv[1]);
      const bytes = Buffer.from(process.argv[2], "hex");
      const raw = Uint8Array.from(bytes).buffer;
      const shared = new SharedArrayBuffer(bytes.length); new Uint8Array(shared).set(bytes);
      const resizable = new ArrayBuffer(bytes.length, { maxByteLength: bytes.length + 1 }); new Uint8Array(resizable).set(bytes);
      const detached = raw.slice(0); structuredClone(detached, { transfer: [detached] });
      const cases = { wholeFixed: raw, offsetView: new Uint8Array(raw, 1), wholeView: new Uint8Array(raw), shared, resizable, detached, proxy: new Proxy(raw, {}), oversized: new ArrayBuffer(api.ACTOR_RETURN_RESPONSE_MAXIMUM_BYTES + 1) };
      process.stdout.write(JSON.stringify(Object.fromEntries(Object.entries(cases).map(([name, value]) => { try { api.decodeActorReturnResponse(value); return [name, true]; } catch { return [name, false]; } }))));
    `;
    const actual = JSON.parse(execFileSync("node", ["--experimental-transform-types", "--input-type=module", "--eval", source, testSource.url, fixture.vectors[0]!.hex], { encoding: "utf8", timeout: 10000 }));
    expect(actual).toEqual(fixture.backings);
  });

}
