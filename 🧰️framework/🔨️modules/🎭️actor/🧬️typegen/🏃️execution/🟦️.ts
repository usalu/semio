import { existsSync, mkdtempSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, join } from "node:path";
import { buildBudgetMs, BundleScript } from "../../../../🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { actorTypegenPlan, actorTypegenTarget } from "../📋️plan/🟦️.ts";
import { publishActorTypegen } from "../📤️publication/🟦️.ts";

const TYPEGEN_TEST_FILTER = "exports_typescript_bindings";
export const ACTOR_TYPEGEN_PREVIEW_DEADLINE_MS = 600_000;

export interface ActorTypegenProcess {
  readonly exited: Promise<number>;
  kill(): void;
}

export interface ActorTypegenWaitOptions {
  readonly deadlineMs: number;
  readonly isCancelled?: () => boolean;
  readonly now?: () => number;
  readonly pollMs?: number;
}

/** ⏱️ Waits for native export while enforcing cooperative cancellation and an optional deadline. */
export async function waitForActorTypegenProcess(child: ActorTypegenProcess, options: ActorTypegenWaitOptions): Promise<number> {
  const now = options.now ?? Date.now;
  const started = now();
  for (;;) {
    const outcome = await Promise.race([child.exited.then((code) => ({ code })), Bun.sleep(options.pollMs ?? 50).then(() => null)]);
    if (outcome) return outcome.code;
    if (options.isCancelled?.()) {
      child.kill();
      throw new Error("actor typegen export cancelled");
    }
    if (options.deadlineMs > 0 && now() - started >= options.deadlineMs) {
      child.kill();
      throw new Error(`actor typegen export exceeded its ${options.deadlineMs}ms deadline`);
    }
  }
}

function cancellationRequested(): boolean {
  const path = process.env.SEMIO_GENERATOR_PREVIEW_CANCEL_FILE;
  return path !== undefined && existsSync(path);
}

/** 🦀️ Executes the authoritative native exporter into an isolated output path. */
export async function exportActorTypegen(packageRoot: string, outputPath: string, targetDir?: string, deadlineMs = buildBudgetMs()): Promise<void> {
  if (cancellationRequested()) throw new Error("actor typegen export cancelled");
  const child = Bun.spawn(["cargo", "test", "--locked", "--features", "typegen", TYPEGEN_TEST_FILTER], {
    cwd: packageRoot,
    env: { ...process.env, ...(targetDir ? { CARGO_TARGET_DIR: targetDir } : {}), SEMIO_TYPEGEN_OUT: outputPath },
    stdout: "inherit",
    stderr: "inherit",
  });
  const code = await waitForActorTypegenProcess(child, { deadlineMs, isCancelled: cancellationRequested });
  if (code !== 0) throw new Error(`framework-actor typegen exporter exited with status ${code}`);
  if (!existsSync(outputPath)) throw new Error(`framework-actor typegen exporter did not write ${outputPath}`);
}

async function stagedActorExport(script: BundleScript, privateCargoTarget: boolean): Promise<Buffer> {
  const temp = mkdtempSync(join(tmpdir(), "semio-actor-typegen-"));
  try {
    const output = join(temp, basename(actorTypegenTarget(script.repoRoot)));
    const deadlineMs = privateCargoTarget ? buildBudgetMs() || ACTOR_TYPEGEN_PREVIEW_DEADLINE_MS : buildBudgetMs();
    await exportActorTypegen(script.root, output, privateCargoTarget ? join(temp, "target") : undefined, deadlineMs);
    return readFileSync(output);
  } finally {
    rmSync(temp, { recursive: true, force: true });
  }
}

export class TypegenScript extends BundleScript {
  async run(): Promise<void> {
    console.log("framework-actor typegen export started");
    publishActorTypegen(actorTypegenTarget(this.repoRoot), await stagedActorExport(this, false), this.repoRoot);
    console.log(`framework-actor typescript mirror refreshed -> ${actorTypegenTarget(this.repoRoot)}`);
  }
}

export class PreviewGeneratedScript extends BundleScript {
  async run(): Promise<void> {
    console.error("framework-actor typegen preview export started");
    process.stdout.write(`${JSON.stringify(actorTypegenPlan(this.repoRoot, await stagedActorExport(this, true)))}\n`);
  }
}
