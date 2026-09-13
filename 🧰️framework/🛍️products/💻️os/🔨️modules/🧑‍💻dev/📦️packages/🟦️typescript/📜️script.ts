#!/usr/bin/env bun
/** 🧭️ Routes OS development commands to their semantic owners. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { PlaygroundSessionGenerateScript, PlaygroundSessionPreviewScript } from "../../🎮️playground-session/🏃️execution/🟦️.ts";
import { PreparationScript } from "../../♻️activation/🧰️preparation/🟦️.ts";
import { ActivationScript } from "../../♻️activation/🏃️execution/🟦️.ts";
import { ServeScript } from "../../♻️activation/🌐️serve/🟦️.ts";
import { CanonicalBootstrapFolderMirrorCheckScript } from "../../🧪️tests/📇️canonical-bootstrap-folder-mirror/🟦️.ts";
import { TestScript } from "../../🧪️tests/🏃️execution/🟦️.ts";
import { VerifyScript } from "../../🧪️tests/✅️verification/🟦️.ts";
import { BenchPluginsScript } from "../../📊️benchmarks/🔌️plugins/🏃️execution/🟦️.ts";
import { ScaleFixtureGenerateScript, ScaleFixturePreviewGeneratedScript, ScaleFixtureCheckScript } from "../../../../🧫️fixtures/⚖️scale/📤️publication/🟦️.ts";
import { DistributionBundleScript } from "../../🚚️distribution/🏃️execution/🟦️.ts";
import { CapabilityLayeringLintScript } from "../../🧪️tests/🧹️layering-policy/🟦️.ts";
import { PluginIndexExportPathLintScript } from "../../🧪️tests/🧹️export-path-policy/🟦️.ts";
import { HostHandleReachLintScript } from "../../🧪️tests/🧹️host-handle-policy/🟦️.ts";
import { ParitySmokeScript, ParityTriageScript, ParityProbeScript, ParityVerifyScript, ParitySweepScript } from "../../🧪️tests/⚖️parity/🏃️execution/🟦️.ts";
import { PluginWatchScript } from "../../../🔌️plugin/🏗️build/👁️watch/🟦️.ts";
import { PluginCapabilityLintScript } from "../../🧪️tests/🧹️capability-policy/🟦️.ts";
import { PluginSizeScript } from "../../../🔌️plugin/📊️size/🟦️.ts";
import { PluginBuildScript } from "../../../🔌️plugin/🏗️build/🏃️execution/🟦️.ts";
import { ensurePluginRegistry } from "../../../🔌️plugin/📇️registry/🔄️refresh/🟦️.ts";

const router = new ScriptRouter(import.meta.dir)
  .register("prepare", PreparationScript)
  .register("activate", ActivationScript)
  .register("serve", ServeScript)
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
      if (segments[0] === "check") return new ScaleFixtureCheckScript(this.root, this.repoRoot).run(segments.slice(1));
      throw new Error(`unknown scale-fixture subcommand: ${segments[0]} (expected check)`);
    }
  })
  .register("preview-generated", ScaleFixturePreviewGeneratedScript)
  .register("distribution", DistributionBundleScript)
  .register("layer-lint", CapabilityLayeringLintScript)
  .register("index-lint", PluginIndexExportPathLintScript)
  .register("host-handle-lint", HostHandleReachLintScript)
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
      if (segments[0] === "lint") { await new PluginCapabilityLintScript(this.root).run(segments.slice(1)); return new CapabilityLayeringLintScript(this.root).run(); }
      if (segments[0] === "registry") return ensurePluginRegistry(segments[1] || process.env.SEMIO_PLUGIN || process.env.PLAYGROUND_APP_KIND);
      if (segments[0] === "size") return new PluginSizeScript(this.root).run(segments.slice(1));
      return new PluginBuildScript(this.root).run(segments);
    }
  });

if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
