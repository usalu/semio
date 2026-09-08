type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { OS_SHELL_SCHEMA_EXPORT_IDS, OS_SHELL_SCHEMA_ID, ShellSchemaError, osShellSchemaDocument, parseAnchor, parseByAnchor, parseLayoutNode, parseLoadedPlugin, parseShellCommand, parseShellError, parseShellEvent, parseShellState, parseUiLocale } = dependencies;

  const { describe, expect, it } = vitest;

  describe("os.shell schema module", () => {
    it("declares draft-07 and the owned $id", () => {
      expect(osShellSchemaDocument.$schema).toBe("http://json-schema.org/draft-07/schema#");
      expect(osShellSchemaDocument.$id).toBe(OS_SHELL_SCHEMA_ID);
    });

    it("has one $defs export per exported type of the rendered mirror", async () => {
      const { readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const mirror = readFileSync(join(dirname(fileURLToPath(source.url)), "..", "🤖️generated", "🟦️.ts"), "utf8");
      const rendered = [...mirror.matchAll(/^export type ([A-Za-z0-9_]+)/gmu)].map((match) => match[1]).sort();
      expect([...OS_SHELL_SCHEMA_EXPORT_IDS].sort()).toEqual(rendered);
    });

    it("accepts every committed fixture's state, command and expectation", async () => {
      const { readdirSync, readFileSync } = await import("node:fs");
      const { fileURLToPath } = await import("node:url");
      const { dirname, join } = await import("node:path");
      const fixturesDir = join(dirname(fileURLToPath(source.url)), "..", "🧫️fixtures");
      const files = readdirSync(fixturesDir).filter((name) => name.endsWith(".json"));
      expect(files.length).toBeGreaterThan(0);
      for (const file of files) {
        const fixture = JSON.parse(readFileSync(join(fixturesDir, file), "utf8")) as { state: unknown; command: unknown; expected: { state?: unknown; events?: unknown[]; error?: unknown } };
        expect(() => parseShellState(fixture.state), file).not.toThrow();
        expect(() => parseShellCommand(fixture.command), file).not.toThrow();
        if (fixture.expected.state !== undefined) expect(() => parseShellState(fixture.expected.state), file).not.toThrow();
        for (const event of fixture.expected.events ?? []) expect(() => parseShellEvent(event), file).not.toThrow();
        if (fixture.expected.error !== undefined) expect(() => parseShellError(fixture.expected.error), file).not.toThrow();
      }
    });

    it("rejects values outside the declared value space", () => {
      expect(() => parseAnchor("middle")).toThrow(ShellSchemaError);
      expect(() => parseUiLocale("fr")).toThrow(ShellSchemaError);
      expect(parseUiLocale("de")).toBe("de");
      expect(() => parseLoadedPlugin({ pluginId: "p", moduleUrl: "u" })).toThrow(/missing required property 'label'/u);
      expect(() => parseLoadedPlugin({ pluginId: "p", moduleUrl: "u", label: null, extra: 1 })).toThrow(/unexpected property 'extra'/u);
      expect(() => parseLayoutNode({ kind: "split", orientation: "sideways", children: [], sizes: [] })).toThrow(ShellSchemaError);
      expect(parseLayoutNode({ kind: "split", orientation: "horizontal", children: [{ kind: "leaf", windowId: "w1" }], sizes: [1] })).toEqual({ kind: "split", orientation: "horizontal", children: [{ kind: "leaf", windowId: "w1" }], sizes: [1] });
      expect(() => parseShellCommand({ type: "setPanelVisible", anchor: "left" })).toThrow(ShellSchemaError);
      expect(() => parseByAnchor({ left: 1, right: 2, top: 3 })).toThrow(ShellSchemaError);
    });
  });

}
