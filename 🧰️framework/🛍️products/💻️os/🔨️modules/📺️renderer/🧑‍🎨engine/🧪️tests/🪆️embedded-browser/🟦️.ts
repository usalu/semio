import { describe, expect, it } from "vitest";
import Ajv from "ajv/dist/2020";
import fixture from "../../🧫️fixtures/🪆️embedded-browser/🔣️.json";
import schema from "../../🧬️schema/🪆️embedded-browser/🔣️.json";
import { parseEmbeddedBrowserConfiguration } from "../../🎯️targets/🧊️wgpu/🧪️tests/🪆️embedded-browser/🟦️.ts";

describe("embedded physical browser authority", () => {
  it("independently validates neutral roots, physical offsets, locales and lifecycle phases", () => {
    const validate = new Ajv({ strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    for (const mutate of [
      (f: typeof fixture) => { f.roots[1]!.rootId = f.roots[0]!.rootId; },
      (f: typeof fixture) => { f.roots[1]!.locale = "en"; },
      (f: typeof fixture) => { f.roots[1]!.rect[0] = 20; },
      (f: typeof fixture) => { f.phases.pop(); },
      (f: typeof fixture) => { f.roots[0]!.suppressAutoIntroduction = false; },
    ]) { const invalid = structuredClone(fixture); mutate(invalid); expect(validate(invalid)).toBe(false); }
  });
  it("requires explicit different plugin lists and public artifact authority", () => {
    const configuration = { renderer: "wgpu", serve: "http://localhost:7301/", libraryModuleUrl: "http://localhost:7301/library.js", rendererModuleUrl: "http://localhost:7301/renderer.js", rendererWasmUrl: "http://localhost:7301/renderer.wasm", frameWorkerUrl: "http://localhost:7301/worker.js", roots: fixture.roots.map(root => ({ rootId: root.rootId, plugins: root.requiredPlugins.map(pluginId => ({ pluginId, moduleUrl: `http://localhost:7301/${pluginId}.js` })) })) };
    expect(parseEmbeddedBrowserConfiguration(configuration)).toEqual(configuration);
    for (const mutate of [
      (c: typeof configuration) => { c.roots[1]!.plugins = c.roots[0]!.plugins; },
      (c: typeof configuration) => { c.frameWorkerUrl = ""; },
      (c: typeof configuration) => { c.roots[0]!.plugins[0]!.moduleUrl = "https://user:password@example.org/plugin.js"; },
      (c: typeof configuration) => { c.roots[0]!.plugins.push(c.roots[0]!.plugins[0]!); },
    ]) { const invalid = structuredClone(configuration); mutate(invalid); expect(() => parseEmbeddedBrowserConfiguration(invalid)).toThrow(); }
  });
});
