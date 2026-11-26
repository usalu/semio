import {bindCatalogV1,textV1,type TextValuesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {repoErrorV1,type RepoErrorCodeV1,type RepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import texts from "./🔣️.json" with {type:"json"};

/** 📖️ The texts of `repo typecheck`: usage, refusals, progress lines, diagnostics and verdicts in English and German. */
export const TYPECHECK_TEXTS_V1=bindCatalogV1("typecheck",texts);

/** 🔑️ Every message key of this catalogue. */
export type TypecheckTextKeyV1=keyof typeof texts.messages;

/** 🌍️ The two languages of the texts. */
export type TypecheckLanguageV1="en"|"de";

/** 🔤️ Renders one message in the chosen language. */
export const typecheckTextV1=(language:TypecheckLanguageV1,key:TypecheckTextKeyV1,values:TextValuesV1={}):string=>textV1(TYPECHECK_TEXTS_V1.catalog,key,language,values);

/** 🚨️ Builds one refusal of this catalogue as data; the command renders it in the resolved language. */
export const refuseTypecheckV1=(code:RepoErrorCodeV1,key:TypecheckTextKeyV1,values:TextValuesV1={}):RepoErrorV1=>repoErrorV1(TYPECHECK_TEXTS_V1,code,key,values);
