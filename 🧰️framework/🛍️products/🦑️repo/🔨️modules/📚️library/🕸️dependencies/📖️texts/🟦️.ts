import { bindCatalogV1, type TextValuesV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of the dependency verification: the refusals of `verify dependencies` in English and German. */
export const DEPENDENCY_TEXTS_V1 = bindCatalogV1("dependencies", catalog);

/** 🔑️ Every message key of this catalog. */
export type DependencyTextKeyV1 = keyof typeof catalog.messages;

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const refuseDependencyV1 = (code: RepoErrorCodeV1, key: DependencyTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(DEPENDENCY_TEXTS_V1, code, key, values, options);
