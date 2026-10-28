/** 🚧️ Laws of the bun test operand guard: every operand is classified like real bun reads it (a path runs the file, a filter scans and runs nothing), the whole checkout and its parents are refused, the refusal is worded in English and German and names the fix, the guard lifts with its switch, and a command of another program passes. */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import{bunTestOperandIsPathV1,classifyBunTestOperandsV1,describeBunTestOperandV1,requireExplicitBunTestOperandsV1,splitBunTestArgumentsV1}from"../🟦️.ts";
import fixture from"./🧫️fixtures/🔣️.json";
import schema from"../🧬️schema/🔣️.json";

test("the fixture is closed under its schema",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate({definitions:schema.definitions,$ref:"#/definitions/Corpus"},fixture),JSON.stringify(ajv.errors)).toBe(true);
});

test("each operand is a path, a filter or the whole checkout as the corpus says",()=>{
 for(const row of fixture.cases){
  const found=classifyBunTestOperandsV1([row.operand],fixture.cwd,fixture.checkout)[0];
  expect<string>(found?.kind??"path",row.operand).toBe(row.kind);
 }
 expect(bunTestOperandIsPathV1("./x")&&bunTestOperandIsPathV1("/x")&&!bunTestOperandIsPathV1("x/y")).toBe(true);
});

test("real bun reads exactly the operands this guard calls paths as paths (a path runs the file, a filter runs nothing)",()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"operands-")));
 try{
  mkdirSync(join(base,"sub"));writeFileSync(join(base,"sub","🟦️.ts"),'import{test}from"bun:test";test("ran",()=>{});\n');
  const runs=(operand:string,cwd:string)=>{const out=spawnSync(process.execPath,["test",operand],{cwd,encoding:"utf8",env:{...process.env,SEMIO_TEST_ORPHAN_GUARD:"off"}});return/\b1 pass\b/u.test(out.stdout+out.stderr);};
  for(const[operand,cwd]of[["./sub/🟦️.ts",base],["sub/🟦️.ts",base],[join(base,"sub","🟦️.ts"),base],["../sub/🟦️.ts",join(base,"sub")],["🟦️.ts",join(base,"sub")]]as const){
   const guard=classifyBunTestOperandsV1([operand],cwd,base)[0]===undefined;
   expect(guard,`${operand} in ${cwd.slice(base.length)||"/"}: guard says ${guard?"path":"filter"}, real bun ${runs(operand,cwd)?"ran it":"did not run it"}`).toBe(runs(operand,cwd));
  }
 }finally{rmSync(base,{recursive:true,force:true});}
},120000);

test("the refusal names the operand and the fix in English and German, an empty operand list in the checkout is refused, other programs and the switch pass",()=>{
 const env={LANG:"en_US.UTF-8"},bun=process.execPath;
 expect(()=>requireExplicitBunTestOperandsV1(bun,["test","--timeout","9","a/b.ts"],"/c",env,"/c")).toThrow("[tests] `bun test a/b.ts` reads the operand as a filter, not as a path: bun would scan the whole working directory (/c), hold every directory open and run no test. Prefix it with ./ (as ./a/b.ts) or give the absolute path.");
 expect(()=>requireExplicitBunTestOperandsV1(bun,["test","a/b.ts"],"/c",{LANG:"de_DE.UTF-8"},"/c")).toThrow("[tests] `bun test a/b.ts` liest den Operanden als Filter, nicht als Pfad: bun würde das ganze Arbeitsverzeichnis (/c) durchsuchen, jedes Verzeichnis offen halten und keinen Test ausführen. Stelle ./ voran (als ./a/b.ts) oder gib den absoluten Pfad an.");
 expect(()=>requireExplicitBunTestOperandsV1(bun,["test","./"],"/c",env,"/c")).toThrow(/names the checkout or one of its parents/u);
 expect(()=>requireExplicitBunTestOperandsV1(bun,["test"],"/c",env,"/c")).toThrow(/names the checkout or one of its parents/u);
 expect(()=>requireExplicitBunTestOperandsV1(bun,["test"],"/c/pkg",env,"/c")).not.toThrow();
 expect(()=>requireExplicitBunTestOperandsV1(bun,["test","./a/b.ts","/c/x/y.ts"],"/c",env,"/c")).not.toThrow();
 expect(()=>requireExplicitBunTestOperandsV1(bun,["test","a/b.ts"],"/c",{SEMIO_TEST_OPERAND_GUARD:"off"},"/c")).not.toThrow();
 expect(()=>requireExplicitBunTestOperandsV1("git",["test","a/b.ts"],"/c",env,"/c")).not.toThrow();
 expect(()=>requireExplicitBunTestOperandsV1(bun,["run","a/b.ts"],"/c",env,"/c")).not.toThrow();
 expect(describeBunTestOperandV1({operand:"x",kind:"whole"},"/c",{LANG:"de_DE.UTF-8"})).toMatch(/nennt den Checkout/u);
});

test("the flags with a value never count as operands",()=>{
 expect(splitBunTestArgumentsV1(["--timeout","5","--preload","p.ts","./a.ts","-t","name"]).operands).toEqual(["./a.ts"]);
});
