import { BundleScript } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { TEST_LEVEL_BUDGET_MS, resolveTestLevel } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import { runRepositoryTestCommand } from "../../🧪️testing/🎛️repository-command/🟦️.ts";
import { repoTestArtifactEnvironment } from "../../🏃️process/🌿️environment/🧪️test-output/🟦️.ts";
import { runFixtureAndNxRoutes } from "./🧷️fixture-and-nx-routes/🟦️.ts";
import { runRuntimeAndPathRoutes } from "./🛤️runtime-and-path-routes/🟦️.ts";
import { runBuildAndDirectionRoutes } from "./🏗️build-and-direction-routes/🟦️.ts";
import { runSourceTopologyRoutes } from "./🗺️source-topology-routes/🟦️.ts";
import { runOwnershipLawRoutes } from "./⚖️ownership-law-routes/🟦️.ts";
import { runReferenceAuthorityRoutes } from "./🔗️reference-authority-routes/🟦️.ts";
import { runReadmeAndMutationRoutes } from "./📖️readme-and-mutation-routes/🟦️.ts";
import type { TestRouteContext } from "./🧭️route-context/🟦️.ts";


export class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const context: TestRouteContext = { invocation: this.invocation, repoRoot: this.repoRoot, root: this.root, run: (routeSegments) => this.run(routeSegments) };
    for (const route of [runFixtureAndNxRoutes, runRuntimeAndPathRoutes, runBuildAndDirectionRoutes, runSourceTopologyRoutes, runOwnershipLawRoutes, runReferenceAuthorityRoutes, runReadmeAndMutationRoutes]) if (await route(context, segments)) return;
    const { level, rest } = resolveTestLevel(segments);
    const lawTimeout = level === "long" || level === "exhaustive" ? ["--timeout", String(TEST_LEVEL_BUDGET_MS[level])] : [];
    await runRepositoryTestCommand(process.execPath, ["test", "../../🧪️tests/🔬️workspace-contract/🟦️.ts", ...lawTimeout, ...rest], { cwd: this.root, env: repoTestArtifactEnvironment(this.repoRoot, "workspace-contract") });
  }
}
