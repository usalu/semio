type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: Pick<typeof import("../../🟦️.ts"), "parseMediaTransportProps" | "renderMedia">, source: TestSource): Promise<void> {
  const { parseMediaTransportProps, renderMedia } = dependencies;
  const { describe, expect, it } = vitest;
  const view = {
    durationMs: 60_000,
    positionMs: 61_000,
    selectionStartMs: 1_000,
    selectionEndMs: 70_000,
    kind: "video" as const,
    mediaType: "video/mp4",
    revision: "9007199254740993",
    locale: "de" as const,
    resource: { kind: "artifact-media-export" as const, controllerId: "stdio.mp4.editor", appInstanceId: 23, parentDocumentId: "document-mp4-1", outputPort: "playback:out" as const, revision: "9007199254740993", generation: "9007199254740995" },
    capability: { status: "loading" as const, reason: null },
    hostContentHeight: 360,
  };

  describe("media transport contract", () => {
    it("projects localized bounded timing into the reserved host extension", () => {
      const node = renderMedia(view);
      if (node.component.type !== "extension") throw new Error("expected extension");
      expect(node.component.extension).toBe("framework.media.transport@1");
      expect(node.component.props).toMatchObject({ durationMs: 60_000, positionMs: 60_000, selectionStartMs: 1_000, selectionEndMs: 60_000, locale: "de", labels: { play: "Wiedergabe", cancel: "Abbrechen" } });
    });

    it("keeps unknown duration honest and rejects lossy revisions or foreign fields", () => {
      const node = renderMedia({ ...view, durationMs: null, positionMs: 25, selectionStartMs: 1, selectionEndMs: 2 });
      if (node.component.type !== "extension") throw new Error("expected extension");
      const props = parseMediaTransportProps(node.component.props);
      expect(props).toMatchObject({ durationMs: null, positionMs: 0, selectionStartMs: null, selectionEndMs: null });
      expect(() => parseMediaTransportProps({ ...props, revision: "18446744073709551616" })).toThrow("media-transport.identity");
      expect(() => parseMediaTransportProps({ ...props, unexpected: true })).toThrow("media-transport.props");
    });

    it("matches the language-agnostic JSON Schema through Ajv", async () => {
      const { readFile } = await import("node:fs/promises");
      const { default: Ajv2020 } = await import("ajv/dist/2020");
      const schema = JSON.parse(await readFile(new URL("../../🧬️contract/🔣️.json", source.url), "utf8"));
      const node = renderMedia(view);
      if (node.component.type !== "extension") throw new Error("expected extension");
      const validate = new Ajv2020({ strict: true }).compile(schema);
      expect(validate(node.component.props), JSON.stringify(validate.errors)).toBe(true);
    });
  });
}
