import { mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { createServer, type Plugin } from "vite";
import { afterEach, describe, expect, it } from "vitest";
import fixture from "../../🧫️fixtures/🩺️runtime-diagnostics/🔣️.json" with { type: "json" };
import { resolveRuntimeDiagnosticsPreference, stampShardWorkerDiagnostics } from "../../../../../../../🔨️modules/🎭️actor/🩺️diagnostics/🟦️.ts";
import { wgpuBrowserSelectionPlugin } from "../../🎯️targets/🧊️wgpu/🌐️server/🟦️.ts";

const originalServerSwitch = process.env.VITE_SEMIO_RUNTIME_DIAGNOSTICS;

afterEach(() => {
  if (originalServerSwitch === undefined) delete process.env.VITE_SEMIO_RUNTIME_DIAGNOSTICS;
  else process.env.VITE_SEMIO_RUNTIME_DIAGNOSTICS = originalServerSwitch;
});

describe("WGPU runtime diagnostics launch propagation", () => {
  it("projects the neutral precedence rows into the exact worker URL", () => {
    for (const row of fixture.cases) {
      const enabled = resolveRuntimeDiagnosticsPreference(row.stored, row.server);
      expect(enabled, row.name).toBe(row.enabled);
      expect(stampShardWorkerDiagnostics(fixture.workerUrl, enabled), row.name).toBe(row.workerUrl);
    }
  });

  it("serves the launch environment through the real Vite HTML transform", async () => {
    const root = mkdtempSync(join(tmpdir(), "semio-wgpu-diagnostics-"));
    writeFileSync(join(root, "index.html"), '<!doctype html><html><head></head><body></body></html>');
    process.env.VITE_SEMIO_RUNTIME_DIAGNOSTICS = "1";
    const server = await createServer({ configFile: false, root, logLevel: "silent", server: { host: "127.0.0.1", port: 0 }, plugins: [wgpuBrowserSelectionPlugin(undefined) as Plugin] });
    try {
      await server.listen();
      const address = server.httpServer?.address();
      if (!address || typeof address === "string") throw new Error("Vite did not publish its test address");
      const html = await (await fetch(`http://127.0.0.1:${address.port}/`)).text();
      expect(html).toContain(`<meta name="${fixture.metaName}" content="1">`);
    } finally {
      await server.close();
      rmSync(root, { recursive: true, force: true });
    }
  });
});
