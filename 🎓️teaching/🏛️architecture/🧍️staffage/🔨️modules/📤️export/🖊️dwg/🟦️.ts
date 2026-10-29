/** 🖊️ DWG export: genuine AC1024 produced by the in-repo Rust writer (`📦️packages/🦀️rust`), reached as a raw C-ABI WebAssembly module
 * (no wasm-bindgen) in the browser and in Node, or as the `staffage-dwg` CLI through {@link configureDwg}. When neither is present the
 * result is an explicit `unavailable` naming the missing artifact; a DXF is never renamed to DWG.
 * <https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf> */
import type { JobControl, ModelScene } from "../../🔗️contracts/🟦️.ts";
import type { Text } from "../../../🧬️schema/🟦️.ts";

/** 🧩️ Where the encoder comes from. */
export type DwgSource = { readonly wasm?: URL | string | BufferSource; readonly cli?: (json: string) => Promise<Uint8Array> };

/** 🔎️ Whether DWG can be written here, and if not exactly which dependency is missing. */
export type DwgAvailability = { readonly available: true; readonly via: "wasm" | "cli" } | { readonly available: false; readonly missing: string; readonly reason: Text };

/** 🚫️ Thrown by {@link writeDwg} when the encoder is not available. */
export class DwgUnavailableError extends Error {
  constructor(readonly missing: string, readonly reason: Text) {
    super(`DWG unavailable: ${missing}`);
    this.name = "DwgUnavailableError";
  }
}

/** 📍️ The default location of the compiled encoder, next to this module. */
export const DWG_WASM_URL = new URL("./staffage_dwg.wasm", import.meta.url);

type Exports = { readonly memory: WebAssembly.Memory; readonly staffage_alloc: (length: number) => number; readonly staffage_free: (pointer: number, length: number) => void; readonly staffage_encode: (pointer: number, length: number) => number; readonly staffage_result_ptr: () => number; readonly staffage_result_len: () => number };
type Encoder = { readonly via: "wasm" | "cli"; readonly encode: (json: string) => Promise<Uint8Array> };

let source: DwgSource = {};
let encoder: Promise<Encoder | DwgAvailability & { available: false }> | undefined;

/** ⚙️ Selects the encoder source (a wasm URL/bytes, or a CLI runner); resets the cached instance. */
export function configureDwg(next: DwgSource): void {
  source = next;
  encoder = undefined;
}

const missing = (what: string, en: string, de: string): DwgAvailability & { available: false } => ({ available: false, missing: what, reason: { en, de } });

async function wasmBytes(location: URL | string | BufferSource): Promise<Uint8Array> {
  if (typeof location !== "string" && !(location instanceof URL)) return ArrayBuffer.isView(location) ? new Uint8Array(location.buffer, location.byteOffset, location.byteLength) : new Uint8Array(location);
  const url = new URL(location, import.meta.url);
  if (url.protocol === "file:") {
    const specifier = "node:fs/promises";
    const { readFile } = (await import(/* @vite-ignore */ specifier)) as { readFile: (path: URL) => Promise<Uint8Array> };
    return readFile(url);
  }
  const response = await fetch(url);
  if (!response.ok) throw new Error(`${url.href}: HTTP ${response.status}`);
  return new Uint8Array(await response.arrayBuffer());
}

async function wasmEncoder(bytes: Uint8Array): Promise<Encoder> {
  const { instance } = (await WebAssembly.instantiate(bytes as BufferSource, {})) as unknown as WebAssembly.WebAssemblyInstantiatedSource;
  const api = instance.exports as unknown as Exports;
  const text = new TextEncoder();
  return {
    via: "wasm",
    encode: async (json: string): Promise<Uint8Array> => {
      const input = text.encode(json);
      const pointer = api.staffage_alloc(input.length);
      new Uint8Array(api.memory.buffer, pointer, input.length).set(input);
      const status = api.staffage_encode(pointer, input.length);
      api.staffage_free(pointer, input.length);
      const result = new Uint8Array(api.memory.buffer, api.staffage_result_ptr(), api.staffage_result_len()).slice();
      if (status !== 0) throw new Error(`DWG encoder: ${new TextDecoder().decode(result)}`);
      return result;
    },
  };
}

async function resolve(): Promise<Encoder | (DwgAvailability & { available: false })> {
  if (source.cli) return { via: "cli", encode: source.cli };
  const location = source.wasm ?? DWG_WASM_URL;
  try {
    return await wasmEncoder(await wasmBytes(location));
  } catch (error) {
    const where = typeof location === "string" || location instanceof URL ? String(location) : "the supplied wasm bytes";
    return missing(`${where} (staffage_dwg.wasm, built from 🖊️dwg/📦️packages/🦀️rust for wasm32-unknown-unknown) or a staffage-dwg CLI runner via configureDwg({ cli })`, `The DWG encoder module could not be loaded: ${(error as Error).message}`, `Das DWG-Encoder-Modul konnte nicht geladen werden: ${(error as Error).message}`);
  }
}

/** 🔎️ Probes the encoder without writing anything. */
export async function dwgAvailability(): Promise<DwgAvailability> {
  encoder ??= resolve();
  const found = await encoder;
  return "encode" in found ? { available: true, via: found.via } : found;
}

/** 🖊️ Writes `scene` as a genuine AC1024 DWG; throws {@link DwgUnavailableError} when the encoder is missing. */
export async function writeDwg(scene: ModelScene, control?: JobControl): Promise<Uint8Array> {
  control?.signal?.throwIfAborted();
  encoder ??= resolve();
  const found = await encoder;
  if (!("encode" in found)) throw new DwgUnavailableError(found.missing, found.reason);
  control?.progress?.(0, 1, { en: "Encoding DWG", de: "DWG wird kodiert" });
  const bytes = await found.encode(JSON.stringify(scene));
  control?.signal?.throwIfAborted();
  control?.progress?.(1, 1, { en: "Encoding DWG", de: "DWG wird kodiert" });
  return bytes;
}
