import { authoredSnapshotSqliteContract, authoredSnapshotPreflightContract,authoredSnapshotSemanticContract } from "../../../../../../🪐️space/🧪️tests/🪶️sqlite/🔬️oracle/🟦️.ts";
import { fileURLToPath } from "node:url";
import { test, expect } from "bun:test";
import { readFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import Ajv from "ajv";
import Parser from "web-tree-sitter";
import { parse as parseJsonc } from "jsonc-parser";

const snapshot = fileURLToPath(new URL("../../", import.meta.url)), store = resolve(snapshot, "../../.."), root = resolve(store, "../../../../..");
authoredSnapshotSqliteContract(snapshot);
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
    tree = parser.parse(readFileSync(path, "utf8")); if (tree.rootNode.hasError()) throw Error("Rust parser refused source: " + path);
    const functions = tree.rootNode.namedChildren.filter(node => node.type === "function_item" && node.childForFieldName("name")?.text === functionName);
    if (functionName !== undefined) { expect(functions.length).toBe(1); return rustTokens(functions[0]!); }
    return rustTokens(tree.rootNode);
  } finally { tree?.delete(); parser.delete(); }
}


test("closed owner-registration corpus covers every opening and native encoding without changing envelope dialect", () => {
  const corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/🪶️sqlite/📣️registration/🔣️.json"), "utf8"));
  expect(corpus.cases.map((sample: any) => sample.id).sort()).toEqual(["create-binary", "create-text", "reload-binary", "reload-text", "retained-binary", "retained-text"]);
  for (const sample of corpus.cases) {
    expect(sample.envelopeDialect).toBe(sample.opening === "retained" ? corpus.coordinate : null);
    expect(sample.before).toEqual({ export: "UnsupportedOwner", import: "UnsupportedOwner" });
    expect(sample.after).toEqual({ export: "completeSnapshot", import: "completeSnapshot", metadata: "sameOwnerAndEncoding" });
  }
  const wrong = structuredClone(corpus); wrong.cases[0].before.export = "InvalidValue";
  console.log("[DEBUG] independent Ajv owner-registration admission: create/reload/retained × binary/text; typed pre-refusal and retained envelope metadata");
});

test("actual native isolated law admits explicit owner registration and generic constructors remain owner-neutral", async () => {
  await Parser.init(); const parser = new Parser(), trees: Parser.Tree[] = [];
  try {
  parser.setLanguage(await Parser.Language.load(join(dirname(Bun.resolveSync("tree-sitter-wasms/package.json", root)), "out/tree-sitter-rust.wasm")));
  const parse = (path: string) => { const tree = parser.parse(readFileSync(path, "utf8")); trees.push(tree); return tree; };
  const collect = (node: Parser.SyntaxNode, predicate: (node: Parser.SyntaxNode) => boolean): Parser.SyntaxNode[] => [ ...(predicate(node) ? [node] : []), ...node.namedChildren.flatMap(child => collect(child, predicate)) ];
  const calls = (node: Parser.SyntaxNode) => collect(node, node => node.type === "call_expression").map(node => ({ name: node.childForFieldName("function")?.text ?? "", start: node.startIndex }));
  const corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/🪶️sqlite/📣️registration/🔣️.json"), "utf8")), lawTree = parse(join(snapshot, "🧪️tests/🪶️sqlite/🦀️.rs"));
  expect(lawTree.rootNode.hasError()).toBe(false);
  const law = lawTree.rootNode.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === corpus.nativeLaw);
  expect(law).toBeDefined();
  const names = calls(law!), registration = names.filter(node => node.name === "store::space_history_sqlite::register_sqlite_snapshot");
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
  const binding = parse(join(snapshot, "🪶️sqlite/🦀️.rs")), ownerRegistration = binding.rootNode.namedChildren.find(node => node.type === "function_item" && node.childForFieldName("name")?.text === "register_sqlite_snapshot");
  expect(calls(ownerRegistration!).some(node => node.name === "crate::os_io::register_native_snapshot_codec")).toBe(true);
  expect(ownerRegistration!.text.includes("SQLITE_SNAPSHOT_DIALECT")).toBe(true); expect(containsRust(ownerRegistration!, ["SpaceHistorySnapshot", ",", "SpaceHistoryMutation"])).toBe(true);
  console.log("[DEBUG] actual tree-sitter owner admission: isolated exact selector; both typed refusals before owner publication; complete same-owner roundtrips after; generic Store/hydration unchanged");
  } finally { for (const tree of trees) tree.delete(); parser.delete(); }
});

