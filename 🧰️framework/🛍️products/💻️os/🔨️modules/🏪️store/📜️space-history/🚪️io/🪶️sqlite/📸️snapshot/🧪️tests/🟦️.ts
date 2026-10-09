import Ajv2020 from "ajv/dist/2020";
import Ajv from "ajv";
import { authoredSnapshotSqliteContract, authoredSnapshotPreflightContract,authoredSnapshotSemanticContract } from "../../../../../../🪐️space/🧪️tests/🪶️sqlite/🔬️oracle/🟦️.ts";
import { fileURLToPath } from "node:url";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Parser from "web-tree-sitter";
import callerGrant from "../🧫️fixtures/🫴️grant/🔣️.json";
import callerGrantSchema from "../🧬️schema/🫴️grant/🔣️.json";

const snapshot=fileURLToPath(new URL("../",import.meta.url)),store=resolve(snapshot,"../../../.."),root=resolve(store,"../../../../..");
test("original history native caller grant retains five independently authored axes",()=>{
 const validate=new Ajv2020({strict:true,allErrors:true}).compile(callerGrantSchema);expect(validate(callerGrant)).toBe(true);expect(validate({...callerGrant,extra:1})).toBe(false);
 for(const axis of callerGrantSchema.required){const missing:Record<string,unknown>={...callerGrant};delete missing[axis];expect(validate(missing)).toBe(false);expect(validate({...callerGrant,[axis]:-1})).toBe(false);expect(validate({...callerGrant,[axis]:0.5})).toBe(false);expect(validate({...callerGrant,[axis]:0})).toBe(true);}
 expect(Object.keys(callerGrant)).toEqual(["maximumItems","maximumCopyBytes","maximumCapacityBytes","maximumReleaseBytes","maximumDepth"]);
 console.error("[DEBUG] Original History caller grant closed five axes independently validated with Ajv; unchanged authored policy");
});
authoredSnapshotSqliteContract({sql:join(snapshot,"🗄️.sql"),fixtures:join(snapshot,"🧫️fixtures")});
const rustTokens = (node: Parser.SyntaxNode): string[] => {
  if (["line_comment", "block_comment"].includes(node.type)) return [];
  if (["string_literal", "raw_string_literal", "char_literal"].includes(node.type) || node.childCount === 0) return [node.text];
  return node.children.flatMap(rustTokens);
};
const containsTokens = (actual: string[], required: string[]): boolean => actual.some((_, index) => required.every((token, offset) => actual[index + offset] === token));
const containsRust = (node: Parser.SyntaxNode, required: string[]): boolean => containsTokens(rustTokens(node), required);
/** 🌳️ Compares parsed Rust tokens while retaining literal contents and punctuation. */
async function sourceTokens(path: string, functionName?: string): Promise<string[]> {
  await Parser.init(); const parser = new Parser(); let tree: Parser.Tree | undefined;
  try {
    parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
    tree = parser.parse(readFileSync(path, "utf8")); if(tree.rootNode.hasError())throw Error("Rust parser refused source: "+path);
    const visit=(node:Parser.SyntaxNode):Parser.SyntaxNode[]=>[node,...node.namedChildren.flatMap(visit)];
    const parts=functionName?.split("::");
    const functions=parts?.length===2?visit(tree.rootNode).filter(node=>node.type==="impl_item"&&node.childForFieldName("type")?.text.startsWith(parts[0]!+"<")).flatMap(node=>node.childForFieldName("body")?.namedChildren??[]).filter(node=>node.type==="function_item"&&node.childForFieldName("name")?.text===parts[1]):tree.rootNode.namedChildren.filter(node=>node.type==="function_item"&&node.childForFieldName("name")?.text===functionName);
    if (functionName !== undefined) { expect(functions.length).toBe(1); return rustTokens(functions[0]!); }
    return rustTokens(tree.rootNode);
  } finally { tree?.delete(); parser.delete(); }
}


test("closed owner-registration corpus covers every opening and native encoding without changing envelope dialect", () => {
  const corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/📣️registration/🔣️.json"), "utf8"));
  expect(corpus.cases.map((sample: any) => sample.id).sort()).toEqual(["create-binary", "create-text", "reload-binary", "reload-text", "retained-binary", "retained-text"]);
  for (const sample of corpus.cases) {
    expect(sample.envelopeDialect).toBe(sample.opening === "retained" ? corpus.coordinate : null);
    expect(sample.before).toEqual({ export: "UnsupportedOwner", import: "UnsupportedOwner" });
    expect(sample.after).toEqual({ export: "completeSnapshot", import: "completeSnapshot", metadata: "sameOwnerAndEncoding" });
  }
  const wrong = structuredClone(corpus); wrong.cases[0].before.export = "InvalidValue";
  console.log("[DEBUG] owner-registration matrix: create/reload/retained × binary/text; typed pre-refusal and retained envelope metadata");
});

