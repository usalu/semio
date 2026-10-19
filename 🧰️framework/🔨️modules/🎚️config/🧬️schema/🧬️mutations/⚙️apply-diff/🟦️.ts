/** ⚙️ TypeScript twin of the central `protocol::apply_diff` for the two OS config facets a shell and its store worker fold by
 * diff: the local document catalog and the identity record. A mutation leaf only NAMES a sparse diff (`diff`) and its concrete
 * inverse (`inverse`); turning a diff into a record happens here and nowhere else, exactly as Rust's `MutationDiff::apply`
 * runs only behind `protocol::apply_diff`. The committed `🔺️diff` fixtures next to each leaf are the language-agnostic
 * specification both implementations are held to.
 * @see ../📥️admit-local-document/🦀️.rs
 * @see ../🪪️sign-in/🦀️.rs */

import type { LocalCatalog } from "../📥️admit-local-document/🟦️.ts";
import type { LocalCatalogDiff, LocalCatalogRow, LocalDocument } from "../🗃️local-catalog-config-mutation/🟦️.ts";
import type { Identity, IdentityDiff } from "../🆔️identity-config-mutation/🟦️.ts";

//#region 🔖️LocalCatalog
const byDocumentId = (left: LocalDocument, right: LocalDocument): number => (left.documentId < right.documentId ? -1 : left.documentId > right.documentId ? 1 : 0);

/** 🎯️ The catalog `diff` produces from `base`: each row replaces or removes its id's entry, in row order, and the catalog stays
 * ordered by id. An empty diff answers `base` itself. */
export function applyLocalCatalogDiff(base: LocalCatalog, diff: LocalCatalogDiff): LocalCatalog {
  const rows = diff.documents ?? [];
  if (rows.length === 0) return base;
  let documents = [...base.documents];
  for (const row of rows) {
    documents = documents.filter((entry) => entry.documentId !== row.key);
    if (row.value !== undefined) documents.push(row.value);
  }
  return { documents: documents.sort(byDocumentId) };
}

/** 🧩️ Folds `other` into `held`: a later absolute row of a key replaces the held one, a new key appends. */
export function absorbLocalCatalogDiff(held: LocalCatalogDiff, other: LocalCatalogDiff): LocalCatalogDiff {
  const rows: LocalCatalogRow[] = [...(held.documents ?? [])];
  for (const row of other.documents ?? []) {
    const at = rows.findIndex((candidate) => candidate.key === row.key);
    if (at < 0) rows.push(row);
    else rows[at] = row;
  }
  return rows.length === 0 ? {} : { documents: rows };
}

/** ↩️ The negative of `diff`: for every row it touches, the entry `base` held (no entry removes the row). Built by reading `base`
 * row by row, never by applying `diff`. */
export function inverseLocalCatalogDiff(diff: LocalCatalogDiff, base: LocalCatalog): LocalCatalogDiff {
  const rows = (diff.documents ?? []).map((row): LocalCatalogRow => {
    const prior = base.documents.find((entry) => entry.documentId === row.key);
    return prior === undefined ? { key: row.key } : { key: row.key, value: prior };
  });
  return rows.length === 0 ? {} : { documents: rows };
}

function decodeLocalDocument(entry: unknown): LocalDocument | undefined {
  const candidate = entry as Partial<LocalDocument> | null;
  if (candidate === null || typeof candidate !== "object" || typeof candidate.documentId !== "string" || typeof candidate.schema !== "string" || typeof candidate.name !== "string" || (candidate.storage !== "folder" && candidate.storage !== "file") || typeof candidate.target !== "string" || typeof candidate.admittedAtMs !== "number") return undefined;
  return { documentId: candidate.documentId, schema: candidate.schema, name: candidate.name, storage: candidate.storage, target: candidate.target, admittedAtMs: candidate.admittedAtMs };
}

