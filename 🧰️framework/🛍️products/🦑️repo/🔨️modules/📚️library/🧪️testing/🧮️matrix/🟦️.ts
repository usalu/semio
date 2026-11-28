import {bindCatalogV1,textV1} from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {accessibleInstallScriptV1,describeAccessibleTargetV1,type AccessibleTargetV1} from "../♿️accessible/🟦️.ts";
import {browserLaunchOptionsV1,launchBrowserV1} from "../🌐️browser/🟦️.ts";
import {MountRefusalV1,parseMountPlanV1,runMountPlanV1,type MountReceiptV1,type MountStepV1} from "../🏔️mount/🟦️.ts";
import schema from "./🧬️schema/🔣️.json" with {type:"json"};
import texts from "./📖️texts/🔣️.json" with {type:"json"};

/** 📐️ The closed policy of every matrix.
 * @see ./🧬️schema/🔣️.json */
export const MATRIX_POLICY_V1=schema.definitions.Policy.const;

/** 📖️ The English and German texts of matrix verdicts.
 * @see ./📖️texts/🔣️.json */
export const MATRIX_TEXTS_V1=bindCatalogV1("matrix",texts);

/** 🌍️ A language of the matrix; every name exists in both. */
export type MatrixLanguageV1="en"|"de";
/** 🔤️ A text in English and German. */
export type MatrixLocalizedV1=Readonly<{en:string;de:string}>;
/** 🎯️ An element by role and bilingual accessible name. */
export type MatrixTargetV1=Readonly<{role:string;name:MatrixLocalizedV1;exact?:boolean;nth?:number;within?:MatrixTargetV1}>;
/** 🔬️ What an `expect` step requires of its target. */
export type MatrixStateV1=Readonly<{visible?:boolean;count?:number;pressed?:string;selected?:string;expanded?:string;checked?:string;disabled?:boolean;busy?:string;value?:string;text?:MatrixLocalizedV1}>;
/** 🪜️ One step of a row. */
export type MatrixStepV1=Readonly<{do:"click";target:MatrixTargetV1}|{do:"press";target:MatrixTargetV1;key:string;times?:number}|{do:"fill";target:MatrixTargetV1;text:MatrixLocalizedV1}|{do:"expect";target:MatrixTargetV1;state:MatrixStateV1}>;
/** 📋️ One row: what a user does in order; `budgetMs` is the deadline of each step. */
export type MatrixRowV1=Readonly<{id:string;description:MatrixLocalizedV1;budgetMs:number;steps:readonly MatrixStepV1[]}>;
/** 🧮️ A whole matrix: the app, its mount phases and landmarks, and its rows. */
export type MatrixV1=Readonly<{$schema:string;version:1;app:string;mount:Readonly<{phases:readonly Readonly<{id:string;budgetMs:number}>[];landmarks:readonly MatrixTargetV1[]}>;rows:readonly MatrixRowV1[]}>;

/** 👁️ The observable ARIA state of one located element. */
export type MatrixObservedV1=Readonly<{pressed:string|null;selected:string|null;expanded:string|null;checked:string|null;disabled:boolean;busy:string|null;value:string;text:string}>;

/** 🖥️ The page a row drives; implemented over Playwright by `openMatrixPageV1`, replaceable in tests. Every locator is a role and a name. */
export interface MatrixPageV1{
 goto(path:string,signal:AbortSignal):Promise<void>;
 count(target:AccessibleTargetV1):Promise<number>;
 act(target:AccessibleTargetV1,action:Readonly<{kind:"click"}|{kind:"press";key:string}|{kind:"fill";text:string}>,signal:AbortSignal):Promise<void>;
 read(target:AccessibleTargetV1):Promise<MatrixObservedV1>;
 probe<T>(expression:string):Promise<T>;
 faults():readonly string[];
 close():Promise<void>;
}

/** 🛂️ Validates a matrix against its schema and requires unique row ids; throws one bilingual reason. */
export function parseMatrixV1(value:unknown):MatrixV1{
 const errors=validateJsonSchemaSubset({$ref:"#/definitions/Matrix"},value,schema as never),matrix=value as MatrixV1;
 const problem=errors.length>0?String(errors[0]):new Set(matrix.rows.map(row=>row.id)).size!==matrix.rows.length?"duplicate row id":undefined;
 if(problem!==undefined)throw Error(both("matrix.invalid",{detail:problem.slice(0,400)}));
 return matrix;
}

/** 🌐️ A target in one language, as the resolver takes it. */
export function localizeTargetV1(target:MatrixTargetV1,language:MatrixLanguageV1):AccessibleTargetV1{
 return {role:target.role,name:target.name[language],...target.exact===undefined?{}:{exact:target.exact},...target.nth===undefined?{}:{nth:target.nth},...target.within===undefined?{}:{within:localizeTargetV1(target.within,language)}};
}

