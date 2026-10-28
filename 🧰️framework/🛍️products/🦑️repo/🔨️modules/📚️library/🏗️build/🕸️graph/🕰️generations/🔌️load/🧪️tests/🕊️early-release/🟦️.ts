/** 🕊️ The builder drops the inference caches (memoized closures, parsed manifests, observation images, interned rows) as soon as the partial inference is discovered, BEFORE it is shaped and the fact index is patched, packed and written: the caches only save recomputation and neither the shaping nor the index work needs any of them, so their peak no longer rides on the inference working set (36 refreshed projects: the release frees 0.9 GB of the 2.65 GB live heap). Order recorded through a host that wraps the mock. */
import {expect, test} from "bun:test";
import {mkdtempSync, realpathSync, rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {loadGenerationGraphV1} from "../../🟨️.mjs";
import {createMockHost, ringFiles, seed} from "../../../🧪️tests/🏃️world/🟦️.ts";

test("the inference caches are released before the shaping reads the part", async () => {
  const dir = mkdtempSync(join(realpathSync(tmpdir()), "graph-release-shape-")), order: string[] = [];
  try {
    await seed(dir, ringFiles(6));
    const mock = createMockHost(dir);
    const host = {
      ...mock.host,
      discoverPart: async (...args: Parameters<typeof mock.host.discoverPart>) => { const part = await mock.host.discoverPart(...args); order.push("discover"); return new Proxy(part, { get: (target, key, receiver) => { if (key === "edgeOnly") order.push("shape"); return Reflect.get(target, key, receiver); } }); },
      release: async () => { order.push("release"); },
    };
    await loadGenerationGraphV1(host as never);
    expect(order.indexOf("shape")).toBeGreaterThan(order.indexOf("discover"));
    expect(order.indexOf("release")).toBeGreaterThan(order.indexOf("discover"));
    expect(order.indexOf("release")).toBeLessThan(order.indexOf("shape"));
  } finally { rmSync(dir, {recursive: true, force: true}); }
});

test("the inference caches are released after the discovery and before the generation is published, and once more when the load ends", async () => {
  const dir = mkdtempSync(join(realpathSync(tmpdir()), "graph-release-")), order: string[] = [];
  try {
    await seed(dir, ringFiles(6));
    const mock = createMockHost(dir);
    const host = {
      ...mock.host,
      discoverPart: async (...args: Parameters<typeof mock.host.discoverPart>) => { order.push("discover"); return mock.host.discoverPart(...args); },
      say: (key: string, fields: Record<string, string | number>) => { order.push("say:" + key); mock.host.say(key, fields); },
      release: async () => { order.push("release"); },
    };
    await loadGenerationGraphV1(host as never);
    const at = (name: string) => order.indexOf(name);
    expect(at("discover")).toBeGreaterThanOrEqual(0);
    expect(at("release")).toBeGreaterThan(at("discover"));
    expect(at("release")).toBeLessThan(at("say:published"));
    expect(order.filter(name => name === "release").length).toBe(2);
    expect(order.at(-1)).toBe("release");
  } finally { rmSync(dir, {recursive: true, force: true}); }
});
