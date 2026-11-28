/** 🎡️ The Play extension of the framework's fail-fast mount and interaction matrix: how a Play pane mounts (deep link, shell beacon, quiet settle, first-use tour) and how a matrix row file becomes one bun test per language that reads the service origin forwarded to whichever machine runs it.
 * @see ../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧮️matrix/🟦️.ts
 * @see ../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏔️mount/🟦️.ts */
import {test} from "bun:test";
import {bindCatalogV1,textV1} from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {FLEET_SERVICE_ENVIRONMENT_V1} from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🛰️fleet-service/🟦️.ts";
import {assertMountedV1,MountRefusalV1,mountProgressLineV1,type MountStepV1} from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🏔️mount/🟦️.ts";
import {localizeTargetV1,mountMatrixV1,openMatrixPageV1,parseMatrixV1,runMatrixRowV1,type MatrixLanguageV1,type MatrixPageV1,type MatrixTargetV1,type MatrixV1} from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️testing/🧮️matrix/🟦️.ts";
import texts from "./📖️texts/🔣️.json" with {type:"json"};

const TEXTS=bindCatalogV1("play-matrix",texts);
const both=(key:string,values:Readonly<Record<string,string|number>>={})=>new MountRefusalV1(textV1(TEXTS.catalog,key,"en",values),textV1(TEXTS.catalog,key,"de",values));
const sleep=(ms:number,signal:AbortSignal)=>new Promise<void>((resolve,reject)=>{const timer=setTimeout(resolve,ms);signal.addEventListener("abort",()=>{clearTimeout(timer);reject(signal.reason);},{once:true});});

/** 🧭️ What a Play pane needs to mount: its pane id, the quiet window after the shell reports ready, and the control that closes its first-use tour. */
export type PlayPaneMountV1=Readonly<{pane:string;settleMs:number;introduction:MatrixTargetV1}>;

/** 🏔️ The Play phases: `navigate` deep-links `/#<pane>` (the overview boots exactly that pane and nothing else), `shell` waits for the pane's own shell beacon and refuses its error text, `settle` requires a quiet window without page faults, `introduction` closes the first-use tour by role and name when it is shown. */
export function playMountStepsV1(mount:PlayPaneMountV1,language:MatrixLanguageV1):Readonly<Record<string,MountStepV1<MatrixPageV1>>>{
 const beacon=`(()=>{const shell=document.querySelector(${JSON.stringify(`[data-shell-id="${mount.pane}"]`)});if(!shell)return null;const data=shell.dataset;return data.shellReady!==undefined?"ready":data.shellError!==undefined?"error:"+(data.shellError||shell.textContent||"").slice(0,200):data.shellNotFound!==undefined?"not-found":null;})()`;
 return{
  navigate:(page,signal)=>page.goto(`/#${mount.pane}`,signal),
  async shell(page,signal){
   for(;;){
    const state=await page.probe<string|null>(beacon);
    if(state==="ready")return;
    if(state==="not-found")throw both("shell.not-found",{pane:mount.pane});
    if(state?.startsWith("error:"))throw both("shell.error",{pane:mount.pane,detail:state.slice(6)});
    const faults=page.faults();
    if(faults.length>0)throw both("shell.faults",{pane:mount.pane,faults:faults.slice(0,3).join(" | ").slice(0,300)});
    await sleep(100,signal);
   }
  },
  async settle(page,signal){
   await sleep(mount.settleMs,signal);
   const faults=page.faults();
   if(faults.length>0)throw both("shell.faults",{pane:mount.pane,faults:faults.slice(0,3).join(" | ").slice(0,300)});
  },
  async introduction(page,signal){
   const target=localizeTargetV1(mount.introduction,language);
   if(await page.count(target)===0)return;
   await page.act(target,{kind:"click"},signal);
   while(await page.count(target)>0)await sleep(100,signal);
  },
 };
}

/** 🌐️ The origin of the Play service under test: forwarded to the machine that runs the file (`SEMIO_TEST_SERVICE_ORIGIN`), refused with a bilingual reason when absent. */
export function playServiceOriginV1(environment:Readonly<Record<string,string|undefined>>=process.env):string{
 const origin=environment[FLEET_SERVICE_ENVIRONMENT_V1.origin];
 if(origin===undefined||!/^https?:\/\/[^/\s]+$/u.test(origin))throw both("origin.missing",{variable:FLEET_SERVICE_ENVIRONMENT_V1.origin});
 return origin;
}

/** 🧪️ Declares the bun tests of one matrix file: the `mount` file mounts the pane in English and in German; a row file mounts it and runs that row, per language in its own browser. Every test is fail-fast and throws the first failure in English and German; progress lines go to standard error. */
export function playMatrixTestsV1(document:unknown,mount:PlayPaneMountV1,selection:Readonly<{row:string}>|"mount"):void{
 const matrix:MatrixV1=parseMatrixV1(document);
 const row=selection==="mount"?undefined:matrix.rows.find(candidate=>candidate.id===selection.row);
 if(selection!=="mount"&&row===undefined)throw Error(`${matrix.app} has no row ${selection.row}`);
 const budget=matrix.mount.phases.reduce((sum,phase)=>sum+phase.budgetMs,0)+(row===undefined?0:row.steps.length*row.budgetMs)+15_000;
 for(const language of ["en","de"] as const){
  const name=row===undefined?`${matrix.app} mount ${language}`:`${matrix.app} ${row.id} ${language}`;
  test(name,async()=>{
   const report=(row:Parameters<typeof mountProgressLineV1>[2],index:number,count:number)=>process.stderr.write(mountProgressLineV1(language,matrix.app,row,index,count)+"\n");
   const page=await openMatrixPageV1({origin:playServiceOriginV1(),language});
   try{
    assertMountedV1(await mountMatrixV1(matrix,language,page,playMountStepsV1(mount,language),{onPhase:report}));
    if(row!==undefined)assertMountedV1(await runMatrixRowV1(matrix,row.id,language,page,{onPhase:report}));
   }finally{await page.close();}
  },budget);
 }
}
