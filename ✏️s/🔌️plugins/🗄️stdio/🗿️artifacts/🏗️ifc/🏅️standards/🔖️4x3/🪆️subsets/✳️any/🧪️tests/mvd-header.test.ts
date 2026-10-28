import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020.js";
import schema from "../🧬️schema/👁️mvd/header.schema.json" with {type:"json"};
import cases from "../🧬️schema/👁️mvd/header-cases.json" with {type:"json"};
import {ifc4x3MvdDeclarations,ifc4x3MvdDeclarationsAsync,ifc4x3MvdHeaderMutation,ifc4x3MvdHeaderMutationAsync,type Ifc4x3MvdSelection,type Ifc4x3MvdDeclaration} from "../🧬️schema/👁️mvd/header.ts";
import {IfcEditHistory,ifcMutationDiff,inverseIfcMutation,applyIfcMutation} from "../🧬️schema/🧬️mutations/🟦️.ts";
import {parseIfc4x3Step,writeIfc4x3Step} from "../🚪️io/step.ts";
import type {IfcSnapshot,IfcValue} from "../🧬️schema/📸️snapshot/🟦️.ts";

const text=(value:string):IfcValue=>({kind:"string",value});
const header=(strings:string[]):IfcValue[]=>[{kind:"aggregate",value:strings.map(text)},text("2;1")];
const snapshot=(strings:string[]):IfcSnapshot=>({schema:"stdio.ifc",header:{fileDescription:header(strings),fileName:[text("mvd.ifc"),text("2026-10-21T12:00:00"),{kind:"aggregate",value:[text("Author")]},{kind:"aggregate",value:[text("Organisation")]},text("Writer"),text("Application"),text("Authorisation")],fileSchema:[{kind:"aggregate",value:[text("IFC4X3_ADD2")]}]},entities:[]});

test("closed neutral selection schema admits only exact released names",()=>{
 const admit=new Ajv({strict:true}).compile(schema);for(const row of cases.cases)expect(admit({view:row.view})).toBe(true);
 for(const value of [{},{view:"Alignment-basedView"},{view:"ReferenceView",conforming:true},{view:undefined}])expect(admit(value)).toBe(false);
});
test("neutral header selections share central diff, exact inverse and revision history",async()=>{
 for(const row of cases.cases){const base=snapshot(row.before),selection={view:row.view} as Ifc4x3MvdSelection,mutation=ifc4x3MvdHeaderMutation(base.header.fileDescription,selection);expect(mutation).toEqual({mutation:"setFileDescription",values:header(row.after)});expect(await ifc4x3MvdHeaderMutationAsync(base.header.fileDescription,selection)).toEqual(mutation);
  const changed=applyIfcMutation(base,mutation),inverse=inverseIfcMutation(base,mutation);expect(changed.header.fileDescription).toEqual(header(row.after));expect(changed.header.fileName).toEqual(base.header.fileName);expect(changed.header.fileSchema).toEqual(base.header.fileSchema);expect(applyIfcMutation(changed,inverse)).toEqual(base);
  expect((await parseIfc4x3Step(await writeIfc4x3Step(changed))).header).toEqual(changed.header);const history=IfcEditHistory.createSync(base),event=history.execute(mutation,0);if(JSON.stringify(row.before)===JSON.stringify(row.after)){expect(event).toBeUndefined();expect(ifcMutationDiff(base,mutation)).toEqual({});}else{expect(event?.diff.fileDescription).toEqual(header(row.after));history.undo(1);expect(history.snapshot).toEqual(base);history.redo(2);expect(history.snapshot).toEqual(changed);}
 }
});
test("neutral declarations preserve unknown names and expose malformed syntax",async()=>{
 for(const row of cases.declarations){const expected={index:0,views:row.views,known:row.known as Ifc4x3MvdDeclaration["known"],malformed:row.malformed};expect(ifc4x3MvdDeclarations(header([row.text]))).toEqual([expected]);expect(await ifc4x3MvdDeclarationsAsync(header([row.text]))).toEqual([expected]);if(row.malformed)expect(()=>ifc4x3MvdHeaderMutation(header([row.text]),{view:"ReferenceView"})).toThrow("malformedMvdDeclaration");}
 for(const value of cases.unrelated)expect(ifc4x3MvdDeclarations(header([value]))).toEqual([]);
});
test("invalid header, invalid selection and caller aliases never become authority",()=>{
 for(const values of [[],[text("broken"),text("2;1")],[{kind:"aggregate",value:[]},text("2;1")],[{kind:"aggregate",value:[{kind:"integer",value:1n}]},text("2;1")]])expect(()=>ifc4x3MvdHeaderMutation(values as IfcValue[],{view:"ReferenceView"})).toThrow("invalidMvdHeader");
 for(const selection of [{view:"Alignment-basedView"},{view:null,extra:true},{}])expect(()=>ifc4x3MvdHeaderMutation(header(["Comment [keep]"]),selection as Ifc4x3MvdSelection)).toThrow("invalidMvdSelection");
 const source=header(["Comment [keep]"]),mutation=ifc4x3MvdHeaderMutation(source,{view:"ReferenceView"});(source[1] as {kind:"string";value:string}).value="changed";expect(mutation.values[1]).toEqual(text("2;1"));
});
test("long declaration parsing yields cancellation before publication",async()=>{
 const controller=new AbortController();let ticks=0;await expect(ifc4x3MvdHeaderMutationAsync(header([`ViewDefinition [${"x".repeat(65536)}]`]),{view:"ReferenceView"},{signal:controller.signal,yieldEvery:1,onProgress:progress=>{if(progress.phase==="admitInput"&&progress.total===65553&&progress.completed>256){ticks++;controller.abort();}}})).rejects.toThrow("cancelled");expect(ticks).toBe(1);
});
