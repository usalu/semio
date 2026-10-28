/** 🧪 Measure the same layered components used by the course, play and demonstrator in Chromium. */
import assert from "node:assert/strict";
import fs from "node:fs/promises";
import path from "node:path";
import {fileURLToPath} from "node:url";
import {createRequire} from "node:module";

const root=process.cwd(),here=path.dirname(fileURLToPath(import.meta.url));
const output=path.resolve(process.argv[2]??"");
assert(output.startsWith(root+path.sep)&&output.length<220,"The oracle requires an owned short output directory");
await fs.mkdir(output,{recursive:true});
const fixture=JSON.parse(await fs.readFile(path.join(here,"🔣️.cases.json"),"utf8"));
const require=createRequire(path.join(root,"package.json"));
const {build}=require("esbuild"),{compile}=require("@tailwindcss/node"),{Scanner}=require("@tailwindcss/oxide");
const ui=path.join(root,"🧰️framework/🔨️modules/🖱️ui");
const adapter=path.join(root,"🧰️framework/🔨️modules/🏃️process/🧰️tools/🎭️playwright/🟦️.ts");
const adapterFile=path.join(output,"layered-tools.cjs");
const compiledAdapter=await build({entryPoints:[adapter],bundle:true,write:false,platform:"node",format:"cjs",packages:"external",logLevel:"silent"});
await fs.writeFile(adapterFile,compiledAdapter.outputFiles[0].contents);
const {chromium}=await require(adapterFile).loadPlaywrightV1(root);
const cssPath=path.join(ui,"🧵️styles/🎨️.css");
const compiler=await compile(await fs.readFile(cssPath,"utf8"),{base:path.dirname(cssPath),onDependency(){}});
const scanner=new Scanner({sources:[{base:ui,pattern:"**/*.{ts,tsx}",negated:false},...compiler.sources]});
const css=compiler.build(scanner.scan());
const entry=`import React from 'react'; import {createRoot} from 'react-dom/client'; import {flushSync} from 'react-dom';
import {LayeredOverview} from ${JSON.stringify(path.join(ui,"🧱️elements/🥞️LayeredOverview/🟦️.tsx"))};
import {OverviewCard,OverviewCardOpenChip} from ${JSON.stringify(path.join(ui,"🧱️elements/🃏️OverviewCard/🟦️.tsx"))};
import {CatalogIcon} from ${JSON.stringify(path.join(ui,"🧱️elements/🔣️Icons/🟦️.tsx"))};
import {centeredLastRowCells} from ${JSON.stringify(path.join(ui,"🔨️modules/🥞️layered-overview-geometry/🟦️.ts"))};
const root=createRoot(document.getElementById('root'));
const labels={grid:'Topics',overview:'Overview',waiting:p=>p.label,failed:p=>p.label,neighbour:p=>p.label};
function App({ids,label,teaching=true}){const cells=Object.fromEntries(centeredLastRowCells(ids.length).map((cell,i)=>[ids[i],cell]));
const panes=ids.map(id=>({id,label:id,render:()=> <div data-oracle-content={id} style={{width:'100%',height:'100%',background:'var(--base)'}}>{id}</div>}));
return <LayeredOverview panes={panes} cells={cells} mode='strip' pan='pointer' routing='none' labels={labels} lifecycle={{budget:12,warmStartMs:0,warmIntervalMs:0}} windowing={{radius:10}}
overlayStyle={{display:'grid',gridTemplateColumns:'repeat(3,minmax(0,1fr))',gridTemplateRows:'repeat(2,minmax(0,1fr))',padding:3.2,gap:3.2}}
renderCard={(pane,state)=><OverviewCard as='button' slot='oracle' icon={null} title={pane.label} titleAdornment={teaching?<span data-presentation-card-fullscreen='' data-layered-zoom='' className='inline-flex'><CatalogIcon icon='maximize-2' size='small'/></span>:undefined} onClick={state.open} className='h-full' {...(teaching?{footerRight:<OverviewCardOpenChip slot='oracle' icon={null}>{label}</OverviewCardOpenChip>}:{footerLeft:<OverviewCardOpenChip slot='oracle'>{label}</OverviewCardOpenChip>})}>{pane.label}</OverviewCard>}/>}
window.layeredOracle={render(ids,label,teaching=true){flushSync(()=>root.render(null));flushSync(()=>root.render(<App ids={ids} label={label} teaching={teaching}/>))}};`;
const bundle=await build({stdin:{contents:entry,resolveDir:root,loader:"tsx"},bundle:true,write:false,format:"iife",platform:"browser",alias:{"@semio-tech/framework-transient":path.join(root,"🧰️framework/🔨️modules/🫧️transient/🟦️.ts"),"@semio-tech/framework-interaction":path.join(root,"🧰️framework/🔨️modules/🕹️interaction/🟦️.ts")},define:{"process.env.NODE_ENV":'"production"'},logLevel:"silent"});
const browser=await chromium.launch({headless:true});
try{
  const page=await browser.newPage({viewport:{width:1600,height:900},colorScheme:"dark",reducedMotion:"reduce"});
  await page.clock.install();
  const errors=[];page.on("pageerror",error=>errors.push(error.message));
  await page.route("http://layered.oracle/**",async route=>{
    const pathname=decodeURIComponent(new URL(route.request().url()).pathname);
    if(pathname==="/")return route.fulfill({contentType:"text/html",body:`<html class="dark"><meta charset="utf-8"><body style="margin:0"><div id="root" style="position:absolute;top:28.8px;width:1600px;height:842.4px"></div></body></html>`});
    const local=pathname.startsWith("/🖼️assets/")?path.join(root,"🧰️framework/🔨️modules",pathname.slice(1)):path.resolve(root,"."+pathname);
    if(!local.startsWith(root+path.sep))return route.abort();
    try{return route.fulfill({body:await fs.readFile(local)})}catch{return route.abort()}
  });
  await page.goto("http://layered.oracle/");await page.addStyleTag({content:css});await page.addScriptTag({content:bundle.outputFiles[0].text});
  const measure=()=>page.evaluate(()=>{
    const rect=e=>{const r=e.getBoundingClientRect();return Object.fromEntries(['x','y','width','height'].map(k=>[k,r[k]]))};
    const root=document.querySelector('[data-layered-overview]'),strip=root.querySelector(':scope > [data-layered-strip]'),veil=root.querySelector(':scope > [data-layered-veil]');
    return {rect:rect(root),mode:root.dataset.mode,pan:root.dataset.pan,transform:strip.style.transform,veil:veil?{kind:veil.dataset.veil,visibility:getComputedStyle(veil).visibility,background:getComputedStyle(veil).backgroundColor,backdropFilter:getComputedStyle(veil).backdropFilter}:null,
      panes:[...strip.children].map(p=>({id:p.dataset.layeredPane,rect:rect(p),inert:p.hasAttribute('inert'),opened:p.hasAttribute('data-opened')})),
      cards:[...root.querySelectorAll(':scope > [data-layered-overlay] [data-layered-card]')].map(c=>{const card=c.querySelector('[data-overview-card]'),chip=c.querySelector('[data-slot=oracle-open-chip]'),title=card.querySelector('[data-slot=oracle-title-chip]'),fullscreen=card.querySelector('[data-presentation-card-fullscreen]'),surface=card.querySelector('[data-slot=window-chrome-body-surface]');return{id:c.dataset.layeredCard,rect:rect(card),chip:rect(chip),text:chip.textContent.trim(),titleText:title.textContent.trim(),fullscreen:fullscreen?rect(fullscreen):null,fullscreenIcons:fullscreen?[...fullscreen.querySelectorAll('[data-icon]')].map(icon=>icon.dataset.icon):[],controlInTitleChip:fullscreen?title.contains(fullscreen):false,nativeButtonCount:card.querySelectorAll('button').length+Number(card.tagName==='BUTTON'),icons:[...chip.querySelectorAll('[data-icon]')].map(icon=>icon.dataset.icon),revealed:c.hasAttribute('data-revealed'),tinted:c.hasAttribute('data-tinted'),concealed:c.hasAttribute('data-concealed'),background:getComputedStyle(surface).backgroundColor,backdropFilter:getComputedStyle(surface).backdropFilter}})};
  });
  const result={schema:fixture.schema,samples:[]};
  await page.evaluate(({ids,label})=>window.layeredOracle.render(ids,label,false),{ids:fixture.topics,label:fixture.labels.en});
  await page.clock.runFor(1200);await page.evaluate(()=>document.fonts.ready);
  result.defaultChip=await measure();
  for(const card of result.defaultChip.cards){assert(card.chip.x<card.rect.x+card.rect.width/2);assert.deepEqual(card.icons,[fixture.sharedDefault.icon])}
  console.log('PASS layered-shared-default-chevron-left');
  for(const language of fixture.locales)for(const scenario of fixture.scenarios){
    const ids=fixture[scenario.id],label=fixture.labels[language];
    await page.mouse.move(1610,910);await page.evaluate(({ids,label})=>window.layeredOracle.render(ids,label),{ids,label});
    await page.clock.runFor(1200);await page.evaluate(()=>document.fonts.ready);
    const resting=await measure();assert.equal(resting.mode,"strip");assert.equal(resting.pan,"pointer");assert.equal(resting.cards.length,scenario.count);
    assert.deepEqual(resting.cards.map(c=>c.id),ids);assert(resting.veil.backdropFilter.includes("blur("));
    for(const card of resting.cards){assert.equal(card.text,label);assert(card.chip.x>card.rect.x+card.rect.width/2);assert.deepEqual(card.icons,fixture.openChip.footerIcons);assert.deepEqual(card.fullscreenIcons,[fixture.openChip.icon]);assert(card.controlInTitleChip);assert.equal(card.titleText,card.id);assert.equal(card.nativeButtonCount,1);assert(card.fullscreen.y<card.rect.y+card.rect.height/2);assert(card.chip.y>card.rect.y+card.rect.height/2);assert(card.backdropFilter.includes("blur("))}
    await page.mouse.move(1598,869);await page.clock.runFor(1500);const panned=await measure();if(ids.length>1)assert.notEqual(panned.transform,resting.transform);
    const target=ids.at(-1),chip=page.locator('[data-layered-card="'+target+'"] [data-presentation-card-fullscreen]');await chip.hover();await page.clock.runFor(650);
    const hover=await measure(),pane=hover.panes.find(p=>p.id===target);assert(hover.cards.find(c=>c.id===target).revealed);assert(pane.inert);assert(!pane.opened);
    for(const key of ['x','y','width','height'])assert(Math.abs(pane.rect[key]-hover.rect[key])<fixture.geometryTolerance,target+' hover '+key);
    assert.equal(hover.veil.kind,"clear");assert.equal(hover.veil.visibility,"hidden");
    await chip.click();await page.clock.runFor(650);const opened=await measure();assert(opened.panes.find(p=>p.id===target).opened);assert(!opened.panes.find(p=>p.id===target).inert);
    await page.keyboard.press('Escape');await page.clock.runFor(650);const returned=await measure();assert(!returned.panes.some(p=>p.opened));
    result.samples.push({scenario:scenario.id,language,resting,panned,hover,opened,returned});
    await page.screenshot({path:path.join(output,scenario.id+'-'+language+'.png')});console.log('PASS layered-'+scenario.id+'-'+language);
  }
  assert.deepEqual(errors,[]);result.errors=errors;await fs.writeFile(path.join(output,"oracle.json"),JSON.stringify(result,null,2));
}finally{await browser.close();await fs.rm(adapterFile,{force:true})}
