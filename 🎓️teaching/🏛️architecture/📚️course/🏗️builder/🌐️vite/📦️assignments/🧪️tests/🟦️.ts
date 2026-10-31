import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import ts from"typescript";
import{readFileSync}from"node:fs";
import{resolve}from"node:path";
import{declareBuildTestEnvelopeV1,withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import{createIssuedResourceControlV1,createRequestedIssuedResourceControlV1,requireIssuedResourceControlRefinementV1}from"@semio-tech/process/routing";
import type{ResourceLeaseControl}from"@semio-tech/process/leases";
import corpus from"./🔣️.json"with{type:"json"};
import schema from"./📋️schema.json"with{type:"json"};
declareBuildTestEnvelopeV1({owner:"test:assignment-pdf-control",maximumElapsedMilliseconds:30000,capabilities:{}});
test("retained PDF control lineage agrees with recursive SQLite and refuses sibling compiler entry",()=>withBuildTestInvocationV1(async original=>{
 expect(new Ajv({strict:true,allowUnionTypes:true}).compile(schema)(corpus)).toBe(true);
 const db=new Database(":memory:"),controls=new Map<string,ResourceLeaseControl>();try{
 db.exec("create table controls(name text primary key,parent text)");
 for(const row of corpus.controls){db.query("insert into controls values(?,?)").run(row.name,row.parent);const parent=row.parent===null?undefined:controls.get(row.parent);controls.set(row.name,parent?createRequestedIssuedResourceControlV1(parent,0,new AbortController().signal):createIssuedResourceControlV1(original,"pdf:"+row.name,0));}
 for(const row of corpus.cases){const expected=db.query<{accepted:number},[string,string]>("with recursive ancestors(name) as (select ? union all select controls.parent from controls join ancestors on controls.name=ancestors.name where controls.parent is not null) select exists(select 1 from ancestors where name=?) accepted").get(row.consumer,row.parent)!.accepted===1;expect(expected,row.name).toBe(row.accepted);let accepted=true;try{requireIssuedResourceControlRefinementV1(controls.get(row.parent)!,controls.get(row.consumer)!);}catch{accepted=false;}expect(accepted,row.name).toBe(expected);}
 }finally{db.close();}
}));
test("actual PDF retention passes its last artifact phase to all real compiler consumers",()=>{
 const root=resolve(process.cwd(),"🎓️teaching/🏛️architecture/📚️course"),read=(path:string)=>{const filename=resolve(root,path);return ts.createSourceFile(filename,readFileSync(filename,"utf8"),ts.ScriptTarget.Latest,true);},owner=read("🏗️builder/🌐️vite/📦️assignments/🟦️.ts");
 const calls:ts.CallExpression[]=[];function visit(node:ts.Node):void{if(ts.isCallExpression(node))calls.push(node);ts.forEachChild(node,visit);}visit(owner);
 const body=calls.find(call=>call.expression.getText(owner)==="body");expect(body?.arguments).toHaveLength(2);expect(body?.arguments[1]?.getText(owner)).toBe("execution.phaseControl");
 const recursion=calls.find(call=>call.expression.getText(owner)==="retain"&&call.arguments[1]?.getText(owner)==="index + 1");expect(recursion?.arguments[0]?.getText(owner)).toBe("{ ...execution, phaseControl: artifact.control }");
 const paths:Readonly<Record<string,string>>={"dev-site":"📦️packages/🟦️typescript/📜️script.ts","build":"🚀️deploy/🌐️site/🟦️.ts","test-e2e":"🎭️e2e/🟦️.ts"};for(const consumer of corpus.consumers){const file=read(paths[consumer]!),callbacks:ts.ArrowFunction[]=[];function find(node:ts.Node):void{if(ts.isCallExpression(node)&&node.expression.getText(file)==="withCourseAssignmentPdfsV1"){const callback=node.arguments[2];if(callback&&ts.isArrowFunction(callback))callbacks.push(callback);}ts.forEachChild(node,find);}find(file);expect(callbacks).toHaveLength(1);expect(callbacks[0]!.parameters.map(parameter=>parameter.name.getText(file))).toEqual(["runtimeChannels","control"]);expect(callbacks[0]!.body.getText(file)).not.toContain("scriptCompilerAdmissionControlV1");expect(callbacks[0]!.body.getText(file)).toContain("control");}
});

test("every declared site consumer depends on both actual published language PDF producers",()=>{const root=resolve(process.cwd(),"🎓️teaching/🏛️architecture/📚️course"),project=JSON.parse(readFileSync(resolve(root,"📦️packages/🟦️typescript/📋️project.json"),"utf8")),db=new Database(":memory:");try{for(const target of corpus.requiredTargets){const dependencies=project.targets[target].dependsOn;for(const producer of corpus.producers){expect(dependencies,target).toContain(producer);expect(db.query<{present:number},[string,string]>("select exists(select 1 from json_each(?) where value=?) present").get(JSON.stringify(dependencies),producer)?.present).toBe(1);}}}finally{db.close();}});
