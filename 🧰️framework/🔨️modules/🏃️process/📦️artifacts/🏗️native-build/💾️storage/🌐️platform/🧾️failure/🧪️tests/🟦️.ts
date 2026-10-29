import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import ts from"typescript";
import{nativeRecoveredChildRetirementFailureV1}from"../🟦️.ts";
import schema from"../🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import corpusSchema from"./📋️schema.json";

test("recovered child refusal preserves its primary error and names every exact guard predicate",()=>{
 const ajv=new Ajv({strict:true});expect(ajv.validate(corpusSchema,fixture)).toBe(true);const validate=ajv.compile(schema),db=new Database(":memory:");
 try{for(const row of fixture.cases){const error=nativeRecoveredChildRetirementFailureV1({...row,allocationId:"a".repeat(32)}),data=error.cause as any;expect(error).toBeInstanceOf(Error);expect(error.message).toBe("Actual recovered committed child required");expect(validate(data)).toBe(true);
 const reference=db.query("select ?1 as child,?2='committed' as committed,?3='compiler-state' as compilerState,?4=?6 as backingRoot,?5=?6 as policyRoot").get(row.child?1:0,row.phase,row.kind,row.backingRoot,row.policyRoot,row.parentRoot)as Record<string,number>;expect(data.checks).toEqual(Object.fromEntries(Object.entries(reference).map(([key,value])=>[key,Boolean(value)])));expect(Object.entries(data.checks).filter(([,value])=>!value).map(([key])=>key).sort()).toEqual(row.failed.slice().sort());expect(data.allocationId).toBe("a".repeat(32));expect(data.truncated).toEqual([]);}
 const tooLong=nativeRecoveredChildRetirementFailureV1({...fixture.cases[0],allocationId:"a".repeat(1025)}).cause as any;expect(validate(tooLong)).toBe(true);expect(tooLong.allocationId.length).toBe(1024);expect(tooLong.truncated).toEqual(["allocationId"]);
 const text=readFileSync(resolve(import.meta.dir,"../../🟦️.ts"),"utf8"),tree=ts.createSourceFile("platform.ts",text,ts.ScriptTarget.Latest,true),fn=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="retireNativePlatformFundedChildStorageV1")!,guard=fn.body!.statements.find(node=>ts.isIfStatement(node)&&node.expression.getText(tree).includes("reservation.allocation.phase"))as ts.IfStatement;expect(guard.expression.getText(tree)).toBe(fixture.guard);expect(guard.thenStatement.getText(tree)).toContain("nativeRecoveredChildRetirementFailureV1");
 }finally{db.close();}
});
