/** 🔖️ Valid metadata survives actual output admission; AJV, WebCrypto and files validate freshness. */
import assert from "node:assert/strict";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { webcrypto } from "node:crypto";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import { STAGED_SOURCE_FRESHNESS_FILES, buildComponentSourceStatIndex, readStagedSourceStatIndex, resolveBootSourceContentHashes, writeStagedSourceFreshness } from "../../🏗️build/🔍️freshness/🟦️.ts";
import { assertPluginOutputChildren } from "../../🏗️build/🛂️descriptor/🟦️.ts";

export async function sourceFreshnessOracle(repoRoot: string, artifactRoot: string): Promise<number> {
  const pluginRoot = resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
  const fixture = JSON.parse(readFileSync(join(pluginRoot,"🧫️fixtures/🔍️source-freshness/🔣️.json"),"utf8"));
  const schema = JSON.parse(readFileSync(join(pluginRoot,"🏗️build/🔍️freshness/🧬️schema/🔣️.json"),"utf8"));
  const ajv = new Ajv({ strict: true });
  ajv.addSchema(schema);
  
  assert.deepEqual(STAGED_SOURCE_FRESHNESS_FILES,fixture.metadataFiles);
  mkdirSync(artifactRoot,{ recursive:true });
  const directory = mkdtempSync(join(artifactRoot,"source-freshness-"));
  const sourceRoot = join(directory,"source");
  const moduleDirectory = join(directory,"module");
  try {
    for (const [path, content] of Object.entries(fixture.sourceFiles)) {
      const target = join(sourceRoot,path);
      mkdirSync(dirname(target),{recursive:true});
      writeFileSync(target,content as string);
    }
    const index = await buildComponentSourceStatIndex(sourceRoot);
    assert(ajv.validate(`${schema.$id}#/$defs/SourceStatIndexV1`,index), JSON.stringify(ajv.errors));
    const hash = async (content: Uint8Array): Promise<string> => Buffer.from(await webcrypto.subtle.digest("SHA-256", content)).toString("hex");
    const oracleParts: string[] = [];
    for (const [path, content] of Object.entries(fixture.sourceFiles).sort(([left],[right]) => left < right ? -1 : 1)) {
      const digest = await hash(new TextEncoder().encode(content as string));
      assert.equal(index.files.find((file) => file.path === path)?.sha256,digest);
      oracleParts.push(`${path}\0${digest}\0`);
    }
    const oracle = await hash(new TextEncoder().encode(oracleParts.join("")));
    assert.equal(index.contentSha256,oracle);
    assert.equal(await writeStagedSourceFreshness(moduleDirectory,sourceRoot),oracle);
    const preserved = fixture.metadataFiles.map((name: string) => readFileSync(join(moduleDirectory,name),"utf8"));
    assertPluginOutputChildren(moduleDirectory,fixture.componentBase);
    assert.deepEqual(fixture.metadataFiles.map((name: string) => readFileSync(join(moduleDirectory,name),"utf8")),preserved);
    assert.deepEqual(readStagedSourceStatIndex(moduleDirectory),index);
    const warm = await resolveBootSourceContentHashes({ sourceRoot,moduleDirectory });
    assert.equal(warm.hashedFileCount,0);
    assert.equal(warm.reusedFileCount,index.files.length);
    assert.equal(warm.sourceContentSha256,oracle);
    const refused = join(moduleDirectory,fixture.unexpected);
    writeFileSync(refused,"preserve this unexpected input");
    assert.throws(() => assertPluginOutputChildren(moduleDirectory,fixture.componentBase),/preserved/);
    assert.equal(readFileSync(refused,"utf8"),"preserve this unexpected input");
    const cancel = new AbortController();
    cancel.abort(new Error("fixture cancellation"));
    await assert.rejects(buildComponentSourceStatIndex(sourceRoot,20000,cancel.signal),/fixture cancellation/);
    return 15 + index.files.length;
  } finally { rmSync(directory,{recursive:true,force:true}); }
}
