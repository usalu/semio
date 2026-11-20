import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { type RepositoryToolingStoreV1, withRepositoryToolingOperationV1 } from "../🟦️.ts";
import { pinnedGoEnvironmentV1 } from "../../../../🚀️bootstrap/🛠️tools/🧰️native/🟦️.ts";
import { systemNativeToolPortV1 } from "../../../../🚀️bootstrap/🛠️tools/🧰️native/🔌️port/🟦️.ts";
import { repositoryLeaseDirectoryV1 } from "../../📦️budget/📥️opening/🟦️.ts";
import { repoToolCacheEnv } from "../../../../../🏃️process/🌿️environment/🟦️.ts";
import { checkResourceLeaseControl } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { type CompilerAdmissionControl } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { readBuildMode } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { buildBudgetMs } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { buildGoArtifactsV1 } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🐹️go-build/🟦️.ts";

/** 🐹️ Builds under the issued tooling store, projecting canonical source names when requested. */
export async function runRepositoryToolingGoBuildV1(moduleRoot: string, args: string[], workspace: string, store: RepositoryToolingStoreV1, control: CompilerAdmissionControl, opts: { env?: NodeJS.ProcessEnv; budgetMs?: number; overlay?: () => Readonly<Record<string, string>> } = {}): Promise<void> {
  await withRepositoryToolingOperationV1(workspace, store, control, async actual => {
    const source = opts.env ?? process.env;
    const environment = await pinnedGoEnvironmentV1({
      ...repoToolCacheEnv(workspace, source),
      TMPDIR: store.environment.TMPDIR, TMP: store.environment.TMP, TEMP: store.environment.TEMP,
      GOCACHE: join(store.root, "go", "cache"), GOMODCACHE: join(store.root, "go", "mod"),
    }, store.root, systemNativeToolPortV1(workspace, source.CARGO_NET_OFFLINE === "true"), actual.signal);
    const captureDirectory = resolve(environment.SEMIO_TEST_ARTIFACT_DIR ?? join(moduleRoot, "dist"));
    mkdirSync(captureDirectory, { recursive: true });
    const temporary = opts.overlay ? mkdtempSync(join(captureDirectory, "go-overlay-")) : undefined;
    try {
      const argumentsToBuild = [...args];
      if (temporary && opts.overlay) {
        const overlay = join(temporary, "overlay.json");
        writeFileSync(overlay, JSON.stringify({ Replace: opts.overlay() }));
        argumentsToBuild.unshift(`-overlay=${overlay}`);
      }
      await buildGoArtifactsV1(argumentsToBuild, { version: 1, mode: readBuildMode(source.SEMIO_BUILD_MODE), cwd: resolve(moduleRoot), captureDirectory, leaseDirectory: repositoryLeaseDirectoryV1(workspace), budgetMs: opts.budgetMs ?? buildBudgetMs() }, { environment, control: actual });
      await store.recheck();
      checkResourceLeaseControl(actual);
    } finally {
      if (temporary) rmSync(temporary, { recursive: true, force: true });
    }
  });
}
