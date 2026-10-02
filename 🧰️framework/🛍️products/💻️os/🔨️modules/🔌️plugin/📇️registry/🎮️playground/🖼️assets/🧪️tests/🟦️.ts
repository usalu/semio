import { PLAYGROUND_ASSET_PROVIDERS_V1 } from "../🧩️composition/🟦️.ts";
import assert from "node:assert/strict";
import Ajv from "ajv";
import schema from "../../../../../../../../🔨️modules/🖼️assets/🗺️tile-proxy/🧬️schema/🔣️.json" with { type: "json" };
import corpus from "../../../../../../../../🔨️modules/🖼️assets/🗺️tile-proxy/🧫️fixtures/🔣️.json" with { type: "json" };
import { parseTileProxyAssetSpecV1 } from "../../../../../../../../🔨️modules/🖼️assets/🗺️tile-proxy/🟦️.ts";

/** 🧪️ Compares the owner-neutral portable transport corpus with independent Ajv and JSON oracles. */
export function proveTileProxyAssetCorpusV1(): number {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  for (const row of corpus.cases) {
    const independent = JSON.parse(JSON.stringify(row.value));
    assert.equal(validate(independent), row.accepted, row.id);
    if (row.accepted) assert.deepEqual(parseTileProxyAssetSpecV1(independent), row.value, row.id);
    else assert.throws(() => parseTileProxyAssetSpecV1(independent), undefined, row.id);
  }
  return corpus.cases.length;
}

/** 📦️ Proves that the production Cargo metadata reader admits the same wire rows as independent TOML and Ajv. */
export async function proveTileAssetMetadataV1(artifactRoot: string): Promise<void> {
  const { parseAssetsForCrate } = await import("../../🔎️discovery/🟦️.ts");
  const { default: independentToml } = await import("@iarna/toml");
  const { mkdtempSync, mkdirSync, writeFileSync, rmSync } = await import("node:fs");
  const { join } = await import("node:path");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "neutral-tile-metadata-"));
  const path = join(root, "Cargo.toml");
  const validate = new Ajv({ strict: true }).compile(schema);
  try {
    for (const row of corpus.cases) {
      const text = "[[package.metadata.semio.assets]]\n" + Object.entries(row.value).map(([key, value]) => key + " = " + JSON.stringify(value)).join("\n") + "\n";
      const oracle = independentToml.parse(text) as { package: { metadata: { semio: { assets: unknown[] } } } };
      assert.equal(validate(oracle.package.metadata.semio.assets[0]), row.accepted, row.id);
      writeFileSync(path, text);
      if (row.accepted) assert.deepEqual(parseAssetsForCrate(path, root), [row.value], row.id);
      else assert.throws(() => parseAssetsForCrate(path, root), undefined, row.id);
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
}

/** 🌐️ Exercises exact owner headers, cache bytes and bundle refusal through the production HTTP proxy. */
export async function proveTileProxyTransportV1(artifactRoot: string): Promise<void> {
  const { createServer } = await import("node:http");
  const { once } = await import("node:events");
  const { mkdtempSync, mkdirSync, rmSync, readFileSync, existsSync } = await import("node:fs");
  const { join } = await import("node:path");
  const { createAssetHttpServerV1 } = await import("../../../../../../../../🔨️modules/🖼️assets/🔍️resolver/🧭️dispatch/🟦️.ts");
  mkdirSync(artifactRoot, { recursive: true });
  const root = mkdtempSync(join(artifactRoot, "neutral-tile-transport-"));
  let calls = 0;
  let cancelled = false;
  let started!: () => void;
  let stopped!: () => void;
  const pendingStarted = new Promise<void>(resolve => { started = resolve; });
  const pendingStopped = new Promise<void>(resolve => { stopped = resolve; });
  const source = createServer((request, response) => {
    calls++;
    assert.equal(request.headers["user-agent"], "Fixture/1");
    if (request.url === "/2/0/0.bin") { response.setHeader("content-length", corpus.transport.oversizedBytes); response.end(Buffer.alloc(corpus.transport.oversizedBytes)); return; }
    if (request.url === "/3/0/0.bin") { started(); request.socket.once("close", () => { cancelled = true; stopped(); }); return; }
    response.end(Buffer.from(corpus.transport.bytes));
  });
  source.listen(0, "127.0.0.1");
  await once(source, "listening");
  const address = source.address();
  assert(address && typeof address === "object");
  const spec = parseTileProxyAssetSpecV1({ ...corpus.cases[0]!.value, upstream: "http://127.0.0.1:" + address.port + "/{z}/{x}/{y}.bin" });
  const proxy = createAssetHttpServerV1(root, 0, [spec], PLAYGROUND_ASSET_PROVIDERS_V1, "fetch");
  await once(proxy, "listening");
  const target = proxy.address();
  assert(target && typeof target === "object");
  const url = "http://127.0.0.1:" + target.port;
  let bundle: ReturnType<typeof createAssetHttpServerV1> | undefined;
  try {
    const first = await fetch(url + corpus.transport.request);
    assert.equal(first.status, 200);
    assert.deepEqual([...new Uint8Array(await first.arrayBuffer())], corpus.transport.bytes);
    const second = await fetch(url + corpus.transport.request);
    assert.equal(second.status, 200);
    assert.deepEqual([...new Uint8Array(await second.arrayBuffer())], corpus.transport.bytes);
    assert.deepEqual([...readFileSync(join(root, spec.cache, "0/0/0.bin"))], corpus.transport.bytes);
    bundle = createAssetHttpServerV1(root, 0, [spec], PLAYGROUND_ASSET_PROVIDERS_V1, "bundle");
    await once(bundle, "listening");
    const bundled = bundle.address();
    assert(bundled && typeof bundled === "object");
    assert.equal((await fetch("http://127.0.0.1:" + bundled.port + corpus.transport.missing)).status, corpus.transport.missingStatus);
    assert.equal((await fetch(url + corpus.transport.oversizedRequest)).status, corpus.transport.oversizedStatus);
    assert.equal(existsSync(join(root, spec.cache, "2/0/0.bin")), false);
    const controller = new AbortController();
    const pending = fetch(url + corpus.transport.cancelledRequest, { signal: controller.signal }).then(() => { throw Error("cancelled transport unexpectedly completed"); }, () => undefined);
    await pendingStarted;
    controller.abort();
    await pending;
    let expiry: ReturnType<typeof setTimeout>;
    await Promise.race([pendingStopped, new Promise<void>((_, reject) => { expiry = setTimeout(() => reject(Error("proxy retained a cancelled upstream request")), 2000); })]).finally(() => clearTimeout(expiry));
    assert.equal(cancelled, corpus.transport.cancelledUpstream);
    assert.equal(existsSync(join(root, spec.cache, "3/0/0.bin")), false);
    assert.equal(calls, corpus.transport.upstreamCalls);
  } finally {
    for (const server of [proxy, source, bundle]) if (server) await new Promise<void>((done, reject) => { server.close(error => error ? reject(error) : done()); server.closeAllConnections(); });
    rmSync(root, { recursive: true, force: true });
  }
}
