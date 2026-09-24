type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: Pick<
    typeof import("../../🟦️.ts"),
    "ensureMcpBinary" | "mcpSourceContentHash" | "requireMcpBinary" | "resolveBuiltMcpBinaryPath" | "resolveMcpBinaryContentHashPath" | "resolveMcpBinaryPath"
  > &
    Pick<typeof import("node:fs"), "chmodSync" | "copyFileSync" | "mkdirSync" | "mkdtempSync" | "readFileSync" | "rmSync" | "writeFileSync"> &
    Pick<typeof import("node:path"), "join"> &
    Pick<typeof import("node:os"), "tmpdir">,
  source: TestSource,
): Promise<void> {
  const {
    chmodSync,
    copyFileSync,
    ensureMcpBinary,
    join,
    mcpSourceContentHash,
    mkdirSync,
    mkdtempSync,
    readFileSync,
    requireMcpBinary,
    resolveBuiltMcpBinaryPath,
    resolveMcpBinaryContentHashPath,
    resolveMcpBinaryPath,
    rmSync,
    tmpdir,
    writeFileSync,
  } = dependencies;

  const { describe, expect, it } = vitest;

  describe("resolveMcpBinaryPath", () => {
    const fixture = JSON.parse(readFileSync(new URL("./🎚️config/🧱️binary-gate.json", source.url), "utf8")) as {
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

  describe("ensureMcpBinary", () => {
    it("skips staging when an explicit override is set", () => {
      let staged = 0;
      const path = ensureMcpBinary("/", { SEMIO_OS_MCP_BIN: process.execPath }, process.platform, {
        stage: () => {
          staged += 1;
          return 0;
        },
      });
      expect(path).toBe(process.execPath);
      expect(staged).toBe(0);
    });

    it("stages once when absent, then skips on a matching content-hash stamp", () => {
      const fakeRepo = mkdtempSync(join(tmpdir(), "semio-mcp-repo-"));
      let staged = 0;
      try {
        const relParts = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build".split("/");
        const fakeArtifact = join(fakeRepo, ...relParts);
        mkdirSync(fakeArtifact, { recursive: true });
        const fakeBinary = join(fakeArtifact, process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp");
        const mcpRoot = join(fakeRepo, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp");
        mkdirSync(mcpRoot, { recursive: true });
        writeFileSync(join(mcpRoot, "Cargo.toml"), "[package]\nname=\"probe\"\n");
        const hash = mcpSourceContentHash(fakeRepo);
        expect(hash).toMatch(/^[0-9a-f]{64}$/);
        const fakeStaging = {
          stage: () => {
            staged += 1;
            copyFileSync(process.execPath, fakeBinary);
            if (process.platform !== "win32") chmodSync(fakeBinary, 0o755);
            return 0;
          },
        };
        const first = ensureMcpBinary(fakeRepo, {}, process.platform, fakeStaging);
        expect(first).toBe(fakeBinary);
        expect(staged).toBe(1);
        expect(readFileSync(resolveMcpBinaryContentHashPath(fakeBinary), "utf8").trim()).toBe(hash);
        const second = ensureMcpBinary(fakeRepo, {}, process.platform, fakeStaging);
        expect(second).toBe(fakeBinary);
        expect(staged).toBe(1);
      } finally {
        rmSync(fakeRepo, { recursive: true, force: true });
      }
    });
  });
}
