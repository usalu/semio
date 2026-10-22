#!/usr/bin/env repo
import{scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1}from"../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runAdmittedExactCargoLawsV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️cargo/🟦️.ts";
import { runVitest } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧪️testing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runRepositoryCommand } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { NativeScript } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import { nativeServiceCompositionLaws } from "../../🧪️tests/🔌️service-composition/🟦️.ts";
import { runBuildScript, runOsMcpLiveAgentLoopScript, runOsMcpHubAgentParticipantScript, runOsMcpPluginCoverageScript, runOsMcpUserPathScript, runOsMcpInferenceQuartetScript, runOsMcpSecurityScript, runOsMcpAgentReplyScript, runFixtureOwnershipCompositionScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runBuildScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}
class SourceScript extends BundleScript { async run():Promise<void> {nativeServiceCompositionLaws(this.repoRoot);} }
class TestScript extends BundleScript {
  async run():Promise<void> {
    nativeServiceCompositionLaws(this.repoRoot);
    const laws=await runAdmittedExactCargoLawsV1({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0), invocation: this.invocation, policy: { ...configuredExactCargoLawPolicyV1() }, cwd:this.repoRoot, env:{...process.env,CARGO_BUILD_JOBS:"1",RUST_MIN_STACK:"33554432"}, nativeEnv:{RUST_MIN_STACK:"268435456"}, groups:[{package:"semio-s-dev-gis-services",target:{kind:"lib"},laws:["tests::native_composition_installs_the_real_owner_and_matches_the_portable_oracle","tests::installed_owner_cannot_execute_without_a_verified_document_lease"]}], progress:event=>console.log(`native-service ${event.stage} ${event.package} ${event.law??""}`) });
    console.log(`native-service-runtime: exact=${laws.length} passed`);
  }
}

class OsMcpLiveAgentLoopScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOsMcpLiveAgentLoopScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class OsMcpHubAgentParticipantScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOsMcpHubAgentParticipantScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class OsMcpPluginCoverageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOsMcpPluginCoverageScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class OsMcpUserPathScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOsMcpUserPathScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class OsMcpInferenceQuartetScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOsMcpInferenceQuartetScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class OsMcpSecurityScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOsMcpSecurityScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

class OsMcpAgentReplyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOsMcpAgentReplyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
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
    await runFixtureOwnershipCompositionScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation });
  }
}

class CanonicalPairCompositionScript extends BundleScript {
  async run(): Promise<void> {
    await runRepositoryCommand(this.invocation,process.execPath,["repo", "run", "@semio-tech/framework-os-mcp-rs:canonical-pair-check", "--skip-cache"],this.repoRoot,"canonical-pair-neutral",{version:1,maximumElapsedMilliseconds:3_600_000,output:{maximumBytes:134217728,maximumLines:65536}});
    await runRepositoryCommand(this.invocation,"cargo",["check", "-p", "semio-hub", "--all-features", "--bin", "os-hub"],this.repoRoot,"canonical-pair-peer",{version:1,maximumElapsedMilliseconds:3_600_000,output:{maximumBytes:134217728,maximumLines:65536}});
  }
}

const router=new ScriptRouter(import.meta.dir).register("source-check",SourceScript).register("native-check",TestScript).register("native",NativeScript).register("build",BuildScript).register("live-agent-loop-check",OsMcpLiveAgentLoopScript).register("hub-agent-participant-check",OsMcpHubAgentParticipantScript).register("plugin-coverage-check",OsMcpPluginCoverageScript).register("user-path-check",OsMcpUserPathScript).register("inference-quartet-check",OsMcpInferenceQuartetScript).register("security-check",OsMcpSecurityScript).register("agent-reply-check",OsMcpAgentReplyScript).register("untrusted-content-check",UntrustedContentScript).register("mcp-composition-source-check",McpCompositionSourceScript).register("fixture-ownership-check",FixtureOwnershipCompositionScript).register("canonical-pair-composition-check",CanonicalPairCompositionScript);
if(import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "source-check" }));
