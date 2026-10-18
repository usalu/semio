#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { runCargo } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts";
import { runVitest } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts";
import { policyReadModuleTreeSource } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, extname, join, relative, resolve } from "node:path";

/** 🧪️ Executes the contracts owned by this component. */
import { policyReadFileSafe } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/📖️source-access/🟦️.ts";
import { toolJobFemLiveVisualPublicationExact } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🏗️fem-visual/🟦️.ts";
import { toolJobFemNumericalMicrocursorExact } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🎯️tool-job-coverage/🔁️live-replay/🧮️fem-numerical-microcursor/🟦️.ts";
import { toolJobFemNumericalMicrocursorSelfTests } from "../../../../🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️testing/🔬️tool-job-fem-numerical-microcursor/🟦️.ts";
import { toolJobFemLiveVisualPublicationSelfTests } from "../../../../🔌️plugins/🏗️fem/📦️packages/🟦️typescript/🧪️testing/🔬️tool-job-fem-live-visual-publication/🟦️.ts";
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const args = segments.slice(1);
if (segments[0] === "fem-live-visual-publication") {
      const fem2dModel = policyReadModuleTreeSource(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs");
      const fem2dSession = policyReadModuleTreeSource(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs");
      const fem3dSession = policyReadFileSafe(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs");
      const fem3dEditor = policyReadFileSafe(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🦀️.rs");
      const fem3dModel = policyReadFileSafe(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🧱️model/🦀️.rs");
      const fem3dResults = policyReadFileSafe(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/🦀️.rs");
      const fem3dViewer = policyReadFileSafe(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🦀️.rs");
      const fem3dViewerModel = policyReadFileSafe(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🧱️model/🦀️.rs");
      const femPluginRoot = policyReadFileSafe(this.repoRoot, "🌎️hub/🧩️compositions/🏗️fem/🦀️.rs");
      const femGlue = policyReadFileSafe(this.repoRoot, "🌎️hub/🧩️compositions/🏗️fem/📦️packages/🦀️rust/🦀️.rs");
      const femSparse = policyReadModuleTreeSource(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs");
      const frameworkPlugin = policyReadFileSafe(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs");
      const frameworkWorld = ["🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🗃️model/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧩️components/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧲️cursors/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧱️scene/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🌉️bridge/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧵️interaction/🦀️.rs", "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎛️host/🦀️.rs"].map(root => policyReadModuleTreeSource(this.repoRoot, root)).join("\n");
      const worldSnapshot = policyReadFileSafe(this.repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🌍️world3d-snapshot/🦀️.rs");
      const canvasSnapshot = policyReadFileSafe(this.repoRoot, "🧰️framework/🔨️modules/🖱️ui/🎬️scene/🖼️canvas2d-snapshot/🦀️.rs");
      const canvasRenderer = policyReadModuleTreeSource(this.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs");
      const femAnalyses = policyReadFileSafe(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs");
      const femMesh = policyReadModuleTreeSource(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs");
      const mutations = args.includes("--self-test")
        ? toolJobFemLiveVisualPublicationSelfTests(fem2dModel, fem2dSession, fem3dSession, fem3dEditor, fem3dModel, fem3dResults, fem3dViewer, fem3dViewerModel, femPluginRoot, femGlue, femSparse, frameworkPlugin, frameworkWorld, worldSnapshot, canvasSnapshot, canvasRenderer, femAnalyses, femMesh)
        : 0;
      if (!toolJobFemLiveVisualPublicationExact(fem2dModel, fem2dSession, fem3dSession, fem3dEditor, fem3dModel, fem3dResults, fem3dViewer, fem3dViewerModel, femPluginRoot, femGlue, femSparse, frameworkPlugin, frameworkWorld, worldSnapshot, canvasSnapshot, canvasRenderer, femAnalyses, femMesh))
        throw new Error("[verify interactivity tool-jobs p6i] mounted FEM live visual publication contract failed.");
      console.log("[verify interactivity tool-jobs p6i] live-source clean; hostile-mutations=" + mutations + ".");
      return;
    }
if (segments[0] === "fem-numerical-microcursor") {
      const session = policyReadModuleTreeSource(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/🦀️.rs");
      const sparse = policyReadModuleTreeSource(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🦀️.rs");
      const mesh = policyReadModuleTreeSource(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs");
      const analyses = policyReadFileSafe(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🦀️.rs");
      const model = policyReadFileSafe(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🏗️model/🦀️.rs");
      const elements = policyReadModuleTreeSource(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/📏️elements2d/🦀️.rs");
      const runtime = policyReadFileSafe(this.repoRoot, "🧰️framework/🔨️modules/🧵️job/🦀️.rs");
      const mutations = args.includes("--self-test") ? toolJobFemNumericalMicrocursorSelfTests(sparse, mesh, analyses, model, elements, session, runtime) : 0;
      if (!toolJobFemNumericalMicrocursorExact(sparse, mesh, analyses, model, elements, session, runtime)) throw new Error("[verify interactivity tool-jobs p6h] live FEM numerical microcursor contract failed.");
      console.log(`[verify interactivity tool-jobs p6h] live-source clean; hostile-mutations=${mutations}.`);
      return;
    }
if (segments[0] === "fem-pcg-publication-owners") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("fem-pcg-publication-owners accepts only optional native");
      const testPath = join(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/⛽️publication-grant/🟦️.ts");
      const { testFemPcgPublicationGrantOracle } = await import(testPath);
      testFemPcgPublicationGrantOracle();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "--", "--nocapture", "--test-threads=1", "pcg_job_", "pcg_construction_", "solver_jobs_reject_stale_and_cancelled_"], this.repoRoot);
      }
      return;
    }
if (segments[0] === "fem-assembly-physical-owners") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("fem-assembly-physical-owners accepts only optional native");
      const testPath = join(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/📦️physical-owners/🟦️.ts");
      const { testFemAssemblyPhysicalOwners } = await import(testPath);
      testFemAssemblyPhysicalOwners();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "assembly_triplet_pages_", "--", "--nocapture", "--test-threads=1"], this.repoRoot);
      }
      return;
    }
if (segments[0] === "fem-numerical-page-owners") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("fem-numerical-page-owners accepts only optional native");
      const testPath = join(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📦️numerical-pages/🟦️.ts");
      const { testNumericalPageOwners } = await import(testPath);
      testNumericalPageOwners();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "--", "--nocapture", "--test-threads=1", "numerical_page_", "ldlt_job_checkpoint_resume_", "p6h_ldlt_", "subspace_job_resume_", "p6h_subspace_"], this.repoRoot);
      }
      return;
    }
if (segments[0] === "fem-mesh-preparation-owners") {
      if (segments.length > 2 || (segments[1] !== undefined && segments[1] !== "native")) throw new Error("fem-mesh-preparation-owners accepts only optional native");
      const testPath = join(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🧪️tests/📦️preparation-owners/🟦️.ts");
      const { testMeshPreparationOwners } = await import(testPath);
      testMeshPreparationOwners();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "mesh_preparation_", "--", "--nocapture", "--test-threads=1"], this.repoRoot);
      }
      return;
    }
if (segments[0] === "fem-scalar-owners-native") {
      if (segments.length !== 1) throw new Error("fem-scalar-owners-native accepts no arguments");
      const testPath = join(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/🔢️scalar-owners/🟦️.ts");
      const { testFemScalarOwnerOracle } = await import(testPath);
      testFemScalarOwnerOracle();
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", testPath], { cwd: this.repoRoot });
      const { runCargo } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts");
      await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "--", "--nocapture", "--test-threads=1", "pcg_job_initial_precondition_", "subspace_factor_cursor_", "subspace_publication_"], this.repoRoot);
      return;
    }
if (segments[0] === "fem2d-window-config-contract" || segments[0] === "fem3d-window-config-contract") {
      const dimension = segments[0].startsWith("fem2d") ? "2d" : "3d";
      const testRoot = join(this.repoRoot, "✏️s/🧑‍💻dev/🏗️fem/🧪️tests/🪟️window-config-contract");
      const mountedStiffnessOracle = join(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧱️elements3d/🧪️tests/🧱️mounted-stiffness/🟦️.ts");
      const numericalOwnerOracle = join(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🔢️sparse/🧪️tests/📦️numerical-pages/🟦️.ts");
      if (dimension === "3d") {
        const { runVitest } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧪️vitest/🟦️.ts");
        await runVitest(join(this.repoRoot, "✏️s/🔌️plugins/🏗️fem/📦️packages/🟦️typescript"), ["-t", "story window ownership|story document replacement"], "../../🧪️tests/🎚️config/🟦️.ts");
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
      runCmd("bun", [join(this.repoRoot, "node_modules/typescript/bin/tsc"), "--noEmit", "--strict", "--target", "ESNext", "--module", "ESNext", "--moduleResolution", "bundler", "--resolveJsonModule", "--allowImportingTsExtensions", "--esModuleInterop", "--skipLibCheck", `${testRoot}/🟦️.ts`, ...(dimension === "3d" ? [mountedStiffnessOracle, numericalOwnerOracle, join(this.repoRoot, "✏️s/🧑‍💻dev/🏗️fem/📖️stories/🧭️coordination/🧪️tests/🪟️viewport/🟦️.ts")] : [])], { cwd: this.repoRoot });
      if (segments[1] === "native") {
        const { runCargo } = await import("../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🏃️run/🟦️.ts");
        if (dimension === "2d") {
          await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", "semio-s-artifact-fem-2d", "--lib", "fem2d_window_config_", "--", "--nocapture"], this.repoRoot);
        } else {
          const failures: unknown[] = [];
          for (const [packageName, filters] of [
            ["semio-s-artifact-fem-2d", ["fem2d_window_config_", "mesh_edge_authority_", "mounted_3d_element_interfaces_", "assembly_triplet_pages_", "pcg_job_", "subspace_", "numerical_page_", "ldlt_job_checkpoint_resume_", "p6h_ldlt_", "p6h_subspace_"]],
            ["semio-s-artifact-fem-3d", ["fem3d_window_config_", "live_visual::tests::"]],
          ] as const) {
            try {
              await runCargo(["test", "--manifest-path", "Cargo.toml", "-p", packageName, "--lib", "--", "--nocapture", "--test-threads=1", ...filters], this.repoRoot);
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
}
const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