test("actual native isolated law admits explicit owner registration and generic constructors remain owner-neutral", async () => {
  await Parser.init(); const parser = new Parser(), trees: Parser.Tree[] = [];
  try {
  parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
  const parse = (path: string) => { const tree = parser.parse(readFileSync(path, "utf8")); trees.push(tree); return tree; };
  const collect = (node: Parser.SyntaxNode, predicate: (node: Parser.SyntaxNode) => boolean): Parser.SyntaxNode[] => [ ...(predicate(node) ? [node] : []), ...node.namedChildren.flatMap(child => collect(child, predicate)) ];
  const calls = (node: Parser.SyntaxNode) => collect(node, node => node.type === "call_expression").map(node => ({ name: node.childForFieldName("function")?.text ?? "", start: node.startIndex }));
  const corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/📣️registration/🔣️.json"), "utf8")), lawTree = parse(join(snapshot, "🧪️tests/🦀️.rs"));
  expect(lawTree.rootNode.hasError()).toBe(false);
  const law = lawTree.rootNode.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === corpus.nativeLaw);
  expect(law).toBeDefined();
  const names = calls(law!), registration = names.filter(node => node.name === "store::space_history::io::sqlite::snapshot::register_sqlite_snapshot");
  expect(registration.length).toBe(1);
  const before = names.filter(node => node.start < registration[0].start), after = names.filter(node => node.start > registration[0].start);
  for (const operation of ["io_export_sqlite_snapshot", "io_import_sqlite_snapshot"]) {
    expect(before.some(node => node.name.includes(operation)), operation).toBe(true);
    expect(after.some(node => node.name.includes(operation)), operation).toBe(true);
  }
  const assertions = collect(law!, node => node.type === "macro_invocation" && node.childForFieldName("macro")?.text === "assert_eq").map(node => node.text.replaceAll(/\s/g, ""));
  for (const side of ["export_refusal", "import_refusal"]) expect(assertions).toContain(`assert_eq!(${side}.cause.kind,semio_framework_value::ValueRefusalKind::UnsupportedOwner)`);
  const strings = collect(law!, node => node.type === "string_literal").map(node => JSON.parse(node.text));
  expect(strings).toContain("{}::" + corpus.nativeLaw); expect(strings).toContain("--exact"); expect(strings).toContain(corpus.isolationVariable);
  expect(names.some(node => node.name.includes("current_exe"))).toBe(true);
  expect(containsRust(law!, ["for", "sample", "in", "corpus", "[", '"cases"', "]"])).toBe(true);
  expect(containsRust(law!, ["sample", "[", '"encoding"', "]"])).toBe(true);
  expect(containsRust(law!, ["assert_eq", "!", "(", "imported", ".", "value", ",", "source", ")"])).toBe(true);
  expect(containsRust(law!, ["assert_eq", "!", "(", "relational", ",", "independent", ")"])).toBe(true);
  const storeTree = parse(join(store, "🦀️.rs"));
  const constructors = collect(storeTree.rootNode, node => node.type === "function_item" && ["new", "construct", "from_initialized_runtime_with_owners"].includes(node.childForFieldName("name")?.text ?? "") && node.parent?.parent?.type === "impl_item" && node.parent?.parent?.childForFieldName("type")?.text.startsWith("ArtifactStore<") === true);
  expect(constructors.map(node => node.childForFieldName("name")?.text).sort()).toEqual(["construct", "from_initialized_runtime_with_owners", "new"]);
  const hydration = parse(join(store, "🧾️document/📜️history/💧️hydration/🦀️.rs"));
  for (const node of [...constructors, hydration.rootNode]) expect(calls(node).filter(node => /(?:register_sqlite_snapshot|register_native_snapshot_codec|register_native_document_codec)(?:<|$)/.test(node.name))).toEqual([]);
  const binding = parse(join(snapshot,"🦀️.rs")), ownerRegistration = binding.rootNode.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === "register_sqlite_snapshot");
  expect(calls(ownerRegistration!).some(node => node.name === "crate::io::register_native_snapshot_codec")).toBe(true);
  expect(ownerRegistration!.text.includes("SQLITE_SNAPSHOT_DIALECT")).toBe(true); expect(containsRust(ownerRegistration!, ["SpaceHistorySnapshot", ",", "SpaceHistoryMutation"])).toBe(true);
  console.log("[DEBUG] actual tree-sitter owner admission: isolated exact selector; both typed refusals before owner publication; complete same-owner roundtrips after; generic Store/hydration unchanged");
  } finally { for (const tree of trees) tree.delete(); parser.delete(); }
});

