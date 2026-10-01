#!/usr/bin/env bun
import { runExactCargoLaws, runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";
import { runOwnedCommand } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🎛️owned-execution/🟦️.ts";
import { buildCargoArtifacts } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { NativeScript } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/🟦️.ts";
import { nativeServiceCompositionLaws } from "../../🧪️tests/🔌️service-composition/🟦️.ts";
class BuildScript extends BundleScript {
  async run([kind,profile,...args]:string[]):Promise<void> {
    if(args.length || !["native","mcp"].includes(kind) || !["dev","release"].includes(profile))throw new Error("Service build requires an exact executable and profile");
    const binary=`semio-s-services-${kind}`;
    await buildCargoArtifacts(`${this.root}/Cargo.toml`,["-p","semio-s-dev-services","--bin",binary,"--features",kind==="native"?"native-renderer":"mcp-service",...(profile==="release"?["--release"]:[])],this.repoRoot,{output:`dist/${kind}-${profile}`,sourcesRecord:`${binary}.sources.json`});
  }
}
class SourceScript extends BundleScript { async run():Promise<void> {nativeServiceCompositionLaws(this.repoRoot);} }
class TestScript extends BundleScript {
  async run():Promise<void> {
    nativeServiceCompositionLaws(this.repoRoot);
    const laws=await runExactCargoLaws({cwd:this.repoRoot,env:{...process.env,CARGO_BUILD_JOBS:"1",RUST_MIN_STACK:"33554432"},nativeEnv:{RUST_MIN_STACK:"268435456"},groups:[{package:"semio-s-dev-services",target:{kind:"lib"},laws:["tests::native_composition_installs_the_real_owner_and_matches_the_portable_oracle","tests::installed_owner_cannot_execute_without_a_verified_document_lease"]}],progress:event=>console.log(`native-service ${event.stage} ${event.package} ${event.law??""}`)});
    console.log(`native-service-runtime: exact=${laws.length} passed`);
  }
}
const HARNESS_FLAGS = { "--hub": "OS_MCP_HUB_ORIGIN", "--serve": "S_OS_MCP_LIVE_SHELL_URL", "--locale": "S_OS_MCP_LIVE_LOCALE", "--hub-admin-capability": "OS_HUB_ADMIN_CAPABILITY_FILE" } as const;
type HarnessFlag = keyof typeof HARNESS_FLAGS;

function harnessEnvironment(verb: string, segments: string[], accepted: readonly HarnessFlag[]): NodeJS.ProcessEnv {
  const env: NodeJS.ProcessEnv = { ...process.env };
  for (let index = 0; index < segments.length; index += 2) {
    const flag = segments[index] as HarnessFlag;
    const value = segments[index + 1];
    if (!accepted.includes(flag) || value === undefined || value.startsWith("--")) throw new Error(`${verb} takes ${accepted.map((name) => `${name} <value>`).join(" ") || "no arguments"}; refused ${accepted.includes(flag) ? `${flag} without a value` : `argument ${index + 1}`} (values are never echoed)`);
    if (flag === "--locale" && value !== "en" && value !== "de") throw new Error(`${verb}: --locale is en or de, got ${value}`);
    env[HARNESS_FLAGS[flag]] = value;
  }
  return env;
}

class OsMcpLiveAgentLoopScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const env = harnessEnvironment("live-agent-loop-check", segments, ["--serve", "--locale"]);
    await runOwnedCommand("bun", [join(this.repoRoot, "✏️s/🧑‍💻dev/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts")], this.repoRoot, "os-mcp-live-agent-loop", 900_000, { env });
  }
}

class OsMcpHubAgentParticipantScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const env = harnessEnvironment("hub-agent-participant-check", segments, ["--hub"]);
    await runOwnedCommand("bun", [join(this.repoRoot, "✏️s/🧑‍💻dev/💡️services/🧪️tests/🤖️hub-agent-participant/🟦️.ts")], this.repoRoot, "os-mcp-hub-agent-participant", 900_000, { env });
  }
}

class OsMcpPluginCoverageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const env = harnessEnvironment("plugin-coverage-check", segments, ["--hub"]);
    await runOwnedCommand("bun", [join(this.repoRoot, "✏️s/🧑‍💻dev/💡️services/🧪️tests/🧩️plugin-coverage/🟦️.ts")], this.repoRoot, "os-mcp-plugin-coverage", 7_200_000, { env });
  }
}

class OsMcpUserPathScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const env = harnessEnvironment("user-path-check", segments, ["--hub", "--serve", "--locale"]);
    await runOwnedCommand("bun", [join(this.repoRoot, "✏️s/🧑‍💻dev/💡️services/🧪️tests/🚶️user-path/🟦️.ts")], this.repoRoot, "os-mcp-user-path", 3_600_000, { env });
  }
}

class OsMcpInferenceQuartetScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const env = harnessEnvironment("inference-quartet-check", segments, ["--hub"]);
    await runOwnedCommand("bun", [join(this.repoRoot, "✏️s/🧑‍💻dev/💡️services/🧪️tests/💼️inference-quartet/🟦️.ts")], this.repoRoot, "os-mcp-inference-quartet", 3_600_000, { env });
  }
}

class OsMcpSecurityScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const env = harnessEnvironment("security-check", segments, ["--hub", "--hub-admin-capability"]);
    await runOwnedCommand("bun", [join(this.repoRoot, "✏️s/🧑‍💻dev/💡️services/🧪️tests/🛡️security/🟦️.ts")], this.repoRoot, "os-mcp-security", 3_600_000, { env });
  }
}

class OsMcpHubEditDurabilityScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("hub-edit-durability-check accepts no arguments");
    await runOwnedCommand("bun", [join(this.repoRoot, "✏️s/🧑‍💻dev/💡️services/🧪️tests/🤝️hub-edit-durability/🟦️.ts")], this.repoRoot, "os-mcp-hub-edit-durability", 1_800_000);
  }
}

class OsMcpAgentReplyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const env = harnessEnvironment("agent-reply-check", segments, ["--serve", "--locale"]);
    await runOwnedCommand("bun", [join(this.repoRoot, "✏️s/🧑‍💻dev/💡️services/🧪️tests/💬️agent-reply/🟦️.ts")], this.repoRoot, "os-mcp-agent-reply", 900_000, { env });
  }
}

class UntrustedContentScript extends BundleScript {
  async run(segments: string[]): Promise<void> { await runVitest(this.root, segments, "../../🧪️tests/🎚️config/🟦️.ts"); }
}

class McpCompositionSourceScript extends BundleScript {
  async run(): Promise<void> {
    const { serviceMcpCompositionOwnership } = await import("../../🧪️tests/🌉️mcp-composition/🟦️.ts");
    await serviceMcpCompositionOwnership(this.repoRoot);
  }
}
class FixtureOwnershipCompositionScript extends BundleScript {
  async run(): Promise<void> {
    for (const target of ["@semio-tech/framework-rs:test-fixture-ownership", "@semio-tech/gis-gismap-rs:verify-inference-client-native", "@semio-tech/gis-gismap-rs:verify-inference-mcp"]) await runOwnedCommand(process.execPath, ["nx", "run", target, "--skip-nx-cache"], this.repoRoot, "fixture-owner:" + target, 3_600_000);
  }
}

class CanonicalPairCompositionScript extends BundleScript {
  async run(): Promise<void> {
    await runOwnedCommand(process.execPath, ["nx", "run", "@semio-tech/framework-os-mcp-rs:canonical-pair-check", "--skip-nx-cache"], this.repoRoot, "canonical-pair-neutral", 3_600_000);
    await runOwnedCommand("cargo", ["check", "-p", "semio-hub", "--all-features", "--bin", "os-hub"], this.repoRoot, "canonical-pair-peer", 3_600_000);
  }
}

const router=new ScriptRouter(import.meta.dir).register("source-check",SourceScript).register("native-check",TestScript).register("native",NativeScript).register("build",BuildScript).register("live-agent-loop-check",OsMcpLiveAgentLoopScript).register("hub-agent-participant-check",OsMcpHubAgentParticipantScript).register("plugin-coverage-check",OsMcpPluginCoverageScript).register("user-path-check",OsMcpUserPathScript).register("inference-quartet-check",OsMcpInferenceQuartetScript).register("security-check",OsMcpSecurityScript).register("hub-edit-durability-check",OsMcpHubEditDurabilityScript).register("agent-reply-check",OsMcpAgentReplyScript).register("untrusted-content-check",UntrustedContentScript).register("mcp-composition-source-check",McpCompositionSourceScript).register("fixture-ownership-check",FixtureOwnershipCompositionScript).register("canonical-pair-composition-check",CanonicalPairCompositionScript);
if(import.meta.main) await runScriptMain(router,{defaultCommand:"source-check"});
