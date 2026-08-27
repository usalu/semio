import { describe, expect, mock, test } from "bun:test";
import * as filesystem from "node:fs";
import { dirname, join, parse } from "node:path";
import Ajv from "ajv";

//#region 🔗️CargoProviderBindingFilesystemTrace
type Trace = {
  readonly id: string;
  readonly workspaceRootSegments: readonly string[];
  readonly consumerManifest: string;
  readonly dependencyKey: string;
  readonly symlinkSegments?: readonly string[];
  readonly expectedLstatSegments: readonly (readonly string[])[];
};

type Call = { readonly operation: "lstat" | "read"; readonly path: string };
type VirtualFilesystem = { readonly files: ReadonlyMap<string, string>; readonly directories: ReadonlySet<string>; readonly symlink?: string; readonly calls: Call[] };

const actual = { ...filesystem };
const fixturePath = join(import.meta.dir, "🔣️vectors.json"), schemaPath = join(import.meta.dir, "🛂️schema.json");
const fixture = JSON.parse(actual.readFileSync(fixturePath, "utf8")) as { readonly schemaVersion: 1; readonly traces: readonly Trace[] };
let virtual: VirtualFilesystem | undefined;

const tracedLstat = (path: filesystem.PathLike, options?: unknown): unknown => {
  if (!virtual) return actual.lstatSync(path, options as never);
  const value = String(path);
  virtual.calls.push({ operation: "lstat", path: value });
  if (value === virtual.symlink) return { isSymbolicLink: () => true, isDirectory: () => false, isFile: () => false };
  if (virtual.directories.has(value)) return { isSymbolicLink: () => false, isDirectory: () => true, isFile: () => false };
  if (virtual.files.has(value)) return { isSymbolicLink: () => false, isDirectory: () => false, isFile: () => true };
  throw Object.assign(new Error(`ENOENT ${value}`), { code: "ENOENT" });
};

const tracedRead = (path: filesystem.PathOrFileDescriptor, options?: unknown): unknown => {
  if (!virtual) return actual.readFileSync(path, options as never);
  const value = String(path);
  virtual.calls.push({ operation: "read", path: value });
  const source = virtual.files.get(value);
  if (source === undefined) throw Object.assign(new Error(`ENOENT ${value}`), { code: "ENOENT" });
  return source;
};

mock.module("node:fs", () => ({ ...actual, lstatSync: tracedLstat, readFileSync: tracedRead, default: { ...actual, lstatSync: tracedLstat, readFileSync: tracedRead } }));
const { resolveCargoProviderBinding } = await import("../../🔍️discovery/🟦️component.ts");

function traceFilesystem(root: string, symlink: string | undefined): VirtualFilesystem {
  const files = new Map<string, string>(), directories = new Set<string>();
  const add = (locator: string, source: string): void => {
    const path = join(root, locator);
    files.set(path, source);
    for (let parent = dirname(path); ; parent = dirname(parent)) {
      directories.add(parent);
      if (dirname(parent) === parent) break;
    }
  };
  add("Cargo.toml", "[workspace]\n");
  add("consumer/Cargo.toml", "[package]\nname = 'consumer'\n\n[dependencies]\nupstream-package = { path = '../provider' }\n");
  add("provider/Cargo.toml", "[package]\nname = 'upstream-package'\n\n[lib]\nname = 'actual_library'\npath = 'src/lib.rs'\n");
  add("provider/src/lib.rs", "pub fn marker() {}\n");
  return { files, directories, symlink, calls: [] };
}

describe("cargo provider binding filesystem trace", () => {
  test("fails unsafe input before access and stops no-follow workspace boundaries before manifest reads", () => {
    expect(new Ajv({ strict: true, allErrors: true }).compile(JSON.parse(actual.readFileSync(schemaPath, "utf8")))(fixture)).toBe(true);
    const filesystemRoot = parse(process.cwd()).root;
    for (const trace of fixture.traces) {
      const root = join(filesystemRoot, ...trace.workspaceRootSegments), symlink = trace.symlinkSegments === undefined ? undefined : join(filesystemRoot, ...trace.symlinkSegments);
      const view = traceFilesystem(root, symlink);
      virtual = view;
      try {
        expect(() => resolveCargoProviderBinding({ workspaceRoot: root, consumerManifestLocator: trace.consumerManifest, dependencyKey: trace.dependencyKey }), trace.id).toThrow();
        expect(view.calls, trace.id).toEqual(trace.expectedLstatSegments.map((segments) => ({ operation: "lstat", path: join(filesystemRoot, ...segments) })));
      } finally { virtual = undefined; }
    }
  });
});
//#endregion 🔗️CargoProviderBindingFilesystemTrace