/** 🔎️ A catalog carried by a persisted archive's pack value, or `undefined` for anything else. */
export function decodeLocalCatalogValue(payload: unknown): LocalCatalog | undefined {
  if (payload === null || typeof payload !== "object" || !Array.isArray((payload as { documents?: unknown }).documents)) return undefined;
  const documents: LocalDocument[] = [];
  for (const entry of (payload as { documents: unknown[] }).documents) {
    const decoded = decodeLocalDocument(entry);
    if (decoded === undefined) return undefined;
    documents.push(decoded);
  }
  return { documents };
}

/** 🔎️ A catalog diff carried by an envelope's `diff.payload`, or `undefined` for anything else (a malformed row refuses the whole diff). */
export function decodeLocalCatalogDiff(payload: unknown): LocalCatalogDiff | undefined {
  if (payload === null || typeof payload !== "object" || Array.isArray(payload)) return undefined;
  const documents = (payload as { documents?: unknown }).documents;
  if (documents === undefined) return {};
  if (!Array.isArray(documents)) return undefined;
  const rows: LocalCatalogRow[] = [];
  for (const entry of documents) {
    const row = entry as { key?: unknown; value?: unknown } | null;
    if (row === null || typeof row !== "object" || typeof row.key !== "string") return undefined;
    if (row.value === undefined || row.value === null) {
      rows.push({ key: row.key });
      continue;
    }
    const value = decodeLocalDocument(row.value);
    if (value === undefined || value.documentId !== row.key) return undefined;
    rows.push({ key: row.key, value });
  }
  return rows.length === 0 ? {} : { documents: rows };
}
//#endregion 🔖️LocalCatalog

//#region 🔖️Identity
/** 🎯️ The identity record `diff` produces from `base`: its session replaces the held one (no `value` signs out). */
export function applyIdentityDiff(base: Identity | null, diff: IdentityDiff): Identity | null {
  return diff.session === undefined ? base : (diff.session.value ?? null);
}

/** 🧩️ Folds `other` into `held`: a later absolute session wins. */
export function absorbIdentityDiff(held: IdentityDiff, other: IdentityDiff): IdentityDiff {
  return other.session === undefined ? held : { session: other.session };
}

/** ↩️ The negative of `diff`: the session `base` held. Built by reading `base`, never by applying `diff`. */
export function inverseIdentityDiff(diff: IdentityDiff, base: Identity | null): IdentityDiff {
  return diff.session === undefined ? {} : { session: base === null ? {} : { value: base } };
}

function decodeIdentity(value: unknown): Identity | undefined {
  const candidate = value as Partial<Identity> | null;
  if (candidate === null || typeof candidate !== "object" || typeof candidate.userId !== "string" || typeof candidate.email !== "string" || typeof candidate.displayName !== "string" || typeof candidate.hubBaseUrl !== "string" || typeof candidate.issuedAtMs !== "number") return undefined;
  return { userId: candidate.userId, email: candidate.email, displayName: candidate.displayName, hubBaseUrl: candidate.hubBaseUrl, issuedAtMs: candidate.issuedAtMs };
}

/** 🔎️ The identity a persisted archive's pack value holds: a record, `null` for a signed-out session, `undefined` for anything else. */
export function decodeIdentityValue(payload: unknown): Identity | null | undefined {
  return payload === null ? null : decodeIdentity(payload);
}

/** 🔎️ An identity diff carried by an envelope's `diff.payload`, or `undefined` for anything else. */
export function decodeIdentityDiff(payload: unknown): IdentityDiff | undefined {
  if (payload === null || typeof payload !== "object" || Array.isArray(payload)) return undefined;
  const session = (payload as { session?: unknown }).session;
  if (session === undefined) return {};
  if (session === null || typeof session !== "object" || Array.isArray(session)) return undefined;
  const value = (session as { value?: unknown }).value;
  if (value === undefined || value === null) return { session: {} };
  const identity = decodeIdentity(value);
  return identity === undefined ? undefined : { session: { value: identity } };
}
//#endregion 🔖️Identity
