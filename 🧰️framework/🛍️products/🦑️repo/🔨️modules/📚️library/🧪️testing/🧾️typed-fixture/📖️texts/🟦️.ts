import {bindCatalogV1,textV1,type TextValuesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {repoErrorV1,type RepoErrorCodeV1,type RepoErrorV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/🚨️error/🟦️.ts";
import texts from "./🔣️.json" with {type:"json"};

/** 📖️ The texts of the typed fixture loader and the typed stubs: refusals in English and German. */
export const TYPED_FIXTURE_TEXTS_V1=bindCatalogV1("typed-fixture",texts);

/** 🔑️ Every message key of this catalogue. */
export type TypedFixtureTextKeyV1=keyof typeof texts.messages;

/** 🔤️ Renders one message in the chosen language. */
export const typedFixtureTextV1=(language:"en"|"de",key:TypedFixtureTextKeyV1,values:TextValuesV1={}):string=>textV1(TYPED_FIXTURE_TEXTS_V1.catalog,key,language,values);

/** 🚨️ Builds one refusal of this catalogue as data; the boundary renders it in the resolved language. */
export const refuseTypedFixtureV1=(code:RepoErrorCodeV1,key:TypedFixtureTextKeyV1,values:TextValuesV1={}):RepoErrorV1=>repoErrorV1(TYPED_FIXTURE_TEXTS_V1,code,key,values);
