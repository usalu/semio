/** 📦️ Package assets in the browser: a plugin's descriptor declares files by name, exact size and SHA-256 (`PluginBuilder::asset`),
 * and the kernel delivers their bytes on the guest's `instance-open` (`events::instance-open-event.assets`), the same contract the
 * native hosts keep (`🌉️mcp/🏠️workspace/📦️package-assets`). A declared name is the repository-relative path of its file; the
 * page fetches it from {@link PACKAGE_ASSET_ROUTE} when the name lies below a directory in {@link PACKAGE_ASSET_DIRECTORIES}, which
 * is exactly what a dev server mounts, so nothing outside those directories is ever requested or served. */
import type { ShardAsset } from "@semio-tech/framework-actor/shard-client";
import { RUNTIME_ASSET_CHANNELS } from "../../🖼️canvas/📦️packages/🟦️typescript/🔌️runtime-assets/🟦️.ts";

/** 🚏️ The route package asset directories are served under; a directory is served at `${route}/${directory}`. */
export const PACKAGE_ASSET_ROUTE = "/renderer-modules/package-assets";

/** 📂️ The repository directories whose files a plugin may declare as package assets: the shared font catalog and the three canvas
 * runtime asset directories (the compiler fonts, the label face and the icon pack, `PluginBuilder::canvas_runtime_assets`). */
export const PACKAGE_ASSET_DIRECTORIES: readonly string[] = ["🧰️framework/🔨️modules/🖼️assets/🔤️fonts", ...RUNTIME_ASSET_CHANNELS.map(({ directory }) => directory)];

/** 🏷️ One declared package asset as the descriptor's JSON carries it (`AssetDeclaration`, camelCase). */
export interface PackageAssetDeclaration {
  readonly name: string;
  readonly sizeBytes: number;
  readonly sha256: string;
}

/** 🚏️ The static-directory mounts a dev server needs so every declared directory answers at its route: `[route, directory]`. */
export function packageAssetMounts(): readonly (readonly [string, string])[] {
  return PACKAGE_ASSET_DIRECTORIES.map(directory => [`${PACKAGE_ASSET_ROUTE}/${directory}`, directory] as const);
}

/** 🚏️ [`packageAssetMounts`] as the `static-dir` deliveries a repository dev server mounts (`root` is repository-relative). */
export function packageAssetDeliveries(): readonly { readonly kind: "static-dir"; readonly route: string; readonly root: string }[] {
  return packageAssetMounts().map(([route, root]) => ({ kind: "static-dir" as const, route, root }));
}

/** 🔗️ The URL one declared name is fetched from, or `undefined` for a name that lies in no served directory (it is not a file
 * asset this page can deliver, like an externalized example body). */
export function packageAssetUrl(name: string, base: string): string | undefined {
  const segments = name.split("/");
  if (segments.some(segment => segment === "" || segment === "." || segment === "..")) return undefined;
  if (!PACKAGE_ASSET_DIRECTORIES.some(directory => name.startsWith(directory + "/"))) return undefined;
  return new URL(`${PACKAGE_ASSET_ROUTE}/${segments.map(encodeURIComponent).join("/")}`, base).href;
}

/** #️⃣ Lowercase hex SHA-256 of `bytes`. */
export async function sha256Hex(bytes: ArrayBuffer): Promise<string> {
  const digest = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
  return Array.from(digest, byte => byte.toString(16).padStart(2, "0")).join("");
}

/** 📥️ The bytes of every declared asset that lies in a served directory, as `[name, bytes]` in declaration order, each checked
 * against its declared size and SHA-256 before it is handed on. A failed fetch or a mismatch rejects naming the asset, because a
 * guest must never run on bytes its descriptor did not declare. */
export async function fetchPackageAssets(declared: readonly PackageAssetDeclaration[], base: string, fetchAsset: (url: string) => Promise<Response>, digest: (bytes: ArrayBuffer) => Promise<string> = sha256Hex): Promise<readonly ShardAsset[]> {
  const delivered: ShardAsset[] = [];
  for (const declaration of declared) {
    const url = packageAssetUrl(declaration.name, base);
    if (url === undefined) continue;
    const response = await fetchAsset(url);
    if (!response.ok) throw new Error(`package-assets: ${response.status} for ${declaration.name}`);
    const bytes = await response.arrayBuffer();
    if (bytes.byteLength !== declaration.sizeBytes) throw new Error(`package-assets: ${declaration.name} has ${bytes.byteLength} bytes, its declaration says ${declaration.sizeBytes}`);
    const sha256 = await digest(bytes);
    if (sha256 !== declaration.sha256) throw new Error(`package-assets: ${declaration.name} hashes to ${sha256}, its declaration says ${declaration.sha256}`);
    delivered.push([declaration.name, bytes]);
  }
  return delivered;
}

/** 🔍️ Reads the `assets` array of a package descriptor document, admitting only well-formed rows (name, whole-number size,
 * 64-digit hex SHA-256); anything else is refused so a malformed declaration cannot reach a fetch. */
export function readPackageAssetDeclarations(descriptor: object): readonly PackageAssetDeclaration[] {
  const raw = (descriptor as { readonly assets?: unknown }).assets;
  if (raw === undefined) return [];
  if (!Array.isArray(raw)) throw new Error("package-assets: descriptor assets is not a list");
  return raw.map(row => {
    const candidate = row as { readonly name?: unknown; readonly sizeBytes?: unknown; readonly sha256?: unknown };
    if (typeof candidate.name !== "string" || candidate.name.length === 0 || typeof candidate.sizeBytes !== "number" || !Number.isSafeInteger(candidate.sizeBytes) || candidate.sizeBytes < 0 || typeof candidate.sha256 !== "string" || !/^[0-9a-f]{64}$/u.test(candidate.sha256)) throw new Error("package-assets: malformed asset declaration");
    return { name: candidate.name, sizeBytes: candidate.sizeBytes, sha256: candidate.sha256 };
  });
}
