import { createReadStream } from "node:fs";
import { open, unlink } from "node:fs/promises";
import { pipeline } from "node:stream/promises";
import { createGzip } from "node:zlib";

import { isBrowserCoreFactory } from "../../🏗️materialization/🧬️jco-module-rewrites/🟦️.ts";
export { isBrowserCoreFactory };

export const browserCoreName = /\.core\d*\.wasm$/u;
export const gzipCoreSuffix = ".gz.bin";

/** 🗜️ Streams an unchanged component core into an opaque gzip payload for static hosts. */
export async function compressBrowserCore(input: string, output: string, signal?: AbortSignal): Promise<void> {
  signal?.throwIfAborted();
  const target = await open(output, "wx");
  try {
    await pipeline(createReadStream(input), createGzip(), target.createWriteStream(), {signal});
  } catch (error) {
    await target.close();
    await unlink(output);
    throw error;
  }
}

/** 📦️ Selects the encoded siblings of a component binding and decodes them before compilation. */
export function gzipCoreLoader(source: string, cores: readonly string[]): string {
  if (isBrowserCoreFactory(source)) {
    const remaining = new Set(cores);
    const encoded = source.replace(/getCoreModule\((['"])([^'"]+\.core\d*\.wasm)\1\)/gu, (_call, quote: string, name: string) => {
      if (!cores.includes(name)) throw new Error("Missing staged core for factory request: " + name);
      remaining.delete(name);
      return "getCoreModule(" + quote + name + gzipCoreSuffix + quote + ")";
    });
    if (remaining.size) throw new Error("Missing factory request for component core: " + [...remaining].join(", "));
    return encoded;
  }
  const fetchCore = "return fetch(url).then(WebAssembly.compileStreaming);";
  const fileCore = "return WebAssembly.compile(await _fs.readFile(url));";
  if (source.split(fetchCore).length !== 2 || source.split(fileCore).length !== 2) throw new Error("Unrecognized browser core loader");
  let result = source;
  for (const core of cores) {
    const quoted = ["'", '"'].map(quote => quote + "./" + core + quote);
    if (!quoted.some(token => result.includes(token))) throw new Error("Missing core URL in binding: " + core);
    for (const token of quoted) result = result.replaceAll(token, token.slice(0, -1) + gzipCoreSuffix + token.slice(-1));
  }
  return result.replace(fileCore, `const { gunzipSync } = await import("node:zlib");
    return WebAssembly.compile(gunzipSync(await _fs.readFile(url)));`).replace(fetchCore, `const response = await fetch(url);
  if (!response.ok || !response.body) throw new Error("Component core fetch failed (" + response.status + "): " + url);
  const bytes = response.body.pipeThrough(new DecompressionStream("gzip"));
  return WebAssembly.compileStreaming(new Response(bytes, { headers: { "Content-Type": "application/wasm" } }));`);
}
