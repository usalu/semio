/** 🪡 Render two deliberately adjacent clip-style windows through the shared browser compositor. */
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import {createRequire} from "node:module";
import {fileURLToPath} from "node:url";
import {browserLaunchOptions,finishTransitions,launchBrowser} from "../🧰️tools.mjs";

const root=process.cwd(),output=path.resolve(process.argv[2]??""),here=path.dirname(fileURLToPath(import.meta.url));
assert(output.startsWith(root+path.sep)&&output.length<220);
const fixture=JSON.parse(await fs.readFile(path.join(here,"seam.cases.json"),"utf8")),require=createRequire(path.join(root,"package.json"));
const {build}=require("esbuild"),{compile}=require("@tailwindcss/node"),{Scanner}=require("@tailwindcss/oxide");
const ui=path.join(root,"🧰️framework/🔨️modules/🖱️ui"),adapter=path.join(root,"🧰️framework/🔨️modules/🏃️process/🧰️tools/🎭️playwright/🟦️.ts"),adapterFile=path.join(output,"seam-tools.cjs");
await fs.mkdir(output,{recursive:true});
await fs.writeFile(adapterFile,(await build({entryPoints:[adapter],bundle:true,write:false,platform:"node",format:"cjs",packages:"external",logLevel:"silent"})).outputFiles[0].contents);
const {loadPlaywrightV1,playwrightV1}=require(adapterFile),{chromium}=await loadPlaywrightV1(root),cssPath=path.join(ui,"🧵️styles/🎨️.css");
const compiler=await compile(await fs.readFile(cssPath,"utf8"),{base:path.dirname(cssPath),onDependency(){}}),scanner=new Scanner({sources:[{base:ui,pattern:"**/*.{ts,tsx}",negated:false},...compiler.sources]});
const entry=`import React from 'react';import {createRoot} from 'react-dom/client';import {WindowChrome} from ${JSON.stringify(path.join(ui,"🧱️elements/🗂️WindowChrome/🟦️.tsx"))};
const f=${JSON.stringify(fixture)};createRoot(document.getElementById('root')).render(<div className="semio-scope" style={{position:'absolute',left:20,top:20,width:2*f.width,height:f.height,overflow:'hidden',transform:'scale('+f.scale+')',transformOrigin:'0 0'}}>{['left','right'].map((id,i)=><WindowChrome key={id} level="window" active={i===1} stackDataAttrs={{'data-seam-window':id}} style={{position:'absolute',left:i*f.width,top:0,width:f.width,height:f.height}} titleChips={<span>{id}</span>} body={<span>{id}</span>}/>)}</div>);`;
const bundle=await build({stdin:{contents:entry,resolveDir:root,loader:"tsx"},bundle:true,write:false,format:"iife",platform:"browser",alias:{"@semio-tech/framework-transient":path.join(root,"🧰️framework/🔨️modules/🫧️transient/🟦️.ts"),"@semio-tech/framework-interaction":path.join(root,"🧰️framework/🔨️modules/🕹️interaction/🟦️.ts")},define:{"process.env.NODE_ENV":'"production"'},logLevel:"silent"});
const cancellation=new AbortController();
let browser;
const abort=()=>{cancellation.abort();process.exitCode=130;void browser?.close().catch(()=>{})};
process.once("SIGINT",abort);process.once("SIGTERM",abort);
try{
  browser=await launchBrowser({launch:()=>chromium.launch(browserLaunchOptions),provision:()=>playwrightV1(["install","chromium"],{cwd:root,heavy:false,signal:cancellation.signal,stdout:"stderr",onProgress:event=>console.error("[DEBUG] Chromium provision "+event.phase)}),signal:cancellation.signal});
  if(cancellation.signal.aborted)throw Error("Seam browser rendering cancelled");
  const page=await browser.newPage({viewport:{width:400,height:250},deviceScaleFactor:1,reducedMotion:"reduce"});
  await page.setContent('<html class="dark"><body style="margin:0;background:#000"><div id="root"></div></body></html>');
  await page.addStyleTag({content:compiler.build(scanner.scan())+'\npath[data-shared-border-state="active"]{stroke:rgb(255,0,0)!important}:root,html.dark,.semio-scope,html.dark .semio-scope{--border-emphasized-color:rgb(0,255,0)!important}'});
  await page.addScriptTag({content:bundle.outputFiles[0].text});
  const measure=()=>page.evaluate(()=>{
    const left=document.querySelector('[data-seam-window="left"]'),right=document.querySelector('[data-seam-window="right"]'),a=left.getBoundingClientRect(),b=right.getBoundingClientRect();
    const x=(a.right+b.left)/2,y=(Math.max(a.top,b.top)+Math.min(a.bottom,b.bottom))/2;
    const owners=[left,right].flatMap(element=>[...element.querySelectorAll('[data-window-silhouette-border] path[data-shared-border-segment]')].filter(path=>{const matrix=path.getScreenCTM();return matrix&&path.isPointInStroke(new DOMPoint(x,y).matrixTransform(matrix.inverse()))}).map(path=>({owner:element.dataset.seamWindow,state:path.dataset.sharedBorderState})));
    return {x,y,gap:Math.abs(a.right-b.left),owners};
  });
  const results=[];
  for(const sample of fixture.states){
    if(cancellation.signal.aborted)throw Error("Seam browser rendering cancelled");
    if(sample.id==="hover")await page.locator('[data-seam-window="left"]').hover({position:{x:20,y:80}});
    await page.waitForFunction(()=>document.querySelector('[data-shared-border-root]')&&!document.querySelector('[data-shared-border-pending]'));
    await finishTransitions(page);
    const actual=await measure();assert(actual.gap<0.01);assert.deepEqual(actual.owners,[{owner:sample.owner,state:sample.state}],sample.id);
    await page.screenshot({path:path.join(output,sample.id+".png")});results.push({id:sample.id,...actual});
  }
  await fs.writeFile(path.join(output,"seam.json"),JSON.stringify(results));
}catch(error){if(!cancellation.signal.aborted)throw error}
finally{process.removeListener("SIGINT",abort);process.removeListener("SIGTERM",abort);try{await browser?.close()}catch(error){if(!cancellation.signal.aborted)throw error}finally{await fs.rm(adapterFile,{force:true})}}
