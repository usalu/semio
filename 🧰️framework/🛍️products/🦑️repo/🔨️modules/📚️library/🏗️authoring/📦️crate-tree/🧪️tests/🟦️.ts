/** 🏗️ Contract: the crate-tree facade lists the forwardable features of a manifest, renders and forwards them to carved crates, and adds dev-dependency rows (including the shared fixture reader) in sorted place without duplicating a known dependency; the corpus is language-agnostic and `@iarna/toml` is the independent third-party oracle for the feature table and every edited manifest. */
import {expect,setDefaultTimeout,test} from "bun:test";
import Ajv from "ajv";
import {parse} from "@iarna/toml";
import {resolve} from "node:path";
import {FIXTURE_READER_CRATE,FIXTURE_READER_PACKAGE_DIR} from "../🧪️test-closure/🟦️.ts";
import {manifestRowKey} from "../📝️render/🟦️.ts";
import {featureRow,forwardFeatures,forwardableFeatures,insertDevDependencyRow,withDevDependencyRows,withFixtureReaderDevDependency} from "../🟦️.ts";
import corpus from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

setDefaultTimeout(120000);
const platform=(path:string):string=>process.platform==="win32"?"C:"+path.split("/").join("\\"):path;
test("the features of a manifest are the array-valued ones in declaration order (@iarna/toml agrees)",()=>{
 expect(forwardableFeatures(corpus.features.manifest)).toEqual(corpus.features.expected);
 const document=parse(corpus.features.manifest) as any;expect(Object.entries<any>(document.features).filter(([,value])=>Array.isArray(value)).map(([name,value])=>({name,entries:value.map(String)}))).toEqual(corpus.features.expected);
});
for(const row of corpus.features.rows)test("feature row of "+row.feature.name,()=>{expect(featureRow(row.feature,row.forward)).toBe(row.expected);});
test("forwarded features keep their original entries and add the carved crates' feature (every other line untouched)",()=>{
 const features=forwardableFeatures(corpus.features.manifest),text=forwardFeatures(corpus.features.manifest,features,corpus.features.forwardTo);expect(text).toBe(corpus.features.forwarded);
 const document=parse(text) as any;for(const feature of features){expect(document.features[feature.name]).toEqual([...feature.entries,...corpus.features.forwardTo.map(crate=>crate+"/"+feature.name)]);}
 expect(document.features.not_array).toBe("x");expect(document.dependencies).toEqual((parse(corpus.features.manifest) as any).dependencies);expect(forwardFeatures(corpus.features.manifest,[],["x"])).toBe(corpus.features.manifest);
});
for(const row of corpus.dev)test("dev-dependency rows: "+row.name,()=>{expect(withDevDependencyRows(row.manifest,row.rows)).toBe(row.expected);});
for(const row of corpus.insert)test("insert dev-dependency row: "+row.name,()=>{const text=insertDevDependencyRow(row.manifest,row.row);expect(text).toBe(row.expected);const document=parse(text) as any;expect(Object.keys(document["dev-dependencies"]??{})).toContain(manifestRowKey(row.row));});
test("the fixture reader dev-dependency points from the manifest directory to the shared package",()=>{
 const text=withFixtureReaderDevDependency(corpus.fixture.manifest,resolve(platform(corpus.fixture.directory)),resolve(platform(corpus.fixture.root))),document=parse(text) as any,dependency=document["dev-dependencies"][FIXTURE_READER_CRATE];
 expect(dependency.path.startsWith("../../")).toBe(true);expect(resolve(platform(corpus.fixture.directory),dependency.path)).toBe(resolve(platform(corpus.fixture.root),FIXTURE_READER_PACKAGE_DIR));expect(withFixtureReaderDevDependency(text,resolve(platform(corpus.fixture.directory)),resolve(platform(corpus.fixture.root)))).toBe(text);
});
test("the corpus is closed under its schema",()=>{const ajv=new Ajv({strict:true});expect(ajv.validate(schema,corpus),JSON.stringify(ajv.errors)).toBe(true);});
