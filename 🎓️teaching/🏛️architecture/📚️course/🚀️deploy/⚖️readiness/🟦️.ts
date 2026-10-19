/** ⚖️ The `deploy-check` verb: the readiness gate over drift, the course, `publish`, the image build, both checks and the end-to-end gate when the package has one.
 * @see ../🟦️.ts — the constants, tools and drift check shared by the operator verbs */

import { courseOriginalBuildModeV1 } from "../../🏗️builder/🎚️mode/🟦️.ts";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import course from "../../🔣️.json" with { type: "json" };
import deployment from "../🔣️.json" with { type: "json" };
import { publishCourseSite } from "../🌐️site/🟦️.ts";
import { buildCourseImage } from "../🐳️docker-image/🟦️.ts";
import { checkCourse, deploymentDrift, deploymentWarnings, flag, interruption, say } from "../🟦️.ts";
import { checkCourseStack } from "../🧱️docker-stack/🟦️.ts";
import { checkCourseImage } from "../🩺️docker-image-check/🟦️.ts";

//#region ⚖️Readiness
const DEPLOY_CHECK_STEPS = ["no drift from 🚀️deploy/🔣️.json", "sources type-checked", "course valid in the Rust core", "site built and verified as the CDN artifact", "proctor image built", "proctor image checked", "stack checked", "end-to-end gate"] as const;
type DeployCheckStep = (typeof DEPLOY_CHECK_STEPS)[number];

/** 🪜️ The steps of `deploy-check` in order for a package with the nx `targets`: the cheap proofs first — no drift, then
 * the sources against the compiler when the package has a `typecheck` target, before anything is built — and the
 * end-to-end gate last, when the package has one. */
export function deployCheckSteps(targets: Readonly<Record<string, unknown>>): readonly DeployCheckStep[] {
  return DEPLOY_CHECK_STEPS.filter((step) => (step !== "sources type-checked" || "typecheck" in targets) && (step !== "end-to-end gate" || "test-e2e" in targets));
}

/** ⚖️ `deploy-check [--tag <tag>]` — everything that must hold before the owner deploys, in the order of
 * {@link deployCheckSteps}, each step announced with its number and time, stopping at the first failure or at Ctrl-C:
 * no drift from `🔣️.json`, the package's sources against the compiler, the course valid in the Rust core, the site
 * built and verified as the CDN artifact, the image built, the image check, the stack check, and the package's
 * end-to-end gate. Nothing is pushed or uploaded. */
export async function checkCourseDeployment(repoRoot: string, bundleRoot: string, checks: Readonly<{ build: () => void | Promise<void>; typecheck: () => void | Promise<void>; endToEnd: () => void | Promise<void> }>, segments: readonly string[], invocation: ScriptInvocation): Promise<void> {
  courseOriginalBuildModeV1(invocation, "release");
  if (segments.includes("--jobs")) throw Error("The native compiler owner controls concurrency; --jobs is not a deployment option");
  const tagged = flag(segments, "--tag") ? ["--tag", flag(segments, "--tag")!] : [];
  const targets = (JSON.parse(readFileSync(join(bundleRoot, "📋️project.json"), "utf8")) as { targets: Record<string, unknown> }).targets;
  const run: { readonly [S in DeployCheckStep]: () => Promise<void> | void } = {
    "no drift from 🚀️deploy/🔣️.json": () => {
      const drift = deploymentDrift(repoRoot);
      if (drift.length > 0) throw new Error(`the deployment drifted:\n- ${drift.join("\n- ")}`);
    },
    "sources type-checked": checks.typecheck,
    "course valid in the Rust core": () => checkCourse(repoRoot, invocation),
    "site built and verified as the CDN artifact": () => publishCourseSite(repoRoot, bundleRoot, invocation, checks.build),
    "proctor image built": () => buildCourseImage(repoRoot, tagged, invocation),
    "proctor image checked": () => checkCourseImage(repoRoot, tagged, invocation),
    "stack checked": () => checkCourseStack(tagged, invocation),
    "end-to-end gate": checks.endToEnd,
  };
  const steps = deployCheckSteps(targets);
  const { signal, release } = interruption(invocation);
  const took: string[] = [];
  try {
    for (const [index, label] of steps.entries()) {
      if (signal.aborted) throw new Error("cancelled");
      say(`step ${index + 1}/${steps.length}: ${label}`);
      const started = Date.now();
      await run[label]();
      took.push(`${label} (${Math.round((Date.now() - started) / 1000)} s)`);
    }
  } finally {
    release();
  }
  for (const warning of deploymentWarnings()) say(`WARNING: ${warning}`);
  say(`ready to deploy: ${took.join("; ")}`);
}
//#endregion ⚖️Readiness
