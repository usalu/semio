import type { LocalCatalogDiff } from "../🗃️local-catalog-config-mutation/🟦️.ts";
/** 📤️ Authoritative direct TypeScript leaf for unlisting one document from this device's local catalog. */

import type { LocalCatalogConfigMutation, RetireLocalDocument } from "../🗃️local-catalog-config-mutation/🟦️.ts";
import { admitLocalDocument } from "../🗃️local-catalog-config-mutation/🟦️.ts";
import type { LocalCatalog } from "../📥️admit-local-document/🟦️.ts";

//#region 🔖️Mutation
/** 🔺️ Drops the entry for the payload's id; an unlisted id leaves the catalog as it was. */
export function diff(payload: RetireLocalDocument, base: LocalCatalog): LocalCatalogDiff {
  if (!base.documents.some((entry) => entry.documentId === payload.documentId)) return {};
  return { documents: [{ key: payload.documentId }] };
}

/** ↩️ Re-admits the prior entry, or emits no step when the id was not listed. */
export function inverse(payload: RetireLocalDocument, base: LocalCatalog): LocalCatalogConfigMutation[] {
  const prior = base.documents.find((entry) => entry.documentId === payload.documentId);
  return prior ? [admitLocalDocument(prior)] : [];
}
//#endregion 🔖️Mutation