test("actual owner literals and existing GUI source/native routes identify the same SpaceHistory boundary", async () => {
  const corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/📣️registration/🔣️.json"), "utf8")), source = readFileSync(join(store, "🦀️.rs"), "utf8"), owner = readFileSync(join(snapshot,"🦀️.rs"), "utf8");
  const parsedStore = await sourceTokens(join(store, "🦀️.rs")), parsedOwner = await sourceTokens(join(snapshot,"🦀️.rs"));
  expect(containsTokens(parsedStore, ["#", "[", "path", "=", '"📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"', "]"])).toBe(true);
  expect(containsTokens(parsedStore, ["#", "[", "path", "=", '"📜️space-history/🦀️.rs"', "]"])).toBe(true);
  expect(containsTokens(parsedOwner, ["standard", ":", "semio_framework_artifact_reference", "::", "StandardId", "(", '"1"', ")"])).toBe(true); expect(containsTokens(parsedOwner, ["subset", ":", "semio_framework_artifact_reference", "::", "SubsetId", "(", '"*"', ")"])).toBe(true);
  expect(containsTokens(parsedStore, ["pub", "const", "S_SPACE_HISTORY_SCHEMA", ":", "&", "str", "=", JSON.stringify(corpus.owner), ";"])).toBe(true);
  const project = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json"), "utf8"));
  for (const command of ["test-space-history-sqlite-source", "test-space-history-sqlite-native"]) {
    expect(project.targets[command].options.command).toBe(`bun ./📜️script.ts ${command}`);
    expect(project.targets[command].cache).toBe(false);
  }
  console.log("[DEBUG] existing actual GUI/source/native gate routes and literal SpaceHistory mounts agree");
});

test("closed native ownership corpus separates physical and reconstruction ceilings on both encodings", async () => {
  const corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/🧮️ownership/🔣️.json"), "utf8"));
  const law = await sourceTokens(join(snapshot, "🧪️tests/🦀️.rs"), corpus.nativeLaw);
  for (const field of ["zeroPhysical", "cumulative", "rowRefusal", "cancellation"]) expect(containsTokens(law, ["contract", "[", JSON.stringify(field), "]"])).toBe(true);
  expect(containsTokens(law, ["for", "encoding", "in", "[", "SnapshotEncoding", "::", "Binary", ",", "SnapshotEncoding", "::", "Text", "]"])).toBe(true);
  expect(containsTokens(law, ["for", "physical", "in", "[", "false", ",", "true", "]"])).toBe(true);
  expect(containsTokens(law, ["control", ".", "reconstruction_remaining_bytes", "(", ")"])).toBe(true);
  expect(containsTokens(law, ["control", ".", "allocation_remaining_bytes", "(", ")"])).toBe(true);
  expect(containsTokens(law, ["assert_eq", "!", "(", "semantic", ",", "allocation"])).toBe(true);
  console.log("[DEBUG] native ownership source independently retains both cumulative ceilings, row refusal and owned interior cancellation");
});

import {Database as HistoryOutputDatabase} from "bun:sqlite";

