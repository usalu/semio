/** 📂️ Canonical artifact-opening relay conformance against shared language-neutral vectors. */

import Ajv from "ajv";
import { AppRouter, type AppRouterManifest, type OpeningPreferences } from "@semio-tech/framework";
import { resolveArtifactOpeningRelay } from "@semio-tech/framework-os";
import { describe, expect, it } from "vitest";
import { resolveDocumentOpeningTarget } from "../../../../🧱️elements/ShellHost/🧭️opening/🟦️.ts";
import artifactOpeningSchema from "../../../../🧱️elements/ShellHelpers/🧪️fixtures/📂️open-artifact/🧬️schema.json";
import artifactOpeningFixture from "../../../../🧱️elements/ShellHelpers/🧪️fixtures/📂️open-artifact/🔣️.json";

describe("artifact opening relay", () => {
  it("resolves every schema-valid vector through the live router and opening preferences", () => {
    const validate = new Ajv({ strict: true, allErrors: true }).compile(artifactOpeningSchema);
    expect(validate(artifactOpeningFixture)).toBe(true);
    const router = AppRouter.build(artifactOpeningFixture.manifests as readonly AppRouterManifest[]);
    const preferences = artifactOpeningFixture.preferences as OpeningPreferences;
    for (const vector of artifactOpeningFixture.valid) {
      expect(resolveArtifactOpeningRelay(vector.actionId, vector.args, router, preferences), vector.id).toEqual(vector.expected);
    }
    for (const vector of artifactOpeningFixture.invalid) {
      expect(() => resolveArtifactOpeningRelay(vector.actionId, vector.args, router, preferences), vector.id).toThrow(vector.error);
    }
  });

  it("attaches the document to the newly-created session before React publishes it", () => {
    const previous = { pluginId: "draw", instanceId: 1 };
    const next = { pluginId: "ink", instanceId: 2 };
    const draw = { pluginId: "draw" };
    const ink = { pluginId: "ink" };
    expect(resolveDocumentOpeningTarget({ session: next, plugin: ink }, previous, [{ handle: draw }, { handle: ink }])).toEqual({ session: next, plugin: ink });
    expect(resolveDocumentOpeningTarget(undefined, previous, [{ handle: draw }, { handle: ink }])).toEqual({ session: previous, plugin: draw });
  });
});
