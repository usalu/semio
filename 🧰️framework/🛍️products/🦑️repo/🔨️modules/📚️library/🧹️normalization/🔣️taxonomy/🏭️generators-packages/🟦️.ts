import type { PackageImplementationAdmission } from "../../../🔍️discovery/🧱️package-implementation-admission/🟦️.ts";
import type { GeneratorPreviewProgressPolicyV1 } from "../../../🏭️generator/👁️preview/📈️progress/🟦️.ts";
import type { GeneratorProjectionActivation, RegistryCatalogInputDiscovery } from "../../../🔍️discovery/📜️taxonomy-contracts/🟦️.ts";
import type { SemanticPackageGeneration } from "../../../🔍️discovery/📦️package-projection-contracts/🟦️.ts";

export type GeneratorOwnership = "owned" | "external";

export interface GeneratorOutputRootSpec {
  readonly path: string;
  readonly inclusion: "tracked" | "ignored";
}

export interface GeneratorContractSpec {
  readonly ownership: GeneratorOwnership;
  readonly ownerPath: string | null;
  readonly target: string | null;
  readonly previewTarget?: string;
  readonly previewArguments?: readonly string[];
  readonly previewScript?: string;
  readonly previewProgress?: GeneratorPreviewProgressPolicyV1;
  readonly previewLimits?: { readonly maxOutputBytes: number; readonly timeoutMs: number };
  readonly compilerInputManifest?: { readonly kind: "compiler-input-manifest-v1"; readonly manifestOutputPath: string; readonly manifestSchemaPath: string; readonly staticAuthorityPath: string; readonly maxFiles: number };
  readonly checkTarget?: string;
  readonly inputPatterns: readonly string[];
  readonly inputDiscovery?: RegistryCatalogInputDiscovery;
  readonly packageGeneration?: SemanticPackageGeneration;
  readonly projectionActivation?: GeneratorProjectionActivation;
  readonly outputRoots: readonly GeneratorOutputRootSpec[];
  readonly reason: string;
}

export interface PackageBoundaryRule {
  readonly manifestContractId: string | null;
  readonly entryContractIds: readonly string[];
  readonly allowedFixedContractIds: readonly string[];
  readonly allowedFileKindIds: readonly string[];
  readonly allowedDirectoryKindIds: readonly string[];
  readonly glueGrammarId: string;
  readonly recursive: true;
  readonly uncertainRole: "problem";
  readonly implementationRole: "problem";
  readonly implementationAdmission: PackageImplementationAdmission | null;
}

export interface PackageBoundaryProfile {
  readonly admission: "blocked-until-language-directory-registered";
  readonly allowedFileKindIds: readonly string[];
  readonly allowedDirectoryKindIds: readonly string[];
  readonly allowedFixedContractIds: readonly string[];
  readonly glueGrammarId: string;
  readonly recursive: true;
  readonly uncertainRole: "problem";
  readonly implementationRole: "problem";
  readonly reason: string;
}

export interface PackageSourceDisposition {
  readonly contractKind: "fixed" | "configurable";
  readonly disposition: "adapter-source" | "tool-metadata";
  readonly validator: "package-glue" | "command-router" | "vitest-configuration" | "tool-config-vitest" | "tool-config-tailwind" | "tool-config-postcss" | "tool-config-eslint" | "tool-config-dependency-cruiser" | "pytest-configuration" | "eslint-configuration" | "vscode-test-configuration";
  readonly grammarId?: string;
  readonly authority: string;
  readonly verification: string;
}

export interface EcosystemSpec {
  readonly packageIdentity: "manifest" | "boundary-only";
  readonly manifestContractId: string | null;
}

export interface PackageGlueGrammar {
  readonly analyzer: "rust" | "typescript" | "javascript" | "go" | "python" | "dotnet" | "c-cpp" | "tex";
  readonly allowedRoles: readonly ("declaration" | "registration" | "bootstrap" | "thin-delegation")[];
  readonly maxDelegationStatements: number;
}