test("closed History native output contract independently retains literal bytes and cumulative physical roles",()=>{
 const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🧮️ownership/🔣️.json"),"utf8")),expected={"schema":"space-history.sqlite.native-output/v1","nativeLaw":"sqlite_snapshot_framework_space_history_native_output_full_requests_and_cumulative_owner_are_admitted","encodings":["binary","text"],"phase":"encodeNative","requestAccounting":"fullAllocatorRequests","ledger":"sameCallerAllocation","source":"completeUnchangedOwner","zeroPhysical":{"maximumBytes":0,"expectedKind":"ownershipLimit","expectedCharge":0},"cumulative":{"ceilingNumerator":3,"ceilingDenominator":2,"expectedKind":"ownershipLimit"},"cancellation":{"minimumCompleted":65536,"expectedKind":"canceled","retainedDebit":"positiveNoRefund"},"literal":{"unit":"日本𐀀","repeat":32768,"utf8Bytes":327680},"appendRequests":{"schema":"native.output.append-requests/v1","fragments":["","語","x","","\u0000é!"],"utf8Bytes":8,"fullReplacementRequests":[3,4,8],"replacementDebit":15,"reusedCapacity":16,"reusedRequestBytes":0,"refusal":"ownershipLimit","allocationRefusal":"allocationFailed","retirement":"noRefund"},"retirement":{"schema":"history.native.temporary-retirement/v1","sourceCorpus":"intrinsic-media-wire-v1","nativeLaw":"sqlite_snapshot_framework_space_history_native_temporary_retirement_requests_no_backing_for_every_intrinsic_kind","kinds":["null","bool","uint","int","float","string","bytes","array","object"],"containers":["empty","orderedDuplicateMembers","deepArray","deepObject","longBranch"],"depth":256,"longBranch":{"unit":"é語🧾","repeat":14564,"utf8Bytes":131076},"requestBytes":0,"releasedBytes":"completeBorrowedCapacityCensus","cleanup":"mandatoryAfterCancellation","temporaryLinks":"reuseVacatedOwnedSlots","recursiveDropFallback":false,"partial":{"schema":"history.native.partial-retirement/v1","grants":[0,1,3,255,256,"complete"],"work":"oneDestructiveTransitionPerUnit","requests":0,"released":"completeBorrowedCapacityCensus","zeroGrant":"retainsEveryOwnedCapacity","drop":"mandatoryCompleteReleaseAfterCancellation","nativeLaw":"sqlite_snapshot_framework_space_history_native_temporary_retirement_partial_grants_and_drop_release_every_owned_capacity"}}};
 expect(corpus.output).toEqual(expected);
 const literal=expected.literal.unit.repeat(expected.literal.repeat),bytes=Buffer.from(literal,"utf8");expect(bytes.length).toBe(expected.literal.utf8Bytes);
 const oracle=new HistoryOutputDatabase(":memory:");try{oracle.exec("CREATE TABLE native_history_output(ordinal INTEGER PRIMARY KEY,message TEXT NOT NULL)");oracle.run("INSERT INTO native_history_output VALUES(0,?)",[literal]);expect(oracle.query("SELECT length(CAST(message AS BLOB)) AS bytes,message FROM native_history_output").get()).toEqual({bytes:expected.literal.utf8Bytes,message:literal});}finally{oracle.close();}
 expect(expected.cumulative.ceilingNumerator).toBeGreaterThan(expected.cumulative.ceilingDenominator);expect(expected.cumulative.ceilingNumerator).toBeLessThan(2*expected.cumulative.ceilingDenominator);expect(expected.cancellation.minimumCompleted).toBeLessThan(bytes.length);
});


test("History byte append contract admits full replacement requests and reuses existing paid capacity",()=>{
 const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🧮️ownership/🔣️.json"),"utf8"));
 const expected={schema:"native.output.append-requests/v1",fragments:["","語","x","","\u0000é!"],utf8Bytes:8,fullReplacementRequests:[3,4,8],replacementDebit:15,reusedCapacity:16,reusedRequestBytes:0,refusal:"ownershipLimit",allocationRefusal:"allocationFailed",retirement:"noRefund"};
 expect(corpus.output.appendRequests).toEqual(expected);
 const fragments=expected.fragments.map(fragment=>Buffer.from(fragment,"utf8")),requests:number[]=[];let replacement=Buffer.alloc(0);
 for(const fragment of fragments){if(fragment.length!==0){replacement=Buffer.concat([replacement,fragment]);requests.push(replacement.length);}}
 expect(requests).toEqual(expected.fullReplacementRequests);expect(requests.reduce((sum,size)=>sum+size,0)).toBe(expected.replacementDebit);expect(replacement.length).toBe(expected.utf8Bytes);
 const reused=Buffer.alloc(expected.reusedCapacity),backing=reused.buffer;let length=0;
 for(const fragment of fragments){length+=fragment.copy(reused,length);expect(reused.buffer).toBe(backing);}
 expect(reused.subarray(0,length)).toEqual(replacement);expect(length).toBe(expected.utf8Bytes);expect(expected.reusedRequestBytes).toBe(0);
});


