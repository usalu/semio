import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "./🔣️.json" with {type:"json"};
import {selectNativeStorageBackendV1} from "../../🔌️backends/🧭️selection/🟦️.ts";
import {nativeImageDirectoryNamesV1,requireNativeStoragePathV1} from "../../🌐️platform/📋️contract/🟦️.ts";
import {mkdtempSync,mkdirSync,symlinkSync,lstatSync,rmSync,writeFileSync} from "node:fs";
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
 expect(fixed).toContain('await observeNativeStorageVerificationAwaitV1("allocatedUsage",()=>meter.admit(request,actual.signal))');expect(fixed).toContain("createNativeDirectoryMeterV1({root,filesystemBytes:options.filesystemBytes,maximumPhysicalBytes:options.maximumPhysicalBytes})");
 expect(fixed).not.toContain("measureCommitBytes");expect(port).toContain('mode:"reserve"|"commit"|"recheck"');
});


test("incomplete fixed births preserve only the inactive mount envelope",async()=>{
 const {nativeFixedIncompleteBirthV1}=await import("../🌱️birth/🟦️.ts"),db=new Database(":memory:");try{for(const row of fixture.incompleteBirths){const oracle=db.query("SELECT ? = 0 AND json_array_length(?) = 1 AND json_extract(?, '$[0]') = 'm' AS accepted").get(Number(row.attached),JSON.stringify(row.entries),JSON.stringify(row.entries)) as {accepted:number};expect(Boolean(oracle.accepted),row.name).toBe(row.accepted);expect(nativeFixedIncompleteBirthV1(row.entries,row.attached),row.name).toBe(row.accepted);}}finally{db.close();}
});


test("fresh receipts route directory durability through the portable native interface",async()=>{
 const ts=(await import("typescript")).default,text=await Bun.file(new URL("../🟦️.ts",import.meta.url)).text(),tree=ts.createSourceFile("fixed.ts",text,ts.ScriptTarget.Latest,true),db=new Database(":memory:");try{db.exec("CREATE TABLE call(name TEXT)");const visit=(node:import("typescript").Node)=>{if(ts.isCallExpression(node)&&ts.isIdentifier(node.expression))db.query("INSERT INTO call VALUES(?)").run(node.expression.text);ts.forEachChild(node,visit);};const target=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="syncFixedPersistentReceiptV1");expect(target).toBeDefined();visit(target!);expect((db.query("SELECT count(*) AS count FROM call WHERE name=?").get(fixture.receiptDirectorySync.requiredCall) as {count:number}).count).toBe(1);expect((db.query("SELECT count(*) AS count FROM call WHERE name=?").get(fixture.receiptDirectorySync.forbiddenCall) as {count:number}).count).toBe(0);}finally{db.close();}
});
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                          
test("an opener waiting for the exclusive writer re-reads the image and retries as a reader once a peer activated it",async()=>{
 const fixed=await Bun.file(new URL("../🟦️.ts",import.meta.url)).text(),wait=fixed.slice(fixed.indexOf("async function awaitFixedPeerActivationV1"),fixed.indexOf("export function openNativeFixedPlatformV1"));
 expect(wait).toContain('state==="mounted"');expect(wait).toContain("throw new FixedPeerActivationV1()");
 expect(fixed).toContain('{onWait:()=>awaitFixedPeerActivationV1(options,platform)}');expect(fixed).toContain("mode===\"exclusive\"&&!receiver.parent");
 expect(fixed).toContain("retryNativeFixedOpenAfterPeerActivationV1(()=>openNativeFixedPlatformOnceV1(input))");
 const{retryNativeFixedOpenAfterPeerActivationV1,FixedPeerActivationV1}=await import("../🟦️.ts");
 let calls=0;
 expect(await retryNativeFixedOpenAfterPeerActivationV1(async()=>{if(++calls<3)throw calls===1?new FixedPeerActivationV1():new AggregateError([new Error("close",{cause:new FixedPeerActivationV1()})],"wrapped");return "opened";})).toBe("opened");expect(calls).toBe(3);
 const failure=await(async()=>{try{await retryNativeFixedOpenAfterPeerActivationV1(async()=>{throw new Error("probe");});}catch(error){return error as Error;}})();expect(failure?.message).toBe("probe");
});

test("disposable fixed retirement includes only the directory backend usage records",async()=>{
 const {nativeDirectoryProtocolV1}=await import("../../📁️directory/🟦️.ts"),directoryProtocol=nativeDirectoryProtocolV1(),source=await Bun.file(new URL("../🟦️.ts",import.meta.url)).text(),ts=(await import("typescript")).default;
 const tree=ts.createSourceFile("fixed.ts",source,ts.ScriptTarget.Latest,true),node=tree.statements.find(value=>ts.isFunctionDeclaration(value)&&value.name?.text==="requireDisposableFixedDirectoryV1");
 if(!node)throw Error("Actual fixed retirement inventory required");
 const code=ts.transpileModule(node.getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText,ports={directoryProtocol,nativeImageDirectoryNamesV1,join,lstatSync},requireInventory=new Function(...Object.keys(ports),code+";return requireDisposableFixedDirectoryV1")(...Object.values(ports)),database=new Database(":memory:");
 const base=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!base)throw Error("Fixed retirement tests require an artifact root");
 try{for(const row of fixture.disposableInventories){const root=mkdtempSync(join(resolve(base),"fixed-"));
  try{for(const name of row.entries){const path=join(root,name);if(["m","v","usage"].includes(name))mkdirSync(path);else writeFileSync(path,"");}
   for(const name of row.usage??[])writeFileSync(join(root,"usage",name),"{}");
   const owner=row.directory?{filename:"fixed.dir",members:["fixed.dir","m","v"]}:{filename:"fixed.dmg",members:["fixed.dmg","m"]},oracle=database.query("select not exists(select 1 from json_each(?1) where value not in (select value from json_each(?2)) and value <> ?3 and not (value = ?4 and ?5)) and not exists(select 1 from json_each(?6) where value <> ?7 and not (value like ?8 and substr(value, 12, length(value)-15) not glob ?9)) accepted").get(JSON.stringify(row.entries),JSON.stringify(owner.members),"image.json","usage",Number(row.directory),JSON.stringify(row.usage??[]),"usage.json","usage.json.%.tmp","*[^0-9]*") as {accepted:number};
   expect(Boolean(oracle.accepted)).toBe(row.accepted);
   if(row.accepted)expect(()=>requireInventory(root,join(root,"m"),owner)).not.toThrow();else expect(()=>requireInventory(root,join(root,"m"),owner)).toThrow();
  }finally{rmSync(root,{recursive:true,force:true});}
 }}finally{database.close();}
});
