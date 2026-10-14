import {expect,test} from "bun:test";
import {existsSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {parse as parseJson} from "jsonc-parser";
import ts from "typescript";
import bindings from "../\u{1f9eb}\uFE0Ffixtures/\u{1f9eb}\uFE0Fcad-draw-live-bindings/\u{1f523}\uFE0F.json";
test("CAD whole golden binding wrapper has no production schema or whole-example admission",()=>{expect(existsSync(join(import.meta.dir,"../../\u{1f9ec}\uFE0Fschema/\u{1f523}\uFE0Fcad-draw-live-bindings/\u{1f523}\uFE0F.json"))).toBe(false);const source=readFileSync(join(import.meta.dir,"../\u{1f52c}\uFE0Fworkspace-contract/\u{1f7e6}\uFE0F.ts"),"utf8"),ast=ts.createSourceFile("original.ts",source,ts.ScriptTarget.Latest,true),names:string[]=[];function visit(node:ts.Node){if(ts.isIdentifier(node)&&node.text==="CAD_LIVE_BINDINGS_SCHEMA")names.push(node.text);ts.forEachChild(node,visit);}visit(ast);expect(names).toEqual([]);console.log("[DEBUG] CAD fixed golden binding schema and whole-fixture admission are absent");});
test("original CAD binding examples remain plain independently parsed inputs",()=>{const text=readFileSync(join(import.meta.dir,"../\u{1f9eb}\uFE0Ffixtures/\u{1f9eb}\uFE0Fcad-draw-live-bindings/\u{1f523}\uFE0F.json"),"utf8");expect(parseJson(text)).toEqual(bindings);expect(new Set(bindings.bindings.map(row=>row.live)).size).toBe(bindings.bindings.length);console.log("[DEBUG] Original CAD carrier examples retain unique plain paths");});
