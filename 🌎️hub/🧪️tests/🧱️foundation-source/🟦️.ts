import { expect, test } from "bun:test";
import Ajv from "ajv";
import { createHmac, webcrypto } from "node:crypto";
import { EventEmitter } from "node:events";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { PassThrough } from "node:stream";
import type { ChildProcess } from "node:child_process";
import ts from "typescript";
import { loadTaxonomy, semanticDirectoryKindId } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { orderedDirectoryPublicationOracle } from "../../📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts";
import { directChildEnvironment, directChildLaunch, deliverCredentialEnvelopeToChild, sealedDirectChildEnvironment, type CredentialChildOperations } from "../../🔐️auth/📤️credential-delivery/🟦️.ts";
import { mcpCredentialSourceOrderConforms, nativeCredentialSourceOrderConforms, proveMcpCredentialSourceOrder, proveNativeCredentialSourceOrder, sourceDefinitionBodies } from "../../🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts";
import { assertHubFixtureExpectation } from "../🧬️schema/🛂expectation/🟦️.ts";
import { authenticatedFrame, hmacProof, verifyAuthenticatedFrame } from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { LOCAL_BOOTSTRAP_FRAME_MAX, LocalFrameReader, writeLocalFrame } from "../../🚀️local-bootstrap/📡️framing/🟦️.ts";
import { allocateLocalHubRunRoot, finishLocalHub, localHubReadinessAdmitted, type LocalHubRun } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES } from "../../💡️inference/🧬️schema/🟦️.ts";

const repoRoot = resolve(import.meta.dir, "../../..");
const hubRoot = join(repoRoot, "🌎️hub");
const schemaPath = join(hubRoot, "🧬️schema/🧱️foundation-source/🔣️.json");
const fixturePath = join(hubRoot, "🧫️fixtures/🧱️foundation-source/🔣️.json");
const fixtureSource = readFileSync(fixturePath, "utf8");
const fixture = JSON.parse(fixtureSource) as {
  readonly schemaVersion: 1;
  readonly owners: readonly { readonly path: string; readonly declarations: readonly string[]; readonly imports: readonly string[]; readonly rootImports: readonly string[]; readonly contextChain: readonly string[] }[];
  readonly contexts: readonly { readonly directoryName: string; readonly parentKindId: string; readonly kindId: string }[];
  readonly route: { readonly command: string; readonly target: string; readonly launchName: string; readonly launchCommand: string; readonly launchGroup: string; readonly launchOrder: number; readonly inputs: readonly string[] };
  readonly limits: { readonly localFrameBytes: number; readonly gisControlFrameBytes: number; readonly profiles: number; readonly outstandingReads: number; readonly cargoBuildStack: string; readonly cargoNativeStack: string };
  readonly sourceBoundary: Readonly<Record<"typescript" | "rust", { readonly source: string; readonly definition: string; readonly expectedBodies: number; readonly expectedCalls: readonly string[] }>>;
};

function exportedDeclarations(path: string): Set<string> {
  const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const names = new Set<string>();
  source.forEachChild((node) => {
    if (
      (ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node) || ts.isInterfaceDeclaration(node) || ts.isTypeAliasDeclaration(node) || ts.isVariableStatement(node)) &&
      node.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)
    ) {
      if ("name" in node && node.name) names.add(node.name.text);
      if (ts.isVariableStatement(node)) for (const declaration of node.declarationList.declarations) if (ts.isIdentifier(declaration.name)) names.add(declaration.name.text);
    }
  });
  return names;
}

