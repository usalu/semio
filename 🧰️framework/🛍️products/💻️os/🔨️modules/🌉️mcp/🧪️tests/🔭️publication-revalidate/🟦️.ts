import { expect, test } from "bun:test";
import ts from "typescript";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, statSync, utimesSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import corpus from "./🔣️.json";
import { createMcpPublicationProbeV1 } from "../../🟦️.ts";

const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
const ast = ts.createSourceFile("mcp.ts", source, ts.ScriptTarget.Latest, true);
const owner = ast.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === "servePublishedMcpStdioV1");
if (!owner) throw Error("Actual publication owner required");
const body = ts.transpileModule(owner.getText(ast).replace(/^export /u, "").replace(/^\s*const \{[^\n]+await import\([^\n]+$/gmu, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;


/** 🔭️ Runs the real launcher body against a temporary publication and reports its trace and changed verdict. */
const launch = async (row: (typeof corpus.cases)[number]): Promise<{ trace: string[]; changed: boolean | undefined; served: string | null; copied: boolean }> => {
  const directory = mkdtempSync(join(tmpdir(), "semio-revalidate-"));
  try {
    const binary = join(directory, "semio-os-mcp");
    const catalog = join(directory, "catalog");
    writeFileSync(binary, "binary-1");
    if (row.catalog === "present") {
      mkdirSync(catalog);
      writeFileSync(join(catalog, "🦀️catalog.json"), "{}");
      writeFileSync(join(catalog, "🔌️plugins.json"), "[]");
    }
    const env: NodeJS.ProcessEnv = { SEMIO_PLUGIN_CATALOG: catalog };
    const trace: string[] = [];
    let changed: boolean | undefined, served: string | null = null, copied = false;
    const components = async (_root: string, _invocation: unknown, _phase: unknown, environment: NodeJS.ProcessEnv, consume: (env: NodeJS.ProcessEnv) => Promise<unknown>) => {
      trace.push("components-enter");
      try { return await consume(environment); } finally { trace.push("components-leave"); }
    };
    const reader = async (_manifest: string, _name: string, _root: string, _mode: string, _invocation: unknown, _phase: unknown, consume: (artifact: { path: string }) => Promise<unknown>) => {
      trace.push("reader-enter");
      try { return await consume({ path: binary }); } finally { trace.push("reader-leave"); }
    };
    const gateway = async (options: { stage: (progress: () => void, signal: AbortSignal, serving: () => void) => Promise<number>; changed: () => boolean; command: () => string; onStop: () => void }) => {
      let first = "";
      const status = await options.stage(() => {}, new AbortController().signal, () => { trace.push("serving"); first = options.command(); });
      await new Promise(resolve => setTimeout(resolve, 0));
      trace.push(`status:${status}`);
      if (first !== "") { served = readFileSync(first, "utf8"); copied = first !== binary; }
      if (status !== 0) { options.onStop(); return 0; }
      changed = options.changed();
      trace.push(`changed:${changed}`);
      options.onStop();
      return 0;
    };
    const actual = new Function("withAdmittedPublishedCargoExecutableV1", "readBuildMode", "createRequestedIssuedResourceControlV1", "serveWarmMcpGateway", "withMcpHostDriverPolicy", "MCP_MANIFEST_REL", "MCP_BINARY_NAME", "MCP_BINARY_TARGET", "MCP_STDIO_BUILD_MODE_V1", "process", "withMcpComponentEnvironmentV1", "MCP_COMPONENT_MODELS_ENV", body + ";return servePublishedMcpStdioV1;")(reader, (mode: string) => mode, (_parent: unknown, _time: number, signal: AbortSignal) => ({ signal }), gateway, (args: string[]) => args, "owner/Cargo.toml", "semio-os-mcp", "mcp:build", "prototype", { platform: "linux" }, components, "SEMIO_MCP_COMPONENT_MODELS");
    const stage = async () => {
      trace.push("build");
      await new Promise(resolve => setTimeout(resolve, 10));
      if (row.mutate === "binary" || row.mutate === "both") writeFileSync(binary, "binary-22");
      if (row.mutate === "catalog" || row.mutate === "both") writeFileSync(join(catalog, "🔌️plugins.json"), '["x"]');
      return row.status;
    };
    expect(await actual("repo", [], {}, {}, env, (row as { catalogTarget?: string }).catalogTarget, stage, undefined, createMcpPublicationProbeV1(env))).toBe(0);
    return { trace, changed, served, copied };
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
};

for (const row of corpus.cases) test(`published launch revalidates behind a usable server: ${row.name}`, async () => {
  const result = await launch(row);
  expect(result.trace).toEqual(row.trace);
  if (row.status === 0) expect(result.changed).toBe(row.changed);
  expect(result.served).toBe(row.servedCopy);
  expect(result.copied).toBe(row.servedCopy !== null);
});

test("probe fingerprints agree with an independent SHA-256 hasher and stat facts", () => {
  const directory = mkdtempSync(join(tmpdir(), "semio-probe-"));
  try {
    const binary = join(directory, "semio-os-mcp");
    writeFileSync(binary, "abc");
    writeFileSync(join(directory, "🦀️catalog.json"), "{\"a\":1}");
    writeFileSync(join(directory, "🔌️plugins.json"), "[]");
    const probe = createMcpPublicationProbeV1({ SEMIO_PLUGIN_CATALOG: directory });
    expect(probe.usable()).toBe(true);
    const facts = statSync(binary);
    const oracle = (name: string): string => new Bun.CryptoHasher("sha256").update(readFileSync(join(directory, name))).digest("hex");
    const first = probe.fingerprint(binary);
    expect(first.split("\n")).toEqual([`${binary}:3:${facts.mtimeMs}:${facts.ino}`, `${join(directory, "🦀️catalog.json")}:${oracle("🦀️catalog.json")}`, `${join(directory, "🔌️plugins.json")}:${createHash("sha256").update("[]").digest("hex")}`]);
    utimesSync(binary, new Date(1e12), new Date(1e12));
    expect(probe.fingerprint(binary)).not.toBe(first);
    rmSync(join(directory, "🔌️plugins.json"));
    expect(probe.usable()).toBe(false);
    expect(createMcpPublicationProbeV1({}).usable()).toBe(true);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