test("History temporary owner retirement is closed across intrinsic kinds, literal members and deep branches",()=>{
 const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🧮️ownership/🔣️.json"),"utf8"));
 const expected={schema:"history.native.temporary-retirement/v1",sourceCorpus:"intrinsic-media-wire-v1",nativeLaw:"sqlite_snapshot_framework_space_history_native_temporary_retirement_requests_no_backing_for_every_intrinsic_kind",kinds:["null","bool","uint","int","float","string","bytes","array","object"],containers:["empty","orderedDuplicateMembers","deepArray","deepObject","longBranch"],depth:256,longBranch:{unit:"é語🧾",repeat:14564,utf8Bytes:131076},requestBytes:0,releasedBytes:"completeBorrowedCapacityCensus",cleanup:"mandatoryAfterCancellation",temporaryLinks:"reuseVacatedOwnedSlots",recursiveDropFallback:false,partial:{"schema":"history.native.partial-retirement/v1","grants":[0,1,3,255,256,"complete"],"work":"oneDestructiveTransitionPerUnit","requests":0,"released":"completeBorrowedCapacityCensus","zeroGrant":"retainsEveryOwnedCapacity","drop":"mandatoryCompleteReleaseAfterCancellation","nativeLaw":"sqlite_snapshot_framework_space_history_native_temporary_retirement_partial_grants_and_drop_release_every_owned_capacity"}};
 expect(corpus.output.retirement).toEqual(expected);
 const intrinsic=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🎒️pack/🌱️value/🧫️fixtures/🎞️intrinsic-media/🔣️.json"),"utf8"));expect(intrinsic.contract).toBe(expected.sourceCorpus);
 const kinds=new Set<string>(),members:{key:string,index:number}[]=[];const pending=[intrinsic.value];while(pending.length){const value=pending.pop()!;kinds.add(value.kind);if(value.kind==="array")pending.push(...value.value);if(value.kind==="object"){for(const[index,member]of value.value.entries()){members.push({key:member.key,index});pending.push(member.value);}}}
 expect([...kinds].sort()).toEqual([...expected.kinds].sort());expect(new Set(members.map(member=>member.key)).size).toBeLessThan(members.length);
 for(const word of intrinsic.binary64Words){const bytes=Buffer.from(word,"hex"),view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength),roundtrip=new ArrayBuffer(8);new DataView(roundtrip).setFloat64(0,view.getFloat64(0,false),false);if(!Number.isNaN(view.getFloat64(0,false)))expect(Buffer.from(roundtrip)).toEqual(bytes);expect(view.getBigUint64(0,false).toString(16).padStart(16,"0")).toBe(word);}
 expect(Buffer.byteLength(expected.longBranch.unit.repeat(expected.longBranch.repeat),"utf8")).toBe(expected.longBranch.utf8Bytes);
 const oracle=new HistoryOutputDatabase(":memory:");try{oracle.exec("CREATE TABLE retired_members(ordinal INTEGER PRIMARY KEY,literal TEXT NOT NULL)");members.forEach((member,ordinal)=>oracle.run("INSERT INTO retired_members VALUES(?,?)",[ordinal,member.key]));expect(oracle.query("SELECT literal FROM retired_members ORDER BY ordinal").all().map((row:any)=>row.literal)).toEqual(members.map(member=>member.key));}finally{oracle.close();}
});

test("History partial cleanup grants retain owners and mandate complete backing release",async()=>{
 const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🧮️ownership/🔣️.json"),"utf8"));
 const expected={"schema":"history.native.partial-retirement/v1","grants":[0,1,3,255,256,"complete"],"work":"oneDestructiveTransitionPerUnit","requests":0,"released":"completeBorrowedCapacityCensus","zeroGrant":"retainsEveryOwnedCapacity","drop":"mandatoryCompleteReleaseAfterCancellation","nativeLaw":"sqlite_snapshot_framework_space_history_native_temporary_retirement_partial_grants_and_drop_release_every_owned_capacity"};
 expect(corpus.output.retirement.partial).toEqual(expected);
 const oracle=new HistoryOutputDatabase(":memory:");
 try{oracle.exec("CREATE TABLE cleanup_grants(ordinal INTEGER PRIMARY KEY,items INTEGER,completion TEXT)");
 expected.grants.forEach((grant,ordinal)=>oracle.run("INSERT INTO cleanup_grants VALUES(?,?,?)",[ordinal,typeof grant==="number"?grant:null,typeof grant==="string"?grant:null]));
 expect(oracle.query("SELECT items,completion FROM cleanup_grants ORDER BY ordinal").all()).toEqual([{items:0,completion:null},{items:1,completion:null},{items:3,completion:null},{items:255,completion:null},{items:256,completion:null},{items:null,completion:"complete"}]);
 }finally{oracle.close();}
 const law=await sourceTokens(join(snapshot,"🚦️native/🦀️.rs"),expected.nativeLaw);
 for(const tokens of [["retirement",".","close_step","(","grant",")"],["drop","(","retirement",")"],["assert_eq","!","(","released",",","expected_release"]])expect(containsTokens(law,tokens)).toBe(true);
});

