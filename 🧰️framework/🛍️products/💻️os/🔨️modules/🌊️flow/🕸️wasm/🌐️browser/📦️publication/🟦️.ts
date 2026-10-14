import type { ScriptInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readWasmPackageOwnerV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🕸️wasm-build/🟦️.ts";
import { collectArtifactFiles } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🗂️files/🟦️.ts";
import { stageRepositoryArtifacts } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { readBuildMode } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { bundleVirtualJavaScript } from "../../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️bundle/🟦️.ts";
import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
/** 📦️ Publishes the Flow browser, host and declaration projections beside wasm-bindgen output. */
import { copyFileSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { flowBrowserDeclaration, writeFlowBrowserDeclaration } from "../📝️declaration/📤️projection/🟦️.ts";

const browserSource = join(import.meta.dir, "../🏃️runtime/🟨️.js");
const hostSource = join(import.meta.dir, "../../🖥️host/🏃️runtime/🟨️.js");
const corePackageRoot = join(import.meta.dir, "../../../🫀️core/🕸️bindings");
const nativeFiles = ["flow_core_bg.wasm", "flow_core.js", "flow_core.d.ts", "flow_core_bg.wasm.d.ts"] as const;
const packageFiles = [...nativeFiles, "🌐️browser/🟨️.js", "🖥️host/🟨️.js", "🌐️browser/🟦️.d.ts"] as const;
const packageExports = { ".": { types: "./flow_core.d.ts", import: "./flow_core.js" }, "./🌐️flow-browser.js": { types: "./🌐️browser/🟦️.d.ts", import: "./🌐️browser/🟨️.js" }, "./🖥️flow-host.js": "./🖥️host/🟨️.js" } as const;

type FlowBrowserBundle = readonly Readonly<{ text(): Promise<string> }>[];

function writePackageManifest(packageRoot: string, source?: Record<string, unknown>): void {
  const path = join(packageRoot, "package.json");
  const manifest = source ?? (existsSync(path) ? JSON.parse(readFileSync(path, "utf8")) : {});
  manifest.type = "module";
  manifest.version ??= "0.1.0";
  manifest.sideEffects ??= ["./snippets/*"];
  manifest.name = "@semio-tech/flow-core";
  manifest.files = [...packageFiles];
  manifest.main = "flow_core.js";
  manifest.module = "flow_core.js";
  manifest.types = "flow_core.d.ts";
  manifest.exports = packageExports;
  writeFileSync(path, `${JSON.stringify(manifest, null, 2)}\n`, "utf8");
}

/** 🌐️ Bundles the browser entry while retaining only its native wrapper and owned host as package-relative externals. */
export async function bundleFlowBrowserModule(write: boolean, invocation: ScriptInvocation, packageRoot = corePackageRoot): Promise<FlowBrowserBundle> {
  const source = readFileSync(browserSource, "utf8");
  const nativeInitializer = 'import("../../../🫀️core/🕸️bindings/flow_core.js")';
  const hostInitializer = '"../../🖥️host/🏃️runtime/🟨️.js"';
  if (source.split(nativeInitializer).length !== 2 || source.split(hostInitializer).length !== 2) throw new Error("Flow browser source requires one native initializer and one owned host binding");
  const contents = source.replace(nativeInitializer, 'import("../flow_core.js")').replace(hostInitializer, '"../🖥️host/🟨️.js"');
  const workspace = getWorkspaceRoot(), root = process.env.SEMIO_TICKET_DIR ? join(process.env.SEMIO_TICKET_DIR, "🗑️generated") : join(workspace, ".🧬semio/🦑️repo/⚡️cache/bun-workers");
  const result = await bundleVirtualJavaScript({ version: 1, kind: "module", mode: readBuildMode(process.env.SEMIO_BUILD_MODE), entrypoint: browserSource, contents, external: ["../flow_core.js", "../🖥️host/🟨️.js"] }, { workspace, temporaryRoot: root, invocation });
  if (write) { mkdirSync(join(packageRoot, "🌐️browser"), { recursive: true }); writeFileSync(join(packageRoot, "🌐️browser/🟨️.js"), result.source); }
  return [{ text: async () => result.source }];
}

/** 👁️ Previews every package projection from current wasm-bindgen output without publishing files. */
export async function previewFlowBrowserPackage(familyPackageRoot: string, invocation: ScriptInvocation): Promise<Readonly<{ browserBytes: number; declarationBytes: number; files: readonly string[] }>> {
  for (const name of nativeFiles) if (!existsSync(join(familyPackageRoot, name))) throw new Error(`flow-core package preview is missing ${name}`);
  const outputs = await bundleFlowBrowserModule(false, invocation);
  if (outputs.length !== 1) throw new Error(`flow-core browser preview emitted ${outputs.length} outputs`);
  return { browserBytes: Buffer.byteLength(await outputs[0].text()), declarationBytes: Buffer.byteLength(flowBrowserDeclaration()), files: packageFiles };
}

/** 📝️ Writes the source and package declaration projections and assembles their package exports. */
export function publishFlowBrowserDeclarations(packageRoot = corePackageRoot): void {
  writeFlowBrowserDeclaration();
  mkdirSync(join(packageRoot, "🌐️browser"), { recursive: true });
  writeFileSync(join(packageRoot, "🌐️browser", "🟦️.d.ts"), flowBrowserDeclaration(), "utf8");
  writePackageManifest(packageRoot);
}

/** 🌱️ Writes the static `@semio-tech/flow-core` manifest a fresh clone lacks: the package is a root workspace member,
 * and `bun install` refuses the whole workspace while a member directory is missing. The wasm-bindgen payload stays with
 * [[publishFlowBrowserPackage]]. Declared in `⚡️caching/🚀️bootstrap/🌱️sources/🔣️.json`. */
export function bootstrapFlowCorePackage(): void {
  if (existsSync(join(corePackageRoot, "package.json"))) return;
  mkdirSync(corePackageRoot, { recursive: true });
  writePackageManifest(corePackageRoot);
}

/** 📤️ Copies native output, publishes browser projections and preserves wasm-bindgen snippets as one package. */
export async function publishFlowBrowserPackage(familyPackageRoot: string, invocation: ScriptInvocation, packageRoot = corePackageRoot): Promise<void> {
  const signal = invocation.control.signal;
  if (!existsSync(familyPackageRoot)) throw new Error(`flow-core wasm build did not emit ${familyPackageRoot}`);
  const familyManifestPath = join(familyPackageRoot, "package.json");
  if (!existsSync(familyManifestPath)) throw new Error("flow-core wasm build did not emit its transient compiler manifest");
  let familyManifest = JSON.parse(readFileSync(familyManifestPath, "utf8"));
  const metadata = readWasmPackageOwnerV1(familyManifest.semioWasmOwner);
  const canonical = join(getWorkspaceRoot(), metadata.sourceOwner, "dist/wasm", readBuildMode(process.env.SEMIO_BUILD_MODE));
  familyPackageRoot = canonical; familyManifest = JSON.parse(readFileSync(join(canonical, "package.json"), "utf8"));
  for (const name of nativeFiles) if (!existsSync(join(familyPackageRoot, name))) throw new Error(`flow-core wasm build did not emit ${name}`);
  const parent = dirname(packageRoot);
  mkdirSync(parent, { recursive: true });
  const transactionRoot = mkdtempSync(join(parent, ".flow-browser-publication-"));
  const staging = join(transactionRoot, "next");
  mkdirSync(staging);
  try {
    for (const name of nativeFiles) copyFileSync(join(familyPackageRoot, name), join(staging, name));
    writePackageManifest(staging, familyManifest);
    mkdirSync(join(staging, "🖥️host"), { recursive: true });
    copyFileSync(hostSource, join(staging, "🖥️host", "🟨️.js"));
    await bundleFlowBrowserModule(true, invocation, staging);
    mkdirSync(join(staging, "🌐️browser"), { recursive: true });
    writeFileSync(join(staging, "🌐️browser", "🟦️.d.ts"), flowBrowserDeclaration(), "utf8");
    const snippets = join(familyPackageRoot, "snippets");
    if (existsSync(snippets)) cpSync(snippets, join(staging, "snippets"), { recursive: true });
    writeFlowBrowserDeclaration();
    const files = await collectArtifactFiles(staging, signal);
    {
      const owner = JSON.parse(readFileSync(join(canonical, ".semio-artifact.json"), "utf8")).owner;
      await stageRepositoryArtifacts(canonical, owner, files, signal ? { signal } : {});
      await stageRepositoryArtifacts(packageRoot, "flow-browser:active", files, signal ? { signal } : {});
      rmSync(transactionRoot, { recursive: true, force: true });
    }
  } catch (error) {
    rmSync(transactionRoot, { recursive: true, force: true });
    throw error;
  }
}
