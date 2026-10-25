import { bindCatalogV1, type TextValuesV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import { sayV1, type ProgressLevelV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📣️progress/🟦️.ts";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of the build library: the refusals of the observation, the command closures, the graph, the Bazel storage and the Nx tools in English and German. */
export const BUILD_TEXTS_V1 = bindCatalogV1("build", catalog);

/** 🔑️ Every message key of this catalog. */
export type BuildTextKeyV1 = keyof typeof catalog.messages;

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const refuseBuildV1 = (code: RepoErrorCodeV1, key: BuildTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(BUILD_TEXTS_V1, code, key, values, options);

/** 📢️ Says one line of this catalog as data; the CLI boundary renders it in the resolved language. */
export const sayBuildV1 = (key: BuildTextKeyV1, values: TextValuesV1 = {}, level: ProgressLevelV1 = "info"): void => sayV1(BUILD_TEXTS_V1, key, values, level);
