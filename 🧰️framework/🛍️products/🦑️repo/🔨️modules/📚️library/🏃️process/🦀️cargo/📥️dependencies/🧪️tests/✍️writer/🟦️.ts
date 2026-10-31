import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {readFileSync} from "node:fs";
import {fileURLToPath} from "node:url";
import ts from "typescript";
import fixture from "./🔣️.json" with {type:"json"};

const source=(relative:string):ts.SourceFile=>{const path=fileURLToPath(new URL(relative,import.meta.url));return ts.createSourceFile(path,readFileSync(path,"utf8"),ts.ScriptTarget.Latest,true);};
const fn=(file:ts.SourceFile,name:string):ts.FunctionDeclaration=>{const node=file.statements.find((row):row is ts.FunctionDeclaration=>ts.isFunctionDeclaration(row)&&row.name?.text===name);if(!node)throw Error("missing "+name);return node;};
const calls=(node:ts.Node,name:string):ts.CallExpression[]=>{const out:ts.CallExpression[]=[];const visit=(child:ts.Node):void=>{if(ts.isCallExpression(child)){const callee=child.expression;if(ts.isIdentifier(callee)&&callee.text===name||ts.isPropertyAccessExpression(callee)&&callee.name.text===name)out.push(child);}ts.forEachChild(child,visit);};visit(node);return out;};
const within=(inner:ts.Node,outer:ts.Node):boolean=>inner.getStart()>=outer.getStart()&&inner.getEnd()<=outer.getEnd();

test("the repository compiler-input port offers the shared compiler writer instead of holding it, and the native artifact build takes its recipe lease before entering it and publishes after leaving it, identically in the independent text oracle",()=>{
 const port=fn(source(fixture.port.file),fixture.port.function),consumerFile=source(fixture.consumer.file),build=fn(consumerFile,fixture.consumer.build),capture=fn(consumerFile,fixture.consumer.capture);
 const admissions=calls(port,fixture.port.stepAdmission),writers=calls(port,fixture.port.writer),enter=calls(capture,fixture.consumer.enter),lease=calls(build,fixture.consumer.lease),compile=calls(build,fixture.consumer.capture);
 const actual={
  portOffersStepAdmission:admissions.length===1&&admissions[0]!.arguments[0]?.getText()==="selected.configuration",
  portOffersWriter:writers.length===1&&ts.isArrowFunction(writers[0]!.parent)&&ts.isPropertyAssignment(writers[0]!.parent.parent)&&writers[0]!.parent.parent.name.getText()===fixture.port.property,
  leaseBeforeCompile:lease.length===1&&compile.length===1&&lease[0]!.getEnd()<compile[0]!.getStart()&&compile[0]!.arguments.some(argument=>argument.getText()==="actual."+fixture.consumer.enter),
  insideWriter:enter.length===1&&fixture.consumer.insideWriter.every(name=>calls(capture,name).some(call=>within(call,enter[0]!))),
  afterWriter:enter.length===1&&fixture.consumer.afterWriter.every(name=>{const rows=calls(capture,name);return rows.length>0&&rows.every(call=>call.getStart()>=enter[0]!.getEnd());})
 };
 const oracle=spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url)),fileURLToPath(new URL(".",import.meta.url))],{encoding:"utf8"});expect(oracle.status,oracle.stderr).toBe(0);
 expect(actual).toEqual({portOffersStepAdmission:true,portOffersWriter:true,leaseBeforeCompile:true,insideWriter:true,afterWriter:true});
 expect(JSON.parse(oracle.stdout)).toEqual(actual);
});
