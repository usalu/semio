/** 🧱️ Implementation a package boundary admits because its build tool addresses code by directory: a Go package is one directory below the module root, a Rust crate is a directory that carries its own manifest. Moving such files to the owner would merge distinct compilation units, so the schema (`packageBoundaryRules[*].implementationAdmission`) names the unit form and everything else inside a package boundary stays glue-only. A rule that omits `implementationAdmission` (the taxonomy document is read unparsed) admits nothing. */
export type PackageImplementationAdmission =
  | Readonly<{ form: "nested-source-unit"; unitSourceFileKindId: string }>
  | Readonly<{ form: "nested-manifest-unit"; unitManifestContractId: string }>;

/** 🔎️ One package file as the admission sees it. */
export interface PackageImplementationAdmissionInput {
  readonly relativePath: string;
  readonly kindId: string | null;
  readonly directoryHasFile: (name: string) => boolean;
  readonly manifestFilename: (contractId: string) => string | null;
}

/** 📦️ Whether a package file inside a nested unit directory may hold implementation. The package root itself is never a nested unit. */
export function packageImplementationAdmitted(admission: PackageImplementationAdmission | null, input: PackageImplementationAdmissionInput): boolean {
  if (admission === null || !input.relativePath.includes("/")) return false;
  if (admission.form === "nested-source-unit") return input.kindId === admission.unitSourceFileKindId;
  const manifest = input.manifestFilename(admission.unitManifestContractId);
  return manifest !== null && input.directoryHasFile(manifest);
}

/** 📂️ Directories a package boundary admits besides its semantic directory kinds: `manifest-declared-entries` admits every directory on the way to an entry the package manifest declares (a TypeScript `exports` map names its facade leaves); `fixed-contract-trees` admits every directory on the way to a file of a fixed contract whose pattern is anchored below the package root (the Next.js `app/**` route files). */
export type PackageDirectoryAdmission =
  | Readonly<{ form: "manifest-declared-entries"; manifestContractId: string; entryFields: readonly string[] }>
  | Readonly<{ form: "fixed-contract-trees" }>;

/** 📁️ One package directory as the admission sees it. */
export interface PackageDirectoryAdmissionInput {
  readonly relativePath: string;
  readonly unitBelow: boolean;
  readonly declaredEntryDirectories: ReadonlySet<string>;
  readonly contractTreeBelow: boolean;
}

/** 🗂️ Whether a directory below a package root may exist: it is, or leads to, an admitted build unit, an entry the manifest declares or a file of an admitted fixed-contract tree. */
export function packageDirectoryAdmitted(admission: PackageImplementationAdmission | null, directoryAdmissions: readonly PackageDirectoryAdmission[], input: PackageDirectoryAdmissionInput): boolean {
  if (admission !== null && input.unitBelow) return true;
  return directoryAdmissions.some((row) => row.form === "manifest-declared-entries" ? input.declaredEntryDirectories.has(input.relativePath) : input.contractTreeBelow);
}

/** 🌲️ The fixed-contract patterns, relative to the package root, that describe a file below it: only patterns anchored at the package's own `<packages>/<language>/` prefix qualify, so a pattern that matches at any depth (`**\/📜️script.ts`) never admits a directory. */
export function packageContractTreePatterns(patterns: readonly string[], packagePrefix: string): readonly string[] {
  const anchor = `/${packagePrefix}/`;
  return patterns.flatMap((pattern) => {
    const at = pattern.indexOf(anchor);
    const relative = at < 0 ? "" : pattern.slice(at + anchor.length);
    return relative.includes("/") ? [relative] : [];
  });
}

/** 🧭️ Every directory (relative to the package root, POSIX separators) on the way to a file target of the given manifest fields; targets that leave the package root or are not relative paths declare none. */
export function declaredEntryDirectories(manifest: unknown, entryFields: readonly string[]): ReadonlySet<string> {
  const directories = new Set<string>();
  if (typeof manifest !== "object" || manifest === null) return directories;
  const visit = (value: unknown): void => {
    if (typeof value === "string") {
      if (!value.startsWith("./") || value.split("/").includes("..")) return;
      const parts: string[] = [];
      for (const part of value.split("/").slice(0, -1)) {
        if (part === "" || part === ".") continue;
        parts.push(part);
        directories.add(parts.join("/"));
      }
    } else if (Array.isArray(value)) value.forEach(visit);
    else if (typeof value === "object" && value !== null) Object.values(value).forEach(visit);
  };
  for (const field of entryFields) visit((manifest as Record<string, unknown>)[field]);
  return directories;
}