/** 🗺️ The step plan of one row: one phase per step, each with the row's step deadline.
 * @see ../🏔️mount/🟦️.ts */
export function matrixRowPlanV1(matrix:MatrixV1,row:MatrixRowV1){
 return parseMountPlanV1({version:1,app:matrix.app,phases:row.steps.map((step,index)=>({id:`step-${index+1}-${step.do}`,budgetMs:row.budgetMs}))});
}

/** 🏔️ Mounts the matrix app in one language: the app's own phases (`steps`, a domain extension) plus the generic `landmarks` phase, which requires every landmark of the matrix to be visible by role and name.
 * @see ../🏔️mount/🟦️.ts */
export function mountMatrixV1(matrix:MatrixV1,language:MatrixLanguageV1,page:MatrixPageV1,steps:Readonly<Record<string,MountStepV1<MatrixPageV1>>>,options:Parameters<typeof runMountPlanV1>[3]={}):Promise<MountReceiptV1>{
 const plan=parseMountPlanV1({version:1,app:matrix.app,phases:matrix.mount.phases});
 const landmarks:MountStepV1<MatrixPageV1>=async(port,signal)=>{for(const landmark of matrix.mount.landmarks)await settle(port,signal,async()=>{await single(port,localizeTargetV1(landmark,language),false);});};
 return runMountPlanV1(plan,{...steps,...plan.phases.some(phase=>phase.id==="landmarks")?{landmarks}:{}},page,options);
}

/** 🧮️ Runs one row fail-fast in one language on a mounted page: every step polls until it holds or its deadline ends, page faults refuse the step, the first failing step ends the row with its reason in English and German. */
export function runMatrixRowV1(matrix:MatrixV1,rowId:string,language:MatrixLanguageV1,page:MatrixPageV1,options:Parameters<typeof runMountPlanV1>[3]={}):Promise<MountReceiptV1>{
 const row=matrix.rows.find(candidate=>candidate.id===rowId);
 if(row===undefined)throw Error(both("matrix.row",{app:matrix.app,row:rowId}));
 const plan=matrixRowPlanV1(matrix,row);
 const steps=Object.fromEntries(row.steps.map((step,index)=>[plan.phases[index]!.id,(port:MatrixPageV1,signal:AbortSignal)=>runStep(port,step,language,signal)]));
 return runMountPlanV1(plan,steps,page,options);
}

