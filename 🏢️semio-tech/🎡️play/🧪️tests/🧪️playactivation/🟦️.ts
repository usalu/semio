import { readFileSync } from "node:fs";
import { join } from "node:path";
import { runtimeComponentClosure } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";

export async function registerTests1(vitest: NonNullable<ImportMeta["vitest"]>, dependencies: any, repoRoot: string): Promise<void> {
  const { playActivationLanes, playPaneClosureRoot, playRuntimeComponentIds, mergePlayActivationReceipts, playExtensionDirectory, activationFilesDigestSync } = dependencies;
  const { describe, expect, it } = vitest;

  //#region 🧪️PlayActivationTests
  const sha = (seed: string): string => seed.repeat(64).slice(0, 64);
  const lane = (name: string, plugins: readonly (readonly [string, string, number])[], receiptMtimeMs?: number) => ({
    lane: name,
    ...(receiptMtimeMs === undefined ? {} : { receiptMtimeMs }),
    receipt: { schema: "semio.dev.activation/v1", variant: name, profile: "dev", plugins: plugins.map(([pluginId, seed, rebuiltAt]) => ({ pluginId, artifactSha256: sha(seed), rebuiltAt })) },
  });

  /** @emoji 💿️ The staged artifact the merge reads off disk, as a fixture: one sha per component, exactly
   * as {@link playInstalledArtifactSha256} answers it, `undefined` for a component that is not staged. */
  const installed = (seeds: Readonly<Record<string, string | null>>) => (pluginId: string) => seeds[pluginId] === undefined || seeds[pluginId] === null ? undefined : sha(seeds[pluginId]!);

  describe("playActivationLanes", () => {
    it("covers exactly the play union with pane lanes only", () => {
      const components = [...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS];
      const covered = new Set<string>();
      for (const variant of playActivationLanes()) for (const id of runtimeComponentClosure(components, [playPaneClosureRoot(PLAYGROUND_BUILD_TARGETS.find(row => row.variant === variant)!)])) covered.add(id);
      expect([...covered].sort()).toEqual([...playRuntimeComponentIds()].sort());
    });

    it("leaves the host shell lane out", () => {
      expect(playActivationLanes()).not.toContain("s");
    });

    it("is exactly what the Nx graph activates and prepares", () => {
      const project = JSON.parse(readFileSync(join(repoRoot, "🏢️semio-tech/🎡️play/📋️project.json"), "utf8"));
      const lanes = [...playActivationLanes()].sort();
      const from = (target: string, pattern: RegExp) => project.targets[target].dependsOn.flatMap((entry: string) => pattern.exec(entry)?.[1] ?? []).sort();
      expect(from("activate-dev", /^@semio-tech\/framework-os-dev:activate-(.+)-react-dev$/)).toEqual(lanes);
      expect(from("prepare-dev", /^@semio-tech\/framework-os-dev:prepare-(.+)-react-dev$/)).toEqual(lanes);
      expect(from("prepare-release", /^@semio-tech\/framework-os-dev:prepare-(.+)-react-release$/)).toEqual(lanes);
    });
  });

  describe("mergePlayActivationReceipts", () => {
    it("unions every lane into one receipt named after the primary lane", () => {
      const merged = mergePlayActivationReceipts([lane("demonstrator", [["cad", "b", 7], ["demonstrator", "c", 7]]), lane("energy", [["energy", "d", 9]])], ["cad", "demonstrator", "energy"], "demonstrator");
      expect(merged.variant).toBe("demonstrator");
      expect(merged.plugins.map((row: any) => row.pluginId)).toEqual(["cad", "demonstrator", "energy"]);
    });

    it("keeps the earliest rebuiltAt when lanes agree", () => {
      const merged = mergePlayActivationReceipts([lane("draw", [["flow", "b", 12]]), lane("flow", [["flow", "b", 3]])], ["flow"], "draw");
      expect(merged.plugins[0].rebuiltAt).toBe(3);
    });

    it("refuses lanes that disagree about one artifact when nothing on disk decides between them", () => {
      expect(() => mergePlayActivationReceipts([lane("draw", [["flow", "b", 7]]), lane("flow", [["flow", "f", 7]])], ["flow"], "draw")).toThrow(/Stale play activation lane: flow/);
    });

    it("says nothing at all when every lane agrees", () => {
      const warnings: string[] = [];
      mergePlayActivationReceipts([lane("draw", [["flow", "b", 12]]), lane("flow", [["flow", "b", 3]])], ["flow"], "draw", { installedArtifactSha256: installed({ flow: "b" }), warn: (line: string) => warnings.push(line) });
      expect(warnings).toEqual([]);
    });

    it("serves the lane whose row matches the installed artifact and names the stale ones", () => {
      const warnings: string[] = [];
      const merged = mergePlayActivationReceipts(
        [lane("draw", [["flow", "b", 7]]), lane("flow", [["flow", "f", 9]]), lane("gis2d", [["flow", "b", 11]])],
        ["flow"],
        "draw",
        { installedArtifactSha256: installed({ flow: "f" }), warn: (line: string) => warnings.push(line) },
      );
      expect(merged.plugins[0].artifactSha256).toBe(sha("f"));
      expect(merged.plugins[0].rebuiltAt).toBe(9);
      expect(warnings).toEqual([`[play] lane drift: flow served from flow (${sha("f").slice(0, 8)}), stale in draw, gis2d`]);
    });

    it("breaks a tie between two lanes that both activated the installed artifact by the newer receipt, keeping the earliest rebuiltAt", () => {
      const warnings: string[] = [];
      const merged = mergePlayActivationReceipts(
        [lane("draw", [["flow", "b", 4]], 1_000), lane("flow", [["flow", "f", 9]], 2_000), lane("gis2d", [["flow", "b", 6]], 3_000)],
        ["flow"],
        "draw",
        { installedArtifactSha256: installed({ flow: "b" }), warn: (line: string) => warnings.push(line) },
      );
      expect(merged.plugins[0].artifactSha256).toBe(sha("b"));
      expect(merged.plugins[0].rebuiltAt).toBe(4);
      expect(warnings).toEqual([`[play] lane drift: flow served from gis2d (${sha("b").slice(0, 8)}), stale in flow`]);
    });

    it("refuses a disagreement no lane activated, naming every lane and the installed artifact", () => {
      expect(() => mergePlayActivationReceipts([lane("draw", [["flow", "b", 7]]), lane("flow", [["flow", "f", 7]])], ["flow"], "draw", { installedArtifactSha256: installed({ flow: "c" }) }))
        .toThrow(new RegExp(`Stale play activation lane: flow is ${sha("b")} in draw but ${sha("f")} in flow, and the installed artifact \\(${sha("c")}\\) matches none of them`));
    });

    it("refuses a disagreement whose component is not staged at all", () => {
      expect(() => mergePlayActivationReceipts([lane("draw", [["flow", "b", 7]]), lane("flow", [["flow", "f", 7]])], ["flow"], "draw", { installedArtifactSha256: installed({ flow: null }) }))
        .toThrow(/the installed artifact \(unreadable\) matches none of them/);
    });

    it("refuses a union that misses or exceeds the expected components", () => {
      expect(() => mergePlayActivationReceipts([lane("cad", [["cad", "c", 7], ["vcs", "a", 7]])], ["cad", "fem"], "cad")).toThrow(/missing: fem; extra: vcs/);
    });

    it("refuses a lane directory holding another variant's receipt", () => {
      expect(() => mergePlayActivationReceipts([{ ...lane("energy", [["energy", "d", 7]]), lane: "fem3d" }], ["energy"], "fem3d")).toThrow(/lane fem3d carries a energy receipt/);
    });

    it("refuses a release receipt", () => {
      const release = lane("energy", [["energy", "d", 7]]);
      expect(() => mergePlayActivationReceipts([{ ...release, receipt: { ...release.receipt, profile: "release" } }], ["energy"], "energy")).toThrow(/not a dev activation: release/);
    });
  });

  describe("playInstalledArtifactSha256", () => {
    /** @emoji 🔏️ The whole lane-drift resolution rests on play computing the SAME artifact identity the
     * activation writes into a receipt. Play restates that digest synchronously (a Vite config factory
     * cannot await), so this law pins the restatement to the framework's own streaming digest over a
     * fixture holding what a staged module directory holds: nested paths, a `\0` byte, an empty file and
     * a file whose bytes are a prefix of another's — the exact cases a naive concatenation would collide. */
    it("digests staged files byte-for-byte like the framework activation does", async () => {
      const { createReadStream, mkdtempSync, mkdirSync, statSync, writeFileSync } = await import("node:fs");
      const { createHash } = await import("node:crypto");
      const { tmpdir } = await import("node:os");
      // 🔏️ The framework's own digest is executed from its SOURCE TEXT rather than imported: importing
      // `♻️activation/📥️installation/🟦️.ts` drags the whole materialization tool-chain (typescript, the
      // wasm bootstrap, the registry discovery walk) through Vitest's transform and blows this suite's
      // time budget on its own. Extracting the function body keeps the comparison against the LIVE rule.
      const source = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/♻️activation/📥️installation/🟦️.ts"), "utf8");
      const body = /\nasync function activationFilesDigest\([^)]*\): Promise<string> \{\n([\s\S]*?)\n\}\n/.exec(source)?.[1];
      expect(body, "activationFilesDigest moved or changed shape in ♻️activation/📥️installation/🟦️.ts — re-derive activationFilesDigestSync against it").toBeTruthy();
      const reference = new Function("files", "signal", "createHash", "statSync", "createReadStream", `return (async () => {\n${body}\n})();`) as (files: ReadonlyMap<string, string>, signal: AbortSignal, hash: unknown, stat: unknown, read: unknown) => Promise<string>;
      const root = mkdtempSync(join(tmpdir(), "play-activation-digest-"));
      mkdirSync(join(root, "🪞️vendor"), { recursive: true });
      const files = new Map<string, string>();
      for (const [name, bytes] of [["🌉️bridge.js", "export const a = 1;"], ["🔣️.json", ""], ["🪞️vendor/🧵️shard.js", "export const a = 1"], ["🪞️vendor/🧊️component.wasm", "\0asm\0\0\0"]] as const) {
        writeFileSync(join(root, ...name.split("/")), bytes);
        files.set(name, join(root, ...name.split("/")));
      }
      expect(activationFilesDigestSync(files)).toBe(await reference(files, new AbortController().signal, createHash, statSync, createReadStream));
    });
  });

  describe("playExtensionDirectory", () => {
    const modules = join("/repo", "dist", "release", "plugin-modules");

    it("serves a development extension from the lane that staged it", () => {
      expect(playExtensionDirectory("flow-extension-math", modules, new Map([["flow-extension-math", "/repo/lane/flow/extensions/flow-extension-math"]]))).toBe("/repo/lane/flow/extensions/flow-extension-math");
    });

    it("falls back to the build profile's plugin-module root when no lane staged it", () => {
      expect(playExtensionDirectory("flow-extension-math", modules, new Map())).toBe(join(modules, "flow-extension-math"));
    });

    it("falls back the same way in a build, where there is no activation at all", () => {
      expect(playExtensionDirectory("flow-extension-math", modules)).toBe(join(modules, "flow-extension-math"));
    });
  });
  //#endregion 🧪️PlayActivationTests
}
