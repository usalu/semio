export interface SemanticDescendantKindNode {
  readonly pathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly nodeType: "directory" | "file";
  readonly kindId: string;
  readonly sourceFilename?: string;
}

interface SemanticDescendantFixedFileNode {
  readonly pathSegments: readonly Readonly<{ kindId: string; literal: string }>[];
  readonly nodeType: "file";
  readonly fixedFilenameContractId: string;
}

export type SemanticDescendantNode = SemanticDescendantKindNode | SemanticDescendantFixedFileNode;

interface SemanticDescendantAlternative {
  readonly id: string;
  readonly mode: "exactly-one";
  readonly nodes: readonly SemanticDescendantNode[];
}

export interface SemanticExactDescendantContract {
  readonly rootDirectoryKindId: string;
  readonly requiredNodes: readonly SemanticDescendantNode[];
  readonly exclusiveAlternatives: readonly SemanticDescendantAlternative[];
  readonly realizedNodeCount: number;
  readonly pathBudgetReserve: Readonly<{ derivation: "longest-canonical-descendant-suffix"; utf16Units: number }>;
}

interface SemanticCatalogDescendantContract {
  readonly contractKind: "catalog";
  readonly rootDirectoryKindId: string;
  readonly catalogContractId: string;
  readonly leafFileKindId: string;
  readonly rendering: "semantic-member-directory-and-physical-kind-leaf";
  readonly pathBudgetReserve: Readonly<{ derivation: "longest-rendered-catalog-descendant-suffix"; utf16Units: number }>;
}

export type SemanticDescendantContract = SemanticExactDescendantContract | SemanticCatalogDescendantContract;
