import { lstatSync, readlinkSync, realpathSync } from "node:fs";
import { basename, join } from "node:path";
import type { ResourceLeaseControl } from "@semio-tech/process/leases";
import { withCmakeInputsV1 } from "../📥️inputs/🟦️.ts";
import { runCmakeOwnedStageV1 } from "../🎬️stage/🟦️.ts";
import { refuseBuildV1 } from "../../📖️texts/🟦️.ts";

/** 🧾️ Reads compiler command-file output without duplicate Windows variable names. */
export function compilerEnvironmentFromOutput(output: string, base: NodeJS.ProcessEnv): NodeJS.ProcessEnv {
  const entries = new Map(Object.entries(base).map(([key, value]) => [key.toLowerCase(), [key, value] as const]));
  for (const line of output.split(/\r?\n/)) {
    const item = /^([A-Za-z_][A-Za-z0-9_()]*)=(.*)$/.exec(line);
    if (item) entries.set(item[1]!.toLowerCase(), [item[1]!, item[2]!]);
  }
  return Object.fromEntries(entries.values());
}

/** 🛠️ Activates standalone MSVC for Ninja beneath retained inputs and the caller's bounded phase. */
export async function windowsCompilerEnvironment(root: string, base: NodeJS.ProcessEnv, control: ResourceLeaseControl, directory: string): Promise<{ environment: NodeJS.ProcessEnv; installation?: string }> {
  control.signal.throwIfAborted();
  if (process.platform !== "win32") return { environment: base };
  const vswhere = realpathSync(join(base["ProgramFiles(x86)"] ?? "C:\\Program Files (x86)", "Microsoft Visual Studio", "Installer", "vswhere.exe"));
  const installation = await withCmakeInputsV1(new Map([["tools/vswhere", vswhere]]), control, (inputs, actual) => runCmakeOwnedStageV1({ kind: "probe", directory, commands: [{ program: vswhere, arguments: ["-latest", "-version", "[18.0,19.0)", "-products", "*", "-requires", process.arch === "arm64" ? "Microsoft.VisualStudio.Component.VC.Tools.ARM64" : "Microsoft.VisualStudio.Component.VC.Tools.x86.x64", "-property", "installationPath"], cwd: root }] }, inputs, actual, base, async rows => realpathSync(rows[0]!.stdout.trim())));
  const file = realpathSync(join(installation, "VC", "Auxiliary", "Build", "vcvarsall.bat")), program = realpathSync(base.ComSpec ?? join(base.SystemRoot ?? "C:\\Windows", "System32", "cmd.exe"));
  if (/[\r\n"%&|<>^!]/.test(file)) throw Error("Invalid MSVC compiler command file");
  const architectures: Readonly<Record<string, string>> = { x64: "amd64", arm64: "amd64_arm64", ia32: "x86" }, architecture = architectures[process.arch];
  if (!architecture) throw Error(`Unsupported MSVC architecture: ${process.arch}`);
  const environment = await withCmakeInputsV1(new Map([["tools/cmd", program], ["source/vcvars", file]]), control, (inputs, actual) => runCmakeOwnedStageV1({ kind: "probe", directory, commands: [{ program, arguments: ["/d", "/s", "/c", `chcp 65001>nul && call "${file}" ${architecture} >nul && set`], cwd: root }] }, inputs, actual, base, async rows => compilerEnvironmentFromOutput(rows[0]!.stdout, base)));
  if (!["include", "lib", "path"].every(name => Object.entries(environment).some(([key, value]) => key.toLowerCase() === name && value))) throw Error("MSVC compiler environment is incomplete");
  delete environment.CMAKE_BUILD_PARALLEL_LEVEL;
  delete environment.CTEST_PARALLEL_LEVEL;
  return { environment, installation };
}

/** 🧰️ A compiler of the host: `command` is what a build invokes, because its file name selects the driver mode; `file` is the single-link regular file whose identity is pinned as a build input. */
export type SystemCompilerV1 = Readonly<{ command: string; file: string }>;

const toolchainBinaryDirectoriesV1: readonly string[] = Object.freeze(["Toolchains/XcodeDefault.xctoolchain/usr/bin", "usr/bin"]);

/** 🍎️ Resolves a located compiler to the file to pin. A single-link file is its own identity. A hard-linked system shim (the macOS `/usr/bin/c++` has 78 links, one per Apple developer tool) is not copied and not pinned: the real toolchain binary behind it is, found exactly where `xcrun --find` looks, in the order `DEVELOPER_DIR`, the `xcode-select` selection, the Command Line Tools; the toolchain name, not the resolved `clang`, stays the command, because `clang` links C++ without its standard library.
 * @see https://keith.github.io/xcode-man-pages/xcrun.1.html */
export function systemCompilerV1(located: string, requested: string, environment: Readonly<Record<string, string | undefined>>, options: Readonly<{ platform?: NodeJS.Platform; selectionLink?: string; fallbackDirectory?: string }> = {}): SystemCompilerV1 {
  const links = lstatSync(located).nlink;
  if (links === 1) return Object.freeze({ command: located, file: located });
  if ((options.platform ?? process.platform) === "darwin") {
    const selected = (() => { try { return readlinkSync(options.selectionLink ?? "/var/db/xcode_select_link"); } catch { return undefined; } })();
    for (const directory of [environment.DEVELOPER_DIR, selected, options.fallbackDirectory ?? "/Library/Developer/CommandLineTools"]) {
      if (!directory) continue;
      for (const relative of toolchainBinaryDirectoriesV1) {
        const command = join(directory, relative, basename(requested));
        try {
          const file = realpathSync(command), row = lstatSync(file);
          if (row.isFile() && row.nlink === 1) return Object.freeze({ command, file });
        } catch {
          continue;
        }
      }
    }
  }
  throw refuseBuildV1("missing", "cmake.compiler-single-link-required", { compiler: located, links });
}
