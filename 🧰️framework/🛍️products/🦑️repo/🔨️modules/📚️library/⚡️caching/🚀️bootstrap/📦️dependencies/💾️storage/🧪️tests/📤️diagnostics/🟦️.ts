import ts from"typescript";
import{readFileSync}from"node:fs";
import{test,expect}from"bun:test";
import{createHash}from"node:crypto";
import{loadProductFunctionsV1} from"../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧩️extracted-function/🟦️.ts";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import {javascriptInstallSchemaV1 as installSchema} from"../../🧱️schemas/🟦️.ts";
import corpus from"./🔣️.json";

test("Actual installer failures retain bounded redacted child diagnostics under the original output sink",async()=>{
 const source=readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),tree=ts.createSourceFile("installer-diagnostics.ts",source,ts.ScriptTarget.Latest,true),admit=new Ajv({strict:true}).compile(installSchema.$defs.InstallerFailureDiagnostic),db=new Database(":memory:");
 try{for(const row of corpus.cases)for(const sinkFails of[false,true]){expect(new Ajv({strict:true}).validate(installSchema.$defs.InstallerFailureDiagnosticCase,row)).toBe(true);const terminal={version:1,stdout:"never publish raw stdout secret",stderr:row.stderr,status:row.id==="signal"?null:1,signal:row.id==="signal"?"SIGTERM":null},control=Object.freeze({fixture:"original-output-control"});let delivered="",observed:unknown;const ports={installSchema,createHash,Buffer,TextDecoder,Error,Object,validateJsonSchemaSubset:(schema:unknown,value:unknown)=>new Ajv({strict:true}).validate(schema as object,value)?[]:["invalid"],writeOwnedCommandOutputV1:async(sink:unknown,text:string,actual:unknown)=>{expect(actual).toBe(control);expect(sink).toBe(ports.process.stderr);delivered=text;if(sinkFails)throw Error("fixture-output-failure");},process:{stderr:Object.freeze({fixture:"original-stderr"})}},run=(await loadProductFunctionsV1(new URL("../../🟦️.ts",import.meta.url),["failJavascriptDependencyInstallerV1"],ports)).failJavascriptDependencyInstallerV1;try{await run("fixture/scope",terminal,control);}catch(error){observed=error;}expect(observed).toBeInstanceOf(Error);expect((observed as Error).message).toBe("Fixed Store dependency installer failed");const cause=(observed as Error).cause as any;expect(admit(cause),row.id).toBe(true);expect(cause.scope).toBe("fixture/scope");expect(cause.status).toBe(terminal.status);expect(cause.signal).toBe(terminal.signal);expect(cause.stderrTail).toContain(row.expected);expect(cause.stderrSha256).toBe(createHash("sha256").update(row.stderr).digest("hex"));expect(Buffer.byteLength(cause.stderrTail)).toBeLessThanOrEqual(installSchema.$defs.InstallerFailureDiagnosticPolicy.const.maximumBytes);expect(cause.stderrTail.split("\n").length).toBeLessThanOrEqual(installSchema.$defs.InstallerFailureDiagnosticPolicy.const.maximumLines);for(const secret of row.forbidden)expect(delivered).not.toContain(secret);expect(delivered).not.toContain(terminal.stdout);const [evidence,excerpt]=delivered.trimEnd().split("\n");
 expect(JSON.parse(evidence!.slice(evidence!.indexOf("{")))).toEqual(cause);
 const policy=installSchema.$defs.InstallerFailureDiagnosticPolicy.const;
 expect(excerpt,row.id).toBeDefined();
 expect(excerpt!.startsWith(policy.mirroredPrefix)).toBe(true);
 const mirrored=excerpt!.slice(0,policy.mirroredMaximumBytes);
 expect(Buffer.byteLength(mirrored)).toBeLessThanOrEqual(policy.mirroredMaximumBytes);
 const displayed=JSON.parse(mirrored.slice(policy.mirroredPrefix.length)) as{stderrTail:string};
 const oracle=db.query(`with recursive suffix(last) as (
   values(?1)
   union all select substr(last,2) from suffix
    where length(cast(json_object('stderrTail',last) as blob))+?2>?3
 ) select last from suffix order by length(last) limit 1`).get(cause.stderrTail.trimEnd(),Buffer.byteLength(policy.mirroredPrefix),policy.mirroredMaximumBytes) as{last:string};
 expect(displayed.stderrTail,row.id).toBe(oracle.last);
 expect(displayed.stderrTail,row.id).toContain(row.expected);expect((db.query("select length(cast(? as blob))<=? ok").get(cause.stderrTail,installSchema.$defs.InstallerFailureDiagnosticPolicy.const.maximumBytes)as{ok:number}).ok).toBe(1);}
 const producer=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="runJavascriptDependencyStoreProducerV1");expect(producer?.getText(tree)).toContain("await failJavascriptDependencyInstallerV1(row.scope,terminal,control)");
 }finally{db.close();}
});

/** 🩺️ Contract: when the installer error output is longer than the tail, the first lines survive in `stderrHead` (the first error of a bun failure is usually followed by dozens of consequences), redacted like the tail; a short output has no head because the tail holds all of it. */
test("the first error of a long installer output is kept in the head, while the tail holds the last lines",async()=>{
 const{javascriptDependencyInstallerFailureDiagnosticV1:diagnose}=await import("../../🟦️.ts"),policy=installSchema.$defs.InstallerFailureDiagnosticPolicy.const;
 const noise=Array.from({length:60},(_,index)=>"error: InstallFailed extracting tarball "+index).join("\n"),stderr="error: moving \"@pkgjs/parseargs\" to cache dir failed\nENOENT: rename()\n\n"+noise+"\nlast line Bearer abcdefghij.klmnop";
 const long=diagnose("scope",{version:1,stdout:"",stderr,status:1,signal:null} as never);
 expect(long.stderrTruncated).toBe(true);
 expect(long.stderrHead.split("\n")).toEqual(["error: moving \"@pkgjs/parseargs\" to cache dir failed","ENOENT: rename()","error: InstallFailed extracting tarball 0","error: InstallFailed extracting tarball 1"].slice(0,policy.maximumHeadLines));
 expect(long.stderrTail).not.toContain("parseargs");
 expect(long.stderrTail).toContain("[credentials]");
 expect(long.stderrTail).not.toContain("abcdefghij");
 const short=diagnose("scope",{version:1,stdout:"",stderr:"error: one\nerror: two",status:1,signal:null} as never);
 expect(short.stderrTruncated).toBe(false);
 expect(short.stderrHead).toBe("");
 const oracle=Bun.spawnSync(["python3","-c","import sys;print([l for l in sys.argv[1].split('\\n') if l.strip()][:int(sys.argv[2])])",stderr,String(policy.maximumHeadLines)]).stdout.toString();
 for(const line of long.stderrHead.split("\n"))expect(oracle).toContain(line);
});