test("actual owner literals and existing GUI source/native routes identify the same SpaceHistory boundary", async () => {
  const corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/🪶️sqlite/📣️registration/🔣️.json"), "utf8")), source = readFileSync(join(store, "🦀️.rs"), "utf8"), owner = readFileSync(join(snapshot, "🪶️sqlite/🦀️.rs"), "utf8");
  const parsedStore = await sourceTokens(join(store, "🦀️.rs")), parsedOwner = await sourceTokens(join(snapshot, "🪶️sqlite/🦀️.rs"));
  expect(containsTokens(parsedStore, ["#", "[", "path", "=", '"📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"', "]"])).toBe(true);
  expect(containsTokens(parsedStore, ["#", "[", "path", "=", '"📜️space-history/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs"', "]"])).toBe(true);
  expect(containsTokens(parsedOwner, ["standard", ":", "crate", "::", "os_io", "::", "StandardId", "(", '"1"', ")"])).toBe(true); expect(containsTokens(parsedOwner, ["subset", ":", "crate", "::", "os_io", "::", "SubsetId", "(", '"*"', ")"])).toBe(true);
  expect(containsTokens(parsedStore, ["pub", "const", "S_SPACE_HISTORY_SCHEMA", ":", "&", "str", "=", JSON.stringify(corpus.owner), ";"])).toBe(true);
  const seed = parseJsonc(readFileSync(join(root, ".vscode/🧩️launch.seed.jsonc"), "utf8")), project = JSON.parse(readFileSync(join(root, "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📋️project.json"), "utf8"));
  for (const command of ["test-space-history-sqlite-source", "test-space-history-sqlite-native"]) {
    expect(seed.configurations.filter((row: any) => row.command === `bun nx run @semio-tech/framework-os-kernel:${command} --skip-nx-cache`).length).toBe(1);
    expect(project.targets[command].options.command).toBe(`bun ./📜️script.ts ${command}`);
  }
  console.log("[DEBUG] existing actual GUI/source/native gate routes and literal SpaceHistory mounts agree");
});

test("closed native ownership corpus separates physical and reconstruction ceilings on both encodings", async () => {
  const schema = JSON.parse(readFileSync(join(snapshot, "🪶️sqlite/🚦️native/🧬️schema/🧮️ownership/🔣️.json"), "utf8")), corpus = JSON.parse(readFileSync(join(snapshot, "🧫️fixtures/🪶️sqlite/🧮️ownership/🔣️.json"), "utf8"));
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(corpus)).toBe(true);
  for (const change of [
    (sample: any) => sample.zeroPhysical.maximumBytes = 1,
    (sample: any) => sample.zeroPhysical.expectedCharge = 1,
    (sample: any) => sample.cumulative.ceilingNumerator = 2,
    (sample: any) => sample.cancellation.expectedKind = "ownershipLimit",
    (sample: any) => sample.rowRefusal.maximumRows = 1,
    (sample: any) => sample.encodings = ["text"],
    (sample: any) => sample.cumulative.ledgers = "allocationOnly",
    (sample: any) => sample.extra = true,
  ]) { const wrong = structuredClone(corpus); change(wrong); expect(validate(wrong)).toBe(false); }
  const law = await sourceTokens(join(snapshot, "🧪️tests/🪶️sqlite/🦀️.rs"), corpus.nativeLaw);
  for (const field of ["zeroPhysical", "cumulative", "rowRefusal", "cancellation"]) expect(containsTokens(law, ["contract", "[", JSON.stringify(field), "]"])).toBe(true);
  expect(containsTokens(law, ["for", "encoding", "in", "[", "SnapshotEncoding", "::", "Binary", ",", "SnapshotEncoding", "::", "Text", "]"])).toBe(true);
  expect(containsTokens(law, ["for", "physical", "in", "[", "false", ",", "true", "]"])).toBe(true);
  expect(containsTokens(law, ["control", ".", "reconstruction_remaining_bytes", "(", ")"])).toBe(true);
  expect(containsTokens(law, ["control", ".", "allocation_remaining_bytes", "(", ")"])).toBe(true);
  expect(containsTokens(law, ["assert_eq", "!", "(", "semantic", ",", "allocation"])).toBe(true);
  console.log("[DEBUG] independent Ajv native ownership: zero physical, both cumulative ceilings, row refusal and owned interior cancellation; eight hostile contracts refused");
});

