import { type SemanticDescendantNode, canonicalTaxonomyEmoji } from "../📜️taxonomy-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import type { SemanticDirectoryMatchContext } from "../📂️semantic-directory-kinds/🟦️.ts";
import { type TaxonomyPathMatcher, createTaxonomyPathMatcher } from "../🎯️taxonomy-path-matching/🟦️.ts";
import type { TaxonomyCliAttemptPreparationFacts, TaxonomyCliWritePreparationFacts } from "../🛠️generator-commands/🟦️.ts";

/** 🪣️ The taxonomy authority functions the rule group validators call; the authority module injects them so no rule group imports the module that loads and validates the taxonomy. */
export type TaxonomyAuditPorts = Readonly<{
  semanticDirectoryKindId: (name: string, taxonomy: Taxonomy, context?: SemanticDirectoryMatchContext) => string | null;
  fileKindIdForSourcePath: (path: string, taxonomy: Taxonomy) => string | null;
  canonicalFilenameForKind: (kindId: string, taxonomy: Taxonomy) => string;
  canonicalFilenamesForKind: (kindId: string, taxonomy: Taxonomy) => string[];
  taxonomyCliAttemptPreparationsProblems: (facts: readonly Readonly<TaxonomyCliAttemptPreparationFacts>[], taxonomy: Taxonomy) => string[];
  taxonomyCliLeaseDirectoryProblems: (facts: Readonly<{ parentKindId: string; directoryName: string; leafNames: readonly string[]; writePreparations: readonly Readonly<TaxonomyCliWritePreparationFacts>[] }>, taxonomy: Taxonomy) => string[];
  artifactPathProjectionCatalogRoots: (artifactRoot: string, contractId: string, taxonomy: Taxonomy) => readonly Readonly<{ sourceRoot: string; destinationRoot: string }>[];
  semanticDescendantNodeRelativePath: (node: SemanticDescendantNode, taxonomy: Taxonomy) => string;
}>;

/** 🪣️ One taxonomy under audit with its shared problem list, its shared check helpers and the injected authority ports. */
export type TaxonomyAudit = TaxonomyAuditPorts & Readonly<{
  taxonomy: Taxonomy;
  problems: string[];
  pathMatcher: TaxonomyPathMatcher;
  canonicalEmoji: (value: unknown) => value is string;
  document: Record<string, unknown>;
  record: (value: unknown, key: string) => value is Record<string, unknown>;
  ids: (values: readonly string[] | undefined, registry: Readonly<Record<string, unknown>>, key: string) => void;
  pattern: (value: string, key: string) => void;
  fullPattern: (value: unknown, key: string) => void;
  pathPattern: (value: unknown, key: string) => void;
  workspacePath: (value: unknown, key: string) => value is string;
  extensionChain: (value: unknown, key: string) => void;
  exactKeys: (value: object, allowed: readonly string[], key: string) => void;
  kebabId: (value: string, key: string) => void;
}>;

/** 🪣️ Creates the audit context for one taxonomy, binding the shared problem list, the shared check helpers and the injected authority ports. */
export function createTaxonomyAudit(taxonomy: Taxonomy, ports: TaxonomyAuditPorts): TaxonomyAudit {
  const problems: string[] = [];
  const pathMatcher = createTaxonomyPathMatcher();
  const emojiVerdicts = new Map<string, boolean>();
  const canonicalEmoji = (value: unknown): value is string => {
    if (typeof value !== "string") return false;
    if (!emojiVerdicts.has(value)) emojiVerdicts.set(value, canonicalTaxonomyEmoji(value));
    return emojiVerdicts.get(value)!;
  };
  const document = taxonomy as unknown as Record<string, unknown>;
  const record = (value: unknown, key: string): value is Record<string, unknown> => {
    const valid = typeof value === "object" && value !== null && !Array.isArray(value);
    if (!valid || Object.keys(value as Record<string, unknown>).length === 0) problems.push(`${key} must be a non-empty object.`);
    return valid;
  };
  const ids = (values: readonly string[] | undefined, registry: Readonly<Record<string, unknown>>, key: string): void => {
    if (!Array.isArray(values)) {
      problems.push(`${key} must be an array.`);
      return;
    }
    const seen = new Set<string>();
    for (const id of values) {
      if (seen.has(id)) problems.push(`${key} contains duplicate id ${JSON.stringify(id)}.`);
      seen.add(id);
      if (!(id in registry)) problems.push(`${key} references missing id ${JSON.stringify(id)}.`);
    }
  };
  const pattern = (value: string, key: string): void => {
    try {
      void new RegExp(value, "u");
    } catch {
      problems.push(`${key} is not a valid Unicode regular expression.`);
    }
  };
  const fullPattern = (value: unknown, key: string): void => {
    if (typeof value !== "string") {
      problems.push(`${key} must be a string.`);
      return;
    }
    pattern(value, key);
    if (!value.startsWith("^") || !value.endsWith("$")) problems.push(`${key} must be an anchored full-match pattern.`);
  };
  const pathPattern = (value: unknown, key: string): void => {
    if (typeof value !== "string" || !value || value !== value.normalize("NFC") || value.startsWith("/") || value.includes("\\") || /[{}!]/u.test(value) || /(^|\/)\*\*[^/]|[^/]\*\*(\/|$)/u.test(value)) {
      problems.push(`${key} must be one NFC workspace-relative v7 path pattern.`);
      return;
    }
    try {
      void pathMatcher.matches("", value);
    } catch {
      problems.push(`${key} is not a valid v7 path pattern.`);
    }
  };
  const workspacePath = (value: unknown, key: string): value is string => {
    const valid = typeof value === "string" && value.length > 0 && value === value.normalize("NFC") && !value.startsWith("/") && !value.endsWith("/") && !value.includes("\\")
      && !/[*?\[\]{}!]/u.test(value) && value.split("/").every((segment) => segment.length > 0 && segment !== "." && segment !== "..");
    if (!valid) problems.push(`${key} must be one exact NFC workspace-relative path.`);
    return valid;
  };
  const extensionChain = (value: unknown, key: string): void => {
    if (typeof value !== "string" || !/^\.[a-z0-9]+(?:[.-][a-z0-9]+)*$/u.test(value)) problems.push(`${key} must be one lowercase dot-prefixed extension chain.`);
  };
  const exactKeys = (value: object, allowed: readonly string[], key: string): void => {
    const actual = Object.keys(value).sort();
    const expected = [...allowed].sort();
    if (actual.join("\0") !== expected.join("\0")) problems.push(`${key} must contain exactly ${expected.join(", ")}.`);
  };
  const kebabId = (value: string, key: string): void => {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/u.test(value)) problems.push(`${key} must be kebab-case.`);
  };
  return { taxonomy, problems, pathMatcher, canonicalEmoji, document, record, ids, pattern, fullPattern, pathPattern, workspacePath, extensionChain, exactKeys, kebabId, ...ports };
}
