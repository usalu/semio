import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { getWorkspaceRoot } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";

const repoRoot = getWorkspaceRoot();

/** @emoji 🥖️ Rejects browser code generation unless it uses the repository's exact Bun toolchain. */
export function assertPinnedBunVersion(actualVersion: string = Bun.version): string {
  const packageManager = (JSON.parse(readFileSync(join(repoRoot, "package.json"), "utf8")) as { readonly packageManager?: string }).packageManager ?? "";
  const expectedVersion = /^bun@(\d+\.\d+\.\d+)$/u.exec(packageManager)?.[1];
  if (!expectedVersion) throw new Error(`wgpu-frame-worker requires an exact root packageManager bun@x.y.z pin; received ${JSON.stringify(packageManager)}`);
  if (actualVersion !== expectedVersion) throw new Error(`wgpu-frame-worker requires Bun ${expectedVersion} from root packageManager, received ${actualVersion}`);
  return expectedVersion;
}

/** @emoji 🔓️ `Bun.build({ target: "browser" })` escapes every astral-plane (non-BMP) code point in
 * string/template literal text to a `\uXXXX\uXXXX` UTF-16 surrogate-pair escape (this repo's kind
 * and subject taxonomy emoji all live in the astral planes, so any embedded taxonomy path gets
 * mangled this way); BMP characters are left as literal UTF-8. The escape and the literal character
 * are byte-identical to any JS engine — decoding is a lossless, purely syntactic no-op — but the
 * escaped form hides taxonomy path tokens from the reference scanner, which reads physical bytes,
 * not evaluated string values. Undoing exactly this (and only this) transformation keeps the
 * generated bundle scannable/rewritable without touching anything the bundler did for a real reason
 * (minification, tree-shaking, module wrapping, …). */
export function decodeAstralEscapes(text: string): string {
  return text.replace(/\\u(d[89ab][0-9a-f]{2})\\u(d[c-f][0-9a-f]{2})/gi, (_match, hi: string, lo: string) => String.fromCharCode(parseInt(hi, 16), parseInt(lo, 16)));
}

/** @emoji 🧾️ Bundles one browser entry entirely in memory for identical generate/check bytes. */
export async function renderBrowserEntry(entryPath: string): Promise<string> {
  assertPinnedBunVersion();
  const runtime = globalThis as typeof globalThis & {
    Bun: { build(options: { entrypoints: string[]; target: "browser"; format: "esm"; define: Record<string, string> }): Promise<{ success: boolean; logs: unknown[]; outputs: { text(): Promise<string> }[] }> };
  };
  const result = await runtime.Bun.build({ entrypoints: [entryPath], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" } });
  if (!result.success || result.outputs.length !== 1) throw new Error(`browser bundle render failed for ${entryPath}: ${result.logs.map(String).join("\n")}`);
  return decodeAstralEscapes(await result.outputs[0]!.text());
}

/** 🚀️ Renders the browser boot artifact without changing generated files. */
export async function renderBrowserBoot(bundleRoot: string): Promise<{ path: string; content: string }> {
  const bootTs = join(bundleRoot, "../../🚀️browser-boot/🟦️.ts");
  const bootJs = join(bundleRoot, "🟦️typescript/🚀️boot.js");
  return { path: bootJs, content: await renderBrowserEntry(bootTs) };
}

/** 🔎️ Checks generated boot bytes without creating or rewriting the artifact. */
export async function checkBrowserBoot(bundleRoot: string): Promise<void> {
  const artifact = await renderBrowserBoot(bundleRoot);
  if (!existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content) throw new Error("🚀️boot.js is missing or stale; run the generate-browser-boot target");
}