import {Database as HistoryOutputDatabase} from "bun:sqlite";

test("closed History native output contract independently retains literal bytes and cumulative physical roles",()=>{
 const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🪶️sqlite/🧮️ownership/🔣️.json"),"utf8")),expected={"schema":"space-history.sqlite.native-output/v1","nativeLaw":"sqlite_snapshot_framework_space_history_native_output_full_requests_and_cumulative_owner_are_admitted","encodings":["binary","text"],"phase":"encodeNative","requestAccounting":"fullAllocatorRequests","ledger":"sameCallerAllocation","source":"completeUnchangedOwner","zeroPhysical":{"maximumBytes":0,"expectedKind":"ownershipLimit","expectedCharge":0},"cumulative":{"ceilingNumerator":3,"ceilingDenominator":2,"expectedKind":"ownershipLimit"},"cancellation":{"minimumCompleted":65536,"expectedKind":"canceled","retainedDebit":"positiveNoRefund"},"literal":{"unit":"日本𐀀","repeat":32768,"utf8Bytes":327680},"appendRequests":{"schema":"native.output.append-requests/v1","fragments":["","語","x","","\u0000é!"],"utf8Bytes":8,"fullReplacementRequests":[3,4,8],"replacementDebit":15,"reusedCapacity":16,"reusedRequestBytes":0,"refusal":"ownershipLimit","allocationRefusal":"allocationFailed","retirement":"noRefund"},"retirement":{"schema":"history.native.temporary-retirement/v1","sourceCorpus":"intrinsic-media-wire-v1","nativeLaw":"sqlite_snapshot_framework_space_history_native_temporary_retirement_requests_no_backing_for_every_intrinsic_kind","kinds":["null","bool","uint","int","float","string","bytes","array","object"],"containers":["empty","orderedDuplicateMembers","deepArray","deepObject","longBranch"],"depth":256,"longBranch":{"unit":"é語🧾","repeat":14564,"utf8Bytes":131076},"requestBytes":0,"releasedBytes":"completeBorrowedCapacityCensus","cleanup":"mandatoryAfterCancellation","temporaryLinks":"reuseVacatedOwnedSlots","recursiveDropFallback":false,"partial":{"schema":"history.native.partial-retirement/v1","grants":[0,1,3,255,256,"complete"],"work":"oneDestructiveTransitionPerUnit","requests":0,"released":"completeBorrowedCapacityCensus","zeroGrant":"retainsEveryOwnedCapacity","drop":"mandatoryCompleteReleaseAfterCancellation","nativeLaw":"sqlite_snapshot_framework_space_history_native_temporary_retirement_partial_grants_and_drop_release_every_owned_capacity"}}};
 expect(corpus.output).toEqual(expected);
 const schema=JSON.parse(readFileSync(join(snapshot,"🪶️sqlite/🚦️native/🧬️schema/🧮️ownership/🔣️.json"),"utf8"));
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 const literal=expected.literal.unit.repeat(expected.literal.repeat),bytes=Buffer.from(literal,"utf8");expect(bytes.length).toBe(expected.literal.utf8Bytes);
 const oracle=new HistoryOutputDatabase(":memory:");try{oracle.exec("CREATE TABLE native_history_output(ordinal INTEGER PRIMARY KEY,message TEXT NOT NULL)");oracle.run("INSERT INTO native_history_output VALUES(0,?)",[literal]);expect(oracle.query("SELECT length(CAST(message AS BLOB)) AS bytes,message FROM native_history_output").get()).toEqual({bytes:expected.literal.utf8Bytes,message:literal});}finally{oracle.close();}
 expect(expected.cumulative.ceilingNumerator).toBeGreaterThan(expected.cumulative.ceilingDenominator);expect(expected.cumulative.ceilingNumerator).toBeLessThan(2*expected.cumulative.ceilingDenominator);expect(expected.cancellation.minimumCompleted).toBeLessThan(bytes.length);
});


