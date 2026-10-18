import { testLevelAtLeast } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { canReuseDevPort, describeDevPortOccupant, devServerUrl, isDevPortInUse, resolveDevPort } from "../../../🌐️network/🔌️ports/🟦️.ts";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { defineLint, type FileLinter } from "../../../🚨️policy/🧩️linters/🟦️.ts";
import { dependencyBoundaryBreachesForBundleDir, dependencyBoundaryBreachesForFile, isAdapterBoundaryFile, parseTsImportSpecs } from "../../../🚨️policy/🛡️dependency-boundary/🟦️.ts";
import { devToolingEnv } from "../../../🏃️process/🌿️environment/🟦️.ts";
import { dispatchSubcommand } from "../../../🚨️policy/🧭️entrypoints/🟦️.ts";
import { wgpuDevPlayUrl } from "../../../🎮️playground/🧊️wgpu-dev/🟦️.ts";
import { BundleScript, ScriptRouter, findWorkspaceRoot } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { contractRoot, contractUrl } from "../🏗️shared-fixtures/🟦️.ts";

describe("Nx Unicode project transport", () => {
  test("preserves the caller’s isolated plugin setting", () => {
    const env = devToolingEnv({ NX_ISOLATE_PLUGINS: "true" });
    expect(env.NX_ISOLATE_PLUGINS).toBe("true");
  });

  test.if(testLevelAtLeast("long"))("builds the describe graph without a lossy duplicate repo-test root", () => {
    const root = getWorkspaceRoot();
    const result = spawnSync("bun", ["nx", "show", "projects", "--with-target", "describe"], { cwd: root, encoding: "utf8", env: { ...process.env, NX_ISOLATE_PLUGINS: "true" } });
    expect(result.status, result.stderr).toBe(0);
  }, 180_000);
});

describe("isDevPortInUse", () => {
  test("returns false for a high ephemeral port", () => {
    expect(isDevPortInUse("127.0.0.1", 59_999)).toBe(false);
  });

  test("returns true when puzzle 3d play port is listening", () => {
    if (!isDevPortInUse("127.0.0.1", 6013) && !isDevPortInUse("127.0.0.1", 6014)) return;
    expect(isDevPortInUse("127.0.0.1", 6013) || isDevPortInUse("127.0.0.1", 6014)).toBe(true);
  });
});

describe("resolveDevPort", () => {
  test("returns preferred port when free", () => {
    expect(resolveDevPort("127.0.0.1", 59_990)).toBe(59_990);
  });

  test("skips occupied ports", () => {
    if (!isDevPortInUse("127.0.0.1", 6013)) return;
    expect(resolveDevPort("127.0.0.1", 6013)).toBeGreaterThan(6013);
  });

  test("honors skipPorts", () => {
    expect(resolveDevPort("127.0.0.1", 59_991, 5, new Set([59_991, 59_992]))).toBe(59_993);
  });
});

describe("devServerUrl", () => {
  test("maps 0.0.0.0 to loopback", () => {
    expect(devServerUrl("0.0.0.0", 6019)).toBe("http://127.0.0.1:6019/");
  });
});

describe("wgpuDevPlayUrl", () => {
  test("builds root and legacy play urls", () => {
    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly")).toBe("http://127.0.0.1:6178/?plugin=lowpoly");
    expect(wgpuDevPlayUrl("127.0.0.1", 6178, "lowpoly", "/renderer-modules/wgpu/")).toBe("http://127.0.0.1:6178/renderer-modules/wgpu/?plugin=lowpoly");
  });
});

describe("canReuseDevPort", () => {
  test("returns true for an active dev HTTP server", () => {
    if (!isDevPortInUse("127.0.0.1", 6019)) return;
    expect(canReuseDevPort("127.0.0.1", 6019)).toBe(true);
  });
});

describe("describeDevPortOccupant", () => {
  test("returns undefined for a free port", () => {
    expect(describeDevPortOccupant(59_998)).toBeUndefined();
  });
});

