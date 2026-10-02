import { lstatSync, readdirSync, readFileSync } from "node:fs";
import { join, relative } from "node:path";
import { createRequire } from "node:module";
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

type PhysicalPayloadOwnership = (owner: PackagePayloadOwner, payload: PackagePayloadOwner, operations: Pick<PackagePayloadOperations, "state">) => boolean;
const physical: { readonly ownsPayload: PhysicalPayloadOwnership } = createRequire(import.meta.url)("./🟨️.cjs");

/** 🔗️ Binds a payload to its nearest package owner through a concrete physical export. */
export const ownsPayload: PhysicalPayloadOwnership = physical.ownsPayload;