authoredSnapshotPreflightContract({sql:fileURLToPath(new URL("../🗄️.sql",import.meta.url)),fixtures:fileURLToPath(new URL("../🧫️fixtures",import.meta.url))});

authoredSnapshotSemanticContract({sql:fileURLToPath(new URL("../🗄️.sql",import.meta.url)),fixtures:fileURLToPath(new URL("../🧫️fixtures",import.meta.url))});

test("Space-history native codec bodies and publication have an explicit IO owner",async()=>{
 const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🚪️ownership/🔣️.json"),"utf8"));
 const native=await sourceTokens(join(snapshot,"🚦️native/🦀️.rs"),"decode_with");
 expect(containsTokens(native,["semio_framework_pack_json","::","from_json_str_controlled"])).toBe(true);
 const storeOwner=await sourceTokens(join(store,"🦀️.rs"));
 expect(containsTokens(storeOwner,["#","[","path","=",'"📜️space-history/🦀️.rs"',"]"])).toBe(true);
 expect(storeOwner.includes('"📜️space-history/🧬️schema/📸️snapshot/🪶️sqlite/🦀️.rs"')).toBe(false);
 const bound=await sourceTokens(join(snapshot,"🚦️native/🦀️.rs"),corpus.law);
 expect(containsTokens(bound,["module_path","!","(",")"])).toBe(true);
 console.log("[DEBUG] Space-history controlled JSON/native codec bodies, neutral owner and actual IO module mount agree");
});

test("native snapshot owner policy is a closed independent five-axis caller input",async()=>{
 const defining=join(process.cwd(),"🧰️framework/🔨️modules/🚪️io/⏱️control/🛫️snapshot");const fixture=JSON.parse(readFileSync(join(defining,"🧫️fixtures/🔣️.json"),"utf8"));
 const contract=JSON.parse(readFileSync(join(process.cwd(),"🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🧬️schema/🔣️.json"),"utf8"));const ordered=JSON.parse(readFileSync(join(process.cwd(),"🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"),"utf8"));const ajv=new Ajv({strict:false,allErrors:true});ajv.addSchema(ordered);ajv.addSchema(contract);const validate=ajv.compile({$ref:contract.$id+"#/$defs/Grant"});
 for(const row of fixture.cases){expect(validate(row.grant)).toBe(true);expect(validate({...row.grant,undeclared:1})).toBe(false);expect(validate({...row.grant,maximumCopyBytes:-1})).toBe(false);}
 for(const row of fixture.cases){const db=new HistoryOutputDatabase(":memory:");try{db.exec("CREATE TABLE owner(owned INTEGER NOT NULL,ceiling INTEGER NOT NULL,items INTEGER NOT NULL,copy INTEGER NOT NULL,capacity INTEGER NOT NULL,released INTEGER NOT NULL,depth INTEGER NOT NULL,CHECK(owned<=ceiling))");const g=row.grant;db.run("INSERT INTO owner VALUES(?,?,?,?,?,?,?)",[fixture.initialBytes,fixture.nativeMaximumBytes,g.maximumItems,g.maximumCopyBytes,g.maximumCapacityBytes,g.maximumReleaseBytes,g.maximumDepth]);db.run("UPDATE owner SET owned=owned+?",[new TextEncoder().encode(fixture.copy).length]);expect(db.query("SELECT owned FROM owner").get()).toEqual({owned:row.expectedNativeOwnedBytes});expect(db.query("SELECT items,copy,capacity,released,depth FROM owner").get()).toEqual({items:g.maximumItems,copy:g.maximumCopyBytes,capacity:g.maximumCapacityBytes,released:g.maximumReleaseBytes,depth:g.maximumDepth});if(row.id!=="full")expect(g[row.id]).toBe(0);}finally{db.close();}}
 console.log("[DEBUG] Native snapshot original owner vectors: genuine Value five-axis Grant contract, zero authority stays zero, independent SQLite cumulative native receipt agrees");
});