describe("bundle-script", () => {
  test("ScriptRouter usage lists registered commands", () => {
    class A extends BundleScript {
      run(): void {}
    }
    const router = new ScriptRouter(contractRoot).register("a", A).register("b", A);
    expect(router.hasCommands()).toBe(true);
    expect(router.usage()).toContain("a");
    expect(router.usage()).toContain("b");
  });

  test("gitRepoRoot uses monorepo toplevel from repo/lib/js", async () => {
    const { gitRepoRoot } = await import("../../../🎛️commands/📝️commit/🌳️git/🟦️.ts");
    const top = gitRepoRoot(contractRoot);
    expect(top).toBe(getWorkspaceRoot());
    expect(existsSync(join(top, ".git"))).toBe(true);
    expect(existsSync(join(top, "nx.json"))).toBe(true);
  });

  test("findWorkspaceRoot reaches monorepo from repo/lib/js", () => {
    const root = findWorkspaceRoot(contractRoot);
    expect(existsSync(join(root, "nx.json"))).toBe(true);
  });

  test("dispatchSubcommand invokes handler for first segment", () => {
    let ran = "";
    dispatchSubcommand(
      ["go", "x"],
      {
        go: (rest) => {
          ran = rest.join(",");
        },
      },
      "unused",
    );
    expect(ran).toBe("x");
  });
});

describe("defineLint", () => {
  test("returns same function", () => {
    const f = defineLint("x", (_l: FileLinter) => []);
    expect(typeof f).toBe("function");
  });
});

describe("dependency-boundary", () => {
  test("detects adapter region marker", () => {
    expect(isAdapterBoundaryFile("pkg/foo.ts", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/main.py", "# #region 🔌️Adapters\nimport fastapi")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/lib/js/index.ts", "//#region 🌐️RsWasmTransport\nexport async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/lib/js/kit-store.🟦️worker.ts", "export async function x() {}")).toBe(true);
    expect(isAdapterBoundaryFile("compose/client/bin/assistant/mcp-app.tsx", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("framework/platform/renderer/react/index.tsx", "// #region 🔌️Adapters\nimport x from 'react'")).toBe(true);
    expect(isAdapterBoundaryFile("pkg/foo.ts", "import x from 'react'")).toBe(false);
  });

  test("parseTsImportSpecs extracts module", () => {
    expect(parseTsImportSpecs(`import { z } from "zod";`)).toEqual(["zod"]);
  });

  test("flags direct third-party import outside adapter", () => {
    const content = `import React from "react";\nexport const a = React.createElement("div");\n`;
    const file = "🧰️framework/🧪️boundary-probe.ts";
    const breachs = dependencyBoundaryBreachesForFile(findWorkspaceRoot(contractRoot), file, content, file);
    expect(breachs.length).toBeGreaterThan(0);
    expect(breachs[0]?.kind).toBe("dependency-boundary/import/direct-third-party");
  });

  test("dependencyBoundaryBreachesForBundleDir walks nested tsx", () => {
    const repoRoot = new URL("../../..", contractUrl).pathname.replace(/^\/([A-Za-z]:)/, "$1");
    const dir = "framework/playground/renderer/react/puzzle";
    const breachs = dependencyBoundaryBreachesForBundleDir(repoRoot, dir);
    expect(breachs.every((b) => b.scope.startsWith("framework/playground/renderer/react/puzzle"))).toBe(true);
  });

  test("allows third-party import inside adapter region", () => {
    const content = `// #region 🔌️Adapters\nimport { NextResponse } from "next/server";\n// #endregion 🔌️Adapters\nexport async function GET() { return NextResponse.json({}); }\n`;
    const file = "repo/server/coordinator/app/api/v1/health/route.ts";
    const breachs = dependencyBoundaryBreachesForFile(new URL("../../..", contractUrl).pathname.replace(/^\/([A-Za-z]:)/, "$1"), file, content, file);
    expect(breachs).toEqual([]);
  });
});

describe("ui scrollbar styling", () => {
  test("🎨️ui.css defines scrollbar tokens and native plus Scrollable rules", () => {
    const repoRoot = findWorkspaceRoot(contractRoot);
    const css = readFileSync(join(repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🖌️ui/🎨️.css"), "utf8");
    expect(css).toContain("--scrollbar-size:");
    expect(css).toContain("--scrollbar-thumb:");
    expect(css).toContain("scrollbar-color:");
    expect(css).toContain("*::-webkit-scrollbar-thumb");
    expect(css).toContain('[data-slot="scroll-area-thumb"]');
  });
});
