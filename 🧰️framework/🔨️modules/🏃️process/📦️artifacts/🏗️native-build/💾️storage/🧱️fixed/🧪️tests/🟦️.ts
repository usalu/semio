import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json" with {type:"json"};
import {selectNativeStorageBackendV1} from "../../🔌️backends/🧭️selection/🟦️.ts";
import {requireNativeStoragePathV1} from "../../🌐️platform/📋️contract/🟦️.ts";
import {mkdtempSync,mkdirSync,symlinkSync,lstatSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";

test("native fixed storage selects every declared desktop platform and refuses foreign hosts",()=>{
 const database=new Database(":memory:");try{for(const row of fixture.platforms){const expected=database.query("SELECT CASE ? WHEN 'darwin' THEN 'mounted-image' WHEN 'linux' THEN 'directory-partition' WHEN 'win32' THEN 'directory-partition' ELSE NULL END AS backend").get(row.platform) as {backend:string|null};expect(row.backend).toBe(expected.backend);if(expected.backend===null)expect(()=>selectNativeStorageBackendV1({platform:row.platform})).toThrow();else expect(selectNativeStorageBackendV1({platform:row.platform}).id).toBe(expected.backend);}}finally{database.close();}
});

test("native output path admission rejects dangling symlinks before any materialization",()=>{
 if(!process.env.SEMIO_TEST_ARTIFACT_DIR)throw Error("Native storage path tests require explicit SEMIO_TEST_ARTIFACT_DIR");
 const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR),"p-")),database=new Database(":memory:");
 try{mkdirSync(join(root,"nested"));symlinkSync(join(root,"absent"),join(root,"dangling"),process.platform==="win32"?"junction":"dir");expect(lstatSync(join(root,"dangling")).isSymbolicLink()).toBe(true);for(const row of fixture.paths){const path=resolve(root,row.path),oracle=database.query("SELECT instr(? || '/', ? || '/') = 1 AND ? NOT LIKE 'dangling/%' AS admitted").get(path,root,row.path) as {admitted:number};expect(Boolean(oracle.admitted)).toBe(row.accepted);if(row.accepted)expect(requireNativeStoragePathV1(root,path)).toBe(path);else expect(()=>requireNativeStoragePathV1(root,path)).toThrow();}}finally{database.close();rmSync(root,{recursive:true,force:true});}
});

test("every reservation, commit and recheck charges the directory meter before the journal records it",async()=>{
 const index=await Bun.file(new URL("../../🟦️.ts",import.meta.url)).text(),fixed=await Bun.file(new URL("../🟦️.ts",import.meta.url)).text(),port=await Bun.file(new URL("../../🔌️port/🟦️.ts",import.meta.url)).text();
 const acquire=index.slice(index.indexOf("export function acquireNativeStorageV1"),index.indexOf("type FundedChildScope"));
 expect(acquire.indexOf('measure?.(custody.backing,{mode:"reserve",requestBytes:allocation.bytes}')).toBeGreaterThan(0);
 expect(acquire.indexOf('measure?.(custody.backing,{mode:"reserve"')).toBeLessThan(acquire.indexOf('append({type:"reserved",allocation})'));
 const funded=index.slice(index.indexOf("export function withNativeStorageFundedChildReservationV1"));
 expect(funded.indexOf('measure?.(custody.backing,{mode:"reserve",requestBytes:allocation.bytes}')).toBeLessThan(funded.indexOf('append({type:"reserved",allocation})'));
 expect(index).toContain('port.measure?.(backing,{mode:"commit",requestBytes:0},actual);await transition(actual,{type:"committed"');
 expect(index).toContain('await port.measure?.(backing,{mode:"recheck",requestBytes:0},actual);');
 expect(fixed).toContain("await meter.admit(request,actual.signal)");expect(fixed).toContain("createNativeDirectoryMeterV1({root,filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:options.maximumPhysicalBytes})");
 expect(fixed).not.toContain("measureCommitBytes");expect(port).toContain('mode:"reserve"|"commit"|"recheck"');
});


test("incomplete fixed births preserve only the inactive mount envelope",async()=>{
 const {nativeFixedIncompleteBirthV1}=await import("../🌱️birth/🟦️.ts"),db=new Database(":memory:");try{for(const row of fixture.incompleteBirths){const oracle=db.query("SELECT ? = 0 AND json_array_length(?) = 1 AND json_extract(?, '$[0]') = 'm' AS accepted").get(Number(row.attached),JSON.stringify(row.entries),JSON.stringify(row.entries)) as {accepted:number};expect(Boolean(oracle.accepted),row.name).toBe(row.accepted);expect(nativeFixedIncompleteBirthV1(row.entries,row.attached),row.name).toBe(row.accepted);}}finally{db.close();}
});


test("fresh receipts route directory durability through the portable native interface",async()=>{
 const ts=(await import("typescript")).default,text=await Bun.file(new URL("../🟦️.ts",import.meta.url)).text(),tree=ts.createSourceFile("fixed.ts",text,ts.ScriptTarget.Latest,true),db=new Database(":memory:");try{db.exec("CREATE TABLE call(name TEXT)");const visit=(node:import("typescript").Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))db.query("INSERT INTO call VALUES(?)").run(node.expression.text);ts.forEachChild(node,visit);};const target=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="syncFixedPersistentReceiptV1");expect(target).toBeDefined();visit(target!);expect((db.query("SELECT count(*) AS count FROM call WHERE name=?").get(fixture.receiptDirectorySync.requiredCall) as {count:number}).count).toBe(1);expect((db.query("SELECT count(*) AS count FROM call WHERE name=?").get(fixture.receiptDirectorySync.forbiddenCall) as {count:number}).count).toBe(0);}finally{db.close();}
});
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          