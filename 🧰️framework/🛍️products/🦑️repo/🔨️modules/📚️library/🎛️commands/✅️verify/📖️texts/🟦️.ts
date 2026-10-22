import { bindCatalogV1, type TextValuesV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import { repoErrorV1, type RepoErrorCodeV1, type RepoErrorV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import { sayV1, type ProgressLevelV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📣️progress/🟦️.ts";
import catalog from "./🔣️.json" with { type: "json" };

/** 📖️ The texts of `repo verify`: every progress line, verdict and refusal of the lanes in English and German. */
export const VERIFY_TEXTS_V1 = bindCatalogV1("verify", catalog);

/** 🔑️ Every message key of this catalog. */
export type VerifyTextKeyV1 = keyof typeof catalog.messages;

/** 📢️ Says one line of this catalog as data; the CLI boundary renders it in the resolved language. */
export const sayVerifyV1 = (key: VerifyTextKeyV1, values: TextValuesV1 = {}, level: ProgressLevelV1 = "info"): void => sayV1(VERIFY_TEXTS_V1, key, values, level);

/** 🚨️ Builds a refusal of this catalog as data; the CLI boundary renders it in the resolved language. */
export const refuseVerifyV1 = (code: RepoErrorCodeV1, key: VerifyTextKeyV1, values: TextValuesV1 = {}, options?: ErrorOptions): RepoErrorV1 => repoErrorV1(VERIFY_TEXTS_V1, code, key, values, options);
