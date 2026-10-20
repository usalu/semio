import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{mkdtemp,mkdir,rename,rm}from"node:fs/promises";
import{join,resolve}from"node:path";
import{lstatSync}from"node:fs";
import*as mount from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🔣️.json";

test("observation tolerates boot scoped device reassignment without publishing authority",()=>{const api=mount as unknown as Record<string,unknown>,bind=api.bindPublishedNativeImageMountDirectoryV1;expect(typeof bind).toBe("function");const ajv=new Ajv({strict:true}).addSchema(schema),validCase=ajv.compile({$ref:schema.$id+"#/definitions/BindingCase"}),validBinding=ajv.compile({$ref:schema.$id+"#/definitions/PublishedBinding"}),db=new Database(":memory:");try{for(const row of corpus.cases){expect(Boolean(validCase(row))).toBe(true);const input=row.input as {published:{dev:string;ino:string;mode:number};publishedBackingDevice:string;currentBackingDevice:string},oracle=validBinding(input)?(db.query("SELECT CASE WHEN :published=:backing THEN json_set(:directory,'$.dev',:current) ELSE NULL END AS result").get({":published":input.published.dev,":backing":input.publishedBackingDevice,":directory":JSON.stringify(input.published),":current":input.currentBackingDevice})as{result:string|null}).result:null;expect(oracle===null?null:JSON.parse(oracle)).toEqual(row.expected);let actual:unknown=null;try{actual=(bind as Function)(input);}catch{}expect(actual).toEqual(row.expected);}}finally{db.close();}});

test("observation tolerates published device changes while live inode replacement still refuses",async()=>{const api=mount as unknown as Record<string,unknown>,bind=api.bindPublishedNativeImageMountDirectoryV1;expect(typeof bind).toBe("function");const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Original test artifact directory required");await mkdir(base,{recursive:true});const area=await mkdtemp(join(resolve(base),"mount-binding-")),directory=join(area,"m"),previous=join(area,"previous");try{await mkdir(directory);const facts=lstatSync(directory,{bigint:true}),published={dev:String(facts.dev+1n),ino:String(facts.ino),mode:Number(facts.mode)},bound=(bind as Function)({published,publishedBackingDevice:published.dev,currentBackingDevice:String(facts.dev)});expect(Object.isFrozen(bound)).toBe(true);mount.requireNativeImageMountDirectoryV1(directory,bound);await rename(directory,previous);await mkdir(directory);expect(()=>mount.requireNativeImageMountDirectoryV1(directory,bound)).toThrow("changed");expect(published.dev).toBe(String(facts.dev+1n));}finally{await rm(area,{recursive:true,force:true});}});

import "../../../🥾️boot/🧪️tests/🟦️.ts";

import "./📥️inspection.ts";

import "./♻️retirement.ts";
