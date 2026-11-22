import {bindCatalogV1,textV1,type TextValuesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {repoErrorV1,type RepoErrorCodeV1,type RepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import texts from "./🔣️.json" with {type:"json"};

/** 📖️ The texts of `repo test-file`: usage, refusals, plan rows, progress lines and verdicts in English and German. */
export const TEST_FILE_TEXTS_V1=bindCatalogV1("test-file",texts);

/** 🔑️ Every message key of this catalogue. */
export type TestFileTextKeyV1=keyof typeof texts.messages;

/** 🌍️ The two languages of the texts. */
export type TestFileLanguageV1="en"|"de";

/** 🔤️ Renders one message in the chosen language. */
export const testFileTextV1=(language:TestFileLanguageV1,key:TestFileTextKeyV1,values:TextValuesV1={}):string=>textV1(TEST_FILE_TEXTS_V1.catalog,key,language,values);

/** 🚨️ Builds one refusal of this catalogue as data; the command renders it in the resolved language. */
export const refuseTestFileV1=(code:RepoErrorCodeV1,key:TestFileTextKeyV1,values:TextValuesV1={}):RepoErrorV1=>repoErrorV1(TEST_FILE_TEXTS_V1,code,key,values);
