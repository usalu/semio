import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { inventorySchemaScopes, renderSchemaCatalog, renderSchemaCatalogDocument } from "../../🟦️.ts";
import { loadCatalogTaxonomy } from "../../../🔐️taxonomy-authority/🟦️.ts";
import { repoRootFromHere } from "../../../../../🧪️test/🧭️case-discovery/🟦️.ts";

/**
 * 🧮️ The derived schema catalog and its document are byte-identical to what the generator renders from the schema modules on disk.
 *
 * This is `schema generate --check` as a suite member: a schema edited without regenerating, a module added or removed, or a generator change fails here instead of drifting silently (2844 stale rows once did).
 * The failure names the remedy: `repo ./📜️script.ts schema generate` and `schema docs`.
 */
test("the schema catalog and its document are current", () => {
  const root = repoRootFromHere();
  const resolution = loadCatalogTaxonomy().schemaExportResolution;
  const inventory = inventorySchemaScopes(root);
  const stale = [[resolution.catalogPath, renderSchemaCatalog(inventory.catalog)], [resolution.catalogDocumentPath, renderSchemaCatalogDocument(inventory.catalog)]].filter(([rel, rendered]) => readFileSync(join(root, rel!), "utf8") !== rendered).map(([rel]) => rel);
  expect(stale).toEqual([]);
}, 280_000);
