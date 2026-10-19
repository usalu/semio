import type { ScriptInvocation } from "@semio-tech/process/routing";
import { runCmd, runCargo } from "@semio-tech/process/execution";
import { runVitest } from "@semio-tech/repo-lib/testing";
import { policyReadModuleTreeSource, policyReadFileSafe } from "@semio-tech/repo-lib/inventory";
import { join } from "node:path";
import { toolJobFemLiveVisualPublicationExact, toolJobFemNumericalMicrocursorExact } from "@semio-tech/repo-lib/laws";
import { toolJobFemNumericalMicrocursorSelfTests } from "../../../../🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️testing/🔬️tool-job-fem-numerical-microcursor/🟦️.ts";
import { toolJobFemLiveVisualPublicationSelfTests } from "../../../../🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️testing/🔬️tool-job-fem-live-visual-publication/🟦️.ts";

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const args = segments.slice(1);
if (segments[0] === "fem-live-visual-publication") {
    const fem2dModel = policyReadModuleTreeSource(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs");
    const fem2dSession = policyReadModuleTreeSource(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs");
    const fem3dSession = policyReadFileSafe(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs");
    const fem3dEditor = policyReadFileSafe(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs");
    const fem3dModel = policyReadFileSafe(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs");
    const fem3dResults = policyReadFileSafe(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs");
    const fem3dViewer = policyReadFileSafe(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🦀️.rs");
    const fem3dViewerModel = policyReadFileSafe(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs");
    const femPluginRoot = policyReadFileSafe(context.repoRoot, "🌎️hub/🧩️compositions/🏗️fem/🦀️.rs");
    const femGlue = policyReadFileSafe(context.repoRoot, "🌎️hub/🧩️compositions/🏗️fem/📦️packages/🦀️rust/🦀️.rs");
    const femSparse = policyReadModuleTreeSource(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs");
    const frameworkPlugin = policyReadFileSafe(context.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
    const frameworkWorld = ["🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🦀️.rs"].map(root => policyReadModuleTreeSource(context.repoRoot, root)).join("\n");
    const worldSnapshot = policyReadFileSafe(context.repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🌍️world3d-snapshot/🦀️.rs");
    const canvasSnapshot = policyReadFileSafe(context.repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🖼️canvas2d-snapshot/🦀️.rs");
    const canvasRenderer = policyReadModuleTreeSource(context.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs");
    const femAnalyses = policyReadFileSafe(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs");
    const femMesh = policyReadModuleTreeSource(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs");
    const mutations = args.includes("--self-test")
      ? toolJobFemLiveVisualPublicationSelfTests(toolJobFemLiveVisualPublicationExact, fem2dModel, fem2dSession, fem3dSession, fem3dEditor, fem3dModel, fem3dResults, fem3dViewer, fem3dViewerModel, femPluginRoot, femGlue, femSparse, frameworkPlugin, frameworkWorld, worldSnapshot, canvasSnapshot, canvasRenderer, femAnalyses, femMesh)
      : 0;
    if (!toolJobFemLiveVisualPublicationExact(fem2dModel, fem2dSession, fem3dSession, fem3dEditor, fem3dModel, fem3dResults, fem3dViewer, fem3dViewerModel, femPluginRoot, femGlue, femSparse, frameworkPlugin, frameworkWorld, worldSnapshot, canvasSnapshot, canvasRenderer, femAnalyses, femMesh))
      throw new Error("[verify interactivity tool-jobs p6i] mounted FEM live visual publication contract failed.");
    console.log("[verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=" + mutations + ".");
    return;
  }
if (segments[0] === "fem-numerical-microcursor") {
    const session = policyReadModuleTreeSource(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs");
    const sparse = policyReadModuleTreeSource(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs");
    const mesh = policyReadModuleTreeSource(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs");
    const analyses = policyReadFileSafe(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs");
    const model = policyReadFileSafe(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️.rs");
    const elements = policyReadModuleTreeSource(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs");
    const runtime = policyReadFileSafe(context.repoRoot, "🧰️framework/🔨️modules/🧵️job/🦀️.rs");
    const mutations = args.includes("--self-test") ? toolJobFemNumericalMicrocursorSelfTests(toolJobFemNumericalMicrocursorExact, sparse, mesh, analyses, model, elements, session, runtime) : 0;
    if (!toolJobFemNumericalMicrocursorExact(sparse, mesh, analyses, model, elements, session, runtime)) throw new Error("[verify interactivity tool-jobs p6h] live FEM numerical microcursor contract failed.");
    console.log(`[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=${mutations}.`);
    return;
  }
if (segments[0] === "fem-pcg-publication-owners") {
    if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("fem-pcg-publication-owners accepts only optional native");
    const testPath = join(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/⛽️publication-grant/🟦️.ts");
    const { testFemPcgPublicationGrantOracle } = await import(testPath);
    testFemPcgPublicationGrantOracle();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/repo-lib/cargo");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "--", "--nocapture", "--test-threads=1", "pcg_job_", "pcg_construction_", "solver_jobs_reject_stale_and_cancelled_"], context.repoRoot);
    }
    return;
  }
if (segments[0] === "fem-assembly-physical-owners") {
    if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("fem-assembly-physical-owners accepts only optional native");
    const testPath = join(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/📦️physical-owners/🟦️.ts");
    const { testFemAssemblyPhysicalOwners } = await import(testPath);
    testFemAssemblyPhysicalOwners();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/repo-lib/cargo");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "assembly_triplet_pages_", "--", "--nocapture", "--test-threads=1"], context.repoRoot);
    }
    return;
  }
if (segments[0] === "fem-numerical-page-owners") {
    if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("fem-numerical-page-owners accepts only optional native");
    const testPath = join(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📦️numerical-pages/🟦️.ts");
    const { testNumericalPageOwners } = await import(testPath);
    testNumericalPageOwners();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/repo-lib/cargo");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "--", "--nocapture", "--test-threads=1", "numerical_page_", "ldlt_job_checkpoint_resume_", "p6h_ldlt_", "subspace_job_resume_", "p6h_subspace_"], context.repoRoot);
    }
    return;
  }
if (segments[0] === "fem-mesh-preparation-owners") {
    if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("fem-mesh-preparation-owners accepts only optional native");
    const testPath = join(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🧪️tests/📦️preparation-owners/🟦️.ts");
    const { testMeshPreparationOwners } = await import(testPath);
    testMeshPreparationOwners();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/repo-lib/cargo");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "mesh_preparation_", "--", "--nocapture", "--test-threads=1"], context.repoRoot);
    }
    return;
  }
if (segments[0] === "fem-scalar-owners-native") {
    if (segments.length !== 1) throw new Error("fem-scalar-owners-native accepts no arguments");
    const testPath = join(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🔢️scalar-owners/🟦️.ts");
    const { testFemScalarOwnerOracle } = await import(testPath);
    testFemScalarOwnerOracle();
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: context.repoRoot });
    const { runCargo } = await import("@semio-tech/repo-lib/cargo");
    await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "--", "--nocapture", "--test-threads=1", "pcg_job_initial_precondition_", "subspace_factor_cursor_", "subspace_publication_"], context.repoRoot);
    return;
  }
if (segments[0] === "fem2d-window-config-contract" || segments[0] === "fem3d-window-config-contract") {
    const dimension = segments[0].startsWith("fem2d") ? "2d" : "3d";
    const testRoot = join(context.repoRoot, "✏️s/🧑‍💻dev/🏗️fem/🧪️tests/🪟️window-config-contract");
    const mountedStiffnessOracle = join(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🧱️mounted-stiffness/🟦️.ts");
    const numericalOwnerOracle = join(context.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📦️numerical-pages/🟦️.ts");
    if (dimension === "3d") {
      const { runVitest } = await import("@semio-tech/repo-lib/testing");
      await runVitest(join(context.repoRoot, "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript"), ["-t", "story window ownership|story document replacement|story interaction", "--silent=false"], "../../🧪️tests/🎚️config/🟦️.ts");
    }
    const contract = await import(`${testRoot}/🟦️.ts`);
    if (dimension === "2d") contract.testFem2dWindowConfigContract();
    else {
      contract.testFem2dWindowConfigContract();
      contract.testFem3dWindowConfigContract();
      const { testFem3dMountedStiffnessOracle } = await import(mountedStiffnessOracle);
      testFem3dMountedStiffnessOracle();
      const { testNumericalPageOwners } = await import(numericalOwnerOracle);
      testNumericalPageOwners();
    }
    runCmd("bun", [join(context.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${testRoot}/🟦️.ts`, ...(dimension === "3d" ? [mountedStiffnessOracle, numericalOwnerOracle, join(context.repoRoot, "✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🪟️viewport/🟦️.ts"), join(context.repoRoot, "✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🕹️interaction/🟦️.ts")] : [])], { cwd: context.repoRoot });
    if (segments[1] === "native") {
      const { runCargo } = await import("@semio-tech/repo-lib/cargo");
      if (dimension === "2d") {
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "fem2d_window_config_", "--", "--nocapture"], context.repoRoot);
      } else {
        const failures: unknown[] = [];
        for (const [packageName, filters] of [
          ["semio-s-artifact-fem-2d", ["fem2d_window_config_", "mesh_edge_authority_", "mounted_3d_element_interfaces_", "assembly_triplet_pages_", "pcg_job_", "subspace_", "numerical_page_", "ldlt_job_checkpoint_resume_", "p6h_ldlt_", "p6h_subspace_"]],
          ["semio-s-artifact-fem-3d", ["fem3d_window_config_", "live_visual::tests::"]],
        ] as const) {
          try {
            await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", "--", "--nocapture", "--test-threads=1", ...filters], context.repoRoot);
          } catch (error) {
            failures.push(error);
          }
        }
        if (failures.length > 0) throw new AggregateError(failures, "FEM native package validations failed");
      }
    }
    return;
  }
  throw new Error('Unknown owned verification '+segments.join(' '));
}
