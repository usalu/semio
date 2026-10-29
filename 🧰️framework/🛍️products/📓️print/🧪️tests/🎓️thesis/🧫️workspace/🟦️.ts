import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { lstat, mkdir, readFile, readdir, writeFile } from "node:fs/promises";
import { dirname, isAbsolute, join, relative, resolve, sep } from "node:path";
import { privateRepositoryWorkspaceV1, type PrivateRepositoryWorkspaceV1 } from "../../../../🦑️repo/🔨️modules/📚️library/🧪️testing/🧫️private-repository/🟦️.ts";
import { findWorkspaceRoot } from "@semio-tech/process/routing";
import { printDocument, printLibrarySources } from "../../../🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🟦️.ts";
import { renderPrintLatexTokenStylesheet } from "../../../🔨️modules/🎨print-design-token-paints/🟦️.ts";
import fonts from "../../../🔨️modules/🔤print-font-catalog/🔣️.json";
import fixture from "../🔣️.json";

const product = "🧰️framework/🛍️products/📓️print", limits = fixture.privateWorkspace;
const digest = (bytes: Uint8Array): string => createHash("sha256").update(bytes).digest("hex");
export interface ThesisTestWorkspace {
  readonly repository: PrivateRepositoryWorkspaceV1;
  readonly checkout: string;
  readonly artifacts: string;
  recheck(): Promise<void>;
}

/** 📋️ Declares exact source assets and owner metadata consumed by genuine private native birth. */
export function thesisTestSourceInputsV1():readonly string[]{return Object.freeze([...printLibrarySources().map(path=>product+"/"+path),...["thesis","phd"].flatMap(id=>printDocument(id).sources.map(path=>product+"/"+path)),...fonts.map(font=>product+"/🖼️assets/🔤️font/"+font.directory+"/"+font.filename),...limits.ownerInputs.files]);}

/** 🧫️ Snapshots only real declared Print input assets inside the framework-managed private repository. */
export async function openThesisTestWorkspace(): Promise<ThesisTestWorkspace> {
  const checkout = findWorkspaceRoot(import.meta.dirname);
  const configured = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!configured) throw Error("Explicit thesis test artifact directory required");
  const artifacts = resolve(checkout, configured);
  process.env.SEMIO_TEST_ARTIFACT_DIR = artifacts;
  const repository = await privateRepositoryWorkspaceV1(), sources = new Map<string, string>(), consumed = new Map<string, string>();
  const roots = thesisTestSourceInputsV1();
  const provenance = [product + "/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json", product + "/🔨️modules/🔤print-font-catalog/🔣️.json", product + "/🔨️modules/🎨print-design-token-paints/🟦️.ts"];
  const collect = async (paths: readonly string[]): Promise<string[]> => {
    const result = new Set<string>();
    const visit = async (path: string): Promise<void> => {
      const local = relative(checkout, join(checkout, path)).split(sep).join("/");
      assert.ok(local && !isAbsolute(local) && !local.startsWith(".."), "Bounded declared Print source path");
      const status = await lstat(join(checkout, local));
      assert.ok(!status.isSymbolicLink(), "Print source projections reject symlinks");
      if (status.isDirectory()) for (const name of (await readdir(join(checkout, local))).sort()) await visit(join(local, name));
      else {
        assert.ok(status.isFile() && status.nlink === 1 && status.size <= limits.maximumFileBytes, "Bounded regular Print source");
        result.add(local);
        assert.ok(result.size <= limits.maximumFiles, "Bounded Print source roster");
      }
    };
    for (const path of paths) await visit(path);
    return [...result].sort();
  };
  const roster = await collect(roots), observed = [...new Set([...roster, ...provenance])].sort();
  let bytes = 0;
  for (const path of observed) {
    const content = await readFile(join(checkout, path));
    bytes += content.byteLength;
    assert.ok(content.byteLength <= limits.maximumFileBytes && bytes <= limits.maximumBytes, "Bounded Print source snapshot bytes");
    sources.set(path, digest(content));
    if (!roster.includes(path)) continue;
    assert.ok(!/\.(?:ts|mjs|cjs|js)$/i.test(path), "Runtime implementation code stays in the real checkout");
    const target = join(repository.root, path);
    assert.ok(target.length <= limits.maximumPathLength, "Private input path fits every supported host");
    await mkdir(dirname(target), { recursive: true });
    await writeFile(target, content, { flag: "wx" });
    consumed.set(path, digest(content));
  }
  for (const [name, value] of [["package.json", JSON.stringify(limits.package)], ["📋️project.json", JSON.stringify(limits.project)], ["bun.lock", JSON.stringify(limits.lock)], ["MODULE.bazel", limits.module], ["packages/input/package.json", JSON.stringify(limits.inputPackage)]] as const) {
    await mkdir(dirname(join(repository.root, name)), { recursive: true });
    await writeFile(join(repository.root, name), value + "\n", { flag: "wx" });
  }
  const tokenPath = product + "/🖋️latex/semio-tokens.sty", generated = Buffer.from(renderPrintLatexTokenStylesheet());
  await writeFile(join(repository.root, tokenPath), generated);
  consumed.set(tokenPath, digest(generated));
  await mkdir(artifacts, { recursive: true });
  const recheck = async (): Promise<void> => {
    assert.deepEqual(await collect(roots), roster, "Declared live Print input roster remains unchanged");
    for (const [path, hash] of sources) assert.equal(digest(await readFile(join(checkout, path))), hash, "Live Print source remains unchanged: " + path);
    for (const [path, hash] of consumed) assert.equal(digest(await readFile(join(repository.root, path))), hash, "Consumed private Print source remains unchanged: " + path);
  };
  await recheck();
  const proof = { kind: "private-print-source-snapshot", canonicalWorkspaceProduct: false, files: roster, sourceSha256: Object.fromEntries(sources), consumedSha256: Object.fromEntries(consumed), totalOriginalBytes: bytes };
  const manifest = join(artifacts, "source-snapshot.json");
  assert.ok(manifest.length <= limits.maximumPathLength);
  await writeFile(manifest, JSON.stringify(proof, null, 2) + "\n");
  console.log("[print] Private source snapshot " + roster.length + " files/" + bytes + " original bytes");
  return Object.freeze({ repository, checkout, artifacts, recheck });
}
