import { bindCatalogV1, type TextValuesV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of the build caching: the refusals of the cache layout, the native orchestration, the cache reset, the Nx daemon service and the wasm-bindgen lock in English and German. */
export const CACHING_TEXTS_V1 = bindCatalogV1("caching", catalog);

/** 🔑️ Every message key of this catalog. */
export type CachingTextKeyV1 = keyof typeof catalog.messages;

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const refuseCachingV1 = (code: RepoErrorCodeV1, key: CachingTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(CACHING_TEXTS_V1, code, key, values, options);
