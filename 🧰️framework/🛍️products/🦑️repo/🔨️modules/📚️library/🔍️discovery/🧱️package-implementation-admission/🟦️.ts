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

/** 🧱️ Whether a package file inside a nested unit directory may hold implementation. The package root itself is never a nested unit. */
export function packageImplementationAdmitted(admission: PackageImplementationAdmission | null, input: PackageImplementationAdmissionInput): boolean {
  if (admission === null || !input.relativePath.includes("/")) return false;
  if (admission.form === "nested-source-unit") return input.kindId === admission.unitSourceFileKindId;
  const manifest = input.manifestFilename(admission.unitManifestContractId);
  return manifest !== null && input.directoryHasFile(manifest);
}
