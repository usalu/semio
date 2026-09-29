import { createHash } from "node:crypto";
import { copyFileSync, existsSync, mkdirSync, mkdtempSync, renameSync, rmSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import { runCmd } from "../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { prepareTectonic } from "../../🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🔧️toolchain/📜️script.ts";

/** 📌️ Pinned [elan](https://github.com/leanprover/elan/releases/tag/v4.2.4) archives. */
const elan = {
  version: "4.2.4",
  release: "https://github.com/leanprover/elan/releases/download/v4.2.4",
  platforms: [
    { platform: "darwin", architecture: "arm64", archive: "elan-aarch64-apple-darwin.tar.gz", sha256: "7ad829861392c718dfebde3a83b5c8508df47be02af68894b094b0b3952616e5" },
    { platform: "darwin", architecture: "x64", archive: "elan-x86_64-apple-darwin.tar.gz", sha256: "8a340b309d8ed2e96f930761fa223b3af57a38f5d253b53ac90293c9516f8cd4" },
    { platform: "linux", architecture: "arm64", archive: "elan-aarch64-unknown-linux-gnu.tar.gz", sha256: "05febd124d84ebf994b2e7479922a5650b1e950c17ae3bd1ddd776b65bb72bf9" },
    { platform: "linux", architecture: "x64", archive: "elan-x86_64-unknown-linux-gnu.tar.gz", sha256: "42b94d4244e8353142c456ec0e4ca6528fd898a6c604d4059f494e706e431f63" },
    { platform: "win32", architecture: "x64", archive: "elan-x86_64-pc-windows-msvc.zip", sha256: "fad2e980a191c15884cc1d80d170ffc5fa84f3774541020145b66d1a644c6111" },
  ],
};
const documents = ["📯️notes", "🏆️proof"];
const theorems = ["re_pow", "re_pow_even", "c_zero", "c_succ", "a_zero", "a_self", "a_succ", "a_sub", "laplaceBeltrami_defect_general", "laplaceBeltrami_defect", "laplaceBeltrami_iff", "eq_c_of_recursion", "laplaceBeltrami_re_pow", "imI_pow", "imJ_pow", "imK_pow", "s_zero", "s_succ", "s_sub", "defect_VI", "defect_VJ", "laplaceBeltrami_imI_iff", "hyperbolic_imJ_iff", "eq_s_of_recursion", "laplaceBeltrami_imI_pow", "hyperbolic_imJ_pow", "pow_eq_of_sq", "z_pow_eq", "clifford_pow", "clifford_pow_components", "defectN_re", "defectN_im", "defectN_last", "hyperbolic_re_iff", "hyperbolic_im_iff", "hyperbolic_last_iff", "hyperbolic_scalar_pow", "hyperbolic_vector_pow", "hyperbolic_last_pow", "a_five", "a_four", "s_six", "Rpoly_examples", "Spoly_examples", "z_cube_example", "evalPoly_z", "polyA_polyB_eq_zero", "circle_of_roots", "root_not_isolated", "real_quadratic_root_iff", "jPoly_root_iff", "jPoly_root_outside_plane", "quat_pow_eq", "evalPoly_quat", "polyA_polyB_eq_zero_J", "sphere_of_roots_J", "real_poly_root_iff", "real_cubic_root_iff", "real_cubic_circle", "jCubic_root_iff", "pow_eq_j", "exists_pow_eq_j", "pow_eq_j_finite"];
const axioms = new Set(["propext", "Classical.choice", "Quot.sound"]);
const executable = (name: string): string => name + (process.platform === "win32" ? ".exe" : "");

/** 🏠️ Workspace-owned `ELAN_HOME`, separate per pinned elan version. */
function elanHome(repoRoot: string): string {
  return join(repoRoot, ".🧬semio/🦑️repo/⚡️cache/tools/elan", elan.version);
}

/** 🌱️ Environment that resolves `lake` and `lean` only through the workspace-owned elan. */
function leanEnvironment(repoRoot: string): NodeJS.ProcessEnv {
  const home = elanHome(repoRoot);
  return { ...process.env, ELAN_HOME: home, PATH: join(home, "bin") + (process.platform === "win32" ? ";" : ":") + process.env.PATH };
}

/** 📥️ Acquires and verifies the pinned elan, then lets it install into the workspace-owned home. */
async function prepareLean(repoRoot: string): Promise<string> {
  const home = elanHome(repoRoot), lake = join(home, "bin", executable("lake"));
  if (existsSync(lake)) return lake;
  const distribution = elan.platforms.find((row) => row.platform === process.platform && row.architecture === process.arch);
  if (!distribution) throw new Error(`Unsupported elan host: ${process.platform}/${process.arch}`);
  mkdirSync(join(home, ".."), { recursive: true });
  const temporary = mkdtempSync(join(home, "..", ".prepare-"));
  try {
    console.log(`[leutwiler] Downloading elan ${elan.version}: ${distribution.archive}`);
    const response = await fetch(`${elan.release}/${distribution.archive}`);
    if (!response.ok) throw new Error(`elan archive download failed: ${response.status}`);
    const bytes = Buffer.from(await response.arrayBuffer());
    if (createHash("sha256").update(bytes).digest("hex") !== distribution.sha256) throw new Error("elan archive checksum does not match the pinned release");
    const archive = join(temporary, distribution.archive), staged = join(temporary, "home");
    writeFileSync(archive, bytes);
    runCmd(process.platform === "win32" ? join(process.env.SystemRoot ?? "C:\\Windows", "System32", "tar.exe") : "tar", ["-xf", distribution.archive], { cwd: temporary });
    runCmd(join(temporary, executable("elan-init")), ["-y", "--no-modify-path", "--default-toolchain", "none"], { env: { ...process.env, ELAN_HOME: staged } });
    renameSync(staged, home);
  } finally {
    rmSync(temporary, { recursive: true, force: true });
  }
  return lake;
}

/** 📥️ Provisions elan, the Mathlib build cache and the pinned Tectonic. */
export async function prepareDependencies(root: string, repoRoot: string): Promise<void> {
  const lake = await prepareLean(repoRoot);
  runCmd(lake, ["exe", "cache", "get"], { cwd: join(root, "🧘️lean"), env: leanEnvironment(repoRoot), budgetMs: 0 });
  await prepareTectonic(repoRoot);
}

/** 🏗️ Checks the Lean proof (`lean`) or typesets one document (`📯️notes`, `🏆️proof`) under an ASCII job name, because TeX cannot carry an emoji `\jobname`. */
export async function build(root: string, repoRoot: string, target = "lean"): Promise<void> {
  if (target === "lean") {
    runCmd(await prepareLean(repoRoot), ["build"], { cwd: join(root, "🧘️lean"), env: leanEnvironment(repoRoot), budgetMs: 0 });
    return;
  }
  if (!documents.includes(target)) throw new Error(`Unknown build target: ${target}`);
  const output = join(root, "dist", target);
  const job = target.replace(/^[^a-z]+/u, "") + ".tex";
  mkdirSync(output, { recursive: true });
  copyFileSync(join(root, target, "📐️.tex"), join(output, job));
  runCmd(await prepareTectonic(repoRoot), [job], { cwd: output });
}

/** ⚖️ Fails when a main theorem depends on anything but the three standard axioms. */
export async function verifyAxioms(root: string, repoRoot: string): Promise<void> {
  const lake = await prepareLean(repoRoot), cwd = join(root, "🧘️lean");
  const probe = join(cwd, ".lake", "axioms.lean");
  writeFileSync(probe, ["import RealPartsOfPowers", ...theorems.map((name) => `#print axioms RealPartsOfPowers.${name}`), ""].join("\n"));
  const result = spawnSync(lake, ["env", "lean", probe], { cwd, env: leanEnvironment(repoRoot), encoding: "utf8" });
  if (result.status !== 0) throw new Error(`Axiom probe failed:\n${result.stdout}${result.stderr}`);
  const reports = result.stdout.split("\n").filter((line) => line.includes("depends on axioms"));
  if (reports.length !== theorems.length) throw new Error(`Axiom probe reported ${reports.length} of ${theorems.length} theorems:\n${result.stdout}`);
  for (const report of reports) {
    const used = report.slice(report.indexOf("[") + 1, report.indexOf("]")).split(",").map((name) => name.trim());
    const foreign = used.filter((name) => !axioms.has(name));
    if (foreign.length > 0) throw new Error(`Non-standard axioms ${foreign.join(", ")}: ${report}`);
    console.log(`[leutwiler] ${report}`);
  }
}
