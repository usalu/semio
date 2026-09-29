import { describe, expect, it } from "vitest";
import { scopeContributionsJson } from "../../🟦️.ts";
import { dialectFromLegacyExampleAppId, examplesForApp, normalizeManifestExampleRow, normalizeManifestExamples, type PluginManifest } from "../../../🛂️manifest/🟦️.ts";
import fixture from "../../🧫️fixtures/🔬️scope-contributions/🔣️.json";

describe("normalizeManifestExamples", () => {
  it("stamps dialect from a legacy appId so the navbar picker can resolve the row", () => {
    const legacy = { id: "demo", appId: "s.sourcing.curation@1/*#editor", artifactJson: "semio …" };
    const normalized = normalizeManifestExampleRow(legacy);
    expect(normalized.appId).toBeUndefined();
    expect(examplesForApp([normalized], { dialect: dialectFromLegacyExampleAppId("s.sourcing.curation@1/*#editor") }).map((row) => row.id)).toEqual(["demo"]);
  });
  it("leaves rows that already carry dialect unchanged", () => {
    const dialect = { artifactKind: "s.draw.drawing", standard: "1", subset: "*" };
    const row = { id: "demo", dialect, artifactJson: "…" };
    expect(normalizeManifestExamples({ examples: [row] }).examples?.[0]).toEqual(row);
  });
});

describe("scopeContributionsJson", () => {
  const loaded = fixture.loaded.map((entry) => ({ pluginId: entry.pluginId, manifest: { topicContributions: [{ topic: entry.topic, payload: { pluginId: entry.pluginId } }] } as Pick<PluginManifest, "topicContributions"> }));
  for (const row of fixture.cases) {
    it(row.id, () => {
      const scoped = JSON.parse(scopeContributionsJson(loaded, row.receiver, row.consumes)) as { pluginId: string; topicContribution: { topic: string } }[];
      expect(scoped.map((entry) => entry.pluginId)).toEqual(row.expect);
      for (const entry of scoped) expect(entry.pluginId === row.receiver || row.consumes.includes(entry.topicContribution.topic)).toBe(true);
    });
  }
  it("forwards a consumed topic whole, independent of any open document", () => {
    const flow = fixture.cases.find((row) => row.id === "flow-receives-every-extension")!;
    const extensions = fixture.loaded.filter((entry) => entry.topic === "flow.extension").map((entry) => entry.pluginId);
    expect(JSON.parse(scopeContributionsJson(loaded, flow.receiver, flow.consumes)).map((entry: { pluginId: string }) => entry.pluginId)).toEqual(extensions);
  });
});
