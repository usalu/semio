import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { isAbsolute, join, relative, resolve } from "node:path";
import { chromium } from "playwright";
import { MODULE_PLUGIN_ROUTE } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import type { GuestCheck } from "./👷️worker/🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🛂️committed-descriptor/🔣️.json", import.meta.url), "utf8")) as Omit<GuestCheck, "moduleUrl"> & { pluginId: string };

/** 🌐 Checks both BIM apps through their published browser component and the production actor bridge. */
export async function verifyBimGuest(modules: string, plugin: { pluginId: string; moduleUrl: string }, signal: AbortSignal): Promise<void> {
  assert.equal(plugin.pluginId, fixture.pluginId);
  assert.ok(plugin.moduleUrl.startsWith(MODULE_PLUGIN_ROUTE + "/"));
  const bundle = await Bun.build({ entrypoints: [join(import.meta.dir, "👷️worker/🟦️.ts")], target: "browser", define: { "import.meta.vitest": "undefined" } });
  if (!bundle.success || bundle.outputs.length !== 1) throw Error("BIM browser fixture bundle failed: " + bundle.logs.join("\n"));
  const workerSource = await bundle.outputs[0]!.text();
  const server = Bun.serve({
    hostname: "127.0.0.1", port: 0,
    async fetch(request) {
      const path = decodeURIComponent(new URL(request.url).pathname);
      if (path === "/") return new Response("<!doctype html><title>BIM guest check</title>", { headers: { "content-type": "text/html" } });
      if (path === "/guest-worker.js") return new Response(workerSource, { headers: { "content-type": "text/javascript" } });
      if (!path.startsWith(MODULE_PLUGIN_ROUTE + "/")) return new Response(null, { status: 404 });
      const file = resolve(modules, path.slice(MODULE_PLUGIN_ROUTE.length + 1));
      const inside = relative(modules, file);
      if (!inside || inside.startsWith("..") || isAbsolute(inside)) return new Response(null, { status: 404 });
      const body = Bun.file(file);
      if (!(await body.exists())) return new Response(null, { status: 404 });
      const contentType = path.endsWith(".wasm") ? "application/wasm" : path.endsWith(".js") ? "text/javascript" : body.type;
      return new Response(body, { headers: { "content-type": contentType } });
    },
  });
  try {
    signal.throwIfAborted();
    const browser = await chromium.launch({ headless: true });
    const abort = () => { void browser.close(); };
    signal.addEventListener("abort", abort, { once: true });
    try {
      signal.throwIfAborted();
      const page = await browser.newPage();
      await page.goto(server.url.href);
      const result = await page.evaluate(input => new Promise<{ applicationIds: string[] }>((accept, reject) => {
        const worker = new Worker("/guest-worker.js", { type: "module" });
        const timer = setTimeout(() => finish(Error("BIM browser guest check timed out")), input.browserCaller.lifetimeUs / 1000 * input.applicationIds.length);
        function finish(error?: Error, value?: { applicationIds: string[] }) {
          clearTimeout(timer);
          worker.terminate();
          if (error) reject(error);
          else accept(value!);
        }
        worker.onerror = event => finish(Error(event.message));
        worker.onmessage = ({ data }) => data.error ? finish(Error(data.error)) : finish(undefined, data);
        worker.postMessage(input);
      }), { ...fixture, moduleUrl: new URL(plugin.moduleUrl, server.url).href });
      assert.deepEqual(result.applicationIds, fixture.applicationIds);
    } finally {
      signal.removeEventListener("abort", abort);
      await browser.close();
    }
  } finally {
    server.stop(true);
  }
}
