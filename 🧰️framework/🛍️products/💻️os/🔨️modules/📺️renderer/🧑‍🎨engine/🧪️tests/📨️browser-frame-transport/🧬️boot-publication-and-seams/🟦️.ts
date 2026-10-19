/** 🧬️ Boot publication and seams: the introspection hooks after boot, the platform read across the boot seam, the shell brand row and the product discovery seam boundary. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { resolveWgpuBootDescriptor, resolveWgpuHostPlatform } from "@semio-tech/framework-renderer-wgpu/boot-descriptor";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { readFrameWorkerSource } from "../📖️frame-worker-source/🟦️.ts";

describe("browser frame worker transport", () => {
  it("publishes the introspection hooks on the UI isolate only after the Worker reports booted", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const bootSource = (readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8") + "\n" + readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts"), "utf8"));
    const workerSource = readFrameWorkerSource(root);
    expect(bootSource).toContain("host.semioWgpuIntrospection = mountIntrospection(transport)");
    expect(bootSource).not.toContain("wasmBindings =");
    expect(bootSource.indexOf("detachIntrospection = attachIntrospectionBindings(transport)")).toBeGreaterThan(bootSource.indexOf("onReady: () => {"));
    for (const mapping of ['message.probe === "structure" ? bindings.dumpStructure', 'message.probe === "accessibility" ? bindings.dumpAccessibility', 'message.probe === "mesh-stats" ? bindings.dumpMeshStats', 'message.probe === "board2d" ? bindings.dumpBoard2d', 'message.probe === "chrome" ? bindings.dumpChrome', "bindings.dumpFrameStats"]) expect(workerSource).toContain(mapping);
    expect(bootSource).toContain('dumpChrome: probe("chrome")');
    expect(bootSource).toContain('dumpBoard2d: probe("board2d")');
    expect(workerSource).toContain("INTROSPECTION_STEP_BUDGET_MS");
    expect(workerSource).toContain("publishWorld3dAcceptedFrameDiagnostic(input.generation)");
    expect(workerSource).toContain("json === lastWorld3dAcceptedFrameDiagnostic");
    expect(workerSource).toContain('channel: "world3d-accepted-frame"');
    expect(bootSource).toContain("[TRACE] wgpu ${channel} generation=${generation} frame=${frameSequence} ${json}");
  });

  it("carries the page realm's platform read across the boot seam", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const bootSource = (readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8") + "\n" + readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts"), "utf8"));
    const workerSource = readFrameWorkerSource(root);
    const descriptorSource = readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts"), "utf8");
    expect(bootSource).toContain("platform: hostPlatform()");
    expect(workerSource).toContain("loaded.semioWgpuSetHostPlatform?.(message.platform)");
    expect(descriptorSource).toContain("export function resolveWgpuHostPlatform");
    expect(resolveWgpuHostPlatform({ navigator: { platform: "MacIntel" } as Navigator })).toBe("MacIntel");
    expect(resolveWgpuHostPlatform({ navigator: { platform: "Win32", userAgentData: { platform: "macOS" } } as unknown as Navigator })).toBe("macOS");
    expect(resolveWgpuHostPlatform({})).toBe("");
  });

  it("carries the resolved shell brand row on the boot descriptor", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const serveSource = readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🌐️server/🟦️.ts"), "utf8");
    const rendererSource = readModuleTreeSource(join(root, "../../../🎯️targets/🧊️wgpu/🏛️host-base/🦀️.rs"));

    const unbranded = resolveWgpuBootDescriptor({ defaultVariant: "s" });
    expect(unbranded.brandId).toBe("");
    expect(unbranded.brand).toEqual({ windowTitle: "", ephemeral: false, replayIntroductionOnLoad: false });

    const meta: Record<string, string> = {
      "semio-brand": "entwerfen-mit-bestand-aggregator",
      "semio-brand-window-title": "Aggregator",
      "semio-brand-ephemeral": "true",
      "semio-brand-replay-introduction": "true",
    };
    const branded = resolveWgpuBootDescriptor({ defaultVariant: "s", meta: (name) => meta[name] ?? "" });
    expect(branded.brandId).toBe("entwerfen-mit-bestand-aggregator");
    expect(branded.brand).toEqual({ windowTitle: "Aggregator", ephemeral: true, replayIntroductionOnLoad: true });

    expect(resolveWgpuBootDescriptor({ defaultVariant: "s", meta: (name) => meta[name] ?? "", overrides: { brand: { ephemeral: false } } }).brand.ephemeral).toBe(false);

    for (const tag of ["semio-brand-window-title", "semio-brand-ephemeral", "semio-brand-replay-introduction"]) expect(serveSource).toContain(`["${tag}"`);
    for (const variable of ["SEMIO_BRAND_WINDOW_TITLE", "SEMIO_BRAND_EPHEMERAL", "SEMIO_BRAND_REPLAY_INTRODUCTION"]) expect(rendererSource).toContain(variable);
  });

  it("keeps product discovery and native UI capability out of the UI/Worker seams", () => {
    const root = dirname(fileURLToPath(import.meta.url));
    const bootSource = (readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts"), "utf8") + "\n" + readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🌐️browser-host/🟦️.ts"), "utf8"));
    const workerSource = readFrameWorkerSource(root);
    const rustSource = readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🌐️browser-worker/🦀️.rs"), "utf8");
    expect(bootSource).not.toContain("PLUGIN_CATALOG");
    expect(bootSource).not.toContain("resolvePlaygroundBoot");
    expect(bootSource).not.toContain("performance.getEntriesByType");
    expect(workerSource).toContain("new PlaygroundBootPlanner(");
    expect(workerSource).toContain("prepareWgpuPluginModules(");
    expect(workerSource).toContain('monitoredSuspension("renderer-module", () => import');
    expect(workerSource).toContain('closeOwner === "runtime"');
    expect(workerSource).toContain("interactiveJobs?.close()");
    expect(workerSource).toContain('getReader({ mode: "byob" })');
    expect(workerSource).toContain("new Uint8Array(ASSET_RESPONSE_PAGE_BYTES)");
    expect(workerSource).toContain("declared ?? ASSET_RESPONSE_BYTE_CAPACITY");
    expect(workerSource).toContain("declared !== undefined && received !== declared");
    expect(workerSource).toContain("ASSET_RESPONSE_PAGE_BYTES");
    expect(workerSource).toContain("assetAbort?.abort()");
    expect(workerSource).not.toContain("response.arrayBuffer()");
    expect(rustSource).toContain("OffscreenPresentToken::mint_for_dedicated_worker");
    expect(rustSource).toContain("WorldAssetResponsePage::try_from_owned");
    expect(rustSource).not.toContain("UiPresentToken");
    const admittedRetirement = (source: string): boolean => {
      const handoff = source.slice(source.indexOf("if *self.close_phase == 5 {"), source.indexOf("*self.close_phase = 6;"));
      const retirement = source.slice(source.indexOf("if let Some(retired) = self.retired_host.as_mut()"), source.indexOf("Ok(true)", source.indexOf("if let Some(retired) = self.retired_host.as_mut()")));
      return handoff.includes("OsHost::try_into_retirement(self.host, cx)?")
        && handoff.includes("return Ok(false);")
        && handoff.includes("*self.retired_host = Some(retirement);")
        && retirement.includes("if !retired.close_step(cx)?")
        && retirement.includes("if !retired.terminal_is_empty()")
        && retirement.indexOf("if !retired.terminal_is_empty()") < retirement.indexOf("*self.retired_host = None;");
    };
    expect(admittedRetirement(rustSource)).toBe(true);
    for (const required of ["OsHost::try_into_retirement(self.host, cx)?", "*self.retired_host = Some(retirement);", "if !retired.terminal_is_empty()", "if !retired.close_step(cx)?"])
      expect(admittedRetirement(rustSource.replace(required, "unqualified_retirement"))).toBe(false);
    expect(rustSource).not.toContain("drop(self.host.take())");
    expect(rustSource).not.toContain("forget(host)");
    expect(rustSource).toContain("DispatchEvent::TextEditStart");
    expect(rustSource).toContain("DispatchEvent::TextEditChunk");
    expect(rustSource).toContain("DispatchEvent::TextEditCommit");
    expect(rustSource).toContain("pending_discrete.saturating_add(discrete_commits)");
    expect(rustSource).not.toContain("stream.text.push_str");
    const descriptorSource = readFileSync(join(root, "../../../🎯️targets/🧊️wgpu/🧭️boot-descriptor/🟦️.ts"), "utf8");
    expect(bootSource).not.toContain("new URLSearchParams");
    expect(descriptorSource.indexOf("WGPU_BOOT_LOCATION_CAPACITY")).toBeLessThan(descriptorSource.indexOf("new URLSearchParams"));
    expect(descriptorSource).toContain('new URLSearchParams(boundedLocation(input.search ?? "", "location.search"))');
  });
});
