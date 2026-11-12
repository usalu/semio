/** 🧪 Acceptance of the native provisioner without any network: the manifest pins, the digest gate, the host mapping and the environment it hands to uv. Run with `repo tools exec --light bun -- 🟦️.ts`.
 * @see ../🔣️.json */
import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { existsSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { buildEnvironment, fetchVerified, fingerprint, hostSubdir, provisionNative, readManifest, sha256File } from "../🟦️.ts";

const manifest = readManifest();
const hosts = ["osx-arm64", "osx-64"];
const hex = /^[0-9a-f]{64}$/;

assert.equal(manifest.version, 1);
for (const host of hosts) {
  const rows = manifest.packages.filter(row => row.host === host);
  for (const name of ["cairo", "pkgconf", "zlib", "freetype", "expat"]) assert.ok(rows.some(row => row.name === name), `${host} pins ${name}`);
  assert.equal(new Set(rows.map(row => row.name)).size, rows.length, `${host} has one row per package`);
  assert.ok(manifest.micromamba.assets.some(asset => asset.subdir === host && hex.test(asset.sha256)), `${host} pins micromamba`);
  assert.notEqual(fingerprint(manifest, host), fingerprint(manifest, hosts.find(other => other !== host)!));
}
for (const row of manifest.packages) {
  assert.match(row.sha256, hex);
  assert.match(row.md5, /^[0-9a-f]{32}$/);
  assert.ok(row.url.startsWith("https://conda.anaconda.org/conda-forge/") && row.url.endsWith(row.archive), row.url);
  assert.ok(row.bytes > 0);
  assert.ok([row.host, "noarch"].includes(row.subdir), `${row.archive} comes from the host subdir or noarch`);
  assert.equal(row.url, `https://conda.anaconda.org/conda-forge/${row.subdir}/${row.archive}`);
  assert.ok(row.archive.startsWith(`${row.name}-${row.version}-${row.build}.`) && /\.(conda|tar\.bz2)$/.test(row.archive), row.archive);
}

assert.equal(hostSubdir("darwin", "arm64"), "osx-arm64");
assert.equal(hostSubdir("darwin", "x64"), "osx-64");
for (const platform of ["linux", "win32"] as const) {
  assert.equal(hostSubdir(platform, "x64"), undefined);
  assert.equal(await provisionNative({ platform, architecture: "x64", cacheRoot: join(tmpdir(), "semio-native-never") }), undefined);
}
assert.equal(existsSync(join(tmpdir(), "semio-native-never")), false);

const environment = buildEnvironment("/p", { PATH: "/usr/bin", LDFLAGS: "-O2" });
assert.ok(environment.PATH.startsWith(join("/p", "bin")) && environment.PATH.endsWith("/usr/bin"));
assert.equal(environment.PKG_CONFIG, join("/p", "bin", "pkg-config"));
assert.equal(environment.LDFLAGS, `-O2 -Wl,-rpath,${join("/p", "lib")}`);

const directory = mkdtempSync(join(tmpdir(), "semio-native-test-"));
try {
  const bytes = Buffer.from("pinned payload");
  const digest = createHash("sha256").update(bytes).digest("hex");
  const serve = (body: Buffer): typeof fetch => (async () => new Response(body)) as unknown as typeof fetch;
  const good = join(directory, "good");
  await fetchVerified("https://example.invalid/good", good, digest, bytes.length, { fetcher: serve(bytes) });
  assert.equal(readFileSync(good, "utf8"), "pinned payload");
  const independent = process.platform === "win32" ? digest : execFileSync("shasum", ["-a", "256", good], { encoding: "utf8" }).split(" ")[0];
  assert.equal(await sha256File(good), independent);
  const bad = join(directory, "bad");
  await assert.rejects(fetchVerified("https://example.invalid/bad", bad, digest, bytes.length, { fetcher: serve(Buffer.from("tampered data!")) }), /Digest mismatch/);
  assert.equal(existsSync(bad), false);
  assert.equal(existsSync(`${bad}.part`), false);
  await assert.rejects(fetchVerified("http://example.invalid/plain", join(directory, "plain"), digest, bytes.length, { fetcher: serve(bytes) }), /non-https/);
  writeFileSync(join(directory, "kept"), bytes);
  await fetchVerified("https://example.invalid/kept", join(directory, "kept"), digest, bytes.length, { fetcher: (async () => { throw new Error("network used"); }) as unknown as typeof fetch });
} finally {
  rmSync(directory, { recursive: true, force: true });
}
const projectDirectory = join(import.meta.dir, "..", "..");
const constraints = readFileSync(join(projectDirectory, "build-constraints.txt"), "utf8").split(/\n(?=[A-Za-z])/).filter(block => !block.startsWith("#"));
const pinned = new Map(constraints.map(block => [block.split("==")[0].toLowerCase(), [...block.matchAll(/--hash=sha256:([0-9a-f]{64})/g)].map(match => match[1])]));
for (const name of ["meson-python", "meson", "ninja", "packaging", "pyproject-metadata", "setuptools", "wheel", "cython"]) assert.ok((pinned.get(name)?.length ?? 0) > 0, `${name} is pinned with sha256 hashes`);
for (const block of constraints) assert.match(block.split("\n")[0], /^[A-Za-z0-9._-]+==[0-9][0-9A-Za-z.+!-]* \\$/, "exact pins only");
assert.match(readFileSync(join(projectDirectory, "..", "..", "📜️script.ts"), "utf8"), /UV_BUILD_CONSTRAINT: join\(project, "build-constraints\.txt"\)/);
console.log("native provisioner: ok");
