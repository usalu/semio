import {test,expect} from "bun:test";
import ts from "typescript";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {readFileSync} from "node:fs";
import {join,resolve} from "node:path";
import {requireMaterializedPathV1} from "../../../../../../🔒️leases/🟦️.ts";
import {nativeImageRootV1} from "../../../🌐️platform/📋️contract/🟦️.ts";
import schemaComponent from"../../🧬️schema/🔣️.json";const schema=schemaComponent.definitions.MaterializedPaths;
import corpus from "./🔣️.json";
test("persistent Native generic entry refuses every noncanonical or overlong materialized path before its first lease",async()=>{
 const text=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("fixed.ts",text,ts.ScriptTarget.Latest,true),node=tree.statements.find(row=>ts.isFunctionDeclaration(row)&&row.name?.text==="openNativeFixedPlatformV1");if(!node)throw Error("Actual fixed opener required");const source=ts.transpileModule(node.getText(tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ESNext,module:ts.ModuleKind.None}}).outputText,validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 try{for(const row of corpus.cases){expect(validate(row)).toBe(true);const directory="/"+"d".repeat(row.directoryLength-1)+(row.canonicalDirectory?"":"/../owned"),leaseDirectory="/"+"l".repeat(row.leaseLength-1)+(row.canonicalLease?"":"/../leases"),sentinel=Error("first lease boundary");let acquisitions=0;const bindings={Object,join,requireNativeImageEnvelopeV1:()=>{},checkResourceLeaseControl:()=>{},requireMaterializedPathV1,nativeImageRootV1,fixedPlatformV1:()=>({filename:row.filename}),fixedReceivers:new Set(),fixedReceiverGuard:async()=>{acquisitions++;throw sentinel;},unresolvedFixedPreparations:new Set()};const open=new Function(...Object.keys(bindings),source+";return openNativeFixedPlatformV1;")(...Object.values(bindings));let refused=false;try{await open({directory,leaseDirectory,control:{}});}catch{refused=true;}expect(refused).toBe(true);const lengths=[directory,leaseDirectory,join(directory,row.filename),join(directory,"image.json"),join(directory,"m")].map(path=>path.length),oracle=db.query("SELECT (?1<=256 AND ?2<=256 AND ?3<=256 AND ?4<=256 AND ?5<=256 AND ?6 AND ?7) AS admitted").get(...lengths,Number(directory===resolve(directory)),Number(leaseDirectory===resolve(leaseDirectory)))as {admitted:number};expect(oracle.admitted).toBe(row.expectedAcquisitions);expect(acquisitions).toBe(oracle.admitted);}}
 finally{db.close();}
});
