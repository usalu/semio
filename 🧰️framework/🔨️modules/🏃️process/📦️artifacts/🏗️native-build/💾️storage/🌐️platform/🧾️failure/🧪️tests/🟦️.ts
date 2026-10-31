import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import ts from"typescript";
import{nativeRecoveredChildRetirementFailureV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import{evaluateExtractedFunctionV1}from"../../../../../../🧪️testing/🧩️extracted-function/🟦️.ts";

test("recovered child refusal preserves its primary error and names every exact guard predicate",()=>{
 const validate=new Ajv({strict:true}).compile(schema),db=new Database(":memory:");
 try{for(const row of fixture.cases){const error=nativeRecoveredChildRetirementFailureV1({...row,allocationId:"a".repeat(32)}),data=error.cause as any;expect(error).toBeInstanceOf(Error);expect(error.message).toBe("Actual recovered committed child required");expect(validate(data)).toBe(true);
 const reference=db.query("select ?1 as child,?2='committed' as committed,?3='compiler-state' as compilerState,?4=?6 as backingRoot,?5=?6 as policyRoot").get(row.child?1:0,row.phase,row.kind,row.backingRoot,row.policyRoot,row.parentRoot)as Record<string,number>;expect(data.checks).toEqual(Object.fromEntries(Object.entries(reference).map(([key,value])=>[key,Boolean(value)])));expect(Object.entries(data.checks).filter(([,value])=>!value).map(([key])=>key).sort()).toEqual(row.failed.slice().sort());expect(data.allocationId).toBe("a".repeat(32));expect(data.truncated).toEqual([]);}
 const tooLong=nativeRecoveredChildRetirementFailureV1({...fixture.cases[0],allocationId:"a".repeat(1025)}).cause as any;expect(validate(tooLong)).toBe(true);expect(tooLong.allocationId.length).toBe(1024);expect(tooLong.truncated).toEqual(["allocationId"]);
 const text=readFileSync(resolve(import.meta.dir,"../../🟦️.ts"),"utf8"),tree=ts.createSourceFile("platform.ts",text,ts.ScriptTarget.Latest,true),fn=tree.statements.find((node):node is ts.FunctionDeclaration=>ts.isFunctionDeclaration(node)&&node.name?.text==="retireNativePlatformFundedChildStorageV1")!,guard=fn.body!.statements.find(node=>ts.isIfStatement(node)&&node.expression.getText(tree).includes("reservation.allocation.phase"))as ts.IfStatement;expect(guard.thenStatement.getText(tree)).toContain("nativeRecoveredChildRetirementFailureV1");const decide=evaluateExtractedFunctionV1("function refused(child,reservation,parent,policy,existsSync){return "+guard.expression.getText(tree)+";}","refused",{});for(const row of fixture.guardCases){const actual=decide(row.child?{directory:"/child"}:undefined,{allocation:{phase:row.phase,kind:row.kind,namespace:row.namespace},backing:{root:row.backingRoot}},{root:row.parentRoot},{root:row.policyRoot},()=>row.directoryPresent);const independent=db.query("select not ?1 or (?2!='committed' and not (?2='retiring' and ?3 and not ?4)) or ?5!='compiler-state' or ?6!=?8 or ?7!=?8 refused").get(Number(row.child),row.phase,Number(row.namespace),Number(row.directoryPresent),row.kind,row.backingRoot,row.policyRoot,row.parentRoot)as{refused:number};expect(actual,row.id).toBe(Boolean(independent.refused));expect(actual,row.id).toBe(row.refused);}
 }finally{db.close();}
});
