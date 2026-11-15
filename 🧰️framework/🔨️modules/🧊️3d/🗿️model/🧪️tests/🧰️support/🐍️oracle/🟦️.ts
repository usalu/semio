/** 🐍️ Runs the kernel's python oracle (trimesh, shapely) in the hash-locked `geometry` group of the staffage oracle project (`uv.lock` pins it for macOS, Linux and Windows), the same environment the staffage suite builds once per machine in the repo cache; nothing here is imported by product code.
 * @see ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🐍️oracles/pyproject.toml */
import { spawnSync } from "node:child_process";
import { existsSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { homedir, tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const here = dirname(fileURLToPath(import.meta.url));

const repoRoot = (): string => {
  let directory = here;
  while (!existsSync(join(directory, "🎓️teaching")) && dirname(directory) !== directory) directory = dirname(directory);
  return directory;
};

const machineCache = (): string => (process.platform === "darwin" ? join(homedir(), "Library", "Caches", "semio") : process.platform === "win32" ? join(process.env.LOCALAPPDATA ?? join(homedir(), "AppData", "Local"), "semio", "Cache") : join(process.env.XDG_CACHE_HOME ?? join(homedir(), ".cache"), "semio"));

const uvExecutable = (): string => {
  const exe = process.platform === "win32" ? "uv.exe" : "uv";
  const given = process.env.SEMIO_TOOL_UV;
  if (given && existsSync(given)) return given;
  const local = join(homedir(), ".local", "bin", exe);
  return existsSync(local) ? local : exe;
};

/** 🧪️ Writes `input` as JSON to a scratch file, runs `script` (a python file) on it in the locked geometry group and returns the parsed JSON it prints; throws with both streams when `uv` or the oracle fails, so a missing tool is a failing test. */
export function pythonOracle<T>(script: string, input: unknown): T {
  const root = repoRoot();
  const project = join(root, "🎓️teaching", "🏛️architecture", "🧍️staffage", "🧪️tests", "🧰️support", "🐍️oracles");
  const directory = mkdtempSync(join(tmpdir(), "kernel-oracle-"));
  try {
    const file = join(directory, "input.json");
    writeFileSync(file, JSON.stringify(input));
    const result = spawnSync(uvExecutable(), ["run", "--quiet", "--project", project, "--locked", "--only-group", "geometry", "python", "-I", script, file], {
      encoding: "utf8",
      maxBuffer: 1 << 28,
      env: { ...process.env, UV_PYTHON_DOWNLOADS: "never", UV_PROJECT_ENVIRONMENT: join(machineCache(), "staffage-oracles", "geometry") },
    });
    if (result.error) throw new Error(`uv: ${result.error.message}; provision it with \`repo tools setup toolchains uv\``);
    if (result.status !== 0) throw new Error(`oracle ${script} exited ${result.status}\n${result.stdout}\n${result.stderr}`);
    return JSON.parse(result.stdout) as T;
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}
