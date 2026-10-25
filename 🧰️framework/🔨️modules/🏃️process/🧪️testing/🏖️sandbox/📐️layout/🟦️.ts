import { existsSync, mkdirSync, realpathSync } from "node:fs";
import { tmpdir } from "node:os";
import { basename, dirname, isAbsolute, join, parse, relative, resolve, sep } from "node:path";
import protocol from "../🔣️.json" with { type: "json" };
import imageSchema from "../../../📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️.json" with { type: "json" };
import backendSchema from "../../../📦️artifacts/🏗️native-build/💾️storage/🔌️backends/🧬️schema/🔣️.json" with { type: "json" };

/** 🏖️ Carries the closed test sandbox protocol: environment names, layout, port range and guard policy. */
export type TestSandboxProtocolV1 = Readonly<typeof protocol>;

/** 📖️ Reads the authored sandbox protocol and refuses incomplete declarations. */
export function readTestSandboxProtocolV1(value: unknown = protocol): TestSandboxProtocolV1 {
  const row = value as TestSandboxProtocolV1;
  if (row?.version !== 1 || !row.environment || !row.layout || !row.ports || !row.guard || !Array.isArray(row.rules)) throw Error("Complete test sandbox protocol required");
  for (const name of ["sandbox", "artifacts", "images", "cache", "guard", "report", "checkout"] as const) if (!/^SEMIO_TEST_[A-Z_]+$/u.test(row.environment[name] ?? "")) throw Error("Test sandbox environment names required");
  if (!(row.ports.minimum >= 1024 && row.ports.maximum > row.ports.minimum && row.ports.maximum <= 65535 && row.ports.attempts >= 1)) throw Error("Finite test port range required");
  return row;
}

/** 🩳️ The shortest physical base this host offers when the temporary directory leaves no room for a sandbox root (a build tool may point TMPDIR into a deep cache path). */
function shortSandboxBaseV1(): string {
  const base = process.platform === "win32" ? join(parse(tmpdir()).root, "tmp") : "/tmp";
  try { mkdirSync(base, { recursive: true }); } catch {}
  try { return realpathSync(base); } catch { return base; }
}

/** 🧱️ Names the managed native parent mount (`<partition>/m` beside a declared backing file) that hosts a path, if any: a sandbox must never nest its images in it, because the parent then verifies every kernel mount below its root as one of its own funded children and refuses every build while a test image is attached. */
export function managedNativeParentOfV1(path: string): string | undefined {
  const mount = imageSchema.definitions.ImageProtocol.const.mountDirectory, backings = backendSchema.definitions.Protocol.const.backends.map((row) => row.backingFilename);
  for (let current = resolve(path); ; current = dirname(current)) {
    if (basename(current) === mount && backings.some((name) => existsSync(join(dirname(current), name)))) return current;
    if (dirname(current) === current) return undefined;
  }
}

/** 🧭️ Returns a temporary directory outside every managed native parent when the given one lies inside one (a build tool points TMPDIR into the products partition), else undefined. */
export function safeTemporaryDirectoryV1(temporary: string = tmpdir()): string | undefined {
  return managedNativeParentOfV1(temporary) === undefined ? undefined : shortSandboxBaseV1();
}

/** 🛑️ Refuses (EN and DE) an artifact directory inside a managed native parent; returns its resolved path otherwise. */
export function requireArtifactOutsideManagedParentV1(path: string, declared: TestSandboxProtocolV1 = protocol): string {
  let real = resolve(path);
  try { real = realpathSync(real); } catch {}
  const parent = managedNativeParentOfV1(real);
  if (parent !== undefined) {
    const fill = (text: string): string => text.replaceAll("{path}", real).replaceAll("{parent}", parent);
    throw Error(fill(declared.messages.artifactInManagedParent.en) + "\n" + fill(declared.messages.artifactInManagedParent.de));
  }
  return real;
}

/** 📁️ Resolves the shared parent below which every sandbox of this host lives; a short physical path outside every managed native parent: the temporary directory unless a root below it (parent, session segment, id) would pass `maximumRootLength` or it lies inside a managed native parent (a build tool points TMPDIR into the products partition), then the short host base (one directory per user, because that base is shared). An explicit parent is never replaced. */
export function testSandboxParentV1(parent?: string, declared: TestSandboxProtocolV1 = protocol): string {
  let base = resolve(parent ?? tmpdir());
  try { base = realpathSync(base); } catch {}
  const candidate = join(base, declared.layout.parent), budget = declared.layout.maximumRootLength - (declared.layout.sessionRootCharacters + declared.layout.idCharacters + 2);
  if (parent !== undefined || candidate.length <= budget && managedNativeParentOfV1(base) === undefined) return candidate;
  const short = join(shortSandboxBaseV1(), declared.layout.parent + (process.getuid?.() ?? ""));
  return short.length <= budget && managedNativeParentOfV1(short) === undefined ? short : candidate;
}

/** 🫀️ Tells whether a process id still exists on this host. */
export function testProcessAliveV1(pid: number): boolean {
  if (!Number.isSafeInteger(pid) || pid < 1) return false;
  try { process.kill(pid, 0); return true; } catch (error) { return (error as NodeJS.ErrnoException).code === "EPERM"; }
}

/** 📏️ Tells whether a path lies inside a root, comparing case-insensitively where the file system is. */
export function testPathInsideV1(root: string, path: string, platform: NodeJS.Platform = process.platform): boolean {
  const fold = (value: string): string => platform === "win32" || platform === "darwin" ? value.toLowerCase() : value;
  const local = relative(fold(resolve(root)), fold(resolve(path)));
  return local === "" || (local !== ".." && !local.startsWith(".." + sep) && !isAbsolute(local));
}