/** 🎭️ Opens one browser page in one language (`en-US` or `de-DE`) on `origin`: installs the in-repo resolver before any page script, collects page errors, console errors, refused inputs and HTTP errors as faults, and locates every target twice — in-repo and with Playwright `getByRole` — refusing the step when they disagree. */
export async function openMatrixPageV1(input:Readonly<{origin:string;language:MatrixLanguageV1;viewport?:Readonly<{width:number;height:number}>}>):Promise<MatrixPageV1>{
 const browser=await launchBrowserV1(browserLaunchOptionsV1());
 try{
  const context=await browser.newContext({locale:MATRIX_POLICY_V1.browserLocales[input.language],viewport:input.viewport??{width:1600,height:1000}}),page=await context.newPage(),faults:string[]=[];
  await context.addInitScript(accessibleInstallScriptV1());
  page.on("pageerror",error=>faults.push(`pageerror ${error.message}`));
  page.on("console",message=>{if(message.type()==="error")faults.push(`console ${message.text()}`);else if(/^input #\d+ .* refused: /u.test(message.text()))faults.push(`refused ${message.text()}`);});
  page.on("response",response=>{if(response.status()>=400)faults.push(`http ${response.status()} ${response.url()}`);});
  const handles=(target:AccessibleTargetV1)=>page.evaluateHandle(value=>(window as unknown as {__semioAccessibleV1:{queryAll(root:ParentNode,target:unknown):Element[]}}).__semioAccessibleV1.queryAll(document,value),target);
  const theirs=(target:AccessibleTargetV1):import("playwright").Locator=>{const scope=target.within===undefined?page:theirs(target.within);const found=scope.getByRole(target.role as never,{name:target.name,exact:target.exact!==false});return target.nth===undefined?found:found.nth(target.nth);};
  const one=async(target:AccessibleTargetV1)=>{
   const list=await handles(target),count=await list.evaluate(items=>items.length);
   if(count!==1){await list.dispose();throw refusal(count===0?"step.missing":"step.ambiguous",{target:describeAccessibleTargetV1(target),count});}
   const element=(await list.getProperty("0")).asElement()!,other=theirs(target);
   const same=await other.count()===1&&await other.evaluate((node,ours)=>node===ours,element);
   if(!same)throw refusal("step.disagree",{target:describeAccessibleTargetV1(target)});
   return element;
  };
  return{
   async goto(path,signal){signal.throwIfAborted();await page.goto(new URL(path,input.origin).href,{waitUntil:"domcontentloaded"});},
   async count(target){const list=await handles(target);try{return await list.evaluate(items=>items.length);}finally{await list.dispose();}},
   async act(target,action,signal){
    signal.throwIfAborted();
    const element=await one(target);
    if(action.kind==="click")await element.click({timeout:5000});
    else if(action.kind==="press")await element.press(action.key,{timeout:5000});
    else await element.fill(action.text,{timeout:5000});
   },
   async read(target){
    const element=await one(target);
    return element.evaluate(node=>{const at=(name:string)=>node.getAttribute(name),control=node as HTMLInputElement;return{pressed:at("aria-pressed"),selected:at("aria-selected"),expanded:at("aria-expanded"),checked:at("aria-checked")??(control.type==="checkbox"||control.type==="radio"?String(control.checked):null),disabled:control.disabled===true||at("aria-disabled")==="true",busy:at("aria-busy"),value:at("aria-valuenow")??(typeof control.value==="string"?control.value:node.textContent??""),text:(node.textContent??"").replace(/\s+/gu," ").trim()};});
   },
   probe:expression=>page.evaluate(expression),
   faults:()=>[...faults],
   close:()=>browser.close(),
  };
 }catch(error){await browser.close();throw error;}
}

const text=(language:MatrixLanguageV1,key:string,values:Readonly<Record<string,string|number>>):string=>textV1(MATRIX_TEXTS_V1.catalog,key,language,values);
const both=(key:string,values:Readonly<Record<string,string|number>>):string=>`${text("en",key,values)} / ${text("de",key,values)}`;
const refusal=(key:string,values:Readonly<Record<string,string|number>>)=>{const used=key==="step.ambiguous"?values:Object.fromEntries(Object.entries(values).filter(([name])=>name!=="count"));return new MountRefusalV1(text("en",key,used),text("de",key,used));};
const sleep=(ms:number,signal:AbortSignal)=>new Promise<void>((resolve,reject)=>{const timer=setTimeout(resolve,ms);signal.addEventListener("abort",()=>{clearTimeout(timer);reject(signal.reason);},{once:true});});

async function settle(page:MatrixPageV1,signal:AbortSignal,attempt:()=>Promise<void>):Promise<void>{
 let last:unknown;
 while(!signal.aborted){
  const faults=page.faults();
  if(faults.length>0)throw refusal("step.faults",{faults:faults.slice(0,3).join(" | ").slice(0,300)});
  try{await attempt();return;}catch(error){last=error;}
  await sleep(MATRIX_POLICY_V1.pollMs,signal).catch(()=>undefined);
 }
 throw last??signal.reason;
}

async function single(page:MatrixPageV1,target:AccessibleTargetV1,quiet:boolean):Promise<void>{
 const count=await page.count(target);
 if(count!==1&&!quiet)throw refusal(count===0?"step.missing":"step.ambiguous",{target:describeAccessibleTargetV1(target),count});
}

function compare(target:AccessibleTargetV1,state:MatrixStateV1,observed:MatrixObservedV1,language:MatrixLanguageV1):void{
 const expected:Record<string,unknown>={},actual:Record<string,unknown>={};
 for(const key of ["pressed","selected","expanded","checked","disabled","busy","value"] as const)if(state[key]!==undefined&&state[key]!==observed[key]){expected[key]=state[key];actual[key]=observed[key];}
 if(state.text!==undefined&&observed.text!==state.text[language]){expected.text=state.text[language];actual.text=observed.text;}
 if(Object.keys(expected).length>0)throw refusal("step.state",{target:describeAccessibleTargetV1(target),actual:JSON.stringify(actual),expected:JSON.stringify(expected)});
}

async function runStep(page:MatrixPageV1,step:MatrixStepV1,language:MatrixLanguageV1,signal:AbortSignal):Promise<void>{
 const target=localizeTargetV1(step.target,language);
 if(step.do==="expect"){
  const {visible,count,...rest}=step.state;
  await settle(page,signal,async()=>{
   const found=await page.count(target);
   if(count!==undefined&&found!==count)throw refusal("step.state",{target:describeAccessibleTargetV1(target),actual:JSON.stringify({count:found}),expected:JSON.stringify({count})});
   if(visible===false&&found!==0)throw refusal("step.state",{target:describeAccessibleTargetV1(target),actual:JSON.stringify({visible:true}),expected:JSON.stringify({visible:false})});
   if(visible===false||count!==undefined&&Object.keys(rest).length===0)return;
   compare(target,rest,await page.read(target),language);
  });
  return;
 }
 await settle(page,signal,()=>single(page,target,false));
 if(step.do==="click")await page.act(target,{kind:"click"},signal);
 else if(step.do==="fill")await page.act(target,{kind:"fill",text:step.text[language]},signal);
 else for(let index=0;index<(step.times??1);index++)await page.act(target,{kind:"press",key:step.key},signal);
 const faults=page.faults();
 if(faults.length>0)throw refusal("step.faults",{faults:faults.slice(0,3).join(" | ").slice(0,300)});
}
