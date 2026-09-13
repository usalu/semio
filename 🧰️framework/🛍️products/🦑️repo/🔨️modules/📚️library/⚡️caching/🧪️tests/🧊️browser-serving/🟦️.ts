import assert from "node:assert/strict";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { createRequire } from "node:module";
import { dirname, join } from "node:path";

/** 🧊️ Serves completed profile artifacts under native Vite and compares native public-file delivery. */
export async function testWgpuBrowserServing(workspace: string, output: string): Promise<void> {
  const require = createRequire(import.meta.url), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🔣️.json"), "utf8"));
  const validate = new (require("ajv"))().compile(JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🔣️.json"), "utf8")));
  assert.equal(validate(fixture), true, JSON.stringify(validate.errors));
  const engine = join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu");
  const implementation = join(engine, "🌐️server/🟦️.ts"), template = join(engine, "🌐️server/🌐️.html");
  assert.ok(!readFileSync(template, "utf8").includes("data-trunk"), "Browser serving must never invoke Trunk's compiler pipeline");
  const ts = require("typescript"), source = ts.createSourceFile("bootstrap.ts", readFileSync(join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const declaration = source.statements.find((node: any) => ts.isFunctionDeclaration(node) && node.name?.text === "resolveNxInvocation");
  const resolveInvocation = new Function("process", ts.transpileModule(declaration.getText(source), { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText.replace("export ", "") + "; return resolveNxInvocation;")({ env: {} });
  for (const profile of fixture.profiles) for (const command of ["prepare", "activate", "serve", "dev"]) {
    const target = `@semio-tech/framework-os-dev:${command}-sample-wgpu-${profile}`, invocation = resolveInvocation(["run", target]);
    assert.equal(invocation.env.SEMIO_RENDERER, "wgpu");
    assert.equal(invocation.env.SEMIO_BUILD_MODE, profile === "release" ? "ship" : "dev");
    assert.equal(invocation.watch, command === "dev" ? `@semio-tech/framework-os-dev:activate-sample-wgpu-${profile}` : undefined);
  }
  const { serveVite } = await import("../../🌐️vite/🟦️.ts");
  const { createServer } = require("vite"), WebSocket = require("ws");
  const root = mkdtempSync(join(output, "wgpu-serving-"));
  const put = (path: string, content: string) => { mkdirSync(dirname(path), { recursive: true }); writeFileSync(path, content); };
  symlinkSync(join(workspace, "node_modules"), join(root, "node_modules"), process.platform === "win32" ? "junction" : "dir");
  let successful = false;
  try {
    for (const profile of fixture.profiles) {
      const app = join(root, profile), publicRoot = join(app, "public"), config = join(app, "⚙️vite.config.ts");
      const roots = Object.fromEntries(["boot", "worker", "compiler", "modules", "extensions"].map(name => [name, join(app, name)]));
      const reloadFile = join(roots.modules, "reload.json");
      for (const route of fixture.routes) { put(join(roots[route.root], route.file), route.content); put(join(publicRoot, decodeURI(route.url)), route.content); }
      const wasm = new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0]);
      writeFileSync(join(roots.compiler, "semio-framework-os-renderer-wgpu_bg.wasm"), wasm);
      put(reloadFile, "0");
      copyFileSync(template, join(app, "🌐️.html"));
      put(config, 'import { createWgpuBrowserConfig } from ' + JSON.stringify(implementation) + ';\nexport default () => createWgpuBrowserConfig(' + JSON.stringify({ workspace: root, root: app, profile, compilerRoot: roots.compiler, moduleRoot: roots.modules, extensionRoot: roots.extensions, bootRoot: roots.boot, workerRoot: roots.worker, reloadFile, assets: [] }) + ');\n');
      const controller = new AbortController();
      let ready!: (url: string) => void;
      const readiness = new Promise<string>(resolve => ready = resolve);
      const running = serveVite({ root: app, config, configLoader: "native", host: "127.0.0.1", port: 0, signal: controller.signal, ready });
      const url = await Promise.race([readiness, running.then(() => { throw new Error("Server ended before readiness"); })]);
      const oracle = await createServer({ configFile: false, root: app, publicDir: publicRoot, server: { host: "127.0.0.1", port: 0, strictPort: true }, optimizeDeps: { noDiscovery: true } });
      let socket: any;
      try {
        await oracle.listen();
        const oracleUrl = "http://127.0.0.1:" + oracle.httpServer.address().port;
        const document = await (await fetch(url)).text();
        assert.ok(document.includes('/@vite/client') && document.includes('/🚀️boot.js/🟨️.js'), document);
        for (const route of fixture.routes) {
          const actual = await fetch(new URL(route.url, url)), expected = await fetch(new URL(route.url, oracleUrl));
          assert.equal(actual.status, 200, route.url);
          assert.equal(await actual.text(), await expected.text(), route.url);
        }
        const wasmResponse = await fetch(new URL("/renderer-modules/wgpu/semio-framework-os-renderer-wgpu_bg.wasm", url));
        assert.equal(wasmResponse.headers.get("content-type"), "application/wasm");
        assert.ok((await WebAssembly.instantiate(await wasmResponse.arrayBuffer())).instance);
        assert.equal((await fetch(new URL(fixture.missing, url))).status, 404);
        await assert.rejects(() => serveVite({ root: app, config, configLoader: "native", host: "127.0.0.1", port: Number(new URL(url).port), signal: new AbortController().signal, ready: () => assert.fail("Foreign port reused") }), /in use|EADDRINUSE/i);
        socket = new WebSocket(url.replace("http:", "ws:"), "vite-hmr");
        await new Promise<void>((resolve, reject) => { socket.once("open", resolve); socket.once("error", reject); });
        const reloaded = new Promise<void>((resolve, reject) => {
          const timer = setTimeout(() => reject(new Error("Missing completed-artifact reload")), 10000);
          socket.on("message", (message: Buffer) => { if (JSON.parse(message.toString()).type === "full-reload") { clearTimeout(timer); resolve(); } });
        });
        put(join(roots.compiler, fixture.routes[2].file), fixture.reload);
        put(reloadFile, "1");
        await reloaded;
        assert.equal(await (await fetch(new URL(fixture.routes[2].url, url))).text(), fixture.reload);
      } finally { socket?.terminate(); await oracle.close(); controller.abort(); await running; }
      await assert.rejects(() => fetch(url));
    }
    successful = true;
    console.log("[DEBUG] WGPU serves completed dev/release artifacts with native Vite byte parity, 404 boundaries, reload, strict port ownership and cancellation PASS");
  } finally { if (successful) rmSync(root, { recursive: true, force: true }); }
}
