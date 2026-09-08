type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { artifactKindChoices, decodeArtifactKindChoice, decodeSurfaceAppChoice, encodeArtifactKindChoice, encodeSurfaceAppChoice } = dependencies;
  type AppRole = any;
  type ArtifactDialect = any;
  type ArtifactKindChoice = any;
  type PluginManifest = any;
  type SurfaceAppChoice = any;

  const { describe, expect, it } = vitest;

  describe("🔖️HostResolvedArgs", () => {
    const PINNED_ARTIFACT_KIND_CHOICE: ArtifactKindChoice = {
      kindId: "s.draw.draw",
      schema: "draw.document",
      dialect: { artifactKind: "s.draw.draw", standard: "1", subset: "*" },
      label: { en: "Draw", de: "Zeichnung" },
    };
    const PINNED_ARTIFACT_KIND_CHOICE_JSON =
      '{"kindId":"s.draw.draw","schema":"draw.document","dialect":{"artifactKind":"s.draw.draw","standard":"1","subset":"*"},"label":{"en":"Draw","de":"Zeichnung"}}';

    it("encodeArtifactKindChoice matches the contract's pinned byte-identical fixture", () => {
      expect(encodeArtifactKindChoice(PINNED_ARTIFACT_KIND_CHOICE)).toBe(PINNED_ARTIFACT_KIND_CHOICE_JSON);
    });

    it("decodeArtifactKindChoice inverts the pinned fixture", () => {
      expect(decodeArtifactKindChoice(PINNED_ARTIFACT_KIND_CHOICE_JSON)).toEqual(PINNED_ARTIFACT_KIND_CHOICE);
    });

    it("decodeArtifactKindChoice throws naming the missing field", () => {
      expect(() => decodeArtifactKindChoice("{}")).toThrow(/kindId/);
    });

    it("encodeSurfaceAppChoice / decodeSurfaceAppChoice round-trip the frozen shape", () => {
      const choice: SurfaceAppChoice = { app: { pluginId: "draw", appId: "s.draw.draw@1/*#editor" }, role: "editor" };
      const json = encodeSurfaceAppChoice(choice);
      expect(json).toBe('{"pluginId":"draw","appId":"s.draw.draw@1/*#editor","role":"editor"}');
      expect(decodeSurfaceAppChoice(json)).toEqual(choice);
    });

    it("decodeSurfaceAppChoice throws on an invalid role", () => {
      expect(() => decodeSurfaceAppChoice('{"pluginId":"draw","appId":"a","role":"bogus"}')).toThrow(/role/);
    });

    function fakeManifest(pluginId: string, apps: readonly { role: AppRole; dialect: ArtifactDialect; documentSchema: string; label?: { en: string; de: string } }[]): PluginManifest {
      return {
        pluginId,
        label: pluginId,
        version: "1.0.0",
        apps: apps.map((app) => ({ role: app.role, dialect: app.dialect, label: { native: app.label ?? { en: app.dialect.artifactKind, de: app.dialect.artifactKind } }, io: { documentSchema: app.documentSchema } })),
        workflows: [],
        examples: [],
      };
    }

    it("artifactKindChoices dedupes by dialect coordinate (owner manifest first wins), sorts, and filters by role", () => {
      const drawDialect: ArtifactDialect = { artifactKind: "s.draw.draw", standard: "1", subset: "*" };
      const dagDialect: ArtifactDialect = { artifactKind: "s.dag.dag", standard: "1", subset: "*" };
      // 🗂️ Two manifests: "draw" (the owner plugin, passed first) offers the editor for its own
      // dialect plus an unrelated app with an empty `documentSchema` (must never surface — it hasn't
      // opted into `io` yet, mirroring apps that haven't populated `AppIo` in the Rust test's spirit);
      // "draw-contrib" is a later contributor offering only a viewer for the SAME dialect coordinate
      // under a different label, proving the owner's (first) label wins once both roles are in scope.
      const manifests = [
        fakeManifest("draw", [
          { role: "editor", dialect: drawDialect, documentSchema: "draw.document", label: { en: "Draw", de: "Zeichnung" } },
          { role: "editor", dialect: { artifactKind: "s.draw.empty", standard: "1", subset: "*" }, documentSchema: "" },
        ]),
        fakeManifest("draw-contrib", [{ role: "viewer", dialect: drawDialect, documentSchema: "draw.document", label: { en: "Draw (fallback)", de: "Zeichnung (fallback)" } }]),
        fakeManifest("dag", [{ role: "editor", dialect: dagDialect, documentSchema: "dag.document", label: { en: "DAG", de: "DAG" } }]),
      ];

      const editorOnly = artifactKindChoices(manifests, ["editor"]);
      expect(editorOnly.map((choice) => choice.kindId)).toEqual(["s.dag.dag", "s.draw.draw"]);
      expect(editorOnly.find((choice) => choice.kindId === "s.draw.draw")?.label).toEqual({ en: "Draw", de: "Zeichnung" });

      const editorAndViewer = artifactKindChoices(manifests, ["editor", "viewer"]);
      expect(editorAndViewer.map((choice) => choice.kindId)).toEqual(["s.dag.dag", "s.draw.draw"]);
      expect(editorAndViewer.find((choice) => choice.kindId === "s.draw.draw")?.label).toEqual({ en: "Draw", de: "Zeichnung" });

      expect(artifactKindChoices(manifests, ["viewer"]).map((choice) => choice.kindId)).toEqual(["s.draw.draw"]);
    });
  });

}
