import { bindCatalogV1, type TextValuesV1 } from "../🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "../🏗️build/🗣️language/🚨️error/🟦️.ts";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of the process module: the refusals of the budgets, the leases, the routing and the process suites in English and German. */
export const PROCESS_TEXTS_V1 = bindCatalogV1("process", catalog);

/** 🔑️ Every message key of this catalog. */
export type ProcessTextKeyV1 = keyof typeof catalog.messages;

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const refuseProcessV1 = (code: RepoErrorCodeV1, key: ProcessTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(PROCESS_TEXTS_V1, code, key, values, options);
