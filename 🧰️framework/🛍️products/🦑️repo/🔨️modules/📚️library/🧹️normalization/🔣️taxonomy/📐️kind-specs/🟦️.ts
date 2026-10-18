export interface TaxonomyLoadOptions { readonly repoRoot: string; readonly taxonomyPath?: string }

export interface FileKindSpec {
  readonly emoji: string;
  readonly extensionChains: readonly string[];
  readonly role: string;
}

export interface SemanticDirectoryKindSpec {
  readonly emoji: string;
  readonly slugPattern: string;
  readonly allowEmojiOnly: boolean;
  readonly inferWithoutEmoji?: boolean;
  readonly projectionOnly?: boolean;
  readonly parentKindIds?: readonly string[];
}

export type FixedContractScope = Readonly<
  | { kind: "exact-path"; path: string }
  | { kind: "repository-root" }
  | { kind: "package-root"; ecosystemId: string }
  | { kind: "directory-kind"; directoryKindId: string }
  | { kind: "fixed-directory-contract"; fixedDirectoryContractId: string }
  | { kind: "fixed-directory-contract-set"; fixedDirectoryContractIds: readonly string[] }
  | { kind: "sibling-fixed-filename-contract"; fixedFilenameContractId: string }
  | { kind: "path-pattern" }
>;

export interface FixedFilenameContract {
  readonly pathPattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly configurability: "unconfigurable";
  readonly scope: FixedContractScope;
  readonly verification: string;
  readonly expires: string | null;
}

export interface FixedDirectoryContract {
  readonly pathPattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly configurability: "unconfigurable";
  readonly scope: Exclude<FixedContractScope, { readonly kind: "package-root" }>;
  readonly verification: string;
  readonly expires: string | null;
}

export interface FixedFilenameRejectionContract {
  readonly sourcePathIdentities: readonly string[];
  readonly disposition: "normalize" | "relocate";
  readonly reason: string;
}

export interface ConfigurableEntryContract {
  readonly filename: string;
  readonly fileKindId: string;
  readonly ecosystemId: string;
  readonly role: string;
  readonly configurationSources: readonly string[];
}

export interface FileKindResolutionRuleSpec {
  readonly extensionChain: string;
  readonly fileKindId: string;
  readonly priority: number;
  readonly filenamePattern?: string;
  readonly pathPattern?: string;
  readonly parentKindIds?: readonly string[];
  readonly ancestorKindIds?: readonly string[];
}

export interface ScopedFileKindSpec {
  readonly pathPattern: string;
  readonly parentDirectoryKindId?: string;
  readonly emoji: string;
  readonly extensionChains: readonly string[];
  readonly role: string;
  readonly sourceFilenamePattern: string;
  readonly authority: string;
  readonly reason: string;
  readonly verification: string;
  readonly expires: string | null;
}

export interface SemanticDirectoryMemberKindSpec {
  readonly ownerKindIds: readonly string[];
  readonly memberNames: readonly string[];
  readonly memberSlugPattern?: string;
  readonly source: "registry";
}

export interface SemanticProjectedMemberKindSpec {
  readonly ownerKindIds: readonly string[];
  readonly projectionContractId: string;
  readonly sourceMemberKindId: string;
  readonly identityField: "mutationDirectoryName" | "commandDirectoryName";
}
