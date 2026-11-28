/** 🧮️ The interaction matrix without a browser: the schema admits only role-and-name targets (Ajv and the in-repo validator agree, selector fields are refused), a row runs its steps in order and stops at the first failing one with an English and German reason, and expectations poll until they hold. */
import Ajv2020 from "ajv/dist/2020.js";
import {describe,expect,test} from "bun:test";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {describeAccessibleTargetV1,type AccessibleTargetV1} from "../../♿️accessible/🟦️.ts";
import {localizeTargetV1,mountMatrixV1,parseMatrixV1,runMatrixRowV1,type MatrixObservedV1,type MatrixPageV1,type MatrixV1} from "../🟦️.ts";

const name=(en:string,de:string)=>({en,de});
const matrix=(extra:object={}):MatrixV1=>parseMatrixV1({$schema:"x",version:1,app:"forms",mount:{phases:[{id:"landmarks",budgetMs:500}],landmarks:[{role:"region",name:name("Design","Entwurf")}]},rows:[{id:"submit",description:name("Submit a response","Eine Antwort absenden"),budgetMs:400,steps:[{do:"fill",target:{role:"textbox",name:name("Name","Name")},text:name("Ada","Ada")},{do:"click",target:{role:"button",name:name("Submit","Absenden")}},{do:"expect",target:{role:"status",name:name("Saved","Gespeichert")},state:{text:name("Your response was saved.","Ihre Antwort wurde gespeichert.")}}]}],...extra});

/** 🧪️ A page whose elements are rows of role, name and state; `later` elements appear after a delay. */
function fakePage(elements:{role:string;name:string;text?:string;appearsAfterMs?:number}[],faults:string[]=[]):MatrixPageV1&{log:string[]}{
 const started=Date.now(),log:string[]=[];
 const visible=(target:AccessibleTargetV1)=>elements.filter(row=>row.role===target.role&&row.name===target.name&&Date.now()-started>=(row.appearsAfterMs??0));
 return{
  log,
  async goto(){},
  async count(target){return visible(target).length;},
  async act(target,action){log.push(`${action.kind} ${describeAccessibleTargetV1(target)}`);},
  async read(target):Promise<MatrixObservedV1>{const row=visible(target)[0]!;return{pressed:null,selected:null,expanded:null,checked:null,disabled:false,busy:null,value:"",text:row.text??""};},
  async probe(){return undefined as never;},
  faults:()=>faults,
  async close(){},
 };
}

describe("matrix schema",()=>{
 const ajv=new Ajv2020({strict:false}).compile({...schema,$ref:"#/definitions/Matrix"});
 test("a role-and-name matrix is admitted by both validators",()=>{
  const value=matrix();
  expect(ajv(value)).toBe(true);
  expect(localizeTargetV1({role:"button",name:name("Next","Weiter"),within:{role:"region",name:name("Try","Testen")}},"de")).toEqual({role:"button",name:"Weiter",within:{role:"region",name:"Testen"}});
 });
 for(const [label,mutate] of [
  ["a CSS selector field",(value:any)=>{value.rows[0].steps[1].target.selector="#submit";}],
  ["an XPath field",(value:any)=>{value.rows[0].steps[1].target.xpath="//button";}],
  ["a name in one language only",(value:any)=>{value.rows[0].steps[1].target.name={en:"Submit"};}],
  ["an unknown step",(value:any)=>{value.rows[0].steps[0]={do:"hover",target:value.rows[0].steps[0].target};}],
  ["a role with a selector character",(value:any)=>{value.rows[0].steps[1].target.role="button[type]";}],
 ] as const)test(`refuses ${label} in Ajv and in-repo`,()=>{
  const value=structuredClone(matrix()) as any;mutate(value);
  expect(ajv(value)).toBe(false);
  expect(()=>parseMatrixV1(value)).toThrow("Invalid interaction matrix");
 });
 test("refuses duplicate row ids",()=>{
  const value=structuredClone(matrix()) as any;value.rows.push(value.rows[0]);
  expect(()=>parseMatrixV1(value)).toThrow("duplicate row id");
 });
});

describe("matrix rows",()=>{
 test("a row runs every step in order in German and passes",async()=>{
  const page=fakePage([{role:"textbox",name:"Name"},{role:"button",name:"Absenden"},{role:"status",name:"Gespeichert",text:"Ihre Antwort wurde gespeichert.",appearsAfterMs:120}]);
  const receipt=await runMatrixRowV1(matrix(),"submit","de",page);
  expect(receipt.passed).toBe(true);
  expect(page.log).toEqual(['fill textbox "Name"','click button "Absenden"']);
 });
 test("the first missing element ends the row with the step and target in English and German",async()=>{
  const page=fakePage([{role:"textbox",name:"Name"}]);
  const receipt=await runMatrixRowV1(matrix(),"submit","en",page);
  expect(receipt.phases.map(row=>row.status)).toEqual(["passed","failed","skipped"]);
  expect(receipt.failure).toMatchObject({phase:"step-2-click",code:"deadline"});
  expect(receipt.failure?.en).toEndWith('no visible element is button "Submit"');
  expect(receipt.failure?.de).toEndWith('kein sichtbares Element ist button "Submit"');
  expect(page.log).toEqual(['fill textbox "Name"']);
 });
 test("a wrong state names the actual and expected values",async()=>{
  const page=fakePage([{role:"textbox",name:"Name"},{role:"button",name:"Submit"},{role:"status",name:"Saved",text:"Error"}]);
  const receipt=await runMatrixRowV1(matrix(),"submit","en",page);
  expect(receipt.failure?.phase).toBe("step-3-expect");
  expect(receipt.failure?.en).toContain('status "Saved" has {"text":"Error"}, expected {"text":"Your response was saved."}');
 });
 test("page faults refuse the step at once",async()=>{
  const receipt=await runMatrixRowV1(matrix(),"submit","en",fakePage([{role:"textbox",name:"Name"}],["pageerror boom"]));
  expect(receipt.failure).toMatchObject({phase:"step-1-fill"});
  expect(receipt.failure?.de).toContain("die Seite meldete Fehler: pageerror boom");
 });
 test("the mount requires every landmark by role and name",async()=>{
  expect((await mountMatrixV1(matrix(),"de",fakePage([{role:"region",name:"Entwurf"}]),{})).passed).toBe(true);
  const failed=await mountMatrixV1(matrix(),"en",fakePage([{role:"region",name:"Entwurf"}]),{});
  expect(failed.failure).toMatchObject({phase:"landmarks",code:"deadline"});
  expect(failed.failure?.en).toEndWith('last: no visible element is region "Design"');
  expect(failed.failure?.de).toEndWith('zuletzt: kein sichtbares Element ist region "Design"');
 });
});
