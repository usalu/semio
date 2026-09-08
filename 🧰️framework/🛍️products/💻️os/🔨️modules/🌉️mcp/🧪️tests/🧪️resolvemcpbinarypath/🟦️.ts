type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, source: TestSource): Promise<void> {
  const { readFileSync, requireMcpBinary, resolveBuiltMcpBinaryPath, resolveMcpBinaryPath } = dependencies;

  const { describe, expect, it } = vitest;

  describe("resolveMcpBinaryPath", () => {
    const fixture = JSON.parse(readFileSync(new URL("./🧫️fixtures/🧱️binary-gate.json", source.url), "utf8")) as {
      pathCases: Array<{ name: string; platform: NodeJS.Platform; repoRoot: string; environment: NodeJS.ProcessEnv; expected: string }>;
    };

    for (const testCase of fixture.pathCases) {
      it(testCase.name, () => {
        expect(resolveMcpBinaryPath(testCase.repoRoot, testCase.environment, testCase.platform)).toBe(testCase.expected);
      });
    }

    it("accepts an independently executable process artifact", () => {
      expect(requireMcpBinary("/", { SEMIO_OS_MCP_BIN: process.execPath })).toBe(process.execPath);
    });

    it("rejects a missing explicit artifact instead of permitting a skipped suite", () => {
      expect(() => requireMcpBinary("/workspace/semio", { SEMIO_OS_MCP_BIN: "missing/semio-os-mcp" }, "linux")).toThrow("binary gate failed");
    });

    it("keeps the staged consumer independent from mutable compiler directories", () => {
      for (const platform of ["darwin", "linux", "win32"] as const) {
        const root = platform === "win32" ? "C:\\repo" : "/repo";
        expect(resolveMcpBinaryPath(root, { CARGO_TARGET_DIR: "scratch/target" }, platform)).toBe(resolveMcpBinaryPath(root, {}, platform));
        expect(resolveBuiltMcpBinaryPath(root, { CARGO_TARGET_DIR: "scratch/target" }, platform)).not.toBe(resolveMcpBinaryPath(root, {}, platform));
      }
    });

  });


}
