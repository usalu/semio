/** 🔗️ The reference diagnostics of the schema contract layer: JSON pointers, `$ref` resolution and the whole schema contract diagnostic of a tree. */
import { join } from "node:path";
import { normativeSchemaFormat, readSchemaCatalog, type SchemaCatalog, type SchemaCatalogScope, schemaDiagnostic, type SchemaDiagnostic } from "../🟦️.ts";
import { readJson, schemaTreeFiles } from "../🏗️scope-eligibility-and-tree/🟦️.ts";
import { inertSchemaDataPaths, schemaFixtureIsolationDiagnostics, schemaMeasurementDisagreementDiagnostics, schemaOwnerEligibilityDiagnostics, schemaPlacementDiagnostics } from "../🩺️placement-diagnostics/🟦️.ts";
import { schemaExportCompletenessDiagnostics, scopeNormativeDocuments } from "../📤️export-completeness/🟦️.ts";

//#region 🧬️SchemaContracts
/** 🔗️ The member a JSON pointer addresses inside one document, or `undefined` when it addresses none. */
function jsonPointerTarget(document: unknown, pointer: string): unknown | undefined {
  if (pointer.length === 0) return document;
  if (!pointer.startsWith("/")) return undefined;
  let node: unknown = document;
  for (const raw of pointer.slice(1).split("/")) {
    const token = raw.replace(/~1/gu, "/").replace(/~0/gu, "~");
    if (Array.isArray(node)) {
      const index = Number(token);
      if (!Number.isInteger(index) || index < 0 || index >= node.length) return undefined;
      node = node[index];
      continue;
    }
    if (node === null || typeof node !== "object" || !Object.hasOwn(node as Record<string, unknown>, token)) return undefined;
    node = (node as Record<string, unknown>)[token];
  }
  return node;
}

/**
 * 🔗️ Judges one `$ref` written in one document of one scope.
 *
 * Three shapes are legal and they are legal for different reasons:
 *
 * 1. **Module-internal** (`#/definitions/<helper>`, `#/$defs/<Export>`, `#`) — a document addressing
 *    its own members. A helper is deliberately NOT an export, so requiring internal refs to be
 *    export-addressed would forbid the one place helpers are allowed to live. Only a pointer that
 *    lands on nothing is a finding here, and it is `schema-ref-broken-internal` — a defect of one
 *    document, owned by that document's partition, never the cross-document addressing refusal.
 * 2. **Cross-document, whole document** (`<target $id>`) — the aggregate-branch form: a mutation
 *    aggregate is a `$ref` union over leaves, and a leaf is its own scope whose entire document IS
 *    its payload contract.
 * 3. **Cross-document, one export** (`<target $id>#/$defs/<ExportId>`).
 *
 * Everything else — a relative file path, a cross-document pointer into another module's
 * `definitions` — is refused: a reference resolved by filesystem position or into a private helper
 * is a dependency nobody declared. (Execution contract §A, ledger row 77.)
 */
function schemaRefDiagnostics(scopeId: string, scope: SchemaCatalogScope, file: string, document: unknown, catalog: SchemaCatalog, scopeOfId: ReadonlyMap<string, string>): SchemaDiagnostic[] {
  const found: SchemaDiagnostic[] = [];
  for (const ref of jsonRefsIn(document)) {
    if (ref.startsWith("#")) {
      if (jsonPointerTarget(document, ref.slice(1)) === undefined) found.push(schemaDiagnostic("schema-ref-broken-internal", `${file} references ${ref}, and this document declares no such member`, { scope: scopeId, path: file }));
      continue;
    }
    const hash = ref.indexOf("#");
    const target = hash < 0 ? ref : ref.slice(0, hash);
    const fragment = hash < 0 ? "" : ref.slice(hash + 1);
    if (!/^https?:\/\//u.test(target)) {
      found.push(schemaDiagnostic("schema-ref-unresolved", `${file} references ${JSON.stringify(ref)}; a cross-document reference names the target module's $id, never a file path`, { scope: scopeId, path: file }));
      continue;
    }
    const targetScope = scopeOfId.get(target);
    if (targetScope === undefined) {
      found.push(schemaDiagnostic("schema-ref-unresolved", `${file} references ${target}, and no catalog scope declares that $id`, { scope: scopeId, path: file }));
      continue;
    }
    const addressed = fragment.length === 0 ? null : fragment.match(/^\/\$defs\/([A-Za-z0-9_]+)$/u);
    if (fragment.length > 0 && addressed === null) {
      found.push(schemaDiagnostic("schema-ref-unresolved", `${file} references ${JSON.stringify(ref)}; a cross-document reference addresses the whole module or one export, as <$id>#/$defs/<ExportId>`, { scope: scopeId, path: file }));
      continue;
    }
    const exported = addressed === null ? null : addressed[1]!;
    if (targetScope === scopeId) continue;
    if (exported !== null && !Object.hasOwn(catalog.scopes[targetScope]!.exports, exported)) {
      found.push(schemaDiagnostic("schema-export-unknown", `${file} references ${exported} from scope ${targetScope}, which does not export it`, { scope: scopeId, export: exported, path: file }));
      continue;
    }
    if (!scope.dependsOn.includes(targetScope)) found.push(schemaDiagnostic("schema-cross-scope-dependency-forbidden", `scope ${scopeId} references ${targetScope}${exported === null ? "" : `/${exported}`} without declaring ${targetScope} in dependsOn`, { scope: scopeId, export: exported, path: file }));
  }
  return found;
}

