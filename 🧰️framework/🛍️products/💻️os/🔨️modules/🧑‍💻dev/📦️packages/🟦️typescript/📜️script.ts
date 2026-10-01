#!/usr/bin/env bun
/** 🧭️ Routes OS development commands to their semantic owners. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

/** 🔬️ Checks neutral development process and local-session contracts against independent oracles. */
class CanonicalArchitectureScript extends BundleScript {
  async run(): Promise<void> {
    const { proveDevLocalHubProviderContract } = await import("../../🚀️local-hub/🧪️tests/🔬️contract/🟦️.ts");
    const { nextestArtifactLocation } = await import("../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
    const { TestScript } = await import("../../🧪️tests/🏃️execution/🟦️.ts");
    const artifacts = nextestArtifactLocation(this.repoRoot).directory;
    await proveDevLocalHubProviderContract(this.repoRoot, artifacts);
    await new TestScript(this.root).run(["dev-contribution"]);
  }
}

class DistributionOutputCheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("distribution-output-check accepts no arguments");
    const { nextestArtifactLocation } = await import("../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
    const { testDistributionOutputContract } = await import("../../🚚️distribution/📍️output/🧪️tests/🔬️unit/🟦️.ts");
    await testDistributionOutputContract(this.repoRoot, nextestArtifactLocation(this.repoRoot).directory);
  }
}

class SourceWatchCheckScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw Error("source-watch-check accepts no arguments");
    const { nextestArtifactLocation } = await import("../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts");
    const { testNativeSourceWatchPlanV1 } = await import("../../../🔌️plugin/🏗️build/👁️watch/🧪️tests/🔬️source-plan/🟦️.ts");
    await testNativeSourceWatchPlanV1(this.repoRoot, nextestArtifactLocation(this.repoRoot).directory);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("source-watch-check", SourceWatchCheckScript)
  .register("distribution-output-check", DistributionOutputCheckScript)
  .registerLazy("prepare", async () => (await import("../../♻️activation/🧰️preparation/🟦️.ts")).PreparationScript)
  .registerLazy("activate", async () => (await import("../../♻️activation/🏃️execution/🟦️.ts")).ActivationScript)
  .registerLazy("serve", async () => (await import("../../♻️activation/🌐️serve/🟦️.ts")).ServeScript)
  .register("serve-hold", class extends BundleScript {
    /** 🛎️ `serve-hold --serve <url> [--hub <url>] [--variant <v>]`: holds the shared serve fixture (`ensureDevServe`) until
     * SIGINT/SIGTERM, then stops only what it started — the zero-touch serve provider of the repository goal gate. */
    async run(segments: string[]): Promise<void> {
      const { devServePortV1, ensureDevServe } = await import("../../🚀️local-hub/🏃️execution/🟦️.ts");
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
  .registerLazy("cold-boot-check", async () => (await import("../../♻️activation/🩺️readiness/🟦️.ts")).ColdBootCheckScript)
  .registerLazy("canonical-bootstrap-folder-mirror-check", async () => (await import("../../🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts")).CanonicalBootstrapFolderMirrorCheckScript)
  .register("closed-browser-component-factory-check", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments.length) throw new Error("closed-browser-component-factory-check accepts no arguments");
      const { testClosedBrowserComponentFactory } = await import("../../../🔌️plugin/🌐️browser-bundle/📜️script.ts");
      await testClosedBrowserComponentFactory(this.repoRoot);
    }
  })
  .registerLazy("test", async () => (await import("../../🧪️tests/🏃️execution/🟦️.ts")).TestScript)
  .registerLazy("verify", async () => (await import("../../🧪️tests/✅️verification/🟦️.ts")).VerifyScript)
  .register("bench", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments[0] === "plugins") return new (await import("../../📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts")).BenchPluginsScript(this.root).run(segments.slice(1));
      throw new Error(`unknown bench subcommand: ${segments[0] ?? "<none>"} (expected plugins)`);
    }
  })
  .register("generate", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments[0] === "playground-session") {
        const { PlaygroundSessionPreviewScript, PlaygroundSessionGenerateScript } = await import("../../🎮️playground-session/🏃️execution/🟦️.ts");
        return segments[1] === "preview" ? new PlaygroundSessionPreviewScript(this.root).run() : new PlaygroundSessionGenerateScript(this.root).run(segments.slice(1));
      }
      if (segments[0] === "scale-fixture") return new (await import("../../../../🧫️fixtures/⚖️scale/📤️publication/🟦️.ts")).ScaleFixtureGenerateScript(this.root, this.repoRoot).run(segments.slice(1));
      throw new Error(`unknown generate subcommand: ${segments[0]} (expected playground-session|scale-fixture)`);
    }
  })
  .register("scale-fixture", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments[0] === "check") return new (await import("../../../../🧫️fixtures/⚖️scale/📤️publication/🟦️.ts")).ScaleFixtureCheckScript(this.root, this.repoRoot).run();
      throw new Error(`unknown scale-fixture subcommand: ${segments[0]} (expected check)`);
    }
  })
  .registerLazy("preview-generated", async () => (await import("../../../../🧫️fixtures/⚖️scale/📤️publication/🟦️.ts")).ScaleFixturePreviewGeneratedScript)
  .register("distribution", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      const { DistributionBundleScript } = await import("../../🚚️distribution/🏃️execution/🟦️.ts");
      return new DistributionBundleScript(this.root).run(segments);
    }
  })
  .register("canonical-architecture", CanonicalArchitectureScript)
  .registerLazy("index-lint", async () => (await import("../../🧪️tests/🧹️export-path-policy/🟦️.ts")).PluginIndexExportPathLintScript)
  .registerLazy("host-handle-lint", async () => (await import("../../🧪️tests/🧹️host-handle-policy/🟦️.ts")).HostHandleReachLintScript)
  .registerLazy("channel-version", async () => (await import("../../🔖️channel-version/🟦️.ts")).ChannelVersionScript)
  .register("parity", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments[0] === "journey") {
        const { runShellInteractionJourney } = await import("../../⚖️parity/🚶️journey/🟦️.ts");
        return runShellInteractionJourney(segments.slice(1));
      }
      const { ParitySmokeScript, ParityTriageScript, ParityProbeScript, ParityVerifyScript, ParitySweepScript } = await import("../../⚖️parity/🏃️execution/🟦️.ts");
      const routes = { smoke: ParitySmokeScript, triage: ParityTriageScript, probe: ParityProbeScript, verify: ParityVerifyScript, sweep: ParitySweepScript } as const;
      const Route = routes[segments[0] as keyof typeof routes];
      if (!Route) throw new Error(`unknown parity subcommand: ${segments[0]} (expected smoke|triage|probe|verify|sweep|journey)`);
      return new Route(this.root).run(segments.slice(1));
    }
  })
  .register("plugin", class extends BundleScript {
    async run(segments: string[]): Promise<void> {
      if (segments[0] === "watch") return new (await import("../../../🔌️plugin/🏗️build/👁️watch/🟦️.ts")).PluginWatchScript(this.root).run(segments.slice(1));
      if (segments[0] === "lint") return new (await import("../../🧪️tests/🧹️capability-policy/🟦️.ts")).PluginCapabilityLintScript(this.root).run();
      if (segments[0] === "registry") return (await import("../../../🔌️plugin/📇️registry/🔄️refresh/🟦️.ts")).ensurePluginRegistry(segments[1] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND);
      if (segments[0] === "size") return new (await import("../../../🔌️plugin/📊️size/🟦️.ts")).PluginSizeScript(this.root).run(segments.slice(1));
      return new (await import("../../../🔌️plugin/🏗️build/🏃️execution/🟦️.ts")).PluginBuildScript(this.root).run(segments);
    }
  });

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
