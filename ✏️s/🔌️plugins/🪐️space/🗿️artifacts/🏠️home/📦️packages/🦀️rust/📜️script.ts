#!/usr/bin/env bun
/** 📦️ space-home Rust artifact package router. */
import { runArtifactRustPackageMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🦀️rust/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runCmd, runCargo, runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
if (segments[0] === "home-host-panel-owner") {
      const oracle = join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧬️schema/📌️panel-state/🧪️tests/🔬️unit/🟦️.ts");
      const directoryOracle = join(this.repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🫧️transient/🧬️schema/📇️directory-projection/🧪️tests/🔬️unit/🟦️.ts");
      const { testHostPanelStateSchema } = await import(oracle);
      const { testHomeDirectoryProjectionSchema } = await import(directoryOracle);
      const { testResolvedHostContext } = await import(join(this.repoRoot, "🧰️framework/🔨️modules/🛂️manifest/🪟️view-context/🧪️tests/🪟️resolved-host-context/🟦️.ts"));
      testHostPanelStateSchema();
      testHomeDirectoryProjectionSchema(this.repoRoot);
      testResolvedHostContext();
      const homeRoot = join(this.repoRoot, "✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home");
      const homeSources: string[] = [];
      const collectHomeSources = (directory: string): void => {
        for (const entry of readdirSync(directory, { withFileTypes: true })) {
          const path = join(directory, entry.name);
          if (entry.isDirectory()) collectHomeSources(path);
          else if ([".rs", ".ts", ".json", ".graphql", ".proto", ".py", ".feature"].includes(extname(entry.name))) homeSources.push(path);
        }
      };
      collectHomeSources(homeRoot);
      const homePanelMirror = /active_panel_tab|activePanelTab|SetActivePanelTab|setActivePanelTab|active-panel-tab/;
      const homePanelResidue = homeSources.filter((path) => homePanelMirror.test(readFileSync(path, "utf8"))).map((path) => relative(this.repoRoot, path));
      if (homePanelResidue.length !== 0) throw new Error(`Home panel ownership still leaks into app sources: ${homePanelResidue.join(", ")}`);
      const homeSessionIdentityMirror = /\bSetClient\b|\bsetClient\b|\bclient_id\b|\bclient_name\b|\bclientId\b|\bclientName\b/;
      const homeSessionIdentityResidue = homeSources.filter((path) => homeSessionIdentityMirror.test(readFileSync(path, "utf8"))).map((path) => relative(this.repoRoot, path));
      if (homeSessionIdentityResidue.length !== 0) throw new Error(`Home session identity still leaks into app sources: ${homeSessionIdentityResidue.join(", ")}`);
      const spaceConfigRoot = join(this.repoRoot, "✏️s/🔌️plugins/🪐️space/⚙️engine/🪐️space/🎚️config");
      const spaceConfigSources: string[] = [];
      collectHomeSources(spaceConfigRoot);
      for (const path of homeSources) if (path.startsWith(spaceConfigRoot)) spaceConfigSources.push(path);
      const spaceIdentityResidue = spaceConfigSources.filter((path) => homeSessionIdentityMirror.test(readFileSync(path, "utf8"))).map((path) => relative(this.repoRoot, path));
      if (spaceIdentityResidue.length !== 0) throw new Error(`Studio session identity still leaks into app config: ${spaceIdentityResidue.join(", ")}`);
      const spaceManifest = JSON.parse(readFileSync(join(this.repoRoot, "🌎️hub/🧩️compositions/🪐️space/🔣️.json"), "utf8")) as { manifest: { apps: { id: string; windowKinds: { actions: { id: string }[] }[] }[] } };
      const panelActionCount = (appId: string): number => spaceManifest.manifest.apps.find((app) => app.id === appId)?.windowKinds.flatMap((window) => window.actions).filter((action) => action.id === "setActivePanelTab").length ?? 0;
      if (panelActionCount("s.space.home@1/*#editor") !== 0 || panelActionCount("s.space.home@1/*#viewer") !== 0) throw new Error("generated Home apps still declare setActivePanelTab");
      if (panelActionCount("s.space.studio@1/*#editor") !== 3) throw new Error("generated Studio panel actions changed during Home-only cleanup");
      const setClientActionCount = spaceManifest.manifest.apps.flatMap((app) => app.windowKinds).flatMap((window) => window.actions).filter((action) => action.id === "setClient").length;
      if (setClientActionCount !== 0) throw new Error("generated Space manifest still declares the retired setClient action");
      const homeEditor = spaceManifest.manifest.apps.find((app) => app.id === "s.space.home@1/*#editor");
      const generatedHomeActions = new Map(homeEditor?.windowKinds.flatMap((window) => window.actions).map((action) => [action.id, action]) ?? []);
      for (const actionId of ["bindSpaceFile", "importSpace", "deleteVirtualFileSystemNode", "renameSpace"]) {
        if ((generatedHomeActions.get(actionId) as { semantics?: { execution?: { interactiveJob?: string } } } | undefined)?.semantics?.execution?.interactiveJob !== "migrated") throw new Error(`generated Home action ${actionId} is not published as retained (migrated) execution`);
      }
      if (generatedHomeActions.has("foldDirectoryEvents")) throw new Error("generated Home still declares the retired foldDirectoryEvents writer");
      runCmd(
        "bun",
        [
          join(this.repoRoot, "node_modules/typescript/bin/tsc"),
          "--noEmit",
          "--strict",
          "--target",
          "ESNext",
          "--module",
          "ESNext",
          "--moduleResolution",
          "bundler",
          "--resolveJsonModule",
          "--allowImportingTsExtensions",
          "--esModuleInterop",
          "--skipLibCheck",
          oracle,
          directoryOracle,
        ],
        { cwd: this.repoRoot },
      );
      runCmd("bun", [join(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/📜️script.ts"), "directory-home-bootstrap-check"], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-plugin", "--lib", "view_context_capacity_tests", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-framework-os-renderer-wgpu", "--lib", "host_panel_", "--", "--nocapture"], this.repoRoot);
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-space-home", "--features", "component-app-assembly", "--lib", "--", "--nocapture"], this.repoRoot);
      }
      return;
    }
    throw new Error('Unknown owned verification '+segments.join(' '));
  }
}

await runArtifactRustPackageMain(import.meta.dir, "semio-s-artifact-space-home", { commands: { verify: OwnedVerifyScript }, snapshotSqliteTests:["../../🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts"] });
