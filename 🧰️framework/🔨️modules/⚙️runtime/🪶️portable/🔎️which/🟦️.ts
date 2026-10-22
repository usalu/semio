import { accessSync, constants, statSync } from "node:fs";
import { delimiter, isAbsolute, join, resolve } from "node:path";
import type { RuntimeWhichOptionsV1 } from "../../📋️contract/🟦️.ts";

function executable(path: string): boolean {
  try {
    if (!statSync(path).isFile()) return false;
    if (process.platform !== "win32") accessSync(path, constants.X_OK);
    return true;
  } catch { return false; }
}

/** 🔎️ Finds an executable by name through `PATH` (and `PATHEXT` on Windows) without any runtime API; returns an absolute path or null. */
export function whichV1(command: string, options: RuntimeWhichOptionsV1 = {}): string | null {
  const cwd = options.cwd ?? process.cwd(), environment = options.PATH ?? process.env.PATH ?? "";
  const extensions = process.platform === "win32" ? ["", ...(process.env.PATHEXT ?? ".COM;.EXE;.BAT;.CMD").split(";").filter(Boolean)] : [""];
  const candidates = (base: string): string[] => extensions.map(extension => base + extension);
  if (isAbsolute(command) || command.includes("/") || process.platform === "win32" && command.includes("\\")) {
    for (const candidate of candidates(resolve(cwd, command))) if (executable(candidate)) return candidate;
    return null;
  }
  for (const directory of environment.split(delimiter)) {
    if (directory === "") continue;
    for (const candidate of candidates(join(resolve(cwd, directory), command))) if (executable(candidate)) return candidate;
  }
  return null;
}