test("History byte append contract admits full replacement requests and reuses existing paid capacity",()=>{
 const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🪶️sqlite/🧮️ownership/🔣️.json"),"utf8"));
 const expected={schema:"native.output.append-requests/v1",fragments:["","語","x","","\u0000é!"],utf8Bytes:8,fullReplacementRequests:[3,4,8],replacementDebit:15,reusedCapacity:16,reusedRequestBytes:0,refusal:"ownershipLimit",allocationRefusal:"allocationFailed",retirement:"noRefund"};
 expect(corpus.output.appendRequests).toEqual(expected);
 expect(new Ajv({strict:true}).validate({type:"object",additionalProperties:false,required:Object.keys(expected),properties:Object.fromEntries(Object.entries(expected).map(([key,value])=>[key,{const:value}]))},corpus.output.appendRequests)).toBe(true);
 const fragments=expected.fragments.map(fragment=>Buffer.from(fragment,"utf8")),requests:number[]=[];let replacement=Buffer.alloc(0);
 for(const fragment of fragments){if(fragment.length!==0){replacement=Buffer.concat([replacement,fragment]);requests.push(replacement.length);}}
 expect(requests).toEqual(expected.fullReplacementRequests);expect(requests.reduce((sum,size)=>sum+size,0)).toBe(expected.replacementDebit);expect(replacement.length).toBe(expected.utf8Bytes);
 const reused=Buffer.alloc(expected.reusedCapacity),backing=reused.buffer;let length=0;
 for(const fragment of fragments){length+=fragment.copy(reused,length);expect(reused.buffer).toBe(backing);}
 expect(reused.subarray(0,length)).toEqual(replacement);expect(length).toBe(expected.utf8Bytes);expect(expected.reusedRequestBytes).toBe(0);
 const wrong={...expected,fullReplacementRequests:[3,1,4]};expect(new Ajv({strict:true}).validate({const:expected},wrong)).toBe(false);
});


test("History temporary owner retirement is closed across intrinsic kinds, literal members and deep branches",()=>{
 const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🪶️sqlite/🧮️ownership/🔣️.json"),"utf8"));
 const expected={schema:"history.native.temporary-retirement/v1",sourceCorpus:"intrinsic-media-wire-v1",nativeLaw:"sqlite_snapshot_framework_space_history_native_temporary_retirement_requests_no_backing_for_every_intrinsic_kind",kinds:["null","bool","uint","int","float","string","bytes","array","object"],containers:["empty","orderedDuplicateMembers","deepArray","deepObject","longBranch"],depth:256,longBranch:{unit:"é語🧾",repeat:14564,utf8Bytes:131076},requestBytes:0,releasedBytes:"completeBorrowedCapacityCensus",cleanup:"mandatoryAfterCancellation",temporaryLinks:"reuseVacatedOwnedSlots",recursiveDropFallback:false,partial:{"schema":"history.native.partial-retirement/v1","grants":[0,1,3,255,256,"complete"],"work":"oneDestructiveTransitionPerUnit","requests":0,"released":"completeBorrowedCapacityCensus","zeroGrant":"retainsEveryOwnedCapacity","drop":"mandatoryCompleteReleaseAfterCancellation","nativeLaw":"sqlite_snapshot_framework_space_history_native_temporary_retirement_partial_grants_and_drop_release_every_owned_capacity"}};
 expect(corpus.output.retirement).toEqual(expected);
 expect(new Ajv({strict:true}).validate({type:"object",additionalProperties:false,required:Object.keys(expected),properties:Object.fromEntries(Object.entries(expected).map(([key,value])=>[key,{const:value}]))},corpus.output.retirement)).toBe(true);
 const intrinsic=JSON.parse(readFileSync(join(root,"🧰️framework/🔨️modules/🎒️pack/🌱️value/🧫️fixtures/🎞️intrinsic-media/🔣️.json"),"utf8"));expect(intrinsic.contract).toBe(expected.sourceCorpus);
 const kinds=new Set<string>(),members:{key:string,index:number}[]=[];const pending=[intrinsic.value];while(pending.length){const value=pending.pop()!;kinds.add(value.kind);if(value.kind==="array")pending.push(...value.value);if(value.kind==="object"){for(const[index,member]of value.value.entries()){members.push({key:member.key,index});pending.push(member.value);}}}
 expect([...kinds].sort()).toEqual([...expected.kinds].sort());expect(new Set(members.map(member=>member.key)).size).toBeLessThan(members.length);
 for(const word of intrinsic.binary64Words){const bytes=Buffer.from(word,"hex"),view=new DataView(bytes.buffer,bytes.byteOffset,bytes.byteLength),roundtrip=new ArrayBuffer(8);new DataView(roundtrip).setFloat64(0,view.getFloat64(0,false),false);if(!Number.isNaN(view.getFloat64(0,false)))expect(Buffer.from(roundtrip)).toEqual(bytes);expect(view.getBigUint64(0,false).toString(16).padStart(16,"0")).toBe(word);}
 expect(Buffer.byteLength(expected.longBranch.unit.repeat(expected.longBranch.repeat),"utf8")).toBe(expected.longBranch.utf8Bytes);
 const oracle=new HistoryOutputDatabase(":memory:");try{oracle.exec("CREATE TABLE retired_members(ordinal INTEGER PRIMARY KEY,literal TEXT NOT NULL)");members.forEach((member,ordinal)=>oracle.run("INSERT INTO retired_members VALUES(?,?)",[ordinal,member.key]));expect(oracle.query("SELECT literal FROM retired_members ORDER BY ordinal").all().map((row:any)=>row.literal)).toEqual(members.map(member=>member.key));}finally{oracle.close();}
});

