/** 📝️ Emits the browser ABI declarations from the owned operation schema and checks their runtime surface. */
import Ajv from "ajv";
import assert from "node:assert/strict";
import { readFileSync, realpathSync, writeFileSync } from "node:fs";
import { join } from "node:path";

//#region 🧬️Contracts
/** 🧬️ Owned draft-07 contract module for every Flow wasm bridge fixture.
 * @see 🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧬️schema/🔣️.json */
export const FLOW_WASM_SCHEMA_URL = new URL("../../🧬️schema/🔣️.json", import.meta.url);

/** 🔍️ Compiles one named export of the Flow wasm contract module under strict draft-07 Ajv. */
export function flowWasmContract(exportId: string): (value: unknown) => boolean {
  const document = JSON.parse(readFileSync(FLOW_WASM_SCHEMA_URL, "utf8"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(document);
  const validate = ajv.getSchema(`${document.$id}#/$defs/${exportId}`);
  if (!validate) throw new Error(`Flow wasm contract module has no export ${exportId}`);
  return validate as (value: unknown) => boolean;
}
//#endregion 🧬️Contracts

//#region 📝️Declarations
type Field = { name: string; type: "utf8" | "optional-utf8" | "f64" | "u64" | "u32" | "u8" | "bool" | "bytes" };
type Contract = { operations: Record<string, number>; arguments: Record<string, Field[]> };
const excluded = new Set(["open", "attachSurface", "renderFrame"]);
const fieldTypes: Record<Field["type"], string> = { utf8: "string", "optional-utf8": "string | null", f64: "number", u64: "number", u32: "number", u8: "number", bool: "boolean", bytes: "ArrayBufferView | readonly number[]" };
const contractPath = join(import.meta.dir, "../../../🕸️wasm/🧪️fixtures/📡️abi.json");
const declarationName = "../../../🕸️wasm/📦️packages/🟨️javascript/📝️flow-browser.d.ts";

export function flowBrowserDeclaration(): string {
  const contract: Contract = JSON.parse(readFileSync(contractPath, "utf8"));
  const record = (fields: Field[]) => `{ ${fields.map((field) => `readonly ${field.name}${field.type === "optional-utf8" ? "?" : ""}: ${fieldTypes[field.type]}`).join("; ")} }`;
  const names = Object.keys(contract.operations).filter((name) => !excluded.has(name));
  const methods = names.flatMap((name) => {
    const fields = contract.arguments[name];
    const parameters = fields.map((field, index) => `${field.name}${field.type === "optional-utf8" && fields.slice(index).every((value) => value.type === "optional-utf8") ? "?" : ""}: ${fieldTypes[field.type]}`).join(", ");
    const positional = `  ${name}(${parameters}): FlowTask<unknown>;`;
    return fields.length ? [positional, `  ${name}(args: ${record(fields)}): FlowTask<unknown>;`] : [positional];
  });
  return `/** 🧬️ Generated from the owned Flow browser ABI schema. */
//#region 🧬️Contract
export interface FlowTaskEvent { readonly tag: 3; readonly requestId: bigint; readonly generation: number; readonly sequence: number; readonly event: number; readonly status: number; readonly body: Uint8Array; }
export interface FlowTask<T = unknown> { readonly result: Promise<T>; cancel(): boolean; subscribe(observer: (event: FlowTaskEvent) => void): () => void; }
export interface FlowHandle { readonly slot: number; readonly generation: number; }
export interface FlowWasmExports {
  readonly memory: WebAssembly.Memory;
  flow_bridge_allocate(length: number): number;
  flow_bridge_release(pointer: number, length: number): void;
  flow_bridge_send(pointer: number, length: number, credit: number, now: bigint, deadline: bigint): number;
  flow_bridge_poll(pointer: number, capacity: number, credit: number, now: bigint, deadline: bigint): number;
  flow_bridge_begin_close(): void;
  flow_bridge_terminal_is_empty(): number;
}
export interface FlowFeatures {
  readonly lifetime: { readonly session: FlowHandle; close(): Promise<void>; terminalIsEmpty(): boolean; };
${["document", "interaction", "editing", "surface", "drawing"].map((group, index) => {
  const boundaries = [[1, 25], [25, 50], [50, 75], [75, 99], [99, undefined]] as const;
  const [start, end] = boundaries[index];
  const groupNames = Object.keys(contract.operations).slice(start, end);
  return `  readonly ${group}: { ${groupNames.map((name) => { const fields = contract.arguments[name]; return `${name}(${fields.length ? `args${fields.every((field) => field.type === "optional-utf8") ? "?" : ""}: ${record(fields)}` : ""}): FlowTask<unknown>`; }).join("; ")} };`;
}).join("\n")}
}
export interface FlowBrowserOptions { readonly source: unknown; readonly imports?: WebAssembly.Imports; readonly instantiate?: typeof WebAssembly.instantiate; readonly schedule?: (callback: () => void) => void; readonly now?: () => number; readonly maximumInFlight?: number; }
export interface FlowBrowserRuntime { openSession(): FlowSession; close(): Promise<void>; terminalIsEmpty(): boolean; }
export declare function createFlowBrowserRuntime(options: FlowBrowserOptions): Promise<FlowBrowserRuntime>;
export declare class FlowSession {
  private constructor();
${methods.join("\n")}
  attachCanvas(canvas: HTMLCanvasElement, width: number, height: number, dpr: number): FlowTask<unknown>;
  renderCanvas(canvas: HTMLCanvasElement): FlowTask<unknown>;
  close(): Promise<void>;
  free(): Promise<void>;
  [Symbol.dispose](): void;
}
//#endregion 🧬️Contract
`;
}

export function writeFlowBrowserDeclaration(): string {
  const path = join(import.meta.dir, declarationName);
  writeFileSync(path, flowBrowserDeclaration(), "utf8");
  return path;
}
//#endregion 📝️Declarations

//#region 🧪️SchemaOracle
export async function testFlowBrowserDeclaration(packageRoot: string): Promise<void> {
  const ts = await import("typescript");
  const { FlowSession } = await import("./🌐️flow-browser.js");
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "../../🧪️fixtures/📝️browser-types.json"), "utf8"));
  const validate = flowWasmContract("FlowBrowserTypesV1");
  assert.equal(validate(fixture), true);
  const manifest = JSON.parse(readFileSync(join(packageRoot, "package.json"), "utf8"));
  assert.equal(manifest.name, fixture.package.name);
  assert.deepEqual(manifest.files.slice().sort(), fixture.package.files.slice().sort());
  assert.deepEqual(manifest.exports, fixture.package.exports);
  for (const name of fixture.package.files) assert.ok(ts.sys.fileExists(join(packageRoot, name)), name);
  for (const [subpath, entry] of Object.entries(fixture.package.exports)) {
    if (typeof entry === "string") continue;
    const binding = entry as { types: string; import: string };
    const name = fixture.package.name + (subpath === "." ? "" : subpath.slice(1));
    const resolved = ts.resolveModuleName(name, join(import.meta.dir, "consumer.ts"), { moduleResolution: ts.ModuleResolutionKind.Bundler, module: ts.ModuleKind.ESNext }, ts.sys).resolvedModule;
    assert.ok(resolved, name);
    assert.equal(realpathSync(resolved.resolvedFileName), realpathSync(join(packageRoot, binding.types)));
  }
  const text = flowBrowserDeclaration();
  const declarationPath = join(import.meta.dir, declarationName);
  const program = ts.createProgram([declarationPath], { noLib: true, noResolve: true });
  assert.equal(program.getSyntacticDiagnostics().length, 0);
  const parsed = program.getSourceFile(declarationPath);
  assert.ok(parsed);
  const session = parsed.statements.find((node) => ts.isClassDeclaration(node) && node.name?.text === "FlowSession");
  assert.ok(session && ts.isClassDeclaration(session));
  const methods = session.members.filter(ts.isMethodDeclaration);
  const names = [...new Set(methods.map((method) => method.name.getText(parsed)).filter((name) => !name.startsWith("[")))];
  const runtime = Object.getOwnPropertyNames(FlowSession.prototype).filter((name) => name !== "constructor").sort();
  assert.deepEqual(names.slice().sort(), runtime);
  assert.equal(names.filter((name) => ![...fixture.canvasMethods, "close", "free"].includes(name)).length, fixture.operationMethods);
  for (const sample of fixture.samples) {
    const method = methods.find((value) => value.name.getText(parsed) === sample.name);
    assert.ok(method);
    assert.deepEqual(method.parameters.map((parameter) => parameter.getText(parsed)), sample.parameters);
    assert.equal(method.type?.getText(parsed), fixture.result);
  }
  for (const name of fixture.excluded) assert.equal(names.includes(name), false);
  for (const mutate of [(value: typeof fixture) => { value.operationMethods = 111; }, (value: typeof fixture) => { value.result = "void"; }, (value: typeof fixture) => { value.extra = true; }, (value: typeof fixture) => { delete value.package.exports["."]; }, (value: typeof fixture) => { value.package.files.push(value.package.files[0]); }]) { const bad = structuredClone(fixture); mutate(bad); assert.equal(validate(bad), false); }
  assert.equal(readFileSync(join(import.meta.dir, declarationName), "utf8"), text);
  console.log(`[DEBUG] Flow browser declarations: ${fixture.operationMethods} schema methods, runtime prototype and TypeScript parser parity; 3 package exports and 2 TypeScript resolutions; 5 hostile fixtures rejected`);
}
//#endregion 🧪️SchemaOracle
