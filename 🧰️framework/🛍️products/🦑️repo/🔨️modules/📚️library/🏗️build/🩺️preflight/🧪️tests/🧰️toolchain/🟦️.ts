/** 🧰️ The preflight demands exactly what the native toolchain needs and every refusal explains itself: each tool of each platform has an English and German remedy, the probes of the runtime and the policy name the same tools, the Linux packages the policy declares are installed by the devcontainer image and by the bootstrap, and the Windows and macOS bootstraps install what their remedies point to. */
import {describe, expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {requireTestToolV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {findWorkspaceRoot} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {nativeToolchainRequirementsV1,nativeToolchainToolIdsV1} from "../../../../../../../../📦️packages/🟦️typescript/⚙️runtime/🧲️native/🧰️toolchain/🟦️.ts";
import {readBuildPreflightPolicyV1} from "../../🟦️.ts";
import texts from "../../💊️remedies/📖️texts/🔣️.json";

const root = findWorkspaceRoot(import.meta.dir), policy = readBuildPreflightPolicyV1(), platforms = ["linux", "darwin", "win32"] as const;
const bootstrap = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/", read = (path: string): string => readFileSync(join(root, path), "utf8");

describe("native toolchain requirements of the preflight", () => {
  test("every required tool of every platform has an English and a German remedy for exactly that platform", () => {
    for (const platform of platforms) for (const tool of policy.platforms[platform].tools) {
      const row = (texts.tools as Record<string, Record<string, {en: string; de: string}>>)[tool]?.[platform];
      expect(row, `${platform} ${tool}`).toBeDefined();
      expect(row!.en.length).toBeGreaterThan(30);
      expect(row!.de).not.toBe(row!.en);
    }
  });

  test("the runtime probes and the policy name the same tools: what the native toolchain needs is demanded, what is probed has a remedy", () => {
    for (const platform of platforms) {
      const required = nativeToolchainRequirementsV1(platform);
      expect(required.length, platform).toBeGreaterThan(0);
      for (const id of required) expect(policy.platforms[platform].tools, `${platform} ${id}`).toContain(id);
      for (const id of policy.platforms[platform].tools.filter(tool => nativeToolchainToolIdsV1().includes(tool))) expect(required, `${platform} ${id}`).toContain(id);
    }
    for (const platform of ["linux", "darwin"] as const) expect(policy.platforms[platform].tools).toContain("cc");
    expect(policy.platforms.win32.tools).not.toContain("cc");
    expect(policy.platforms.win32.tools).toContain("msvc-linker");
  });

  test("the Linux packages the policy declares are installed by the devcontainer image and named by the bootstrap, as an independent reader sees them", () => {
    const declared = Object.values(policy.platforms.linux.apt ?? {}).flat();
    expect(declared).toContain("build-essential");
    const python = requireTestToolV1("python"), run = spawnSync(python, ["-I", join(import.meta.dir, "🐍️oracle", "🐍️.py"), join(root, ".devcontainer", "Dockerfile"), join(root, bootstrap, "🐚️.sh")], {encoding: "utf8"});
    expect(run.status, run.stderr).toBe(0);
    const lists = JSON.parse(run.stdout) as {dockerfile: string[]; bootstrap: string[]};
    for (const name of declared) { expect(lists.dockerfile, `Dockerfile ${name}`).toContain(name); expect(lists.bootstrap, `bootstrap ${name}`).toContain(name); }
    expect(Object.keys(policy.platforms.linux.apt ?? {}).every(tool => policy.platforms.linux.tools.includes(tool))).toBe(true);
    expect(read(".devcontainer/Dockerfile")).toContain("build-essential");
  });

  test("the bootstraps install what the remedies point to", () => {
    const windows = read(bootstrap + "🔵️.ps1"), posix = read(bootstrap + "🐚️.sh");
    expect(windows).toContain("Microsoft.VisualStudio.BuildTools");
    expect(windows).toContain("Microsoft.VisualStudio.Workload.VCTools");
    expect(windows).toContain("rustup");
    expect(posix).toContain("xcode-select");
    expect(posix).toContain("rustup toolchain install");
    const remedy = texts.tools["msvc-linker"].win32;
    expect(remedy.en).toContain("Microsoft.VisualStudio.BuildTools");
    expect(remedy.de).toContain("Microsoft.VisualStudio.BuildTools");
    expect(texts.tools["developer-tools"].darwin.en).toContain("xcode-select --install");
    for (const platform of platforms) { expect(texts.tools.rust[platform].en).toContain("repo tools setup all"); expect(texts.tools.rust[platform].de).toContain("repo tools setup all"); }
  });
});
