import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { getWorkspaceRoot } from "../../../../../../../../🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { renderWgpuPackageArtifacts } from "../../📽️projection/🟦️.ts";

const repoRoot = getWorkspaceRoot();

/** 🧵️ Renders the frame worker through the schema-owned package projection. */
export async function renderFrameWorker(bundleRoot: string): Promise<{ path: string; content: string }> {
  const workerJs = join(bundleRoot, "../../🎞️frame-worker/🤖️generated/🟨️.js");
  const artifact = (await renderWgpuPackageArtifacts(repoRoot)).nodes.find((node) => resolve(repoRoot, node.path) === resolve(workerJs));
  if (!artifact) throw new Error("WGPU package projection omitted the frame-worker artifact");
  return { path: workerJs, content: artifact.content };
}

export async function generateFrameWorker(bundleRoot: string): Promise<void> {
  const artifact = await renderFrameWorker(bundleRoot);
  writeFileSync(artifact.path, artifact.content, "utf8");
  checkFrameWorkerCarrierCensus(bundleRoot);
}

/** 🔐️ Rejects shipped worker bytes that retain the deleted bearer/query socket carrier. */
export function checkFrameWorkerCarrierCensus(bundleRoot: string): void {
  const workerJs = join(bundleRoot, "../../🎞️frame-worker/🤖️generated/🟨️.js");
  if (!existsSync(workerJs)) throw new Error("🎞️frame-worker.js is missing");
  const deployed = readFileSync(workerJs, "utf8");
  const normalized = deployed.toLowerCase();
  const forbidden = ["/directory/ws?token=", "?token=", "?access_token=", "access_token", "authorization", "bearer ", "credential", "this.token = token", "set_token(", "hub_token", "s_user", "vite_s_user", "s_hub_url", "sessionstorage", "document.cookie"];
  const residue = forbidden.filter((text) => normalized.includes(text));
  if (residue.length) throw new Error(`🎞️frame-worker.js retains legacy credential carriers: ${residue.join(", ")}`);
  for (const required of ["semioWgpuWorkerBootstrap", "duplicate-boot", "InteractiveWorkerScheduler"]) if (!deployed.includes(required)) throw new Error(`🎞️frame-worker.js lacks production worker marker ${required}`);
}

export async function checkFrameWorker(bundleRoot: string): Promise<void> {
  checkFrameWorkerCarrierCensus(bundleRoot);
  const artifact = await renderFrameWorker(bundleRoot);
  if (!existsSync(artifact.path) || readFileSync(artifact.path, "utf8") !== artifact.content) throw new Error("🎞️frame-worker.js is stale; run the generate-frame-worker target");
}
