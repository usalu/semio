/** 🔁️ The declared all-plugin rebuild chain is well formed, builds each component once for describe AND staging, proves
 * its `s` staging before the catalog, and selects contiguous step ranges. */
import { existsSync, mkdtempSync, readFileSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { getWorkspaceRoot } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { guestFrameworkCheckArgs, readGuestFrameworkChecks, readRebuildChain, REBUILD_STAGES, selectRebuildSteps } from "../../🔁️rebuild/🟦️.ts";

describe("rebuild-all chain", () => {
  it("declares input gates, descriptors, registry, guests and catalog in dependency order", () => {
    const chain = readRebuildChain();
    expect(chain.map((step) => step.id)).toEqual(["provenance", "mutation-authority", "guest-framework", "components", "generate", "check", "activate-s", "verify-s", "flow-core-bindings", "preflight-catalog", "publish-catalog"]);
    expect([...new Set(chain.map((step) => step.stage))]).toEqual([...REBUILD_STAGES]);
    expect(chain.find((step) => step.id === "components")?.command.slice(0, 5)).toEqual(["nx", "run-many", "-t", "describe", "materialize-dev"]);
    expect(chain.findIndex((step) => step.id === "preflight-catalog")).toBe(chain.findIndex((step) => step.id === "publish-catalog") - 1);
    expect(chain.find((step) => step.id === "verify-s")?.command.slice(-2)).toEqual(["--variant", "s"]);
    expect(chain.find((step) => step.id === "publish-catalog")?.command.slice(-2)).toEqual(["--packages", "all"]);
  });

  it("selects the contiguous range --from/--to name and refuses unknown or inverted bounds", () => {
    const chain = readRebuildChain();
    expect(selectRebuildSteps(chain, []).map((step) => step.id)).toEqual(chain.map((step) => step.id));
    expect(selectRebuildSteps(chain, ["--to", "verify-s"]).map((step) => step.id)).toEqual(["provenance", "mutation-authority", "guest-framework", "components", "generate", "check", "activate-s", "verify-s"]);
    expect(selectRebuildSteps(chain, ["--from", "generate", "--to", "check"]).map((step) => step.id)).toEqual(["generate", "check"]);
    expect(() => selectRebuildSteps(chain, ["--from", "check", "--to", "generate"])).toThrow(/after --to/);
    expect(() => selectRebuildSteps(chain, ["--to", "nowhere"])).toThrow(/names no step/);
    expect(() => selectRebuildSteps(chain, ["--to", "check", "--to", "generate"])).toThrow(/usage/);
    expect(() => selectRebuildSteps(chain, ["--from"])).toThrow(/usage/);
  });

  it("checks the guest-linked framework crates for both wasm targets before any component builds", () => {
    const chain = readRebuildChain();
    expect(chain.findIndex((step) => step.id === "guest-framework")).toBeLessThan(chain.findIndex((step) => step.id === "components"));
    const checks = readGuestFrameworkChecks();
    expect([...new Set(checks.map((check) => check.target))].sort()).toEqual(["wasm32-unknown-unknown", "wasm32-wasip2"]);
    expect(guestFrameworkCheckArgs(checks[0]!).slice(0, 6)).toEqual(["check", "--manifest-path", "Cargo.toml", "--lib", "--target", "wasm32-wasip2"]);
    expect(checks.flatMap((check) => check.packages)).toContain("semio-framework-os-kernel");
    expect(checks.at(-1)).toEqual({ target: "wasm32-wasip2", workspace: "✏️s/Cargo.toml", packages: ["semio-s-plugin-stdio"], features: [] });
    expect(checks.every((check) => existsSync(join(getWorkspaceRoot(), check.workspace)))).toBe(true);
    const root = mkdtempSync(join(tmpdir(), "rebuild-guest-checks-"));
    const write = (checks: unknown): string => {
      const path = join(root, `${Math.random()}.json`);
      writeFileSync(path, JSON.stringify({ schema: "semio.plugin-registry.rebuild-chain/v1", steps: [], guestFrameworkChecks: checks }));
      return path;
    };
    expect(() => readGuestFrameworkChecks(write([]))).toThrow(/no guestFrameworkChecks/);
    expect(() => readGuestFrameworkChecks(write([{ target: "x86_64-apple-darwin", workspace: "Cargo.toml", packages: ["semio-framework"], features: [] }]))).toThrow(/known target/);
    expect(() => readGuestFrameworkChecks(write([{ target: "wasm32-wasip2", workspace: "Cargo.toml", packages: ["semio-framework"], features: ["other-crate/sync"] }]))).toThrow(/crate-qualified/);
    expect(() => readGuestFrameworkChecks(write([{ target: "wasm32-wasip2", packages: ["semio-framework"], features: [] }]))).toThrow(/workspace Cargo\.toml/);
    expect(() => readGuestFrameworkChecks(write([{ target: "wasm32-wasip2", workspace: "../Cargo.toml", packages: ["semio-framework"], features: [] }]))).toThrow(/workspace Cargo\.toml/);
    expect(() => readGuestFrameworkChecks(write([{ target: "wasm32-wasip2", workspace: "/tmp/Cargo.toml", packages: ["semio-framework"], features: [] }]))).toThrow(/workspace Cargo\.toml/);
    expect(() => readGuestFrameworkChecks(write([{ target: "wasm32-wasip2", workspace: "Cargo.toml", packages: ["semio-framework"], features: [], extra: true }]))).toThrow(/unknown keys/);
  });

  it("validates the committed chain with an independent JSON Schema engine that agrees with the reader on portable workspaces", async () => {
    const { default: Ajv } = await import("ajv");
    const schema = JSON.parse(readFileSync(new URL("../../🔁️rebuild/🧬️schema/🔣️.json", import.meta.url), "utf8"));
    const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
    const committed = JSON.parse(readFileSync(new URL("../../🔁️rebuild/🔣️.json", import.meta.url), "utf8"));
    expect(validate(committed), JSON.stringify(validate.errors)).toBe(true);
    expect(schema.$defs.RebuildStepV1.properties.stage.enum).toEqual([...REBUILD_STAGES]);
    const root = mkdtempSync(join(tmpdir(), "rebuild-guest-schema-"));
    const workspaces = ["Cargo.toml", "✏️s/Cargo.toml", "🌎️hub/Cargo.toml", "../Cargo.toml", "/tmp/Cargo.toml", "..\\x\\Cargo.toml", "x\\Cargo.toml", "C:x/Cargo.toml", "C:/Cargo.toml", "a//Cargo.toml", "./Cargo.toml", "a/../Cargo.toml", "Cargo.tomlx", "x/Cargo.toml/"];
    for (const workspace of workspaces) {
      const check = { target: "wasm32-wasip2", workspace, packages: ["semio-framework"], features: [] };
      const path = join(root, `${Math.random()}.json`);
      writeFileSync(path, JSON.stringify({ ...committed, guestFrameworkChecks: [check] }));
      const independent = validate({ ...committed, guestFrameworkChecks: [check] });
      let accepted = true;
      try { readGuestFrameworkChecks(path); } catch { accepted = false; }
      expect(accepted, workspace).toBe(independent);
    }
    expect(workspaces.filter((workspace) => validate({ ...committed, guestFrameworkChecks: [{ target: "wasm32-wasip2", workspace, packages: ["semio-framework"], features: [] }] }))).toEqual(["Cargo.toml", "✏️s/Cargo.toml", "🌎️hub/Cargo.toml"]);
  });

  it("refuses a chain whose stages run backwards or repeat a step", () => {
    const root = mkdtempSync(join(tmpdir(), "rebuild-chain-"));
    const write = (steps: unknown[]): string => {
      const path = join(root, `${steps.length}-${Math.random()}.json`);
      writeFileSync(path, JSON.stringify({ schema: "semio.plugin-registry.rebuild-chain/v1", steps }));
      return path;
    };
    const describeStep = { id: "components", stage: "descriptors", command: ["nx", "run-many", "-t", "describe", "materialize-dev"] };
    const catalogStep = { id: "publish-catalog", stage: "catalog", command: ["nx", "run", "os-hub:trusted-catalog-bootstrap"] };
    expect(() => readRebuildChain(write([catalogStep, describeStep]))).toThrow(/stage order/);
    expect(() => readRebuildChain(write([describeStep, describeStep]))).toThrow(/duplicated/);
    expect(() => readRebuildChain(write([{ ...describeStep, command: ["cargo", "build"] }]))).toThrow(/nx command/);
  });
});