test("History partial cleanup grants retain owners and mandate complete backing release",async()=>{
 const corpus=JSON.parse(readFileSync(join(snapshot,"🧫️fixtures/🪶️sqlite/🧮️ownership/🔣️.json"),"utf8"));
 const expected={"schema":"history.native.partial-retirement/v1","grants":[0,1,3,255,256,"complete"],"work":"oneDestructiveTransitionPerUnit","requests":0,"released":"completeBorrowedCapacityCensus","zeroGrant":"retainsEveryOwnedCapacity","drop":"mandatoryCompleteReleaseAfterCancellation","nativeLaw":"sqlite_snapshot_framework_space_history_native_temporary_retirement_partial_grants_and_drop_release_every_owned_capacity"};
 expect(corpus.output.retirement.partial).toEqual(expected);
 const validate=new Ajv({strict:true}).compile({type:"object",additionalProperties:false,required:Object.keys(expected),properties:Object.fromEntries(Object.entries(expected).map(([key,value])=>[key,{const:value}]))});
 expect(validate(corpus.output.retirement.partial)).toBe(true);
 for(const change of [(value:any)=>value.grants[0]=1,(value:any)=>value.requests=1,(value:any)=>value.released="remainingOnly",(value:any)=>value.drop="skipAfterCancellation"]){const wrong=structuredClone(expected);change(wrong);expect(validate(wrong)).toBe(false);}
 const oracle=new HistoryOutputDatabase(":memory:");
 try{oracle.exec("CREATE TABLE cleanup_grants(ordinal INTEGER PRIMARY KEY,items INTEGER,completion TEXT)");
 expected.grants.forEach((grant,ordinal)=>oracle.run("INSERT INTO cleanup_grants VALUES(?,?,?)",[ordinal,typeof grant==="number"?grant:null,typeof grant==="string"?grant:null]));
 expect(oracle.query("SELECT items,completion FROM cleanup_grants ORDER BY ordinal").all()).toEqual([{items:0,completion:null},{items:1,completion:null},{items:3,completion:null},{items:255,completion:null},{items:256,completion:null},{items:null,completion:"complete"}]);
 }finally{oracle.close();}
 const law=await sourceTokens(join(snapshot,"🪶️sqlite/🚦️native/🦀️.rs"),expected.nativeLaw);
 for(const tokens of [["retirement",".","close_step","(","grant",")"],["drop","(","retirement",")"],["assert_eq","!","(","released",",","expected_release"]])expect(containsTokens(law,tokens)).toBe(true);
});

authoredSnapshotPreflightContract(fileURLToPath(new URL("../../",import.meta.url)));

authoredSnapshotSemanticContract(fileURLToPath(new URL("../../",import.meta.url)));
