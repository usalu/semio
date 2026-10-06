/** 📂️ Canonical artifact-opening relay conformance against shared language-neutral vectors. */

import { type ValidateFunction } from "ajv";
import { applyPatch } from "fast-json-patch";
import openingScopeFixture from "../../🧱️elements/🏛️ShellHost/🧭️opening/🧫️fixtures/📍️scope/🔣️.json";
import { AppRouter, type AppRouterManifest, type OpeningPreferences } from "@semio-tech/framework";
import { resolveArtifactOpeningRelay, type DirectoryEvent } from "@semio-tech/framework-os";
import { describe, expect, it } from "vitest";
import { resolveDocumentOpeningBindings, resolveDocumentOpeningTarget, sharedDocumentOpeningRoleV1, sharedDocumentRoleSwitchRefusedV1 } from "../../🧱️elements/🏛️ShellHost/🧭️opening/🟦️.ts";
import openPlanFixture from "../../../../../🧫️fixtures/📇️directory/🧭️document-open-plan-v1.json";
import rendererSchema from "../../../🧬️schema/🔣️.json" with { type: "json" };
import artifactOpeningFixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/🚪️open-artifact/🔣️.json";
import { semioSchemaAjvV1 } from "../../../../../../../🔨️modules/🧬️schema/🔮️oracles/✅️validator/🟦️.ts";

const ownedExports = semioSchemaAjvV1({ strict: true, allErrors: true }).addSchema(rendererSchema);
/** 🧬️ Compiles one named `$defs` export of the `os.renderer` schema module. */
const rendererExport = (exportId: string): ValidateFunction =>
  ownedExports.getSchema(`${rendererSchema.$id}#/$defs/${exportId}`) as ValidateFunction;

describe("artifact opening relay", () => {
  it("resolves every schema-valid vector through the live router and opening preferences", () => {
    
    
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
    }
  });
});

describe("shared document opening access", () => {
  it("requests the surface the hub issues for the caller's space role, folded from the space's own events", () => {
    
    for (const row of openingScopeFixture.accessCases) {
      
      expect(sharedDocumentOpeningRoleV1(row.events as unknown as readonly DirectoryEvent[], row.spaceId, row.userId), row.id).toBe(row.role);
    }
  });

  it("refuses a read-only member's switch to the editor, and only that switch", () => {
    for (const row of openingScopeFixture.accessCases) {
      const access = row.role as "editor" | "viewer" | null;
      expect(sharedDocumentRoleSwitchRefusedV1(access, "editor"), `${row.id} → editor`).toBe(access === "viewer");
      expect(sharedDocumentRoleSwitchRefusedV1(access, "viewer"), `${row.id} → viewer`).toBe(false);
    }
    expect(openingScopeFixture.accessCases.some((row) => row.role === "viewer") && openingScopeFixture.accessCases.some((row) => row.role === "editor") && openingScopeFixture.accessCases.some((row) => row.role === null)).toBe(true);
  });

  it("agrees with the hub's open-plan contract for every session role it names (the hub's own laws run the same cases)", () => {
    const editorIntents = openPlanFixture.issueCases.filter((row) => row.subjectKind === "session" && typeof row.role === "string" && !("removePath" in row) && !("replacePath" in row));
    const roles = [...new Set(editorIntents.map((row) => row.role as string))];
    expect(roles.sort()).toEqual(["author", "spectator"]);
    for (const role of roles) {
      const hubIssuesEditor = editorIntents.some((row) => row.role === role && row.expected === "accepted" && "expectedWrite" in row && row.expectedWrite === true);
      const hubRefusesEditor = editorIntents.some((row) => row.role === role && row.expected === "component-unavailable");
      expect(hubIssuesEditor !== hubRefusesEditor, role).toBe(true);
      const events = [
        { seq: 1, spaceId: "space-o", recordedAtMs: 1, body: { kind: "space.created", spaceId: "space-o", name: "Oracle", spaceKind: "studio", visibility: "private", ownerUserId: "u-owner" } },
        { seq: 2, spaceId: "space-o", recordedAtMs: 2, body: { kind: "member.upserted", spaceId: "space-o", userId: "u-caller", role } },
      ] as unknown as readonly DirectoryEvent[];
      expect(sharedDocumentOpeningRoleV1(events, "space-o", "u-caller"), role).toBe(hubIssuesEditor ? "editor" : "viewer");
    }
  });
});
