import type { FixedContractScope, FixedDirectoryContract, FixedFilenameContract } from "../📜️taxonomy-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";

/** 🧭️ Context required to enforce a fixed contract's scope beyond its path pattern. */
export interface FixedContractMatchContext {
  readonly packageRoot?: boolean;
  readonly ecosystemId?: string;
  readonly parentDirectoryKindId?: string;
  readonly parentFixedDirectoryContractIds?: readonly string[];
  readonly siblingFixedFilenameContractIds?: readonly string[];
  readonly pathMatcher?: TaxonomyPathMatcher;
}

function taxonomyPatternExpression(pattern: string): RegExp {
  let expression = "^";
  for (let index = 0; index < pattern.length;) {
    if (pattern.slice(index, index + 3) === "**/") {
      expression += "(?:[^/]+/)*";
      index += 3;
      continue;
    }
    const character = pattern[index]!;
    if (character === "*" && pattern[index + 1] === "*") {
      expression += ".*";
      index += 2;
      continue;
    }
    if (character === "*") expression += "[^/]*";
    else if (character === "?") expression += "[^/]";
    else if (character === "[") {
      const end = pattern.indexOf("]", index + 1);
      if (end < 0) throw new Error(`Invalid taxonomy path pattern ${JSON.stringify(pattern)}.`);
      expression += pattern.slice(index, end + 1);
      index = end;
    } else expression += character.replace(/[\\^$.*+?()[\]{}|]/gu, "\\$&");
    index += 1;
  }
  return new RegExp(`${expression}$`, "u");
}

/** 🪢 Matches one NFC workspace-relative POSIX path against the v7 contract glob grammar. */
export function taxonomyPathPatternMatches(path: string, pattern: string): boolean {
  const normalizedPath = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize("NFC");
  return taxonomyPatternExpression(pattern.normalize("NFC")).test(normalizedPath);
}

/** 🧮️ An invocation-owned matcher with private compiled patterns and no retained path results. */
export interface TaxonomyPathMatcher {
  readonly matches: (path: string, pattern: string) => boolean;
}

/** 🧵️ Reuses successful pure compilations only for this validation or query invocation. */
export function createTaxonomyPathMatcher(): TaxonomyPathMatcher {
  const expressions = new Map<string, RegExp>(), expressionsByPattern = new Map<string, RegExp>();
  let lastPath: string | undefined, lastNormalizedPath = "";
  return Object.freeze({
    matches(path: string, pattern: string): boolean {
      if (path !== lastPath) {
        lastPath = path;
        lastNormalizedPath = path.replaceAll("\\", "/").replace(/^\.\//u, "").normalize("NFC");
      }
      let expression = expressionsByPattern.get(pattern);
      if (!expression) {
        const normalizedPattern = pattern.normalize("NFC");
        expression = expressions.get(normalizedPattern);
        if (!expression) {
          expression = taxonomyPatternExpression(normalizedPattern);
          expressions.set(normalizedPattern, expression);
        }
        expressionsByPattern.set(pattern, expression);
      }
      return expression.test(lastNormalizedPath);
    },
  });
}

/** 🔒️Parses a finite union of literal, separately declared parent-directory authorities. */
export function parseFixedDirectoryContractSetScope(input: unknown, parents: Readonly<Record<string, FixedDirectoryContract>>): Extract<FixedContractScope, { kind: "fixed-directory-contract-set" }> {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Fixed parent scope must be an object");
  const row = input as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "fixedDirectoryContractIds,kind" || row.kind !== "fixed-directory-contract-set" || !Array.isArray(row.fixedDirectoryContractIds) || row.fixedDirectoryContractIds.length < 1 || row.fixedDirectoryContractIds.length > 256) throw new Error("Fixed parent scope requires one nonempty finite parent ID set");
  const ids = new Set<string>(), paths = new Set<string>();
  for (const id of row.fixedDirectoryContractIds) {
    if (typeof id !== "string" || !id || ids.has(id) || !parents[id]) throw new Error("Fixed parent scope contains a duplicate or unknown directory contract ID");
    const parent = parents[id];
    if (/[*?\[\]{}]/u.test(parent.pathPattern) || !["exact-path", "path-pattern", "repository-root"].includes(parent.scope.kind) || paths.has(parent.pathPattern)) throw new Error("Fixed parent scope requires distinct literal parent authorities");
    ids.add(id);
    paths.add(parent.pathPattern);
  }
  return { kind: "fixed-directory-contract-set", fixedDirectoryContractIds: [...ids] };
}

