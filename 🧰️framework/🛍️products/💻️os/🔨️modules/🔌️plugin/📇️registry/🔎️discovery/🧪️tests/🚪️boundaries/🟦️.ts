import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {parse} from "@iarna/toml";
import Ajv from "ajv";
import ts from "typescript";
import {parseTomlStringArray,requiredRegistryEntryV1,tomlBlocksAfterHeader} from "../../🟦️.ts";
import {parseTomlInlineNumberArray} from "../../../🎮️playground/🔎️discovery/🟦️.ts";
import {declaredComponentKind} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import {goProfileToLcov,parseTsImportSpecs} from "../../../../../../../🦑️repo/🔨️modules/📚️library/🟦️.ts";
import corpus from "./🔣️.json" with {type:"json"};

test("neutral registry canonical metadata arrays and table boundaries agree with independent TOML",()=>{
 for(const row of corpus.arrays){const oracle=parse(row.body)as{aliases:string[];ports:{react:number[]}};expect(parseTomlStringArray(row.body,"aliases")).toEqual(row.strings);expect(oracle.aliases).toEqual(row.strings);expect(parseTomlInlineNumberArray(row.body,"react")).toEqual(row.numbers);expect(oracle.ports.react).toEqual(row.numbers);}
 for(const row of corpus.tables){const source=row.lines.join("\n"),actual=tomlBlocksAfterHeader(row.lines,line=>line==="[[items]]");expect(actual).toEqual(row.blocks);expect<unknown[]>(actual.map(block=>parse(block.join("\n")))).toEqual((parse(source)as{items?:unknown[]}).items??[]);}
 for(const row of corpus.components){const table=parse(row.body)as{package?:{metadata?:{semio?:{"component-kind"?:string}}}};expect<string|null>(declaredComponentKind(row.body)??null).toBe(row.kind);expect(table.package?.metadata?.semio?.["component-kind"]??null).toBe(row.kind);}
 const oracle=new Ajv({strict:true}).compile({type:"object",required:["known"],properties:{known:{type:"string"}},additionalProperties:false});expect(oracle({known:"value"})).toBe(true);expect(requiredRegistryEntryV1({known:"value"},"known")).toBe("value");expect(oracle({})).toBe(false);expect(()=>requiredRegistryEntryV1({},"known")).toThrow("entry required");console.log("[DEBUG] Actual registry/discovery metadata functions preserve closed canonical corpus against independent TOML/Ajv, including EOF and absent taxonomy entries");
});

test("neutral repository import and coverage captures agree with independent syntax and SQL",()=>{
 for(const row of corpus.imports){const source=ts.createSourceFile("fixture.ts",row.source,ts.ScriptTarget.Latest,true),actual:string[]=[];for(const node of source.statements){if((ts.isImportDeclaration(node)||ts.isExportDeclaration(node))&&node.moduleSpecifier&&ts.isStringLiteral(node.moduleSpecifier))actual.push(node.moduleSpecifier.text);}expect(actual).toEqual(row.specs);expect(parseTsImportSpecs(row.source)).toEqual(actual);}
 const database=new Database(":memory:");try{database.exec("CREATE TABLE counts(file TEXT, first INTEGER, last INTEGER, hits INTEGER)");for(const line of corpus.coverage.trim().split("\n").slice(1)){const match=/^(.+):(\d+)\.\d+,(\d+)\.\d+ \d+ (\d+)$/.exec(line);if(!match||match[1]===undefined||match[2]===undefined||match[3]===undefined||match[4]===undefined)throw Error("Authored coverage vector required");database.query("INSERT INTO counts VALUES(?,?,?,?)").run(match[1],Number(match[2]),Number(match[3]),Number(match[4]));}const oracle=database.query("WITH RECURSIVE expanded(file,line,last,hits) AS (SELECT file,first,last,hits FROM counts UNION ALL SELECT file,line+1,last,hits FROM expanded WHERE line<last) SELECT file,line,SUM(hits) AS hits FROM expanded GROUP BY file,line ORDER BY file,line").all()as{file:string;line:number;hits:number}[];const actual=goProfileToLcov(corpus.coverage).flatMap(row=>[...row.lines].map(([line,hits])=>[row.path,line,hits]));expect(actual).toEqual(corpus.coverageRows);expect(actual).toEqual(oracle.map(row=>[row.file,row.line,row.hits]));console.log("[DEBUG] Actual repository import/coverage functions agree with TypeScript syntax and independent SQLite interval aggregation");}finally{database.close();}
});
