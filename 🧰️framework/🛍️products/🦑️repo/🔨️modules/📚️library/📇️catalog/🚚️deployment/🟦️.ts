import type { InstallationDirectoryV1 } from "../../../../../../🔨️modules/🪪️identity/📁️installation/🟦️.ts";
import { componentDeploymentDirectoryV1 } from "./🟨️.mjs";

/** 🚚️ Reads exactly one optional deployment declaration from bounded owner-authored Cargo bytes. */
export function declaredComponentDeploymentDirectoryV1(text: string): InstallationDirectoryV1 | undefined {
  if (new TextEncoder().encode(text).length > 64 * 1024) throw new Error("Component manifest exceeds its declaration boundary");
  let active = false, seen = false, value: unknown;
  for (const row of text.split(/\r?\n/u)) {
    const line = row.trim();
    if (line.startsWith("[")) {
      active = line === "[package.metadata.semio]";
      if (active && seen) throw new Error("Repeated component metadata table");
      if (active) seen = true;
      continue;
    }
    if (!active || !/^deployment-directory\s*=/u.test(line)) continue;
    if (value !== undefined) throw new Error("Repeated component deployment declaration");
    const match = /^deployment-directory\s*=\s*("(?:\\.|[^"\\])*"|[^#]+)\s*(?:#.*)?$/u.exec(line);
    if (!match) throw new Error("Invalid component deployment declaration");
    try { value = JSON.parse(match[1]!.trim()); } catch { throw new Error("Invalid component deployment declaration"); }
  }
  return componentDeploymentDirectoryV1(value === undefined ? {} : { "deployment-directory": value });
}
