#!/usr/bin/env bun
/** 🧭️ Routes OS development commands to their semantic owners. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { PlaygroundSessionGenerateScript, PlaygroundSessionPreviewScript } from "../../🎮️playground-session/🏃️execution/🟦️.ts";
import { PreparationScript } from "../../♻️activation/🧰️preparation/🟦️.ts";
import { ActivationScript } from "../../♻️activation/🏃️execution/🟦️.ts";
import { ServeScript } from "../../♻️activation/🌐️serve/🟦️.ts";
import { devServePortV1, ensureDevServe } from "../../🚀️local-hub/🏃️execution/🟦️.ts";
import { ColdBootCheckScript } from "../../♻️activation/🩺️readiness/🟦️.ts";
import { CanonicalBootstrapFolderMirrorCheckScript } from "../../🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts";
import { TestScript } from "../../🧪️tests/🏃️execution/🟦️.ts";
import { VerifyScript } from "../../🧪️tests/✅️verification/🟦️.ts";
import { BenchPluginsScript } from "../../📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts";
import { ScaleFixtureGenerateScript, ScaleFixturePreviewGeneratedScript, ScaleFixtureCheckScript } from "../../../../🧫️fixtures/⚖️scale/📤️publication/🟦️.ts";
import { PluginIndexExportPathLintScript } from "../../🧪️tests/🧹️export-path-policy/🟦️.ts";
import { HostHandleReachLintScript } from "../../🧪️tests/🧹️host-handle-policy/🟦️.ts";
import { ChannelVersionScript } from "../../🔖️channel-version/🟦️.ts";
import { ParitySmokeScript, ParityTriageScript, ParityProbeScript, ParityVerifyScript, ParitySweepScript } from "../../⚖️parity/🏃️execution/🟦️.ts";
import { PluginWatchScript } from "../../../🔌️plugin/🏗️build/👁️watch/🟦️.ts";
import { PluginCapabilityLintScript } from "../../🧪️tests/🧹️capability-policy/🟦️.ts";
import { PluginSizeScript } from "../../../🔌️plugin/📊️size/🟦️.ts";
import { PluginBuildScript } from "../../../🔌️plugin/🏗️build/🏃️execution/🟦️.ts";
import { ensurePluginRegistry } from "../../../🔌️plugin/📇️registry/🔄️refresh/🟦️.ts";

/** 🔬️ Checks neutral development process and local-session contracts against independent oracles. */
class CanonicalArchitectureScript extends BundleScript {
  async run(): Promise<void> {
    const { proveDevLocalHubProviderContract } = await import("../../🚀️local-hub/🧪️tests/🔬️contract/🟦️.ts");
    const { nextestArtifactLocation } = await import("../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
    const artifacts = nextestArtifactLocation(this.repoRoot).directory;
    await proveDevLocalHubProviderContract(this.repoRoot, artifacts);
    await new TestScript(this.root).run(["dev-contribution"]);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("prepare", PreparationScript)
  .register("activate", ActivationScript)
  .register("serve", ServeScript)
  .register("serve-hold", class extends BundleScript {
    /** 🛎️ `serve-hold --serve <url> [--hub <url>] [--variant <v>]`: holds the shared serve fixture (`ensureDevServe`) until
     * SIGINT/SIGTERM, then stops only what it started — the zero-touch serve provider of the repository goal gate. */
    async run(segments: string[]): Promise<void> {
      const flag = (name: string): string | undefined => (segments.indexOf(name) >= 0 ? segments[segments.indexOf(name) + 1] : undefined);
      const serveUrl = flag("--serve");
      if (!serveUrl) throw new Error("usage: serve-hold --serve <url> [--hub <url>] [--variant <v>]");
      const cancel = new AbortController();
      const released = new Promise<void>((resolveRelease) => {
        const release = (): void => {
          cancel.abort();
          resolveRelease();
        };
        process.once("SIGINT", release);
        process.once("SIGTERM", release);
      });
      const fixture = await ensureDevServe({ repoRoot: this.repoRoot, port: devServePortV1(serveUrl), hubUrl: flag("--hub"), variant: flag("--variant"), signal: cancel.signal, onProgress: (_status, line) => console.log(line) });
      await released;
      await fixture.stop();
    }
  })
  .register("cold-boot-check", ColdBootCheckScript)
  .register("canonical-bootstrap-folder-mirror-check", CanonicalBootstrapFolderMirrorCheckScript)
  .register("closed-browser-component-factory-check", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments.length) throw new Error("closed-browser-component-factory-check accepts no arguments");
      const { testClosedBrowserComponentFactory } = await import("../../../🔌️plugin/🌐️browser-bundle/📜️script.ts");
      await testClosedBrowserComponentFactory(this.repoRoot);
    }
  })
  .register("test", TestScript)
  .register("verify", VerifyScript)
  .register("bench", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments[0] === "plugins") return new BenchPluginsScript(this.root).run(segments.slice(1));
      throw new Error(`unknown bench subcommand: ${segments[0] ?? "<none>"} (expected plugins)`);
    }
  })
  .register("generate", class extends BundleScript {
    run(segments: string[]): void {
      if (segments[0] === "playground-session") return segments[1] === "preview" ? new PlaygroundSessionPreviewScript(this.root).run() : new PlaygroundSessionGenerateScript(this.root).run(segments.slice(1));
      if (segments[0] === "scale-fixture") return new ScaleFixtureGenerateScript(this.root, this.repoRoot).run(segments.slice(1));
      throw new Error(`unknown generate subcommand: ${segments[0]} (expected playground-session|scale-fixture)`);
    }
  })
  .register("scale-fixture", class extends BundleScript {
    run(segments: string[]): void {
      if (segments[0] === "check") return new ScaleFixtureCheckScript(this.root, this.repoRoot).run();
      throw new Error(`unknown scale-fixture subcommand: ${segments[0]} (expected check)`);
    }
  })
  .register("preview-generated", ScaleFixturePreviewGeneratedScript)
  .register("distribution", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const { DistributionBundleScript } = await import("../../🚚️distribution/🏃️execution/🟦️.ts");
      return new DistributionBundleScript(this.root).run(segments);
    }
  })
  .register("canonical-architecture", CanonicalArchitectureScript)
  .register("index-lint", PluginIndexExportPathLintScript)
  .register("host-handle-lint", HostHandleReachLintScript)
  .register("channel-version", ChannelVersionScript)
  .register("parity", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const routes = { smoke: ParitySmokeScript, triage: ParityTriageScript, probe: ParityProbeScript, verify: ParityVerifyScript, sweep: ParitySweepScript } as const;
      const Route = routes[segments[0] as keyof typeof routes];
      if (!Route) throw new Error(`unknown parity subcommand: ${segments[0]} (expected smoke|triage|probe|verify|sweep)`);
      return new Route(this.root).run(segments.slice(1));
    }
  })
  .register("plugin", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments[0] === "watch") return new PluginWatchScript(this.root).run(segments.slice(1));
      if (segments[0] === "lint") return new PluginCapabilityLintScript(this.root).run();
      if (segments[0] === "registry") return ensurePluginRegistry(segments[1] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND);
      if (segments[0] === "size") return new PluginSizeScript(this.root).run(segments.slice(1));
      return new PluginBuildScript(this.root).run(segments);
    }
  });

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