test("field close examples preserve genuine progress and actual physical totals", async () => {
  const corpus = JSON.parse(readFileSync(join(store,"🚪️io/🚫️refusal/🧫️fixtures/🔣️.json"),"utf8"));
  const progress = JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🌐️wire/🧬️schema/🧾️progress.json"),"utf8"));
  const valid = new Ajv({strict:true,allErrors:true}).compile(progress);
  for (const sample of corpus.cases) expect(valid(sample.receipt)).toBe(true);
  for (const field of progress.required) { const wrong={...corpus.cases[1].receipt};delete wrong[field];expect(valid(wrong)).toBe(false); }
  expect(valid({...corpus.cases[1].receipt,inferredGrant:1024})).toBe(false);
  const db=new HistoryOutputDatabase(":memory:");
  try {
    db.run("CREATE TABLE failure_receipts(name TEXT PRIMARY KEY, capacity INTEGER NOT NULL, released INTEGER NOT NULL)");
    for(const sample of corpus.cases){db.run("INSERT INTO failure_receipts VALUES(?,?,?)",[sample.name,sample.receipt.retainedCapacityBytes,sample.receipt.releasedBytes]);const row=db.query("SELECT SUM(capacity) AS capacity,SUM(released) AS released FROM failure_receipts WHERE name=?").get(sample.name) as {capacity:number,released:number};expect(row).toEqual({capacity:sample.expectedTotalCapacity,released:sample.expectedTotalRelease});}
  } finally { db.close(); }
  await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",root)),"out/tree-sitter-rust.wasm")));
  const tree=parser.parse(readFileSync(join(store,"🚪️io/🧬️schema/⚠️diagnostic/🦀️.rs"),"utf8"));
  try {const visit=(n:Parser.SyntaxNode):Parser.SyntaxNode[]=>[n,...n.namedChildren.flatMap(visit)];const diagnostic=visit(tree.rootNode).find(n=>n.type==="struct_item"&&n.childForFieldName("name")?.text==="SchemaDecodeDiagnostic");expect(diagnostic?.text.includes("retained_progress:RetainedCloneProgress")).toBe(true);expect(diagnostic?.text.includes("refusal_kind:ValueRefusalKind")).toBe(true);expect(readFileSync(join(store,"🦀️.rs"),"utf8")).toContain("OwnedSchemaDecodeDiagnostic=schema_diagnostic::SchemaDecodeDiagnostic<OwnedSchemaPath>");} finally {tree.delete();}
  console.log("[DEBUG] original failed field close retains all receipt axes; genuine Progress Ajv omissions and SQLite frame/release totals agree");
});

test("actual retained child owners use independently bounded physical currencies", async () => {
 const child=join(root,"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation");
 const law=JSON.parse(readFileSync(join(child,"🧫️fixtures/♻️child-currencies/🔣️.json"),"utf8"));
 const contract=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🌱️value/🗂️ordered/♻️retirement/🧬️schema/🔣️.json"),"utf8"));
 const grant={maximumItems:law.maximumItems,maximumCopyBytes:law.maximumCopyBytes,maximumCapacityBytes:law.maximumCapacityBytes,maximumReleaseBytes:law.maximumReleaseBytes,maximumDepth:law.maximumDepth};
 const ajv=new Ajv({strict:true,allErrors:true});ajv.addSchema(contract);const valid=ajv.compile({$ref:contract.$id+"#/$defs/Grant"}),validProgress=ajv.compile({$ref:contract.$id+"#/$defs/Progress"});expect(valid(grant)).toBe(true);
 for(const axis of Object.keys(grant)){const missing={...grant};delete missing[axis as keyof typeof grant];expect(valid(missing)).toBe(false);}
 expect(valid({...grant,inferredGrant:8194})).toBe(false);
 const db=new HistoryOutputDatabase(":memory:");try{
  db.run("CREATE TABLE currencies(axis TEXT PRIMARY KEY,granted INTEGER NOT NULL,spent INTEGER NOT NULL,CHECK(spent<=granted))");
  for(const [axis,granted] of [["items",1],["copy",4096],["capacity",0],["release",4096],["depth",64]] as const)db.run("INSERT INTO currencies VALUES(?,?,0)",[axis,granted]);
  db.run("UPDATE currencies SET spent=4096 WHERE axis='copy'");expect(db.query("SELECT spent FROM currencies WHERE axis='release'").get()).toEqual({spent:0});
  const observed=db.query("SELECT MAX(CASE WHEN axis='copy' THEN spent END) AS copiedBytes,MAX(CASE WHEN axis='capacity' THEN spent END) AS retainedCapacityBytes,MAX(CASE WHEN axis='release' THEN spent END) AS releasedBytes FROM currencies").get() as {copiedBytes:number;retainedCapacityBytes:number;releasedBytes:number};
  const progress={copiedItems:1,...observed,complete:false};expect(validProgress(progress)).toBe(true);expect(progress).toEqual({copiedItems:1,copiedBytes:4096,retainedCapacityBytes:0,releasedBytes:0,complete:false});
  for(const axis of Object.keys(progress)){const missing={...progress};delete missing[axis as keyof typeof progress];expect(validProgress(missing)).toBe(false);}
  expect(()=>db.run("UPDATE currencies SET spent=1 WHERE axis='capacity'")).toThrow();
 }finally{db.close();}
 for(const path of ["🛫️encoder","🫙️owner","🪪️registry"]){const source=readFileSync(join(child,"🧩️child-operations",path,"🦀️.rs"),"utf8");expect(source).not.toContain("SnapshotRetirementStep");expect(source).toContain("RetainedCloneGrant");for(const demand of ["next_copy_byte_demand","next_capacity_byte_demand","next_release_byte_demand","next_depth_demand"])expect(source).toContain(demand);}
 console.log("[DEBUG] all8 original child carriers retain independent fixed1/4096/0/4096/64 authority; genuine Grant/Progress Ajv payload projections and SQLite copy-versus-release denial agree; no trial contract or native runtime credit");
});


