/** 📂️ Canonical artifact-opening relay conformance against shared language-neutral vectors. */

import Ajv, { type ValidateFunction } from "ajv";
import { applyPatch } from "fast-json-patch";
import openingScopeFixture from "../../🧱️elements/🏛️ShellHost/🧭️opening/🧪️fixtures/📍️scope/🔣️.json";
import { AppRouter, type AppRouterManifest, type OpeningPreferences } from "@semio-tech/framework";
import { resolveArtifactOpeningRelay } from "@semio-tech/framework-os";
import { describe, expect, it } from "vitest";
import { resolveDocumentOpeningBindings, resolveDocumentOpeningTarget } from "../../🧱️elements/🏛️ShellHost/🧭️opening/🟦️.ts";
import rendererSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import artifactOpeningFixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🚪️open-artifact/🔣️.json";

const ownedExports = new Ajv({ strict: true, allErrors: true }).addKeyword("discriminator").addKeyword("x-semio-note").addSchema(rendererSchema);
/** 🧬️ Compiles one named `$defs` export of the `os.renderer` schema module. */
const rendererExport = (exportId: string): ValidateFunction =>
  ownedExports.getSchema(`${rendererSchema.$id}#/$defs/${exportId}`) as ValidateFunction;

describe("artifact opening relay", () => {
  it("resolves every schema-valid vector through the live router and opening preferences", () => {
    const validate = rendererExport("OpenArtifactRelayV1");
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

describe("document opening scope", () => {
  it("pins exact shared destinations and never infers them from the active route", () => {
    const resolution = rendererExport("DocumentOpeningScopeResolutionV1");
    expect(openingScopeFixture.cases.every((row) => resolution(row)) && rendererExport("DocumentFirstOpenV1")(openingScopeFixture.firstOpen)).toBe(true);
    for (const row of openingScopeFixture.cases) {
      if (row.error) {
        expect(() => resolveDocumentOpeningBindings(row.ref, row.context), row.id).toThrow(row.error);
      } else {
        const reference = applyPatch(
          [],
          (row.expected ?? []).map((value) => ({ op: "add" as const, path: "/-", value: structuredClone(value) })),
          true,
          false,
        ).newDocument;
        const actual = resolveDocumentOpeningBindings(row.ref, row.context);
        expect(actual, row.id).toEqual(reference);
        expect(actual, row.id).toEqual(row.expected);
      }
      console.log("[DEBUG] document-opening-scope", row.id, row.error ?? "exact-bindings");
    }
  });
});
