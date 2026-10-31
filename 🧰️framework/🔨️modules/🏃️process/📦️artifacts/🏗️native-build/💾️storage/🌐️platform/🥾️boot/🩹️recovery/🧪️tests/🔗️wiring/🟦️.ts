import ts from"typescript";
import{evaluateExtractedFunctionV1}from"../../../../../../../../🧪️testing/🧩️extracted-function/🟦️.ts";
import{nativeStorageSchemaV1,readNativeStorageDataV1 as admit}from"../../../../../📜️journal/🟦️.ts";
import{bindNativeIdentitiesV1}from"../../📋️plan/🟦️.ts";
import{expect,test}from"bun:test";
import{existsSync,lstatSync,readFileSync,mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join,resolve}from"node:path";
import{createNativeStorageNamespaceEntryV1,removeNativeStorageNamespaceEntryV1,type NativeStorageAllocationV1,type NativeStorageNamespaceReceiptV1}from"../../../../../🟦️.ts";
import{stable}from"../../../../../🌱️birth/🧾️claim-records/🟦️.ts";

/** 🔬️ Checks the actual private namespace observer with its owned schema and real filesystem. */
function namespaceVerifier(){const origin=((import.meta as ImportMeta&{semioPhysicalSourceURL?:string}).semioPhysicalSourceURL??import.meta.url),text=readFileSync(new URL("../../../../../🟦️.ts",origin),"utf8"),tree=ts.createSourceFile("storage.ts",text,ts.ScriptTarget.Latest,true),body=["namespaceIdentityV1","verifyNativeStorageNamespaceV1"].map(name=>{const node=tree.statements.find((row):row is ts.FunctionDeclaration=>ts.isFunctionDeclaration(row)&&row.name?.text===name);expect(node,name).toBeDefined();return node!.getText(tree);}).join("\n"),code=ts.transpileModule(body,{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;return evaluateExtractedFunctionV1(code,"verifyNativeStorageNamespaceV1",{admit,bindNativeIdentitiesV1,dirname,lstatSync,namespaceProtocol:nativeStorageSchemaV1.definitions.NamespaceProtocol.const});}
const verifyNativeStorageNamespaceV1=namespaceVerifier();

const scratch=()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();mkdirSync(base,{recursive:true});return realpathSync(mkdtempSync(join(base,"wr-")));};
const check=()=>{};
const allocation=(receipt:NativeStorageNamespaceReceiptV1):NativeStorageAllocationV1=>({id:"0123456789abcdef0123456789abcdef",owner:"repository:nx-namespace",kind:"compiler-state",bytes:1<<20,phase:"committed",namespace:receipt,child:{version:1,allocation:"0123456789abcdef0123456789abcdef",parentIdentity:"[1,2,3,4]",directory:join(receipt.directory,"native"),filesystemBytes:1<<19,maximumPhysicalBytes:1<<19,policySha256:"0".repeat(64)}});
const renumbered=(receipt:NativeStorageNamespaceReceiptV1,dev:string):NativeStorageNamespaceReceiptV1=>({...receipt,parent:{...receipt.parent,dev},entry:{...receipt.entry,dev}});

test("namespace receipts of an earlier boot verify through relative device binding and every other change is refused",()=>{
 const dir=scratch();try{
  const root=join(dir,"m");mkdirSync(root);const receipt=createNativeStorageNamespaceEntryV1(root,join(root,"ns"),check);
  verifyNativeStorageNamespaceV1(allocation(receipt),root,check);
  verifyNativeStorageNamespaceV1(allocation(renumbered(receipt,"4242")),root,check);
  expect(()=>verifyNativeStorageNamespaceV1(allocation({...receipt,entry:{...receipt.entry,ino:String(BigInt(receipt.entry.ino)+1n)}}),root,check)).toThrow("Exact recorded Native namespace identity required");
  expect(()=>verifyNativeStorageNamespaceV1(allocation({...receipt,parent:{...receipt.parent,mode:receipt.parent.mode^0o1}}),root,check)).toThrow("Exact recorded Native namespace identity required");
  expect(()=>verifyNativeStorageNamespaceV1(allocation({...receipt,entry:{...receipt.entry,dev:"4243"},parent:{...receipt.parent,dev:"4242"}}),root,check)).toThrow("Exact recorded Native namespace identity required");
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("an empty namespace entry of an earlier boot is removed exactly once and a replaced one is retained",()=>{
 const dir=scratch();try{
  const root=join(dir,"m");mkdirSync(root);
  const old=createNativeStorageNamespaceEntryV1(root,join(root,"a"),check);removeNativeStorageNamespaceEntryV1(root,renumbered(old,"4242"),check);expect(existsSync(join(root,"a"))).toBe(false);
  const kept=createNativeStorageNamespaceEntryV1(root,join(root,"b"),check);
  expect(()=>removeNativeStorageNamespaceEntryV1(root,{...kept,entry:{...kept.entry,ino:String(BigInt(kept.entry.ino)+1n)}},check)).toThrow("Exact owned empty Native namespace required");expect(existsSync(join(root,"b"))).toBe(true);
 }finally{rmSync(dir,{recursive:true,force:true});}
});

test("creator claim identities compare by inode and mode only",()=>{
 const row={dev:"4141",ino:"424182066",mode:16877};
 expect(stable(row,{...row,dev:"4242"})).toBe(true);expect(stable(row,row)).toBe(true);
 expect(stable(row,{...row,ino:"424182067"})).toBe(false);expect(stable(row,{...row,mode:16832})).toBe(false);
});
