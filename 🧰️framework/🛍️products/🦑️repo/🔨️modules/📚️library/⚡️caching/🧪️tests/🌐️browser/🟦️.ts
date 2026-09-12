import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync, existsSync, symlinkSync } from "node:fs";
import { dirname, join } from "node:path";

/** 📦️ Verifies portable browser distribution ownership and byte-preserving copies. */
export async function testBrowserDistribution(workspace: string, outputDirectory: string): Promise<void> {
  const moduleRoot = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution");
  const { copyBrowserArtifacts } = await import(join(moduleRoot, "🟦️.ts"));
  const fixture = JSON.parse(readFileSync(join(moduleRoot, "🧫️cases.json"), "utf8"));
  const temporary = mkdtempSync(join(outputDirectory, "browser-distribution-")), source = join(temporary, "source");
  const put = (path: string, value: string) => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, value); };
  for (const [name, value] of Object.entries({ ...fixture.files, ...fixture.ambient })) put(join(source, name), String(value));
  const marker = { version: 1, owner: fixture.owner, files: Object.keys(fixture.files) }, manifest = join(source, ".nx-artifact.json");
  put(manifest, JSON.stringify(marker));
  const require = createRequire(import.meta.url), schema = JSON.parse(readFileSync(join(moduleRoot, "../🧬️schema/🔣️.json"), "utf8"));
  const validator = new (require("ajv").default)(); validator.addSchema(schema);
  const viteRoot = join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite");
  const lifecycle = JSON.parse(readFileSync(join(viteRoot, "🧫️cases.json"), "utf8"));
  assert.ok(new (require("ajv").default)().validate(JSON.parse(readFileSync(join(viteRoot, "🧬️schema/🔣️.json"), "utf8")), lifecycle));
  assert.ok(validator.validate({ $ref: schema.$id + "#/$defs/BrowserArtifactDistributionV1" }, marker));
  const sources = [{ root: source, destination: fixture.destination, owner: fixture.owner, shimDirectory: fixture.shimDirectory }];
  try {
    put(join(temporary, "package.json"), JSON.stringify(lifecycle.storage.package));
    mkdirSync(join(temporary, lifecycle.storage.modules));
    const { collectArtifactFiles } = await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🗂️files/🟦️.ts"));
    const oracle = (await require("fast-glob")("**/*", { cwd: source, dot: true, onlyFiles: true, followSymbolicLinks: false })).sort();
    assert.deepEqual([...(await collectArtifactFiles(source)).keys()], oracle);
    const output = join(temporary, "output"), progress: string[] = [];
    const count = await copyBrowserArtifacts(output, sources, { progress: (file: string) => progress.push(file) });
    assert.equal(count, Object.keys(fixture.files).length);
    assert.equal(progress.length, count);
    for (const [name, value] of Object.entries(fixture.files)) assert.equal(readFileSync(join(output, fixture.destination, name), "utf8"), name.endsWith("bridge.js") ? fixture.rewrittenBridge : value);
    for (const name of [...Object.keys(fixture.ambient), ".nx-artifact.json"]) assert.equal(existsSync(join(output, fixture.destination, name)), false);
    for (const owner of fixture.invalidOwners) {
      put(manifest, JSON.stringify({ ...marker, owner }));
      await assert.rejects(() => copyBrowserArtifacts(join(temporary, "bad-owner"), sources), /owner/i);
    }
    for (const name of fixture.invalidFiles) {
      put(manifest, JSON.stringify({ ...marker, files: [name] }));
      await assert.rejects(() => copyBrowserArtifacts(join(temporary, "bad-path"), sources), /path/i);
    }
    put(manifest, JSON.stringify(marker));
    await assert.rejects(() => copyBrowserArtifacts(output, sources), /exists|collision/i);
    put(manifest, JSON.stringify({ ...marker, files: [marker.files[0], marker.files[0]] }));
    await assert.rejects(() => copyBrowserArtifacts(join(temporary, "duplicate"), sources), /collision/i);
    put(manifest, JSON.stringify(marker));
    const linked = join(temporary, "linked");
    symlinkSync(source, linked, process.platform === "win32" ? "junction" : "dir");
    await assert.rejects(() => copyBrowserArtifacts(join(temporary, "linked-output"), [{ ...sources[0], root: linked }]), /symlink/i);
    await assert.rejects(() => collectArtifactFiles(linked), /root/i);
    const abort = new AbortController(); abort.abort();
    await assert.rejects(() => copyBrowserArtifacts(join(temporary, "cancelled"), sources, { signal: abort.signal }), /abort/i);
    assert.equal(existsSync(join(temporary, "cancelled")), false);
    await assert.rejects(() => collectArtifactFiles(source, abort.signal), /abort/i);
    const midCopy = new AbortController(); let copied = 0;
    await assert.rejects(() => copyBrowserArtifacts(join(temporary, "mid-copy"), sources, { signal: midCopy.signal, progress: () => { copied++; midCopy.abort(); } }), /abort/i);
    assert.equal(copied, 1);
    const { init, parse } = await import("es-module-lexer"); await init;
    const [imports] = parse(readFileSync(join(output, fixture.destination, "🌉️bridge.js"), "utf8"));
    assert.equal(imports[0]?.n, "../../" + fixture.shimDirectory + "/io.js");
    const { browserArtifactVitePlugin } = await import(join(moduleRoot, "⚡️vite/🟦️.ts"));
    const app = join(temporary, "app"), buildOutput = join(temporary, "vite-output");
    put(join(app, "index.html"), '<!doctype html><title>Artifact fixture</title><script type="module" src="/entry.js"></script>');
    put(join(app, "entry.js"), 'document.body.dataset.ready = "true";');
    const { build, resolveConfig } = await import("vite");
    await build({ root: app, configFile: false, publicDir: false, logLevel: "silent", plugins: [browserArtifactVitePlugin(sources)], build: { outDir: buildOutput, emptyOutDir: true } });
    assert.ok(readFileSync(join(buildOutput, "index.html"), "utf8").includes("/assets/"));
    for (const [name, value] of Object.entries(fixture.files)) assert.equal(readFileSync(join(buildOutput, fixture.destination, name), "utf8"), name.endsWith("bridge.js") ? fixture.rewrittenBridge : value);
    for (const name of [...Object.keys(fixture.ambient), ".nx-artifact.json"]) assert.equal(existsSync(join(buildOutput, fixture.destination, name)), false);
    await assert.rejects(() => build({ root: app, configFile: false, publicDir: false, logLevel: "silent", plugins: [browserArtifactVitePlugin([{ ...sources[0], owner: "wrong" }])], build: { outDir: join(temporary, "bad-vite-output"), emptyOutDir: true } }), /owner/i);
    const { buildViteArtifact, serveVite } = await import(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts"));
    const config = join(app, "⚙️vite.config.ts"), published = join(temporary, "published");
    put(config, 'export default { publicDir: false, logLevel: "silent" };');
    await buildViteArtifact({ root: app, workspace, config, output: published, owner: fixture.owner, temporaryRoot: temporary });
    assert.ok(existsSync(join(temporary, lifecycle.storage.config)), "Vite config compilation must stay inside the private fixture");
    const resolvedConfig = await resolveConfig({ root: app, configFile: false }, "serve");
    assert.equal(resolvedConfig.cacheDir, join(temporary, lifecycle.storage.optimizer), "Vite dependency optimization must stay inside the private fixture");
    const publishedIndex = readFileSync(join(published, "index.html"), "utf8"), publishedMarker = JSON.parse(readFileSync(join(published, ".nx-artifact.json"), "utf8"));
    assert.equal(publishedMarker.owner, fixture.owner);
    assert.ok(publishedMarker.files.includes("index.html"));
    assert.deepEqual(publishedMarker.files, (await require("fast-glob")("**/*", { cwd: published, dot: true, onlyFiles: true })).filter((file: string) => file !== ".nx-artifact.json").sort());
    put(config, 'throw new Error("fixture build failure"); export default {};');
    await assert.rejects(() => buildViteArtifact({ root: app, workspace, config, output: published, owner: fixture.owner, temporaryRoot: temporary }), /Vite build failed/i);
    assert.equal(readFileSync(join(published, "index.html"), "utf8"), publishedIndex);
    await assert.rejects(() => buildViteArtifact({ root: app, workspace, config, output: published, owner: fixture.owner, temporaryRoot: temporary, signal: abort.signal }), /abort/i);
    assert.equal((await (await import("node:fs/promises")).readdir(temporary)).some(name => name.startsWith("vite-build-")), false);
    put(config, 'export default { publicDir: false, logLevel: "silent" };');
    const serving = new AbortController();
    let ready!: (url: string) => void, failed!: (error: unknown) => void;
    const listening = new Promise<string>((resolve, reject) => { ready = resolve; failed = reject; });
    const server = serveVite({ root: app, config, host: lifecycle.server.host, port: 0, signal: serving.signal, ready });
    void server.catch(failed);
    let url: string, socket: WebSocket | undefined;
    try {
      url = await Promise.race([listening, new Promise<never>((_, reject) => setTimeout(() => reject(new Error("Vite readiness timed out")), 10000).unref())]);
      assert.ok((await (await fetch(new URL(lifecycle.server.path, url), { signal: AbortSignal.timeout(5000) })).text()).includes(lifecycle.server.expected));
      const client = await (await fetch(new URL(lifecycle.hmr.clientPath, url), { signal: AbortSignal.timeout(5000) })).text();
      assert.ok(client.includes("const hmrPort = null;"), "HMR must use the allocated HTTP listener");
      const token = client.match(/const wsToken = ("[^"]+");/); assert.ok(token, "Vite client must expose its HMR token");
      const websocketUrl = new URL(url); websocketUrl.protocol = "ws:"; websocketUrl.searchParams.set("token", JSON.parse(token[1]));
      socket = new WebSocket(websocketUrl, lifecycle.hmr.protocol);
      const messages: { type: string }[] = [];
      socket.addEventListener("message", message => messages.push(JSON.parse(String(message.data))));
      const until = async (condition: () => boolean) => { const deadline = Date.now() + 5000; while (!condition()) { assert.ok(Date.now() < deadline, "Vite HMR lifecycle timed out"); await Bun.sleep(20); } };
      await until(() => messages.some(message => message.type === lifecycle.hmr.connected));
      put(join(app, lifecycle.hmr.changedFile), lifecycle.hmr.contents);
      await until(() => messages.some(message => message.type === lifecycle.hmr.update));
      await assert.rejects(() => serveVite({ root: app, config, host: lifecycle.server.host, port: Number(new URL(url).port), signal: new AbortController().signal, ready: () => assert.fail("Occupied port was reused") }), /in use|EADDRINUSE/i);
      serving.abort(); await server;
      await until(() => socket!.readyState === WebSocket.CLOSED);
    } finally { serving.abort(); await server; socket?.close(); }
    const net = await import("node:net"), rebound = net.createServer();
    await new Promise<void>((resolve, reject) => { rebound.once("error", reject); rebound.listen(Number(new URL(url!).port), lifecycle.server.host, resolve); });
    await new Promise<void>((resolve, reject) => rebound.close(error => error ? reject(error) : resolve()));
    await assert.rejects(() => serveVite({ root: app, config, host: lifecycle.server.host, port: 0, signal: abort.signal, ready: () => assert.fail("Cancelled server became ready") }), /abort/i);
    console.log("[DEBUG] Vite serves HTTP and real HMR updates on one allocated listener, rejects occupied ports and closes live sockets on cancellation PASS");
    console.log("[DEBUG] Vite config compilation and dependency optimization use the private fixture's module directory PASS");
    console.log("[DEBUG] Vite publication owns the complete file inventory, preserves prior bytes after build failure and cleans private staging PASS");
    console.log("[DEBUG] Real Vite production build emits owned runtime bytes and rejects a mismatched producer PASS");
    console.log(`[DEBUG] Browser distribution copies ${count} owned files, relocates module imports, excludes ambient metadata, rejects corrupt ownership/paths and pre-cancellation PASS`);
  } finally { rmSync(temporary, { recursive: true, force: true }); }
}
