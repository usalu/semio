import type { ScriptInvocation } from "@semio-tech/process/routing";
import { join } from "node:path";
import { verifyCapturedCargoArtifactsV1 } from "@semio-tech/process/native-build";
import { scriptCompilerAdmissionControlV1 } from "@semio-tech/framework/process/compiler-capacity";
import { readBuildMode, buildModeEnvironment } from "@semio-tech/process/build-mode";
import { buildAdmittedCargoArtifactsV1 } from "@semio-tech/process/cargo-test";
import { runRepositoryCommand } from "@semio-tech/process/execution";

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

/** 🏃️ Runs the `BuildScript` command under the caller's roots and invocation. */
export async function runBuildScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, [kind,profile,...args]:string[]): Promise<void> {
  if(args.length || (!kind||!["native","mcp"].includes(kind)))throw new Error("Service build requires an exact executable and mode");
  const mode=readBuildMode(profile);
  if(process.env.SEMIO_BUILD_MODE!==undefined&&readBuildMode(process.env.SEMIO_BUILD_MODE)!==mode)throw new Error("Service build mode contradicts its original environment");
  const binary=`semio-s-gis-services-${kind}`;
  await buildAdmittedCargoArtifactsV1(`${context.root}/Cargo.toml`,["-p","semio-s-dev-gis-services","--bin",binary],context.repoRoot,{producer:"artifact",invocation:context.invocation,consume:verifyCapturedCargoArtifactsV1,control: scriptCompilerAdmissionControlV1(context.invocation, "cargo:build", 0), environment:buildModeEnvironment(mode),output:`dist/${kind}-${mode}`,sourcesRecord:`${binary}.sources.json`});
}

/** 🏃️ Runs the `OsMcpLiveAgentLoopScript` command under the caller's roots and invocation. */
export async function runOsMcpLiveAgentLoopScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const env = harnessEnvironment("live-agent-loop-check", segments, ["--serve", "--locale"]);
  await runRepositoryCommand(context.invocation,"bun",[join(context.repoRoot, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts")],context.repoRoot,"os-mcp-live-agent-loop",{version:1,maximumElapsedMilliseconds:900_000,output:{maximumBytes:134217728,maximumLines:65536}},{env});
}

/** 🏃️ Runs the `OsMcpHubAgentParticipantScript` command under the caller's roots and invocation. */
export async function runOsMcpHubAgentParticipantScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const env = harnessEnvironment("hub-agent-participant-check", segments, ["--hub"]);
  await runRepositoryCommand(context.invocation,"bun",[join(context.repoRoot, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️hub-agent-participant/🟦️.ts")],context.repoRoot,"os-mcp-hub-agent-participant",{version:1,maximumElapsedMilliseconds:900_000,output:{maximumBytes:134217728,maximumLines:65536}},{env});
}

/** 🏃️ Runs the `OsMcpPluginCoverageScript` command under the caller's roots and invocation. */
export async function runOsMcpPluginCoverageScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const env = harnessEnvironment("plugin-coverage-check", segments, ["--hub"]);
  await runRepositoryCommand(context.invocation,"bun",[join(context.repoRoot, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🧩️plugin-coverage/🟦️.ts")],context.repoRoot,"os-mcp-plugin-coverage",{version:1,maximumElapsedMilliseconds:7_200_000,output:{maximumBytes:134217728,maximumLines:65536}},{env});
}

/** 🏃️ Runs the `OsMcpUserPathScript` command under the caller's roots and invocation. */
export async function runOsMcpUserPathScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const env = harnessEnvironment("user-path-check", segments, ["--hub", "--serve", "--locale"]);
  await runRepositoryCommand(context.invocation,"bun",[join(context.repoRoot, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🚶️user-path/🟦️.ts")],context.repoRoot,"os-mcp-user-path",{version:1,maximumElapsedMilliseconds:3_600_000,output:{maximumBytes:134217728,maximumLines:65536}},{env});
}

/** 🏃️ Runs the `OsMcpInferenceQuartetScript` command under the caller's roots and invocation. */
export async function runOsMcpInferenceQuartetScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const env = harnessEnvironment("inference-quartet-check", segments, ["--hub"]);
  await runRepositoryCommand(context.invocation,"bun",[join(context.repoRoot, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💼️inference-quartet/🟦️.ts")],context.repoRoot,"os-mcp-inference-quartet",{version:1,maximumElapsedMilliseconds:3_600_000,output:{maximumBytes:134217728,maximumLines:65536}},{env});
}

/** 🏃️ Runs the `OsMcpSecurityScript` command under the caller's roots and invocation. */
export async function runOsMcpSecurityScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const env = harnessEnvironment("security-check", segments, ["--hub", "--hub-admin-capability"]);
  await runRepositoryCommand(context.invocation,"bun",[join(context.repoRoot, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🛡️security/🟦️.ts")],context.repoRoot,"os-mcp-security",{version:1,maximumElapsedMilliseconds:3_600_000,output:{maximumBytes:134217728,maximumLines:65536}},{env});
}

/** 🏃️ Runs the `OsMcpAgentReplyScript` command under the caller's roots and invocation. */
export async function runOsMcpAgentReplyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const env = harnessEnvironment("agent-reply-check", segments, ["--serve", "--locale"]);
  await runRepositoryCommand(context.invocation,"bun",[join(context.repoRoot, "✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/💬️agent-reply/🟦️.ts")],context.repoRoot,"os-mcp-agent-reply",{version:1,maximumElapsedMilliseconds:900_000,output:{maximumBytes:134217728,maximumLines:65536}},{env});
}

/** 🏃️ Runs the `FixtureOwnershipCompositionScript` command under the caller's roots and invocation. */
export async function runFixtureOwnershipCompositionScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>): Promise<void> {
  for (const target of ["@semio-tech/framework-rs:test-fixture-ownership", "@semio-tech/gis-gismap-rs:verify-inference-client-native", "@semio-tech/gis-gismap-rs:verify-inference-mcp"]) await runRepositoryCommand(context.invocation,process.execPath,["repo", "run", target, "--skip-cache"],context.repoRoot,"fixture-owner:" + target,{version:1,maximumElapsedMilliseconds:3_600_000,output:{maximumBytes:134217728,maximumLines:65536}});
}