/**
 * 🔗️ Export resolution over the whole catalog: every `$ref` lands where it says it does, and a
 * cross-scope reference is only legal when the referring scope DECLARED that dependency.
 *
 * Every document of a scope is read, not only its module root: a facet child (`🔺️diff/`,
 * `🧬️mutations/`, …) carries exports of the same scope and writes its own refs.
 */
export function schemaResolutionDiagnostics(repoRoot: string, under = ""): SchemaDiagnostic[] {
  const { catalog, diagnostics } = readSchemaCatalog(repoRoot);
  if (catalog === null) return [...diagnostics];
  const found: SchemaDiagnostic[] = [...diagnostics];
  const selected = (scope: SchemaCatalogScope): boolean => under.length === 0 || scope.path === under || scope.path.startsWith(`${under}/`);
  const normative = normativeSchemaFormat(repoRoot);
  // 🪪️Every normative document of a scope declares its own `$id` — a facet child keeps the scope
  // path and varies only the facet filename — and a reference may address any of them. Indexing the
  // module root alone would report a legal `<facet $id>#/$defs/<Export>` as unresolved.
  const scopeOfId = new Map<string, string>();
  for (const [id, scope] of Object.entries(catalog.scopes)) {
    for (const file of scopeNormativeDocuments(scope, normative)) {
      const declaredId = (readJson(join(repoRoot, file)) as { $id?: unknown } | undefined)?.$id;
      if (typeof declaredId === "string" && !scopeOfId.has(declaredId)) scopeOfId.set(declaredId, id);
    }
  }
  for (const [id, scope] of Object.entries(catalog.scopes)) {
    // 📏️The `$id` index above is built over EVERY scope even under a filter: a subtree's refs
    // point out of it, and an index narrowed to the filter would report every one of them unresolved.
    if (!selected(scope)) continue;
    found.push(...schemaExportCompletenessDiagnostics(repoRoot, id, scope));
    for (const file of scopeNormativeDocuments(scope, normative)) {
      const document = readJson(join(repoRoot, file));
      if (document === undefined) continue;
      found.push(...schemaRefDiagnostics(id, scope, file, document, catalog, scopeOfId));
    }
  }
  return found;
}

/** 🔗️ Every `$ref` string anywhere in a JSON document. */
function jsonRefsIn(document: unknown): string[] {
  const refs: string[] = [];
  const visit = (node: unknown): void => {
    if (Array.isArray(node)) {
      for (const item of node) visit(item);
      return;
    }
    if (node === null || typeof node !== "object") return;
    for (const [key, value] of Object.entries(node)) {
      if (key === "$ref" && typeof value === "string") refs.push(value);
      else visit(value);
    }
  };
  visit(document);
  return refs;
}

/**
 * 🩺️ Every schema-contract invariant, over one subtree or the whole tree. One walk, one ordered list.
 *
 * `under` narrows BOTH measurements, never just the cheap one (ledger row 135): the placement walk is
 * rooted at the subtree and the catalog‑driven resolution half selects the scopes whose module path
 * lies inside it. Skipping resolution under a filter — what this did before — made every per‑partition
 * run report zero `schema-export-*` and `schema-ref-*` rows BY CONSTRUCTION, which reads as "this
 * subtree is clean" to everyone who does not know the gate declined to look.
 */
export function schemaContractDiagnostics(repoRoot: string, under = ""): SchemaDiagnostic[] {
  const files = schemaTreeFiles(repoRoot, under);
  const inert = inertSchemaDataPaths(repoRoot, files);
  return [...schemaPlacementDiagnostics(repoRoot, files, inert), ...schemaOwnerEligibilityDiagnostics(repoRoot, files), ...schemaFixtureIsolationDiagnostics(repoRoot, files, inert), ...schemaMeasurementDisagreementDiagnostics(repoRoot, files, under), ...schemaResolutionDiagnostics(repoRoot, under)];
}
//#endregion 🧬️SchemaContracts
