import {bindCatalogV1,textV1,type TextValuesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {repoErrorV1,type RepoErrorCodeV1,type RepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import texts from "./🔣️.json" with {type:"json"};

/** 📖️ The texts of `repo land`: usage, refusals, progress lines, gate verdicts and totals in English and German. */
export const LAND_TEXTS_V1=bindCatalogV1("land",texts);

/** 🔑️ Every message key of this catalogue. */
export type LandTextKeyV1=keyof typeof texts.messages;

/** 🌍️ The two languages of the texts. */
export type LandLanguageV1="en"|"de";

/** 🔤️ Renders one message in the chosen language. */
export const landTextV1=(language:LandLanguageV1,key:LandTextKeyV1,values:TextValuesV1={}):string=>textV1(LAND_TEXTS_V1.catalog,key,language,values);

/** 🚨️ Builds one refusal of this catalogue as data; the command renders it in the resolved language. */
export const refuseLandV1=(code:RepoErrorCodeV1,key:LandTextKeyV1,values:TextValuesV1={}):RepoErrorV1=>repoErrorV1(LAND_TEXTS_V1,code,key,values);
