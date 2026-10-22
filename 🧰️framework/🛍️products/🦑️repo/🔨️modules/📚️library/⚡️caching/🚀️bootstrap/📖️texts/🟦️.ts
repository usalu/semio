import { bindCatalogV1, type TextValuesV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import { sayV1, type ProgressLevelV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📣️progress/🟦️.ts";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of the Nx bootstrap: the progress lines of the owner admission, the watcher, the graph and the generation receipt in English and German. */
export const BOOTSTRAP_TEXTS_V1 = bindCatalogV1("bootstrap", catalog);

/** 🔑️ Every message key of this catalog. */
export type BootstrapTextKeyV1 = keyof typeof catalog.messages;

/** 📢️ Says one line of this catalog as data; the CLI boundary renders it in the resolved language. */
export const sayBootstrapV1 = (key: BootstrapTextKeyV1, values: TextValuesV1 = {}, level: ProgressLevelV1 = "info"): void => sayV1(BOOTSTRAP_TEXTS_V1, key, values, level);

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const refuseBootstrapV1 = (code: RepoErrorCodeV1, key: BootstrapTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(BOOTSTRAP_TEXTS_V1, code, key, values, options);
