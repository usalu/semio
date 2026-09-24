type TestSource = { readonly directory: string; readonly url: string };

export async function registerTests1(
  vitest: NonNullable<ImportMeta["vitest"]>,
  dependencies: Pick<
    typeof import("../../🟦️.ts"),
    "ensureMcpBinary" | "mcpBinaryFreshness" | "requireMcpBinary" | "resolveBuiltMcpBinaryPath" | "resolveMcpBinarySourcesPath" | "resolveMcpBinaryPath"
  > &
    Pick<typeof import("node:fs"), "chmodSync" | "copyFileSync" | "mkdirSync" | "mkdtempSync" | "readFileSync" | "rmSync" | "utimesSync" | "writeFileSync"> &
    Pick<typeof import("node:path"), "join"> &
    Pick<typeof import("node:os"), "tmpdir">,
  source: TestSource,
): Promise<void> {
  const {
    chmodSync,
    copyFileSync,
    ensureMcpBinary,
    join,
    mcpBinaryFreshness,
    mkdirSync,
    mkdtempSync,
    readFileSync,
    requireMcpBinary,
    resolveBuiltMcpBinaryPath,
    resolveMcpBinarySourcesPath,
    resolveMcpBinaryPath,
    rmSync,
    tmpdir,
    utimesSync,
    writeFileSync,
  } = dependencies;
  const { CARGO_BINARY_SOURCES_SCHEMA_V1, CARGO_BUILD_OWNER_PID_ENV, cargoBinarySourcesFreshnessV1, cargoBuildOwnerAliveV1, cargoDepInfoSourcesV1 } = await import("../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts");
  const gate = JSON.parse(readFileSync(new URL("./🎚️config/🧱️binary-gate.json", source.url), "utf8")) as {
    sourcesFile: string;
    depInfoCases: Array<{ name: string; text: string; sources: string[] }>;
    freshnessCases: Array<{ name: string; builtAtMs: number; modified: Record<string, number | null>; fresh: boolean; changed: string | null }>;
  };

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

    it("stages once when absent, skips while every recorded source is unchanged, and restages after a dependency changes", () => {
      const fakeRepo = mkdtempSync(join(tmpdir(), "semio-mcp-repo-"));
      let staged = 0;
      try {
        const relParts = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/dist/build".split("/");
        const fakeArtifact = join(fakeRepo, ...relParts);
        mkdirSync(fakeArtifact, { recursive: true });
        const fakeBinary = join(fakeArtifact, process.platform === "win32" ? "semio-os-mcp.exe" : "semio-os-mcp");
        const ownSource = join(fakeRepo, "own.rs");
        const dependencySource = join(fakeRepo, "kernel.rs");
        writeFileSync(ownSource, "fn main() {}\n");
        writeFileSync(dependencySource, "pub fn abi() {}\n");
        const past = new Date(Date.now() - 60_000);
        utimesSync(ownSource, past, past);
        utimesSync(dependencySource, past, past);
        const fakeStaging = {
          stage: () => {
            staged += 1;
            copyFileSync(process.execPath, fakeBinary);
            if (process.platform !== "win32") chmodSync(fakeBinary, 0o755);
            writeFileSync(resolveMcpBinarySourcesPath(fakeBinary), JSON.stringify({ schema: CARGO_BINARY_SOURCES_SCHEMA_V1, builtAtMs: Date.now(), sources: [ownSource, dependencySource] }));
            return 0;
          },
        };
        expect(mcpBinaryFreshness(fakeBinary).fresh).toBe(false);
        expect(ensureMcpBinary(fakeRepo, {}, process.platform, fakeStaging)).toBe(fakeBinary);
        expect(staged).toBe(1);
        expect(resolveMcpBinarySourcesPath(fakeBinary).endsWith(gate.sourcesFile)).toBe(true);
        expect(ensureMcpBinary(fakeRepo, {}, process.platform, fakeStaging)).toBe(fakeBinary);
        expect(staged).toBe(1);
        const future = new Date(Date.now() + 60_000);
        utimesSync(dependencySource, future, future);
        expect(mcpBinaryFreshness(fakeBinary)).toEqual({ fresh: false, reason: `${dependencySource} changed after the staged build` });
        expect(ensureMcpBinary(fakeRepo, {}, process.platform, fakeStaging)).toBe(fakeBinary);
        expect(staged).toBe(2);
      } finally {
        rmSync(fakeRepo, { recursive: true, force: true });
      }
    });

    it("treats a binary without its sources record as stale, never as fresh", () => {
      const fakeRepo = mkdtempSync(join(tmpdir(), "semio-mcp-repo-"));
      try {
        const fakeBinary = join(fakeRepo, "semio-os-mcp");
        copyFileSync(process.execPath, fakeBinary);
        expect(mcpBinaryFreshness(fakeBinary)).toEqual({ fresh: false, reason: "the staged binary carries no sources record" });
        writeFileSync(resolveMcpBinarySourcesPath(fakeBinary), "{\"schema\":\"other\"}");
        expect(mcpBinaryFreshness(fakeBinary)).toEqual({ fresh: false, reason: "the staged binary's sources record is malformed" });
      } finally {
        rmSync(fakeRepo, { recursive: true, force: true });
      }
    });
  });

  describe("cargo binary sources record", () => {
    for (const row of gate.depInfoCases) {
      it(`dep-info: ${row.name}`, () => {
        expect(cargoDepInfoSourcesV1(row.text)).toEqual(row.sources);
      });
    }
    it("a staging build lives exactly as long as the process it was started for", async () => {
      const { spawnSync } = await import("node:child_process");
      const finished = spawnSync(process.execPath, ["-e", "0"]).pid;
      expect(cargoBuildOwnerAliveV1({})).toBe(true);
      expect(cargoBuildOwnerAliveV1({ [CARGO_BUILD_OWNER_PID_ENV]: String(process.pid) })).toBe(true);
      expect(cargoBuildOwnerAliveV1({ [CARGO_BUILD_OWNER_PID_ENV]: String(finished) })).toBe(false);
    });
    for (const row of gate.freshnessCases) {
      it(`freshness: ${row.name}`, () => {
        const record: import("../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts").CargoBinarySourcesV1 = { schema: CARGO_BINARY_SOURCES_SCHEMA_V1, builtAtMs: row.builtAtMs, sources: Object.keys(row.modified) };
        expect(cargoBinarySourcesFreshnessV1(record, (path) => row.modified[path] ?? null)).toEqual({ fresh: row.fresh, changed: row.changed });
      });
    }
  });
}