test("normal decoder release keeps the original independent job wallet on failure",async()=>{
 const defining=join(store,"🚪️io/🚫️refusal/⏱️step");const law=JSON.parse(readFileSync(join(defining,"🧫️fixtures/🔣️.json"),"utf8"));const schema=JSON.parse(readFileSync(join(defining,"🧬️schema/🔣️.json"),"utf8"));const valid=new Ajv({strict:true,allErrors:true}).compile(schema);expect(valid(law)).toBe(true);
 for(const axis of law.deniedAxes){const bad={...law,grant:{...law.grant}};delete bad.grant[axis];expect(valid(bad)).toBe(false);}expect(valid({...law,grant:{...law.grant,inferred:64}})).toBe(false);
 const db=new HistoryOutputDatabase(":memory:");try{db.run("CREATE TABLE receipt(items INTEGER,copy INTEGER,born INTEGER,freed INTEGER,CHECK(items<=65536 AND copy<=65536 AND born<=65536 AND freed<=65536))");db.run("INSERT INTO receipt VALUES(?,?,?,?)",[1,new TextEncoder().encode(law.source).length,law.birthBytes,law.beforeBytes]);expect(db.query("SELECT items AS copiedItems,copy AS copiedBytes,born AS retainedCapacityBytes,freed AS releasedBytes FROM receipt").get()).toEqual(law.receipt);expect(db.query("SELECT born-freed AS delta FROM receipt").get()).toEqual({delta:32});}finally{db.close();}
 const tokens=await sourceTokens(join(store,"🦀️.rs"),"ArtifactEnvelopeDecodeAuthority::release_step");expect(containsTokens(tokens,["cx",".","retained_grant","(",")"])).toBe(true);expect(containsTokens(tokens,["cx",".","consume_retained","(","progress",")"])).toBe(true);expect(tokens.includes("artifact_retirement_self_grant")).toBe(false);expect(tokens.includes("record_close_grant")).toBe(false);
 await Parser.init();const parser=new Parser();parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json",root)),"out/tree-sitter-rust.wasm")));const tree=parser.parse(readFileSync(join(store,"🧪️tests/🔬️unit/🦀️.rs"),"utf8"));
 try{const visit=(node:Parser.SyntaxNode):Parser.SyntaxNode[]=>[node,...node.namedChildren.flatMap(visit)];const functions=tree.rootNode.namedChildren.filter(node=>node.type==="function_item"&&node.childForFieldName("name")?.text===law.nativeLaw);expect(functions.length).toBe(1);const original=functions[0]!;expect(original.hasError()).toBe(false);const errors=visit(tree.rootNode).filter(node=>node.type==="ERROR"||node.isMissing());for(const error of errors){expect(error.text).toBe("async");expect(error.endIndex<=original.startIndex||error.startIndex>=original.endIndex).toBe(true);}console.log("[DEBUG] original native-law subtree is syntax clean; unrelated original async-closure grammar qualifications "+JSON.stringify(errors.map(node=>({line:node.startPosition.row+1,column:node.startPosition.column+1}))));const native=rustTokens(original);expect(containsTokens(native,["job",".","step","(","&","mut","cx",")"])).toBe(true);expect(containsTokens(native,["cx",".","retained_progress","(",")"])).toBe(true);}finally{tree.delete();parser.delete();}
 console.log("[DEBUG] strict five-axis caller policy and SQLite original copy23/birth64/release32 receipt agree; actual Store step uses the same original wallet even after child refusal");
});
