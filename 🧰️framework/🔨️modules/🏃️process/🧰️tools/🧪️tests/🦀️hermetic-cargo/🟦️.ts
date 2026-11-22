import { afterAll, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { mkdirSync, mkdtempSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { homedir, tmpdir } from "node:os";
import { join } from "node:path";
import { TOOL_REGISTRY_V1, toolForCommandV1 } from "../../📖️registry/🟦️.ts";
import { toolEnvironmentV1 } from "../../🌿️environment/🟦️.ts";

/** 🦀️ Cargo runs on the configuration of the repository and nothing else: a user's configuration in a parent directory (the repository lives below the home directory), ambient compiler flags and `CARGO_*` overrides never reach it. */
const scratch: string[] = [];
afterAll(() => { for (const path of scratch) rmSync(path, { recursive: true, force: true }); });
const cargo = toolForCommandV1("cargo", TOOL_REGISTRY_V1)!;

function hostileHome(): { home: string; project: string; cargoHome: string } {
  const home = realpathSync(mkdtempSync(join(tmpdir(), "hermetic-home-"))); scratch.push(home);
  mkdirSync(join(home, ".cargo"), { recursive: true });
  writeFileSync(join(home, ".cargo", "config.toml"), '[build]\nrustflags = ["--cfg", "hostile_user_config"]\n[env]\nHOSTILE_USER_ENV = "1"\n[alias]\nb = "doc"\n');
  const project = join(home, "Documents", "repo"), cargoHome = join(home, "repo-cargo-home");
  mkdirSync(join(project, ".cargo"), { recursive: true }); mkdirSync(join(project, "src"), { recursive: true }); mkdirSync(cargoHome, { recursive: true });
  writeFileSync(join(project, ".cargo", "config.toml"), '[build]\nrustflags = ["--cfg", "repository_flag"]\n');
  writeFileSync(join(project, "Cargo.toml"), '[package]\nname = "probe"\nversion = "0.1.0"\nedition = "2021"\n');
  writeFileSync(join(project, "src", "main.rs"), "fn main(){}\n");
  return { home, project, cargoHome };
}
const compile = (project: string, home: string, environment: Record<string, string>) => spawnSync("cargo", ["build", "-v", "--target-dir", join(home, "target")], { cwd: project, env: { PATH: process.env.PATH!, HOME: home, RUSTUP_HOME: process.env.RUSTUP_HOME ?? join(homedir(), ".rustup"), RUSTUP_TOOLCHAIN: "nightly-2026-07-20", ...environment }, encoding: "utf8" });

test("the cargo environment of the registry drops ambient compiler flags and target overrides, keeps what the launcher sets, and stops the configuration search at the repository", () => {
  const ambient = { PATH: "/bin", RUSTFLAGS: "--cfg hostile_env", CARGO_ENCODED_RUSTFLAGS: "--cfg\u001fhostile_env", CARGO_BUILD_RUSTFLAGS: "--cfg hostile_env", CARGO_BUILD_TARGET: "wasm32-unknown-unknown", CARGO_TARGET_X86_64_APPLE_DARWIN_RUSTFLAGS: "--cfg hostile_env", CARGO_PROFILE_DEV_OPT_LEVEL: "3", CARGO_ALIAS_B: "doc", CARGO_HOME: "/repo/home", CARGO_TARGET_DIR: "/repo/target", CARGO_NET_OFFLINE: "true", RUSTC_WRAPPER: "/repo/wrapper", SEMIO_CARGO_X: "1" };
  const environment = toolEnvironmentV1(cargo, ambient, {}, { root: "/repo" });
  for (const name of ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_BUILD_RUSTFLAGS", "CARGO_BUILD_TARGET", "CARGO_TARGET_X86_64_APPLE_DARWIN_RUSTFLAGS", "CARGO_PROFILE_DEV_OPT_LEVEL", "CARGO_ALIAS_B"]) expect(environment[name], name).toBeUndefined();
  expect(environment).toMatchObject({ CARGO_HOME: "/repo/home", CARGO_TARGET_DIR: "/repo/target", CARGO_NET_OFFLINE: "true", RUSTC_WRAPPER: "/repo/wrapper", SEMIO_CARGO_X: "1", __CARGO_TEST_ROOT: "/repo" });
  expect(toolEnvironmentV1(cargo, ambient, { __CARGO_TEST_ROOT: undefined }, { root: "/repo" }).__CARGO_TEST_ROOT, "the caller's explicit value wins").toBeUndefined();
});

test("a hostile user configuration in the home directory above the repository, a hostile RUSTFLAGS and a hostile CARGO_BUILD_RUSTFLAGS change nothing: Cargo compiles with the flags of the repository only", () => {
  const { home, project, cargoHome } = hostileHome();
  const hostile = { RUSTFLAGS: "--cfg hostile_env", CARGO_BUILD_RUSTFLAGS: "--cfg hostile_env2", CARGO_HOME: cargoHome };
  const open = compile(project, home, hostile);
  expect(open.status, open.stderr).toBe(0);
  expect(open.stderr, "without the registry the hostile inputs do reach the compiler").toMatch(/hostile/u);
  rmSync(join(home, "target"), { recursive: true, force: true });
  const environment = toolEnvironmentV1(cargo, { ...hostile, PATH: process.env.PATH! }, {}, { root: project });
  const closed = compile(project, home, environment as Record<string, string>);
  expect(closed.status, closed.stderr).toBe(0);
  expect(closed.stderr).toContain("repository_flag");
  expect(closed.stderr).not.toMatch(/hostile/u);
});
