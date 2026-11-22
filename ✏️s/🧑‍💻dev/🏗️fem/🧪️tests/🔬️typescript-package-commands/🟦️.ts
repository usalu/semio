import type { ScriptInvocation } from "@semio-tech/process/routing";
import { runCmd, runCargo } from "@semio-tech/process/execution";
import { runVitest } from "@semio-tech/repo-lib/testing";
import { join } from "node:path";


type BehaviouralLaw = Readonly<{ manifest: string; filters: readonly string[] }>;
const ENGINE = "✏️s/🔨️modules/🏗️fem/⚙️engine";
const EDITOR = "🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor";
/** 🔥️ The p6h microcursor invariants, each carried by the behavioural tests of the crate that owns it: LDLT and subspace max+1/cancel/deadline/stale/replay/parity, mesh edge-phase interrupts and mounted preparation, element stiffness cells and owned assembly lookup/partition scans. */
const FEM_NUMERICAL_MICROCURSOR_LAWS: readonly BehaviouralLaw[] = [
  { manifest: `${ENGINE}/🔢️sparse/🔺️ldlt/📦️packages/🦀️rust/Cargo.toml`, filters: ["p6h_ldlt_"] },
  { manifest: `${ENGINE}/🔢️sparse/🎵️subspace/📦️packages/🦀️rust/Cargo.toml`, filters: ["p6h_subspace_"] },
  { manifest: `${ENGINE}/🕸️mesh/📦️packages/🦀️rust/Cargo.toml`, filters: ["p6h_"] },
  { manifest: `${ENGINE}/🧮️analyses/🧩️assembly/📦️packages/🦀️rust/Cargo.toml`, filters: ["p6h_"] },
  { manifest: `${ENGINE}/⚖️static/📦️packages/🦀️rust/Cargo.toml`, filters: ["p6h_"] },
];
/** 👁️ The mounted fixed-page visual publication invariants: bounded steps, max+1 refusal before owner transfer, stale/cancel/fault/deadline/device-close preserving the last valid publication, for 2D and 3D. */
const FEM_LIVE_VISUAL_LAWS: readonly BehaviouralLaw[] = [
  { manifest: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/${EDITOR}/🎭️modes/✏️edit/🪟️windows/🧱️model/📦️packages/🦀️rust/Cargo.toml`, filters: ["visual"] },
  { manifest: `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/${EDITOR}/🧵️session/📦️packages/🦀️rust/Cargo.toml`, filters: ["visual"] },
];

/** 🧪️ Runs the filtered behavioural tests of each owning crate through the repository cargo gate. */
async function runBehaviouralCargoLaws(repoRoot: string, laws: readonly BehaviouralLaw[]): Promise<void> {
  for (const law of laws) for (const filter of law.filters) await runCargo(["test", "--manifest-path", join(repoRoot, law.manifest), "--lib", "--", "--test-threads=1", filter], repoRoot);
}

/** 🏃️ Runs the `OwnedVerifyScript` command under the caller's roots and invocation. */
export async function runOwnedVerifyScript(context: Readonly<{ root: string; repoRoot: string; invocation: ScriptInvocation }>, segments: string[]): Promise<void> {
  const args = segments.slice(1);
if (segments[0] === "fem-live-visual-publication") {
    if (segments.length > 1) throw new Error("fem-live-visual-publication accepts no arguments");
    await runBehaviouralCargoLaws(context.repoRoot, FEM_LIVE_VISUAL_LAWS);
    console.log("[verify interactivity tool-jobs p6i] behavioural visual laws passed.");
    return;
  }
if (segments[0] === "fem-numerical-microcursor") {
    if (segments.length > 1) throw new Error("fem-numerical-microcursor accepts no arguments");
    await runBehaviouralCargoLaws(context.repoRoot, FEM_NUMERICAL_MICROCURSOR_LAWS);
    console.log("[verify interactivity tool-jobs p6h] behavioural microcursor laws passed.");
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
