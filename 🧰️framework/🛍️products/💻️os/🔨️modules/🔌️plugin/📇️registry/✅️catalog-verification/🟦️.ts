import { COMPONENT_PACKAGE_ID, type ComponentSourceOwnerV1, type CompiledComponentOwnerV1 } from "../🔎️discovery/🟦️.ts";
import { SHIPPED_COMPONENT_MAX_BYTES } from "../../🖨️describe/🏷️component-receipt/🧬️schema/🟦️.ts";
import type { ComponentReceiptV1 } from "../../../../../../🔨️modules/🏷️component-receipt/🟦️.ts";

//#region 🔖️CatalogCompleteness
export const CATALOG_NODE_MAX = 256;

export const CATALOG_DEPENDENCY_MAX = 128;

export const CATALOG_ARTIFACT_MAX_BYTES = SHIPPED_COMPONENT_MAX_BYTES;

export const CATALOG_DESCRIPTOR_MAX_BYTES = 4 * 1024 * 1024;

export const CATALOG_DIAGNOSTIC_MAX_BYTES = 4096;

export const CATALOG_COMMIT_MARKER_MAX_BYTES = 64 * 1024;

export const CATALOG_COMMIT_MARKER_FILENAME = "🧾️.catalog-root.json";

export const CATALOG_IO_CHUNK_BYTES = 64 * 1024;

export const CATALOG_PACKAGE_ID = COMPONENT_PACKAGE_ID;

export const CATALOG_SHA256 = /^[0-9a-f]{64}$/;

export const CATALOG_DESCRIPTOR_PACK_FILENAME = "🛂️.descriptor.semio";

export const CATALOG_DESCRIPTOR_TOP_LEVEL = new Set(["descriptorVersion", "packageId", "role", "manifest", "activationEvents", "capabilityRequests", "extensionPoints", "execution", "executionProtocol", "quotas", "contributions", "assets"]);

export const CATALOG_MANIFEST_FIELDS = new Set(["pluginId", "label", "version", "apps", "examples", "capabilities", "topicContributions", "commands", "artifactKinds", "hostedArtifactKinds", "dependencies", "contributions"]);

export type CatalogVerificationNode = {
  readonly pluginId: string;
  readonly role: "plugin" | "extension";
  readonly dependsOn: readonly string[];
};

export type CatalogVerificationStatus = "verified" | "failed" | "blocked" | "cancelled";

export type CatalogVerificationProgress = {
  readonly schemaVersion: 1;
  readonly completed: number;
  readonly total: number;
  readonly pluginId: string;
  readonly status: CatalogVerificationStatus;
};

export type CatalogVerificationResult = {
  readonly schemaVersion: 1;
  readonly order: readonly string[];
  readonly results: readonly { readonly pluginId: string; readonly status: CatalogVerificationStatus; readonly diagnostic?: string }[];
  readonly publication: "committed" | "withheld" | "failed" | "not-requested";
  readonly publicationDiagnostic?: string;
};

export type CatalogVerificationExecutor<Receipt> = {
  readonly verify: (node: CatalogVerificationNode) => Promise<Receipt>;
  readonly publish?: (verified: readonly { readonly node: CatalogVerificationNode; readonly receipt: Receipt }[]) => Promise<void>;
};

export type CatalogVerificationControl = {
  readonly cancelled: () => boolean;
  readonly progress?: (progress: CatalogVerificationProgress) => void;
};

export type StrictCatalogDescriptor = {
  readonly entry: CompiledComponentOwnerV1;
  readonly jsonPath: string;
  readonly jsonBytes: Uint8Array;
  readonly packPath: string;
  readonly packBytes: Uint8Array;
  readonly descriptor: Readonly<Record<string, unknown>>;
  readonly semanticSha256: string;
};

export type CatalogSourceIssue = {
  readonly code: "manifest-invalid" | "identity-conflict" | "descriptor-pair-missing" | "descriptor-pair-incomplete" | "descriptor-invalid" | "dependency-invalid";
  readonly path: string;
  readonly pluginId?: string;
  readonly diagnostic: string;
};

export type CatalogSourceAudit = {
  readonly manifestCount: number;
  readonly entries: readonly ComponentSourceOwnerV1[];
  readonly sources: readonly StrictCatalogDescriptor[];
  readonly order: readonly string[];
  readonly issues: readonly CatalogSourceIssue[];
};

export type CatalogArtifactProgress = {
  readonly pluginId: string;
  readonly artifact: "raw" | "descriptor" | "receipt";
  readonly bytesRead: number;
  readonly totalBytes: number;
};

export type CatalogFileReceipt = {
  readonly path: string;
  readonly bytes: number;
  readonly sha256: string;
};

/** 🧾️ The marker a producer publishes last for one fresh catalog row: the raw component, its semantic descriptor and the
 * shipped `ComponentReceiptV1` binding the two, each by path, length and digest. */
export type FreshCatalogCommitMarker = {
  readonly schemaVersion: 2;
  readonly packageId: string;
  readonly pluginId: string;
  readonly packageName: string;
  readonly wasmOut: string;
  readonly raw: CatalogFileReceipt;
  readonly descriptor: CatalogFileReceipt;
  readonly receipt: CatalogFileReceipt;
};

export type CatalogArtifactControl = {
  readonly cancelled?: () => boolean;
  readonly progress?: (progress: CatalogArtifactProgress) => void;
  readonly afterArtifact?: (artifact: CatalogArtifactProgress["artifact"]) => void;
};

export type CatalogArtifactReceipt = {
  readonly pluginId: string;
  readonly rawSha256: string;
  readonly descriptorSha256: string;
  readonly receipt: ComponentReceiptV1;
  readonly rawBytes: Uint8Array;
  readonly descriptorBytes: Uint8Array;
};

export interface FreshCatalogBuildVerifier {
  readonly root: string;
  verify(entry: ComponentSourceOwnerV1, control?: CatalogArtifactControl): CatalogArtifactReceipt;
}

export type CatalogDescriptorIdentity = Readonly<{
  pluginId: string;
  packageId: string;
  role: "plugin" | "extension";
  extends?: string;
  dependsOn: readonly string[];
}>;

export const CATALOG_PLACEHOLDER_PLUGIN_IDS: readonly string[] = ["empty", "assembly-failed", "unknown", "placeholder"];

export const CATALOG_PLACEHOLDER_VERSION = "0.0.0";

export type OwnerDescriptorPairV1 = Readonly<{
  pluginId: string;
  packageId: string;
  role: "plugin" | "extension";
  version: string;
  execution: string;
  extends?: string;
  semanticSha256: string;
  descriptor: Readonly<Record<string, unknown>>;
}>;
//#endregion 🔖️CatalogCompleteness
