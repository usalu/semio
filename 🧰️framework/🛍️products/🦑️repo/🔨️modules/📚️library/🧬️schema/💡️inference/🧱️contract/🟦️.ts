export const POLICY_INFERENCES_FACET = "💡️inferences";

export const POLICY_DERIVED_MARKER = "#[derived]";

export type PolicyInferenceSourceIssue = Readonly<{ path: string; state: "unreadable" | "symlink" | "not-directory" | "not-file" }>;

export type PolicyInferenceFamilySource = Readonly<{ inferencesRel: string; artifactRel: string; slugs: readonly string[] }>;

export type PolicyInferenceDiscovery = Readonly<{ families: readonly PolicyInferenceFamilySource[]; issues: readonly PolicyInferenceSourceIssue[] }>;
