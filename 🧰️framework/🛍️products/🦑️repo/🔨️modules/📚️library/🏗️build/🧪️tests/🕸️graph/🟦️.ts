import{test,expect}from"bun:test";
import Ajv from"ajv";
import{resolve,relative}from"node:path";
import{workspaceBuildTestGroupsV1,readWorkspaceBuildTestSelectionV1}from"../../🗂️suites/🌐️workspace/🟦️.ts";
import selection from"../../🗂️suites/🌐️workspace/🔣️.json"with{type:"json"};
import schema from"../../🗂️suites/🌐️workspace/🧬️schema/🔣️.json"with{type:"json"};
import baseline from"../../🗂️suites/🌐️workspace/🧪️tests/🔣️.json"with{type:"json"};
import{captureOwnedCommandV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-workspace-graph-laws",scratch:"checkout"});
const library=resolve(import.meta.dirname,"../../.."),base=resolve(import.meta.dirname,"../../🗂️suites/🌐️workspace");
test("workspace selection is closed and rejects duplicate, escaped and foreign leaves with Ajv parity",()=>{
 const validate=new Ajv({strict:true}).compile(schema);
 for(const[value,accepted]of[[selection,true],[{...selection,extra:true},false],[{...selection,files:[selection.files[0],selection.files[0]]},false],[{...selection,files:["../outside.ts"]},false]]as const){expect(validate(value)).toBe(accepted);if(accepted)expect(readWorkspaceBuildTestSelectionV1(value)).toEqual(selection);else expect(()=>readWorkspaceBuildTestSelectionV1(value)).toThrow();}
});
test("all 27 former entries and 48 leaves remain selected with independent SQLite closure",async()=>{
 const python=testPythonV1()??Bun.which("python3");if(!python)throw Error("Independent Python SQLite oracle required");
 const oracle=JSON.parse(await captureOwnedCommandV1(python,[resolve(base,"🧪️tests/🐍️.py"),resolve(base,"🧪️tests/🔣️.json"),resolve(base,"🔣️.json")],process.cwd(),{output:{maximumBytes:65536,maximumLines:4}}));
 expect(oracle.entryCount).toBe(27);expect(oracle.edgeCount).toBe(22);expect(oracle.leaves).toEqual([...baseline.leaves].sort());expect(oracle.leaves).toHaveLength(48);
 expect(selection.files.map(file=>relative(process.cwd(),resolve(library,file))).sort()).toEqual([...baseline.leaves,...baseline.added].sort());
 expect(oracle.selected).toEqual([...selection.files].sort());
});
test("every selected leaf keeps its canonical suite policy and has exactly one process owner",()=>{
 const groups=workspaceBuildTestGroupsV1(),files=groups.flatMap(group=>group.files.map(file=>relative(library,resolve(group.root,file))));
 expect(files.sort()).toEqual([...selection.files].sort());expect(new Set(files).size).toBe(49);
 for(const group of groups){expect(group.table.suites).toHaveLength(1);expect(group.table.suites[0]!.id).toBe(group.suite.id);const{trees,files:priorFiles,fileLaunchers,...policy}=group.suite;const{files:selectedFiles,fileLaunchers:selectedLaunchers,...selectedPolicy}=group.table.suites[0]!;expect(selectedPolicy).toEqual(policy);expect(selectedFiles).toEqual(group.files);for(const[file,launcher]of Object.entries(selectedLaunchers??{}))expect(fileLaunchers?.[file]).toBe(launcher);}
 const ledger=groups.find(group=>group.suite.id==="build-inputs-storage");expect(ledger?.files).toEqual(["📥️inputs/💾️storage/🧪️tests/📒️ledger/🟦️.ts"]);expect(ledger?.suite.budgetMs).toBe(300000);
});