/** 🏷️Resolves only a declared finite parent set, with no recursive aliases or inferred parents. */
export function parseNamedFixedDirectoryContractSetScope(input: unknown, parents: Readonly<Record<string, FixedDirectoryContract>>, sets: Readonly<Record<string, readonly string[]>>): Extract<FixedContractScope, { kind: "fixed-directory-contract-set" }> {
  if (!input || typeof input !== "object" || Array.isArray(input)) throw new Error("Named fixed parent scope must be an object");
  const row = input as Record<string, unknown>;
  if (Object.keys(row).sort().join(",") !== "fixedDirectoryContractSetId,kind" || row.kind !== "named-fixed-directory-contract-set" || typeof row.fixedDirectoryContractSetId !== "string" || !/^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u.test(row.fixedDirectoryContractSetId) || !Object.hasOwn(sets, row.fixedDirectoryContractSetId)) throw new Error("Named fixed parent scope requires one declared set ID");
  return parseFixedDirectoryContractSetScope({ kind: "fixed-directory-contract-set", fixedDirectoryContractIds: sets[row.fixedDirectoryContractSetId] }, parents);
}

export function fixedScopeMatches(contract: FixedFilenameContract | FixedDirectoryContract, path: string, context: FixedContractMatchContext, sets: Readonly<Record<string, readonly string[]>> = {}): boolean {
  if (contract.scope.kind === "exact-path") return path === contract.scope.path;
  if (contract.scope.kind === "repository-root") return !path.includes("/");
  if (contract.scope.kind === "package-root") return context.packageRoot === true && context.ecosystemId === contract.scope.ecosystemId;
  if (contract.scope.kind === "directory-kind") return context.parentDirectoryKindId === contract.scope.directoryKindId;
  if (contract.scope.kind === "fixed-directory-contract") return context.parentFixedDirectoryContractIds?.includes(contract.scope.fixedDirectoryContractId) === true;
  if (contract.scope.kind === "fixed-directory-contract-set") return contract.scope.fixedDirectoryContractIds.some((id) => context.parentFixedDirectoryContractIds?.includes(id));
  if (contract.scope.kind === "named-fixed-directory-contract-set") return sets[contract.scope.fixedDirectoryContractSetId]?.some((id) => context.parentFixedDirectoryContractIds?.includes(id)) === true;
  if (contract.scope.kind === "sibling-fixed-filename-contract") return context.siblingFixedFilenameContractIds?.includes(contract.scope.fixedFilenameContractId) === true;
  return true;
}

/** 📄️ Extracts the exact literal basename owned by a fixed filename contract. */
export function fixedContractFilename(contract: FixedFilenameContract): string {
  return contract.pathPattern.slice(contract.pathPattern.lastIndexOf("/") + 1);
}

/** 📏️ Comparable specificity tuple for deterministic fixed-contract precedence. */
export function fixedContractSpecificity(contract: FixedFilenameContract | FixedDirectoryContract): readonly [number, number, number, number] {
  const wildcardTokens = contract.pathPattern.match(/\*\*|\*|\?|\[[^\]]+\]/gu) ?? [];
  const literalSegments = contract.pathPattern.split("/").filter((segment) => !/\*|\?|\[/u.test(segment)).length;
  const literalCodePoints = [...contract.pathPattern.replace(/\*\*|\*|\?|\[[^\]]+\]|\//gu, "")].length;
  const scopeStrength = { "path-pattern": 0, "directory-kind": 1, "package-root": 2, "fixed-directory-contract": 3, "fixed-directory-contract-set": 3, "named-fixed-directory-contract-set": 3, "sibling-fixed-filename-contract": 3, "repository-root": 4, "exact-path": 5 }[contract.scope.kind];
  return [literalSegments, literalCodePoints, -wildcardTokens.length, scopeStrength];
}

function compareFixedContracts(left: readonly [string, FixedFilenameContract | FixedDirectoryContract], right: readonly [string, FixedFilenameContract | FixedDirectoryContract]): number {
  const leftScore = fixedContractSpecificity(left[1]);
  const rightScore = fixedContractSpecificity(right[1]);
  for (let index = 0; index < leftScore.length; index += 1) if (leftScore[index] !== rightScore[index]) return rightScore[index]! - leftScore[index]!;
  return left[0].localeCompare(right[0]);
}

export function fixedContractWinner<T extends FixedFilenameContract | FixedDirectoryContract>(kind: "filename" | "directory", matches: readonly (readonly [string, T])[]): string[] {
  if (matches.length === 0) return [];
  const ordered = [...matches].sort(compareFixedContracts);
  if (ordered.length > 1 && fixedContractSpecificity(ordered[0]![1]).every((score, index) => score === fixedContractSpecificity(ordered[1]![1])[index])) {
    throw new Error(`Path resolves to equal-specificity fixed ${kind} contracts ${JSON.stringify(ordered[0]![0])} and ${JSON.stringify(ordered[1]![0])}.`);
  }
  return [ordered[0]![0]];
}

export function exactContractFilename(contractId: string | null, taxonomy: Taxonomy): string | null {
  if (!contractId) return null;
  const fixed = taxonomy.fixedFilenameContracts[contractId];
  return fixed ? fixedContractFilename(fixed) : taxonomy.configurableEntryContracts[contractId]?.filename ?? null;
}
