/** 📝️ Projects the owned Flow browser ABI into its public TypeScript declaration. */
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";

type Field = { name: string; type: "utf8" | "optional-utf8" | "f64" | "u64" | "u32" | "u8" | "bool" | "bytes" };
type Contract = { operations: Record<string, number>; arguments: Record<string, Field[]> };

const excluded = new Set(["open", "attachSurface", "renderFrame"]);
const fieldTypes: Record<Field["type"], string> = { utf8: "string", "optional-utf8": "string | null", f64: "number", u64: "number", u32: "number", u8: "number", bool: "boolean", bytes: "ArrayBufferView | readonly number[]" };
export const FLOW_WASM_ABI_URL = new URL("../../../🧬️schema/📡️abi/🔣️.json", import.meta.url);
const declarationUrl = new URL("../🤖️generated/🟦️.d.ts", import.meta.url);

/** 🧬️ Renders the declaration surface without loading tests or compiler output. */
export function flowBrowserDeclaration(): string {
  const contract: Contract = JSON.parse(readFileSync(FLOW_WASM_ABI_URL, "utf8"));
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

/** 💾️ Writes the tracked source declaration projection and returns its exact path. */
export function writeFlowBrowserDeclaration(): string {
  const path = fileURLToPath(declarationUrl);
  writeFileSync(path, flowBrowserDeclaration(), "utf8");
  return path;
}
