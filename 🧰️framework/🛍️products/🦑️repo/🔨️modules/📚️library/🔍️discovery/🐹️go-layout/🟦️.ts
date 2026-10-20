import { readTaxonomyUnchecked } from "../🧭️workspace-taxonomy-locator/🟦️.ts";
import { taxonomySingleFileKindFilenameV1 } from "../📄️file-kind-query/🟦️.ts";
import { compileJsonSchemaSubset } from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};

export type CanonicalGoLayout=Readonly<{testsDirectory:string;implementationFilename:string;packagesDirectory:string;languageDirectory:string;opaqueDirectoryNames:readonly string[];sourceDirectoryNames:readonly string[]}>;
const protocol=schema.$defs.Protocol.const,inputValidator=compileJsonSchemaSubset(schema.$defs.Input),layoutValidator=compileJsonSchemaSubset(schema.$defs.Layout);
const record=(value:unknown):Record<string,unknown>=>typeof value==="object"&&value!==null&&!Array.isArray(value)?value as Record<string,unknown>:{};

/** 🐹️Reads the authored Go language directory from its closed compiler-layout protocol. */
export function canonicalGoLanguageDirectoryV1():string{return protocol.languageDirectory;}

/** 🧮️Projects only Go compiler-layout data; source basenames retain their authored order and strict uniqueness. */
export function canonicalGoLayoutFromTaxonomyV1(document:unknown):CanonicalGoLayout{
 const taxonomy=record(document),kindId=record(taxonomy.testAdapterFileKinds)[protocol.languageDirectory],kind=typeof kindId==="string"?record(taxonomy.fileKinds)[kindId]:undefined,source=record(record(taxonomy.semanticDirectoryMemberKinds)[protocol.sourceDirectoryKind]).memberNames;
 const input={testsDirectory:taxonomy.testsDirName,fixturesDirectory:taxonomy.testFixturesDirName,oraclesDirectory:taxonomy.testOraclesDirName,packagesDirectory:taxonomy.packagesDirName,fileKind:kind,sourceDirectoryNames:source};
 if(inputValidator.validate(input).length)throw Error("Invalid authored Go compiler-layout data");
 const data=input as {testsDirectory:string;fixturesDirectory:string;oraclesDirectory:string;packagesDirectory:string;fileKind:{emoji:string;extensionChains:string[]};sourceDirectoryNames:string[]},opaque=[data.fixturesDirectory,data.oraclesDirectory,data.packagesDirectory],names=[...data.sourceDirectoryNames],layout={testsDirectory:data.testsDirectory,implementationFilename:taxonomySingleFileKindFilenameV1(kindId as string,{fileKinds:{[kindId as string]:data.fileKind}}),packagesDirectory:data.packagesDirectory,languageDirectory:protocol.languageDirectory,opaqueDirectoryNames:opaque,sourceDirectoryNames:names};
 if(layoutValidator.validate(layout).length||opaque.includes(layout.testsDirectory)||names.includes(layout.testsDirectory)||names.some(name=>opaque.includes(name)))throw Error("Invalid canonical Go compiler-input layout");
 return Object.freeze({...layout,opaqueDirectoryNames:Object.freeze(opaque),sourceDirectoryNames:Object.freeze(names)});
}

/** 📖️Uses the workspace's existing no-follow taxonomy authority for a fresh Go layout query. */
export function readCanonicalGoLayoutV1():CanonicalGoLayout{return canonicalGoLayoutFromTaxonomyV1(readTaxonomyUnchecked());}
