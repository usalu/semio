import { readFileSync, readdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { consumedTopicsForApp, scopeContributionsJson } from "../../🧾️contributions-json/🟦️.ts";
import fixture from "../🧫️fixtures/🔬️consumed-topics-for-app/🔣️.json";

describe("consumedTopicsForApp", () => {
  const loaded = fixture.loaded.map((entry) => ({ pluginId: entry.pluginId, manifest: { apps: entry.apps.map((id) => ({ id })) } }));
  for (const row of fixture.cases) {
    it(row.id, () => {
      const rows = "owner" in row ? loaded.filter((entry) => entry.pluginId !== "cad") : loaded;
      expect([...consumedTopicsForApp(fixture.registry, rows, row.receiver, row.app)]).toEqual(row.expect);
    });
  }
  it("without an app id the receiver row crosses whole", () => {
    expect([...consumedTopicsForApp(fixture.registry, loaded, "demonstrator", undefined)]).toEqual(fixture.registry[0]!.consumes);
  });
});

describe("demonstrator pack budget", () => {
  const root = join(dirname(fileURLToPath(import.meta.url)), "../../../../../");
  const hub = (name: string) => join(root, "🌎️hub/🧩️compositions", name, "🧩️component");
  const descriptor = (path: string) => JSON.parse(readFileSync(path, "utf8")).manifest as { pluginId: string; apps?: { id: string }[]; topicContributions?: { topic: string }[] };
  const consumesOf = (name: string) => [...(readFileSync(join(hub(name), "📦️packages/🦀️rust/Cargo.toml"), "utf8").match(/^consumes\s*=\s*\[([^\]]*)\]/m)?.[1] ?? "").matchAll(/"([^"]+)"/g)].map((match) => match[1]!);
  const budget = Number(/CONTRIBUTIONS_COMMAND_RAW_WIRE_BYTES: usize = ([\d_]+);/.exec(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🦀️.rs"), "utf8"))![1]!.replaceAll("_", ""));
  const hubs = ["🎪️demonstrator", "📐️cad", "🌍️gis", "🌀️procedural", "🏭️process", "🧩️puzzle", "🪵️sourcing"].filter((name) => existsSync(hub(name)));
  const extensionDirs = readdirSync(join(root, "✏️s/🔌️plugins")).flatMap((plugin) => existsSync(join(root, "✏️s/🔌️plugins", plugin, "🧩️extensions")) ? readdirSync(join(root, "✏️s/🔌️plugins", plugin, "🧩️extensions")).map((extension) => join(root, "✏️s/🔌️plugins", plugin, "🧩️extensions", extension, "🧩️component/🔣️.json")) : []).filter(existsSync);
  const manifests = [...hubs.map((name) => descriptor(join(hub(name), "🔣️.json"))), ...extensionDirs.map(descriptor)];
  const registry = hubs.map((name) => ({ pluginId: descriptor(join(hub(name), "🔣️.json")).pluginId, consumes: consumesOf(name) }));
  const loaded = manifests.map((manifest) => ({ pluginId: manifest.pluginId, manifest: { apps: manifest.apps ?? [], topicContributions: manifest.topicContributions ?? [] } }));
  const wire = (appId: string) => {
    const json = scopeContributionsJson(loaded as never, "demonstrator", consumedTopicsForApp(registry, loaded, "demonstrator", appId));
    return Buffer.byteLength(JSON.stringify(["setContributions", { json }]));
  };
  const retained = ["s.cad.cad@1/*#editor", "s.process.process3d@1/*#editor", "s.sourcing.curation@1/*#editor"];

  it("reads the budget from its single Rust source", () => expect(budget).toBe(49152));
  it("the union alone would overflow, which is why the cut is per app", () => {
    const union = Buffer.byteLength(JSON.stringify(["setContributions", { json: scopeContributionsJson(loaded as never, "demonstrator", registry.find((row) => row.pluginId === "demonstrator")!.consumes) }]));
    expect(union).toBeGreaterThan(budget);
  });
  for (const appId of retained) {
    it(`${appId} receives a pack inside the retained wire budget`, () => {
      const bytes = wire(appId);
      expect(bytes).toBeGreaterThan(0);
      expect(bytes).toBeLessThanOrEqual(budget);
    });
  }
  it("the generator app still receives every flow extension", () => {
    const pack = JSON.parse(scopeContributionsJson(loaded as never, "demonstrator", consumedTopicsForApp(registry, loaded, "demonstrator", "s.procedural.generation3d@1/*#editor"))) as { pluginId: string }[];
    expect(pack.filter((entry) => entry.pluginId.startsWith("flow-extension-")).length).toBeGreaterThanOrEqual(8);
  });
});
