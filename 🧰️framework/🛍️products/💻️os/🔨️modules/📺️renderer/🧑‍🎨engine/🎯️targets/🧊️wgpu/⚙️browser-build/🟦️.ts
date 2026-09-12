import { existsSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { getWorkspaceRoot } from "../../../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { MODULE_ROUTES } from "../../../../../🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { pluginModulesRoot } from "../../../../../🧑‍💻dev/♻️activation/🟦️.ts";

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

let bundleLane: Promise<void> = Promise.resolve();

/** @emoji 🧾️ Bundles one browser entry entirely in memory for identical generate/check bytes.
 * Bun labels every bundled module with a banner comment spelling that module's path RELATIVE TO
 * `process.cwd()` — the `root` build option does not govern it — so the identical sources rendered
 * from an Nx target's project cwd and from a repo-root dev lane differ in thousands of comment bytes
 * and the generate/check pair can never agree (measured: 316 differing lines, `🧰️framework/…` against
 * `../../../../../../../../../…`). The render therefore always runs with the process anchored at the
 * workspace root — the one cwd every caller can name — and restores the caller's cwd afterwards.
 * Renders are serialized through {@link bundleLane} so a concurrent caller never observes the
 * anchored cwd of another render. */
export async function renderBrowserEntry(entryPath: string): Promise<string> {
  assertPinnedBunVersion();
  const rendered = bundleLane.then(() => bundleAtWorkspaceRoot(resolve(entryPath)));
  bundleLane = rendered.then(
    () => undefined,
    () => undefined,
  );
  return await rendered;
}

async function bundleAtWorkspaceRoot(entryPath: string): Promise<string> {
  const runtime = globalThis as typeof globalThis & {
    Bun: { build(options: { entrypoints: string[]; target: "browser"; format: "esm"; define: Record<string, string> }): Promise<{ success: boolean; logs: unknown[]; outputs: { text(): Promise<string> }[] }> };
  };
  const callerCwd = process.cwd();
  process.chdir(repoRoot);
  try {
    const result = await runtime.Bun.build({ entrypoints: [entryPath], target: "browser", format: "esm", define: { "import.meta.vitest": "undefined" } });
    if (!result.success || result.outputs.length !== 1) throw new Error(`browser bundle render failed for ${entryPath}: ${result.logs.map(String).join("\n")}`);
    return decodeAstralEscapes(await result.outputs[0]!.text());
  } finally {
    process.chdir(callerCwd);
  }
}

/** 🚀️ Renders the browser boot artifact without changing generated files. */
export async function renderBrowserBoot(bundleRoot: string): Promise<{ path: string; content: string }> {
  const bootTs = join(bundleRoot, "../../🚀️browser-boot/🟦️.ts");
  const bootJs = join(bundleRoot, "../../🚀️browser-boot/🤖️generated/🟨️.js");
  return { path: bootJs, content: await renderBrowserEntry(bootTs) };
}

/** 🔎️ Checks generated boot bytes without creating or rewriting the artifact. */
export async function checkBrowserBoot(bundleRoot: string): Promise<void> {
  const artifact = await renderBrowserBoot(bundleRoot);
  if (!existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content) throw new Error("🚀️boot.js is missing or stale; run the generate-browser-boot target");
}

/** 🗺️ Every `copy-dir` directive of the trunk bundle document, keyed by the served path it publishes. */
export function bundleCopyDirectives(documentText: string): ReadonlyMap<string, string> {
  const directives = new Map<string, string>();
  for (const [, attributes] of documentText.matchAll(/<link\s([^>]*)>/gu)) {
    if (!/\bdata-trunk\b/u.test(attributes) || !/\brel\s*=\s*"copy-dir"/u.test(attributes)) continue;
    const href = /\bhref\s*=\s*"([^"]*)"/u.exec(attributes)?.[1];
    const target = /\bdata-target-path\s*=\s*"([^"]*)"/u.exec(attributes)?.[1];
    if (href && target) directives.set(`/${target}`, href);
  }
  return directives;
}

/** 🚏️ The dev session publishes module descriptors and wasm under TWO routes
 * (`MODULE_ROUTES.plugin` and `MODULE_ROUTES.extension`); the React dev server mounts both, and the
 * trunk bundle must carry both as `copy-dir` targets or a descriptor fetch under the missing one is
 * answered by trunk's SPA index instead of 404 — `plugin.descriptor-invalid: … returned HTML`, which
 * leaves the scoped `setContributions` pack empty and no flow operator installed
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Fails the build rather than the boot. */
export function assertBundleModuleRoutes(bundleRoot: string, routes: readonly string[] = Object.values(MODULE_ROUTES)): void {
  const documentPath = join(bundleRoot, "🌐️.html");
  const directives = bundleCopyDirectives(readFileSync(documentPath, "utf8"));
  for (const route of routes) {
    const href = directives.get(route);
    if (!href) throw new Error(`🌐️.html carries no copy-dir for module route ${route}; a fetch under it would fall through to the SPA index`);
    if (!existsSync(resolve(bundleRoot, href))) throw new Error(`🌐️.html copy-dir for module route ${route} points at a missing directory ${href}`);
  }
  // 🔌️ The plugin route must copy THE staging root, never a second tree: the bundle is a dev-profile
  // artifact, and a `copy-dir` aimed anywhere else is exactly how this bundle served a two-day-old guest
  // while every producer wrote the canonical root (26/09/09/PROCEDURAL-3D-END-TO-END).
  const pluginHref = directives.get(MODULE_ROUTES.plugin);
  const expected = pluginModulesRoot("dev");
  if (pluginHref && resolve(bundleRoot, pluginHref) !== expected) {
    throw new Error(`🌐️.html copy-dir for ${MODULE_ROUTES.plugin} must resolve to the one staging root ${expected}, not ${resolve(bundleRoot, pluginHref)}`);
  }
}
