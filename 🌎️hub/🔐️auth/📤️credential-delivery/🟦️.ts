import { spawn, type ChildProcess } from "node:child_process";
import type { Duplex } from "node:stream";
import { writeLocalFrame } from "../../🚀️local-bootstrap/📡️framing/🟦️.ts";

export const DIRECT_CHILD_BENIGN_ENV_KEY = "SEMIO_DIRECT_CHILD_BENIGN";
export const DIRECT_CHILD_BENIGN_ENV_VALUE = "preserved";

export type CredentialChildLaunch = Readonly<{
  executable: string;
  args: readonly string[];
  env: NodeJS.ProcessEnv;
  stdio: readonly ["ignore" | "pipe", "pipe", "pipe", "pipe"];
}>;

export type CredentialChildOperations = Readonly<{
  spawnChild: (launch: CredentialChildLaunch) => ChildProcess;
  terminateChild: (child: ChildProcess) => void;
}>;

const nativeCredentialChildOperations: CredentialChildOperations = {
  spawnChild: ({ executable, args, env, stdio }) => spawn(executable, [...args], { shell: false, env, stdio: [...stdio] }),
  terminateChild: (child) => child.kill(),
};

/** 🧹 Identifies inherited values that may carry authority into a direct child. */
export function isProtectedDirectChildEnvironmentKey(key: string): boolean {
  const normalized = key.toUpperCase();
  return (
    normalized === "S_USER" ||
    normalized === "VITE_S_USER" ||
    normalized === "S_HUB_URL" ||
    normalized.includes("TOKEN") ||
    normalized.includes("SESSION") ||
    normalized.includes("CREDENTIAL") ||
    normalized.includes("BEARER") ||
    normalized.includes("CAPABILITY") ||
    normalized.includes("AUTHORIZATION") ||
    normalized.includes("COOKIE")
  );
}

/** 🧹 Removes inherited credential carriers while retaining ordinary process values. */
export function sealedDirectChildEnvironment(source: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  const environment: NodeJS.ProcessEnv = {};
  for (const [key, value] of Object.entries(source)) if (!isProtectedDirectChildEnvironmentKey(key)) environment[key] = value;
  environment[DIRECT_CHILD_BENIGN_ENV_KEY] = DIRECT_CHILD_BENIGN_ENV_VALUE;
  return environment;
}

/** 📍 Adds the only credential marker admitted by direct consumers. */
export function directChildEnvironment(source: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  const environment = sealedDirectChildEnvironment(source);
  environment.S_LOCAL_CREDENTIAL_FD = "3";
  return environment;
}

/** 🧾 Projects one native or MCP child launch before any process is started. */
export function directChildLaunch(executable: string, args: readonly string[], expectedClass: "native" | "mcp", environmentSource: NodeJS.ProcessEnv): CredentialChildLaunch {
  return {
    executable,
    args: [...args],
    env: directChildEnvironment(environmentSource),
    stdio: [expectedClass === "mcp" ? "pipe" : "ignore", "pipe", "pipe", "pipe"],
  };
}

/** 📤 Starts one direct child, transfers one bounded envelope through fd 3, and erases authority. */
export async function deliverCredentialEnvelopeToChild(
  executable: string,
  args: readonly string[],
  envelope: Record<string, any>,
  expectedClass: "native" | "mcp",
  hubOrigin: string,
  environmentSource: NodeJS.ProcessEnv = process.env,
  operations: CredentialChildOperations = nativeCredentialChildOperations,
): Promise<ChildProcess> {
  let child: ChildProcess | undefined;
  try {
    if (envelope.clientClass !== expectedClass) throw new Error("credential envelope client class mismatch");
    if (!/^http:\/\/127\.0\.0\.1:\d+$/u.test(hubOrigin)) throw new Error("credential hub origin mismatch");
    child = operations.spawnChild(directChildLaunch(executable, args, expectedClass, environmentSource));
    const pipe = child.stdio[3] as Duplex;
    if (!pipe) throw new Error("one-shot credential endpoint was not created");
    await writeLocalFrame(pipe, {
      schema: "semio.local.consumer-credential/v1",
      clientClass: expectedClass,
      hubOrigin,
      sessionId: envelope.sessionId,
      authorizationGeneration: envelope.authorizationGeneration,
      expiresAtMs: envelope.expiresAt,
      capability: envelope.capability,
    });
    pipe.end();
    return child;
  } catch (error) {
    if (child) operations.terminateChild(child);
    throw error;
  } finally {
    envelope.capability = "";
  }
}

/** 🖥️ Delivers one native credential through the shared direct-child authority. */
export async function deliverNativeCredentialEnvelope(executable: string, args: readonly string[], envelope: Record<string, any>, hubOrigin: string): Promise<ChildProcess> {
  return deliverCredentialEnvelopeToChild(executable, args, envelope, "native", hubOrigin);
}

/** 🌉️ Delivers one MCP credential through the shared direct-child authority. */
export async function deliverMcpCredentialEnvelope(executable: string, args: readonly string[], envelope: Record<string, any>, hubOrigin: string): Promise<ChildProcess> {
  return deliverCredentialEnvelopeToChild(executable, args, envelope, "mcp", hubOrigin);
}
