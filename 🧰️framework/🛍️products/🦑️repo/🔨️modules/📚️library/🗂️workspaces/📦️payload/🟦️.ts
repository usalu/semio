import { lstatSync, readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
export interface PackagePayloadOwner { readonly absDir: string; readonly name?: string; readonly exports?: unknown; }
export interface PackagePayloadEntry {
  readonly kind: "directory" | "file" | "symlink" | "other";
  readonly name: string;
}

export interface PackagePayloadOperations {
  readonly list: (path: string) => readonly PackagePayloadEntry[];
  readonly readText: (path: string) => string;
  readonly state: (path: string) => "directory" | "file" | "missing" | "symlink" | "other";
}

function errorCode(error: unknown): string | undefined {
  return typeof error === "object" && error !== null && "code" in error ? String((error as { code?: unknown }).code) : undefined;
}

function nativeState(path: string): "directory" | "file" | "missing" | "symlink" | "other" {
  try {
    const state = lstatSync(path);
    if (state.isSymbolicLink()) return "symlink";
    if (state.isDirectory()) return "directory";
    if (state.isFile()) return "file";
    return "other";
  } catch (error) {
    if (errorCode(error) === "ENOENT") return "missing";
    throw new Error(`Workspace source is unreadable: ${path}`, { cause: error });
  }
}

export const NATIVE_DISCOVERY_OPERATIONS: PackagePayloadOperations = {
  list: (path) => {
    try {
      return readdirSync(path, { withFileTypes: true }).map((entry) => ({
        kind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other",
        name: entry.name,
      }));
    } catch (error) {
      if (errorCode(error) === "ENOENT") return [];
      throw new Error(`Workspace directory is unreadable: ${path}`, { cause: error });
    }
  },
  readText: (path) => {
    try {
      return readFileSync(path, "utf8");
    } catch (error) {
      throw new Error(`Workspace manifest is unreadable: ${path}`, { cause: error });
    }
  },
  state: nativeState,
};

/** 📦️ Enumerates explicit export targets across package subpaths and conditions. */
function exportTargets(value: unknown, subpaths = true): string[] {
  if (typeof value === "string") return [value];
  if (Array.isArray(value)) return value.flatMap((entry) => exportTargets(entry, false));
  if (!value || typeof value !== "object") return [];
  const entries = Object.entries(value);
  if (entries.some(([key]) => key.startsWith("."))) {
    if (!subpaths || entries.some(([key]) => key !== "." && (!key.startsWith(".") || key.includes("*")))) return [];
    return entries.flatMap(([, entry]) => exportTargets(entry, false));
  }
  if (entries.some(([key]) => !key || /^\d+$/u.test(key))) return [];
  const targets: string[] = [];
  for (const [condition, entry] of entries) {
    targets.push(...exportTargets(entry, false));
    if (condition === "default") break;
  }
  return targets;
}

/** 🔗️ Binds a payload to its nearest package owner through a concrete physical export. */
export function ownsPayload(owner: PackagePayloadOwner, payload: PackagePayloadOwner, operations: Pick<PackagePayloadOperations, "state">): boolean {
  if (!owner.name || owner.name !== payload.name) return false;
  const prefix = relative(owner.absDir, payload.absDir).replaceAll("\\", "/") + "/";
  return exportTargets(owner.exports).some((target) => {
    if (!target.startsWith(".") || /[\\:*?%#\u0000]/u.test(target)) return false;
    const segments = target.slice(2).split("/");
    if (segments.some((segment) => !segment || segment === "." || segment === ".." || segment === "node_modules")) return false;
    if (!segments.join("/").startsWith(prefix)) return false;
    let path = owner.absDir;
    return segments.every((segment, index) => {
      path = join(path, segment);
      return operations.state(path) === (index === segments.length - 1 ? "file" : "directory");
    });
  });
}