function internalOwnerImports(path: string, owners: ReadonlySet<string>): string[] {
  const absolute = join(repoRoot, path);
  const source = ts.createSourceFile(path, readFileSync(absolute, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  return source.statements
    .flatMap((node) => {
      if (!ts.isImportDeclaration(node) || !ts.isStringLiteral(node.moduleSpecifier) || !node.moduleSpecifier.text.startsWith(".")) return [];
      const imported = `${relative(repoRoot, resolve(dirname(absolute), node.moduleSpecifier.text)).replaceAll("\\", "/")}.ts`.replace(".ts.ts", ".ts");
      return owners.has(imported) ? [imported] : [];
    })
    .sort();
}

function rootImportsForOwner(routerPath: string, ownerPath: string): string[] {
  const source = ts.createSourceFile(routerPath, readFileSync(routerPath, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const owner = join(repoRoot, ownerPath);
  return source.statements.flatMap((node) => {
    if (!ts.isImportDeclaration(node) || !ts.isStringLiteral(node.moduleSpecifier) || !node.moduleSpecifier.text.startsWith(".")) return [];
    if (resolve(dirname(routerPath), node.moduleSpecifier.text) !== owner) return [];
    const bindings = node.importClause?.namedBindings;
    return bindings && ts.isNamedImports(bindings) ? bindings.elements.map((element) => element.name.text) : [];
  });
}

test("Hub foundation contract is schema-first and independently parsed", async () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(readFileSync(schemaPath, "utf8")));
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  expect(validate({ ...fixture, schemaVersion: 2 })).toBe(false);
  expect(validate({ ...fixture, owners: fixture.owners.slice(1) })).toBe(false);
  const jsonc = await import("jsonc-parser");
  const errors: import("jsonc-parser").ParseError[] = [];
  expect(jsonc.parse(fixtureSource, errors, { allowTrailingComma: false, disallowComments: true })).toEqual(fixture);
  expect(errors).toEqual([]);
});

test("Hub foundation owners are anonymous, exported, and acyclic", () => {
  const ownerPaths = new Set(fixture.owners.map((owner) => owner.path));
  expect(fixture.owners).toHaveLength(10);
  for (const owner of fixture.owners) {
    const absolute = join(repoRoot, owner.path);
    expect(absolute.endsWith("/🟦️.ts")).toBe(true);
    expect(existsSync(absolute), owner.path).toBe(true);
    const declarations = exportedDeclarations(absolute);
    for (const declaration of owner.declarations) expect(declarations.has(declaration), `${owner.path}:${declaration}`).toBe(true);
    expect(internalOwnerImports(owner.path, ownerPaths), owner.path).toEqual([...owner.imports].sort());
  }
  const visiting = new Set<string>();
  const visited = new Set<string>();
  const edges = new Map(fixture.owners.map((owner) => [owner.path, owner.imports]));
  const visit = (path: string): void => {
    if (visiting.has(path)) throw new Error(`cycle at ${path}`);
    if (visited.has(path)) return;
    visiting.add(path);
    for (const dependency of edges.get(path) ?? []) visit(dependency);
    visiting.delete(path);
    visited.add(path);
  };
  for (const path of ownerPaths) visit(path);
  expect(visited.size).toBe(10);
});

test("Hub foundation owner graph typechecks without a command-module back edge", { timeout: 30_000 }, () => {
  const paths = fixture.owners.map((owner) => join(repoRoot, owner.path));
  const routerPath = join(hubRoot, "📦️packages/🦀️rust/📜️script.ts");
  const program = ts.createProgram(paths, {
    target: ts.ScriptTarget.ESNext,
    module: ts.ModuleKind.ESNext,
    moduleResolution: ts.ModuleResolutionKind.Bundler,
    strict: true,
    noUncheckedIndexedAccess: true,
    allowImportingTsExtensions: true,
    allowJs: true,
    skipLibCheck: true,
    noEmit: true,
    types: ["node"],
  });
  expect(
    paths.flatMap((path) => [...program.getSyntacticDiagnostics(program.getSourceFile(path)), ...program.getSemanticDiagnostics(program.getSourceFile(path))]).map((diagnostic) => ts.flattenDiagnosticMessageText(diagnostic.messageText, "\n")),
  ).toEqual([]);
  for (const path of paths) {
    const source = ts.createSourceFile(path, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
    for (const statement of source.statements) {
      if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier) || !statement.moduleSpecifier.text.startsWith(".")) continue;
      expect(resolve(dirname(path), statement.moduleSpecifier.text), path).not.toBe(routerPath);
    }
  }
});

test("Hub foundation owner paths bind their full taxonomy contexts", () => {
  const taxonomy = loadTaxonomy();
  for (const context of fixture.contexts) expect(semanticDirectoryKindId(context.directoryName, taxonomy, { parentKindId: context.parentKindId }), context.kindId).toBe(context.kindId);
  for (const owner of fixture.owners) {
    const segments = relative(hubRoot, join(repoRoot, owner.path)).replaceAll("\\", "/").split("/").slice(0, -1);
    const directories = owner.contextChain.map((kindId) => fixture.contexts.find((context) => context.kindId === kindId)!.directoryName);
    expect(segments.slice(-directories.length), owner.path).toEqual(directories);
  }
});

test("frame authentication matches Node and Web Crypto oracles", async () => {
  const pipeFixture = JSON.parse(readFileSync(join(hubRoot, "🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json"), "utf8"));
  const key = Buffer.from(pipeFixture.channelKey, "hex");
  const unsigned = pipeFixture.helloWithoutProof;
  const canonical = Buffer.from(JSON.stringify(unsigned));
  const length = Buffer.alloc(4);
  length.writeUInt32BE(canonical.length);
  const nodeProof = createHmac("sha256", key).update("semio/hub/local-bootstrap/v1\0").update(length).update(canonical).digest("hex");
  const webKey = await webcrypto.subtle.importKey("raw", key, { name: "HMAC", hash: "SHA-256" }, false, ["sign"]);
  const webProof = Buffer.from(await webcrypto.subtle.sign("HMAC", webKey, Buffer.concat([Buffer.from("semio/hub/local-bootstrap/v1\0"), length, canonical]))).toString("hex");
  expect(hmacProof(key, unsigned)).toBe(pipeFixture.hello.proof);
  expect(nodeProof).toBe(pipeFixture.hello.proof);
  expect(webProof).toBe(pipeFixture.hello.proof);
  expect(authenticatedFrame(key, unsigned)).toEqual(pipeFixture.hello);
  expect(() => verifyAuthenticatedFrame(key, pipeFixture.hello)).not.toThrow();
  expect(() => verifyAuthenticatedFrame(key, { ...pipeFixture.hello, proof: pipeFixture.hostile.wrongProof })).toThrow(/proof invalid/u);
  expect(LOCAL_BOOTSTRAP_FRAME_MAX).toBe(fixture.limits.localFrameBytes);
  expect(GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES).toBe(fixture.limits.gisControlFrameBytes);
});

test("frame transport handles splits and rejects hostile bounds, JSON, deadlines, EOF and excess work", async () => {
  const bytes = Buffer.from(JSON.stringify({ sequence: 7, value: "split" }));
  const frame = Buffer.alloc(bytes.length + 4);
  frame.writeUInt32BE(bytes.length);
  bytes.copy(frame, 4);
  const split = new PassThrough();
  const reader = new LocalFrameReader(split);
  const observed = reader.read(100);
  split.write(frame.subarray(0, 3));
  split.write(frame.subarray(3));
  expect(await observed).toEqual({ sequence: 7, value: "split" });

  const written = new PassThrough();
  const chunks: Buffer[] = [];
  written.on("data", (chunk) => chunks.push(Buffer.from(chunk)));
  await writeLocalFrame(written, { accepted: true });
  expect(Buffer.concat(chunks).readUInt32BE(0)).toBe(Buffer.byteLength(JSON.stringify({ accepted: true })));

  const malformed = new PassThrough();
  const malformedRead = new LocalFrameReader(malformed).read(100);
  malformed.write(Buffer.from([0, 0, 0, 1, 0xff]));
  await expect(malformedRead).rejects.toThrow(/not JSON/u);
  const zero = new PassThrough();
  const zeroRead = new LocalFrameReader(zero).read(100);
  zero.write(Buffer.from([0, 0, 0, 0]));
  await expect(zeroRead).rejects.toThrow(/fixed bound/u);
  const expired = new PassThrough();
  await expect(new LocalFrameReader(expired).read(5)).rejects.toThrow(/deadline/u);
  const ended = new PassThrough();
  const endedRead = new LocalFrameReader(ended).read(100);
  ended.end();
  await expect(endedRead).rejects.toThrow(/EOF|closed/u);
  const saturated = new PassThrough();
  const saturatedReader = new LocalFrameReader(saturated);
  const pending = Array.from({ length: fixture.limits.outstandingReads }, () => saturatedReader.read(100));
  await expect(saturatedReader.read(100)).rejects.toThrow(/outstanding/u);
  saturated.end();
  await Promise.allSettled(pending);
  expect(() => writeLocalFrame(new PassThrough(), { value: "x".repeat(LOCAL_BOOTSTRAP_FRAME_MAX) })).toThrow(/fixed bound/u);
});

test("allocation, readiness and finish stay injectable and idempotent", async () => {
  const calls: string[] = [];
  const allocation = allocateLocalHubRunRoot("/private/ticket", {
    platform: "linux",
    makeTemporaryDirectory: (prefix) => {
      calls.push(`make:${prefix}`);
      return "/private/ticket/semio-hub-run-fixed";
    },
    protectDirectory: (path) => calls.push(`protect:${path}`),
    removeDirectory: (path) => calls.push(`remove:${path}`),
  });
  expect(allocation.path).toBe("/private/ticket/semio-hub-run-fixed");
  allocation.remove();
  expect(calls).toEqual(["make:/private/ticket/semio-hub-run-", "protect:/private/ticket/semio-hub-run-fixed", "remove:/private/ticket/semio-hub-run-fixed"]);
  const pipeFixture = JSON.parse(readFileSync(join(hubRoot, "🚀️local-bootstrap/🧫️fixtures/🚇️pipe-v1/🔣️.json"), "utf8"));
  expect(localHubReadinessAdmitted(pipeFixture.ready, 200, pipeFixture.ready.runId)).toBe(true);
  expect(localHubReadinessAdmitted(pipeFixture.bootstrapReadyButArtifactUnavailable, 503, pipeFixture.ready.runId, true)).toBe(true);
  expect(localHubReadinessAdmitted(pipeFixture.bootstrapReadyButArtifactUnavailable, 503, pipeFixture.ready.runId)).toBe(false);
  expect(() => localHubReadinessAdmitted({ ...pipeFixture.ready, runId: "wrong" }, 200, pipeFixture.ready.runId)).toThrow(/binding/u);

  const child = Object.assign(new EventEmitter(), { exitCode: 0, kill: () => true }) as unknown as ChildProcess;
  const pipe = new PassThrough();
  let removals = 0;
  const run = {
    child,
    pipe,
    reader: new LocalFrameReader(pipe),
    channelKey: Buffer.alloc(32, 7),
    runId: "0".repeat(32),
    port: 1,
    runRoot: "/private/ticket/semio-hub-run-fixed",
    output: () => "",
    removeRunRoot: () => (removals += 1),
  } as LocalHubRun;
  await finishLocalHub(run);
  await finishLocalHub(run);
  expect(removals).toBe(1);
  expect(run.channelKey.equals(Buffer.alloc(32))).toBe(true);
});

test("credential delivery seals authority and uses one injected fd3 endpoint", async () => {
  const source = {
    PATH: "/bin",
    S_USER: "secret-user",
    VITE_S_USER: "secret-vite",
    S_HUB_URL: "http://secret",
    API_TOKEN: "secret-token",
    SESSION_HINT: "secret-session",
    COOKIE_JAR: "secret-cookie",
    ORDINARY: "kept",
  };
  expect(sealedDirectChildEnvironment(source)).toEqual({ PATH: "/bin", ORDINARY: "kept", SEMIO_DIRECT_CHILD_BENIGN: "preserved" });
  expect(directChildEnvironment(source)).toEqual({ PATH: "/bin", ORDINARY: "kept", SEMIO_DIRECT_CHILD_BENIGN: "preserved", S_LOCAL_CREDENTIAL_FD: "3" });
  expect(directChildLaunch("native", ["--run"], "native", source).stdio).toEqual(["ignore", "pipe", "pipe", "pipe"]);
  expect(directChildLaunch("mcp", ["stdio"], "mcp", source).stdio).toEqual(["pipe", "pipe", "pipe", "pipe"]);
  const fd3 = new PassThrough();
  const chunks: Buffer[] = [];
  fd3.on("data", (chunk) => chunks.push(Buffer.from(chunk)));
  let launch: ReturnType<typeof directChildLaunch> | undefined;
  let terminated = 0;
  const child = Object.assign(new EventEmitter(), { stdio: [null, null, null, fd3], kill: () => true }) as unknown as ChildProcess;
  const operations: CredentialChildOperations = {
    spawnChild: (request) => {
      launch = request;
      return child;
    },
    terminateChild: () => (terminated += 1),
  };
  const envelope = { clientClass: "mcp", sessionId: "session", authorizationGeneration: 2, expiresAt: 1770000000000, capability: `session.v1.${"a".repeat(32)}.${"b".repeat(64)}` };
  await deliverCredentialEnvelopeToChild("mcp", ["stdio"], envelope, "mcp", "http://127.0.0.1:6325", source, operations);
  expect(launch?.stdio).toEqual(["pipe", "pipe", "pipe", "pipe"]);
  expect(launch?.env.S_LOCAL_CREDENTIAL_FD).toBe("3");
  const delivered = Buffer.concat(chunks);
  expect(delivered.readUInt32BE(0)).toBe(delivered.length - 4);
  expect(JSON.parse(delivered.subarray(4).toString("utf8"))).toMatchObject({ schema: "semio.local.consumer-credential/v1", clientClass: "mcp", hubOrigin: "http://127.0.0.1:6325" });
  expect(envelope.capability).toBe("");
  expect(terminated).toBe(0);
  const mismatched = { ...envelope, clientClass: "native", capability: `session.v1.${"c".repeat(32)}.${"d".repeat(64)}` };
  await expect(deliverCredentialEnvelopeToChild("mcp", [], mismatched, "mcp", "http://127.0.0.1:6325", source, operations)).rejects.toThrow(/client class/u);
  expect(mismatched.capability).toBe("");
  const spawnRejected = { ...envelope, capability: `session.v1.${"e".repeat(32)}.${"f".repeat(64)}` };
  await expect(
    deliverCredentialEnvelopeToChild("mcp", [], spawnRejected, "mcp", "http://127.0.0.1:6325", source, {
      ...operations,
      spawnChild: () => {
        throw new Error("injected spawn rejection");
      },
    }),
  ).rejects.toThrow(/spawn rejection/u);
  expect(spawnRejected.capability).toBe("");
});

test("source definition boundaries match TypeScript AST and reject stale credential orders", () => {
  const hostileSource = fixture.sourceBoundary.typescript.source;
  const manualClass = sourceDefinitionBodies(hostileSource, /\bclass\s+Runner\b[^\{]*/gu);
  const manualRun = sourceDefinitionBodies(manualClass[0]!, /\basync\s+run\s*\([^)]*\)\s*(?::[^\{]+)?/gu);
  const parsed = ts.createSourceFile("hostile.ts", hostileSource, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const classNode = parsed.statements.find(ts.isClassDeclaration)!;
  const method = classNode.members.find(ts.isMethodDeclaration)!;
  expect(manualClass).toHaveLength(fixture.sourceBoundary.typescript.expectedBodies);
  expect(manualRun).toHaveLength(fixture.sourceBoundary.typescript.expectedBodies);
  expect(manualRun[0]!.trim()).toBe(method.body!.getText(parsed).slice(1, -1).trim());
  const typescriptCalls = [...manualRun[0]!.matchAll(/\b(?:await\s+)?([a-z][A-Za-z0-9]*)\(\)/gu)].map((match) => match[1]).filter((name) => name !== "if");
  expect(typescriptCalls).toEqual(fixture.sourceBoundary.typescript.expectedCalls);
  expect(sourceDefinitionBodies(hostileSource, /\bfunction\s+declaration\s*\([^)]*\)\s*(?::[^\{;]+)?/gu)).toEqual([]);
  const rustBodies = sourceDefinitionBodies(fixture.sourceBoundary.rust.source, /\bfn\s+main\s*\([^)]*\)\s*/gu);
  expect(rustBodies).toHaveLength(fixture.sourceBoundary.rust.expectedBodies);
  expect(fixture.sourceBoundary.rust.expectedCalls.every((call) => rustBodies[0]!.includes(`${call}();`))).toBe(true);

  const native = {
    entrypoint: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/🦀️.rs"), "utf8"),
    credential: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🦀️.rs"), "utf8"),
    runner: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts"), "utf8"),
    launch: readFileSync(join(repoRoot, ".vscode/🧩️launch.seed.jsonc"), "utf8"),
  };
  expect(nativeCredentialSourceOrderConforms(native)).toBe(true);
  expect(nativeCredentialSourceOrderConforms({ ...native, runner: native.runner.replace("await runNativeSession(", "await staleNativeSession(") })).toBe(false);
  expect(() => proveNativeCredentialSourceOrder(repoRoot)).not.toThrow();

  const mcp = {
    entrypoint: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏗️bootstrap/🦀️.rs"), "utf8"),
    workspace: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs"), "utf8"),
    remote: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🔗️remote/🦀️.rs"), "utf8"),
    directory: native.credential,
    runner: readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/📜️script.ts"), "utf8"),
    launch: native.launch,
  };
  expect(mcpCredentialSourceOrderConforms(mcp)).toBe(true);
  expect(mcpCredentialSourceOrderConforms({ ...mcp, entrypoint: mcp.entrypoint.replace('claim_inherited_local_hub_credential("mcp")', 'claim_inherited_local_hub_credential("late")') })).toBe(false);
  expect(mcpCredentialSourceOrderConforms({ ...mcp, entrypoint: mcp.entrypoint.replace("return;\n    }", "return;\n    }\n    return;") })).toBe(false);
  expect(() => proveMcpCredentialSourceOrder(repoRoot)).not.toThrow();

  const rustOracle = readFileSync(join(hubRoot, "🔐️auth/🧪️tests/🧭️credential-source-order/🔮️oracles/🦀️.rs"), "utf8");
  const cargoManifest = readFileSync(join(hubRoot, "📦️packages/🦀️rust/Cargo.toml"), "utf8");
  const cargoLibrary = readFileSync(join(hubRoot, "📦️packages/🦀️rust/🦀️.rs"), "utf8");
  expect(rustOracle).toContain("syn::parse_file");
  expect(rustOracle).toContain("schema_mirror_json");
  expect(rustOracle).toContain("hub_credential_source_order_syn_parity");
  expect(cargoManifest).toContain('syn = { version = "2", features = ["full", "visit"] }');
  expect(cargoLibrary).toContain('credential-source-order/🔮️oracles/🦀️.rs"]');
});

test("ordered publication and Cargo staging retain their exact authorities", () => {
  expect(orderedDirectoryPublicationOracle(repoRoot)).toBe(9);
  const defaults = exactCargoStageEnvironments({ KEEP: "yes" });
  expect(defaults.env).toEqual({ KEEP: "yes", RUST_MIN_STACK: fixture.limits.cargoBuildStack });
  expect(defaults.nativeEnv).toEqual({ RUST_MIN_STACK: fixture.limits.cargoNativeStack });
  expect(exactCargoStageEnvironments({ SEMIO_BUILD_RUST_MIN_STACK: "67108864" }).env.RUST_MIN_STACK).toBe("67108864");
  expect(() => assertHubFixtureExpectation("negative", { stage: "contract", result: "rejected", code: "hostile" }, true)).toThrow(/hostile/u);
  expect(() => assertHubFixtureExpectation("positive", { stage: "contract", result: "accepted", code: "valid" }, true)).not.toThrow();
});

test("package, target, input and launch registrations bind only the moved owners", async () => {
  const routerPath = join(hubRoot, "📦️packages/🦀️rust/📜️script.ts");
  const router = readFileSync(routerPath, "utf8");
  const moved = fixture.owners.flatMap((owner) => owner.declarations);
  const routerAst = ts.createSourceFile(routerPath, router, ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const declared = new Set<string>();
  routerAst.forEachChild((node) => {
    if ((ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node) || ts.isTypeAliasDeclaration(node)) && node.name) declared.add(node.name.text);
    if (ts.isVariableStatement(node)) for (const declaration of node.declarationList.declarations) if (ts.isIdentifier(declaration.name)) declared.add(declaration.name.text);
  });
  for (const name of moved) expect(declared.has(name), name).toBe(false);
  for (const owner of fixture.owners) expect(rootImportsForOwner(routerPath, owner.path), owner.path).toEqual(owner.rootImports);
  expect(router).toContain('import { HubFoundationSourceScript } from "../../🧪️tests/🧱️foundation-source/🏃️execution/🟦️.ts";');
  expect(router).toContain(`.register("${fixture.route.command}", HubFoundationSourceScript)`);
  const project = JSON.parse(readFileSync(join(hubRoot, "📦️packages/🦀️rust/📋️project.json"), "utf8"));
  expect(project.namedInputs.hubFoundationSources).toEqual(fixture.route.inputs);
  expect(project.targets[fixture.route.target]?.inputs).toEqual(["hubFoundationSources"]);
  expect(project.targets[fixture.route.target]?.options?.command).toBe("bun ./📜️script.ts foundation-source-check");
  const jsonc = await import("jsonc-parser");
  for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
    const launch = jsonc.parse(readFileSync(join(repoRoot, path), "utf8"));
    expect(
      launch.configurations.filter(
        (row: { name?: string; command?: string; presentation?: { group?: string; order?: number } }) =>
          row.name === fixture.route.launchName && row.command === fixture.route.launchCommand && row.presentation?.group === fixture.route.launchGroup && row.presentation?.order === fixture.route.launchOrder,
      ),
      path,
    ).toHaveLength(1);
  }
});
