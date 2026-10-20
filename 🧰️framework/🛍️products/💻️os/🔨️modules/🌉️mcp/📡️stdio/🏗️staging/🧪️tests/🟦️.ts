import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { EventEmitter } from "node:events";
import { PassThrough } from "node:stream";
import { createInterface } from "node:readline";
import { Database } from "bun:sqlite";
import Ajv from "ajv/dist/2020";
import ts from "typescript";

const schema = JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json", import.meta.url), "utf8"));
const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const source = readFileSync(new URL("../../../🟦️.ts", import.meta.url), "utf8");
const ast = ts.createSourceFile("mcp.ts", source, ts.ScriptTarget.Latest, true);
const names = ["mcpStagingEnvironment", "stageMcpBinaryAsync", "nativeMcpBinaryStagingFor"];
const declarations = ast.statements.filter(node => ts.isFunctionDeclaration(node) ? names.includes(node.name?.text ?? "") : ts.isVariableStatement(node) && node.declarationList.declarations.some(declaration => ts.isIdentifier(declaration.name) && names.includes(declaration.name.text)));
const code = ts.transpileModule(declarations.map(node => node.getText(ast).replace(/^export /u, "")).join("\n"), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;

test("native staging neutral commands agree with Ajv and SQLite", () => {
  const validate = new Ajv({ strict: false }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  const db = new Database(":memory:");
  try {
    for (const row of fixture.cases) {
      const oracle = db.query("SELECT 'repo' command, json_array('run','@semio-tech/framework-os-mcp-rs:build','--output=stream') arguments").get() as { command: string; arguments: string };
      expect(row.command).toBe(oracle.command);
      expect(row.arguments).toEqual(JSON.parse(oracle.arguments));
    }
    expect(validate({ ...fixture, cases: fixture.cases.map((row: object) => ({ ...row, command: "bun" })) })).toBe(false);
  } finally { db.close(); }
});

for (const row of fixture.cases) test(`native staging actual ports retain argv, env, progress and signal on ${row.platform}`, async () => {
  expect(declarations.length).toBe(3);
  const calls: Array<{ command: string; args: string[]; options: Record<string, unknown> }> = [];
  const processPort = { env: { SEMIO_SENTINEL: "opaque", SEMIO_SCRIPT_INVOKE: "original-envelope", CARGO_INCREMENTAL: "1" }, pid: 99 };
  const spawn = (command: string, args: string[], options: Record<string, unknown>) => {
    calls.push({ command, args, options });
    const child = Object.assign(new EventEmitter(), { stdout: new PassThrough(), stderr: new PassThrough() });
    setTimeout(() => { child.stdout.end("child-output\n"); child.stderr.end("child-diagnostic\n"); child.emit("close", 0); }, 0);
    return child;
  };
  const spawnSync = (command: string, args: string[], options: Record<string, unknown>) => {
    calls.push({ command, args, options });
    return { stdout: "child-output\n", stderr: "child-diagnostic\n", status: 0 };
  };
  const ports = new Function("spawn", "spawnSync", "createInterface", "process", "CARGO_BUILD_OWNER_PID_ENV", "MCP_BINARY_TARGET", `${code}; return { stageMcpBinaryAsync, nativeMcpBinaryStagingFor };`)(spawn, spawnSync, createInterface, processPort, "SEMIO_CARGO_BUILD_OWNER_PID", row.arguments[1]);
  const controller = new AbortController(), progress: string[] = [];
  expect(await ports.stageMcpBinaryAsync("owned-root", (line: string) => progress.push(line), controller.signal)).toBe(0);
  expect(ports.nativeMcpBinaryStagingFor("owned-root").stage((line: string) => progress.push(line))).toBe(0);
  for (const call of calls) {
    expect(call.command).toBe(row.command);
    expect(call.args).toEqual(row.arguments);
    expect(call.options.cwd).toBe("owned-root");
    expect(call.options.shell).toBe(false);
    expect(call.options.env).toEqual({ ...processPort.env, SEMIO_CARGO_BUILD_OWNER_PID: "99" });
  }
  expect(calls.length).toBe(2);
  expect(calls[0].options.signal).toBe(controller.signal);
  expect(progress.filter(line => line === "child-output").length).toBe(2);
  expect(progress.filter(line => line === "child-diagnostic").length).toBe(2);
  expect(progress.filter(line => line.startsWith("[semio-os-mcp] staging via repo run")).length).toBe(2);
});
