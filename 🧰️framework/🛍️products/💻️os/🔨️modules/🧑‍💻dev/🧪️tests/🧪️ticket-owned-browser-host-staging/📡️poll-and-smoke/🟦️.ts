/** 📡️ The poll census helpers and the catalog smoke aggregation of the ticket-owned browser host staging suite. */
import { createTestKit, type TestDependencies, type TestSource } from "../🧰️test-kit/🟦️.ts";

export async function registerPollAndSmokeTests(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: TestDependencies, source: TestSource): Promise<void> {
  const { EventEmitter, awaitChildExit, awaitHttpOk, awaitTcpReady, catalogSmokeExitCode, catalogSmokeMarkdown, join, readFileSync, repoRoot, summarizeCatalogSmoke } = dependencies;
  const { describe, expect, it } = vitest;
  const { devContractModule } = createTestKit(vitest, dependencies, source);

  type SpawnDaemonHandle = import("@semio-tech/repo-lib/process").SpawnDaemonHandle;

  //#region 🔖️PollHelpers-tests
  describe("awaitTcpReady (W6 poll-census helper)", () => {
    it("honours its deadline and reports timeout — no real sleeps: fake clock + fake probe", async () => {
      let fakeNow = 0;
      const now = () => fakeNow;
      const sleep = async (ms: number) => {
        fakeNow += ms;
      };
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 1000,
        intervalMs: 250,
        probe: () => false,
        now,
        sleep,
      });
      expect(outcome).toBe("timeout");
      expect(fakeNow).toBeGreaterThanOrEqual(1000);
    });

    it("resolves ready as soon as the injected probe reports the port open", async () => {
      let calls = 0;
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 10_000,
        intervalMs: 250,
        probe: () => {
          calls++;
          return calls >= 3;
        },
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("ready");
      expect(calls).toBe(3);
    });

    it("resolves closed-mode ready once the injected probe reports the port free", async () => {
      let calls = 0;
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 10_000,
        intervalMs: 250,
        mode: "closed",
        probe: () => {
          calls++;
          return calls < 2;
        },
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("ready");
      expect(calls).toBe(2);
    });

    it("resolves dead as soon as isDead() reports true, before the deadline", async () => {
      let fakeNow = 0;
      const outcome = await awaitTcpReady("127.0.0.1", 9999, {
        deadlineMs: 10_000,
        intervalMs: 250,
        probe: () => false,
        isDead: () => true,
        now: () => fakeNow,
        sleep: async (ms) => {
          fakeNow += ms;
        },
      });
      expect(outcome).toBe("dead");
      expect(fakeNow).toBe(0);
    });
  });

  describe("awaitHttpOk (W6 poll-census helper)", () => {
    it("honours its deadline and reports timeout — no real sleeps: fake clock + always-throwing fetch", async () => {
      let fakeNow = 0;
      const outcome = await awaitHttpOk("http://127.0.0.1:9999/admin/api/overview", {
        deadlineMs: 1000,
        intervalMs: 500,
        fetchImpl: (async () => {
          throw new Error("ECONNREFUSED");
        }) as unknown as typeof fetch,
        now: () => fakeNow,
        sleep: async (ms) => {
          fakeNow += ms;
        },
      });
      expect(outcome).toBe("timeout");
    });

    it("resolves ready once the injected fetch stops throwing", async () => {
      let calls = 0;
      const outcome = await awaitHttpOk("http://127.0.0.1:9999/admin/api/overview", {
        deadlineMs: 10_000,
        intervalMs: 500,
        fetchImpl: (async () => {
          calls++;
          if (calls < 2) throw new Error("ECONNREFUSED");
          return {} as unknown as Response;
        }) as unknown as typeof fetch,
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("ready");
      expect(calls).toBe(2);
    });

    it("resolves dead as soon as isDead() reports true, before attempting to fetch", async () => {
      let fetchCalled = false;
      const outcome = await awaitHttpOk("http://127.0.0.1:9999/admin/api/overview", {
        deadlineMs: 10_000,
        intervalMs: 500,
        isDead: () => true,
        fetchImpl: (async () => {
          fetchCalled = true;
          return {} as unknown as Response;
        }) as unknown as typeof fetch,
        now: () => 0,
        sleep: async () => {},
      });
      expect(outcome).toBe("dead");
      expect(fetchCalled).toBe(false);
    });
  });

  describe("awaitChildExit (W6 event-driven fix — replaces polling child.exitCode)", () => {
    it("resolves as soon as the child's own 'exit' event fires, without polling", async () => {
      const fakeChild = new EventEmitter() as unknown as SpawnDaemonHandle["child"];
      Object.assign(fakeChild, { exitCode: null });
      const resultPromise = awaitChildExit(fakeChild, 30_000, { timeoutAfter: () => new Promise<"timeout">(() => {}) });
      (fakeChild as unknown as { exitCode: number | null }).exitCode = 0;
      (fakeChild as unknown as InstanceType<typeof EventEmitter>).emit("exit", 0, null);
      const result = await resultPromise;
      expect(result).toBe("exited");
    });

    it("resolves immediately for a child that had already exited before the call", async () => {
      const fakeChild = new EventEmitter() as unknown as SpawnDaemonHandle["child"];
      Object.assign(fakeChild, { exitCode: 0 });
      const result = await awaitChildExit(fakeChild, 30_000, { timeoutAfter: () => new Promise<"timeout">(() => {}) });
      expect(result).toBe("exited");
    });

    it("still times out for a hung child that never emits 'exit' — fake deadline, no real sleep", async () => {
      const fakeChild = new EventEmitter() as unknown as SpawnDaemonHandle["child"];
      Object.assign(fakeChild, { exitCode: null });
      const result = await awaitChildExit(fakeChild, 30_000, {
        timeoutAfter: async () => "timeout" as const,
      });
      expect(result).toBe("timeout");
    });
  });

  //#endregion 🔖️PollHelpers-tests
  //#region 🔖️CatalogSmoke-tests
  describe("catalog smoke aggregation", () => {
    const fixture = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧪️tests/🧫️fixtures/🔬️catalog-smoke.json"), "utf8"));

    it("folds program outcomes and install statuses into the committed language-agnostic report", () => {
      const report = summarizeCatalogSmoke(fixture.input);
      expect(report).toEqual(fixture.report);
      expect(catalogSmokeExitCode(report)).toBe(fixture.exitCode);
      expect(catalogSmokeMarkdown(report)).toBe(fixture.markdown);
    });

    it("matches the committed report schema under an independent validator", async () => {
      const { default: Ajv } = await import("ajv");
      const validate = await devContractModule("CatalogSmokeReportV1");
      expect(validate(fixture.report), JSON.stringify(validate.errors)).toBe(true);
      expect(validate({ ...fixture.report, totals: { pass: 1, fail: 1 } })).toBe(false);
      expect(validate({ ...fixture.report, programs: [{ ...fixture.report.programs[0], status: "unknown" }] })).toBe(false);
    });

    it("exits non-zero for a failed program, a failed plugin, or an empty catalog, and zero only for an all-pass run", () => {
      const green = summarizeCatalogSmoke({ ...fixture.input, programs: [fixture.input.programs[0]], plugins: [{ pluginId: "draw", status: "loaded" }] });
      expect(catalogSmokeExitCode(green)).toBe(0);
      expect(catalogSmokeExitCode(summarizeCatalogSmoke({ ...fixture.input, programs: [fixture.input.programs[1]], plugins: [] }))).toBe(1);
      expect(catalogSmokeExitCode(summarizeCatalogSmoke({ ...fixture.input, programs: [fixture.input.programs[0]], plugins: [{ pluginId: "stdio", status: "crashed" }] }))).toBe(1);
      expect(catalogSmokeExitCode(summarizeCatalogSmoke({ ...fixture.input, programs: [], plugins: [] }))).toBe(1);
    });

    it("fails and reports the shell's own boot diagnostics when no program could ever be spawned", () => {
      const boot = { beacon: "error:s", failure: "shell never reached a ready beacon (error:s)", phaseMs: { commit: 210, firstModule: 260, beacon: 31000 }, bodyExcerpt: "no plugins loaded", consoleErrors: ["[os-shell] plugin install/reload failed", "boom"] };
      const report = summarizeCatalogSmoke({ ...fixture.input, boot, programs: [], plugins: [] });
      expect(report.totals).toEqual({ pass: 0, fail: 0, skipped: 0 });
      expect(catalogSmokeExitCode(report)).toBe(1);
      const markdown = catalogSmokeMarkdown(report);
      expect(markdown).toContain("FAILED: shell never reached a ready beacon (error:s)");
      expect(markdown).toContain("commit 210 ms, first module 260 ms, beacon 31000 ms");
      expect(markdown).toContain("no plugins loaded");
      expect(markdown).toContain("- [os-shell] plugin install/reload failed");
    });

    it("bounds the boot diagnostics it carries instead of embedding a whole session log", () => {
      const boot = { beacon: null, failure: "timeout", phaseMs: { commit: 190, firstModule: null, beacon: null }, bodyExcerpt: "x".repeat(5000), consoleErrors: Array.from({ length: 100 }, (_, index) => `error ${index}`) };
      const report = summarizeCatalogSmoke({ ...fixture.input, boot, programs: [], plugins: [] });
      expect(report.boot.bodyExcerpt).toHaveLength(1000);
      expect(report.boot.consoleErrors).toHaveLength(20);
    });

    it("escapes table-breaking cell content instead of corrupting the markdown row", () => {
      const report = summarizeCatalogSmoke({ ...fixture.input, programs: [{ ...fixture.input.programs[0], firstError: "boom | on\nline two" }], plugins: [] });
      const row = catalogSmokeMarkdown(report).split("\n").find((line) => line.startsWith("| draw "))!;
      expect(row.replaceAll("\\|", "").split("|")).toHaveLength(9);
      expect(row).toContain("boom \\| on line two");
    });
  });
  //#endregion 🔖️CatalogSmoke-tests
}
