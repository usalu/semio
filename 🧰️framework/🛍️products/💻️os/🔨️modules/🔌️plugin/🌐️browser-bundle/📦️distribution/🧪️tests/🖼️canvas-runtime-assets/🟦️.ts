import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, mkdtempSync, readFileSync, readdirSync, realpathSync, rmSync, statSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, relative, resolve } from "node:path";
import { stageArtifacts } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/📤️publication/🟦️.ts";
import { copyBrowserArtifacts } from "../../🟦️.ts";
import { PACKAGE_ASSET_ROUTE, packageAssetUrl } from "../../../../../../../../🔨️modules/🎠️kernel/📦️packages/🟦️typescript/📦️package-assets/🟦️.ts";
import { GUEST_RUNTIME_ASSET_FILES, RUNTIME_ASSET_CHANNELS, RUNTIME_ASSET_FILES } from "@semio-tech/framework-canvas-ts/runtime-assets";
import { PACKAGE_ASSET_DESTINATION, PACKAGE_ASSET_STAGE_OWNER, RUNTIME_ASSET_STAGE_OWNER, canvasRuntimeAssetSources, runtimeAssetStageTrees } from "@semio-tech/framework-canvas-ts/runtime-assets/staging";

type Fixture = { channels: { id: string; route: string; directory: string }[]; assets: { role: string; channel: string; file: string; bytes?: number; sha256?: string }[]; guestAssets: string[]; iconPackDigestFile: string };

const repository = resolve(import.meta.dir, "../../../../../../../../../");
const fixture = JSON.parse(readFileSync(resolve(import.meta.dir, "../../../../../../../../🔨️modules/🖼️canvas/🔌️runtime-assets/🧪️tests/🧫️fixtures/🔣️.json"), "utf8")) as Fixture;
const sha256 = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
const directoryOf = (channel: string): string => fixture.channels.find((candidate) => candidate.id === channel)!.directory;
const digest = JSON.parse(readFileSync(join(repository, directoryOf("icon-pack"), fixture.iconPackDigestFile), "utf8")) as { sizeBytes: number; sha256: string };
const expectedHash = (asset: Fixture["assets"][number]): string => (asset.role === "icon-pack" ? digest.sha256 : asset.sha256!);

function listFiles(root: string): string[] {
  return readdirSync(root, { recursive: true, withFileTypes: true }).filter((entry) => entry.isFile()).map((entry) => relative(root, join(entry.parentPath, entry.name)).replaceAll("\\", "/")).sort();
}

/** 🏗️ Runs the staging command's work and the bundle copy production and play run, against a temporary workspace that reads the real repository files. */
async function buildBundle(): Promise<{ workspace: string; bundle: string }> {
  const workspace = mkdtempSync(join(realpathSync(tmpdir()), "canvas-bundle-"));
  for (const tree of runtimeAssetStageTrees(join(workspace, "products"), "prototype")) {
    if (!tree.files.length) continue;
    await stageArtifacts(tree.root, tree.owner, new Map(tree.files.map(({ name, source }) => [name, join(repository, source)])), { leaseDirectory: join(workspace, "leases") });
  }
  const bundle = join(workspace, "bundle");
  await copyBrowserArtifacts(bundle, canvasRuntimeAssetSources(join(workspace, "products"), "prototype"));
  return { workspace, bundle };
}

test("the stage trees mirror the port catalog, its fixture and the guest subset", () => {
  expect(RUNTIME_ASSET_CHANNELS.map(({ id, route, directory }) => ({ id, route, directory }))).toEqual(fixture.channels);
  expect(RUNTIME_ASSET_FILES.map(({ file }) => file)).toEqual(fixture.assets.map(({ file }) => file));
  expect(GUEST_RUNTIME_ASSET_FILES.map(({ file }) => file)).toEqual(fixture.guestAssets.map((role) => fixture.assets.find((asset) => asset.role === role)!.file));
  const trees = runtimeAssetStageTrees(resolve("/products"), "prototype");
  expect(trees.filter((tree) => tree.owner === RUNTIME_ASSET_STAGE_OWNER).flatMap((tree) => tree.files.map(({ name }) => name))).toEqual(fixture.assets.map(({ file }) => file));
  expect(trees.filter((tree) => tree.owner === PACKAGE_ASSET_STAGE_OWNER).flatMap((tree) => tree.files.map(({ name }) => name))).toEqual(GUEST_RUNTIME_ASSET_FILES.map(({ file }) => file));
  expect(new Set(trees.map((tree) => tree.destination)).size).toBe(trees.length);
  expect(trees.every((tree) => !tree.root.includes("runtime") && !tree.root.includes("activation"))).toBe(true);
  expect(`/${PACKAGE_ASSET_DESTINATION}`).toBe(PACKAGE_ASSET_ROUTE);
});

test("a built bundle serves every port asset at its route and every guest asset under its package asset name, each with the catalog's hash", async () => {
  const { workspace, bundle } = await buildBundle();
  try {
    for (const asset of fixture.assets) {
      const channel = fixture.channels.find((candidate) => candidate.id === asset.channel)!;
      const served = join(bundle, channel.route.slice(1), asset.file);
      expect(existsSync(served), `${channel.route}/${asset.file}`).toBe(true);
      const bytes = readFileSync(served);
      expect(sha256(bytes), asset.file).toBe(expectedHash(asset));
      expect(sha256(bytes)).toBe(sha256(readFileSync(join(repository, channel.directory, asset.file))));
      if (asset.role === "icon-pack") expect(bytes.byteLength).toBe(digest.sizeBytes);
      else expect(bytes.byteLength).toBe(asset.bytes!);
    }
    const guest = fixture.guestAssets.map((role) => fixture.assets.find((asset) => asset.role === role)!);
    for (const asset of guest) {
      const name = `${directoryOf(asset.channel)}/${asset.file}`;
      const served = join(bundle, PACKAGE_ASSET_DESTINATION, name);
      expect(existsSync(served), name).toBe(true);
      expect(sha256(readFileSync(served)), name).toBe(expectedHash(asset));
      expect(packageAssetUrl(name, "http://page.test/"), name).toBe(new URL(`${PACKAGE_ASSET_ROUTE}/${name.split("/").map(encodeURIComponent).join("/")}`, "http://page.test/").href);
    }
    const files = listFiles(bundle);
    expect(files.length).toBe(fixture.assets.length + guest.length);
    expect(files.filter((file) => file.startsWith(PACKAGE_ASSET_DESTINATION + "/")).length).toBe(guest.length);
    expect(files.some((file) => file.endsWith(".semio-artifact.json"))).toBe(false);
    expect(statSync(bundle).isDirectory()).toBe(true);
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});

test("a tree staged under another owner is refused instead of copied", async () => {
  const workspace = mkdtempSync(join(realpathSync(tmpdir()), "canvas-bundle-owner-"));
  try {
    const [tree] = runtimeAssetStageTrees(join(workspace, "products"), "prototype");
    await stageArtifacts(tree!.root, "someone:else", new Map(tree!.files.map(({ name, source }) => [name, join(repository, source)])), { leaseDirectory: join(workspace, "leases") });
    await expect(copyBrowserArtifacts(join(workspace, "bundle"), canvasRuntimeAssetSources(join(workspace, "products"), "prototype").slice(0, 1))).rejects.toThrow("owner mismatch");
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});
