/** 🌐 Compose the teaching hierarchy with the actual shared semio layered UI. */
import React from "react";
import {createRoot} from "react-dom/client";
import {flushSync,createPortal} from "react-dom";
import {WindowChrome,windowChromeTitleChipClass,measureWindowSilhouetteMetrics} from "@presentation/window";
import {createWindowSilhouetteGeometry} from "@presentation/silhouette";
import {LayeredOverview} from "@presentation/layered";
import {OverviewCard,OverviewCardOpenChip} from "@presentation/card";
import {Navbar,SemioLogo,navbarFillItem} from "@presentation/navbar";
import {Footer} from "@presentation/footer";
import {Tree} from "@presentation/tree";
import {Popover,PopoverTrigger,PopoverContent} from "@presentation/popover";
import {interactiveActiveFillClass} from "@presentation/interaction";
import {windowChromeControlButtonClass} from "@presentation/controls";

let config:any;
const root=createRoot(document.getElementById("root")!);
const item=(content:React.ReactNode)=>({content});
const label=(value:string)=><span className="text-xs leading-none">{value}</span>;
const words={en:{courses:"Courses",topics:"Topics",clips:"Clips",chapters:"Chapters",watch:"Watch",empty:"No videos available yet.",waiting:"Loading",failed:"Unable to load",restore:"Restore"},de:{courses:"Kurse",topics:"Themen",clips:"Clips",chapters:"Kapitel",watch:"Ansehen",empty:"Noch keine Videos verfügbar.",waiting:"Laden",failed:"Laden fehlgeschlagen",restore:"Wiederherstellen"}};
const text=()=>({...words[config.language as "en"|"de"],...Object.fromEntries(["courses","topics","clips","chapters","empty","overview"].map(key=>[key,config.contract.labels[key][config.language]])),watch:config.contract.labels.view[config.language]});
const title=(entry:any)=>entry.title[config.language];
const sameGeometry=(actual:string|null|undefined,expected:string)=>{if(!actual)return false;const numbers=/-?\d+(?:\.\d+)?/g,a=actual.match(numbers)??[],b=expected.match(numbers)??[];return actual.replace(numbers,"#")===expected.replace(numbers,"#")&&a.length===b.length&&a.every((value,index)=>Math.abs(Number(value)-Number(b[index]))<.01)};
function Icon({name}:{name:string}) {return <span data-presentation-icon={name} aria-hidden="true" style={{display:"contents"}} dangerouslySetInnerHTML={{__html:config.icons[name]}}/>}
function Thumbnail({symbol}:{symbol:string}) {
  const muted="var(--color-light-5-7)",primary="var(--color-primary)";
  const line=(d:string)=><path d={d} fill="none" stroke={muted} strokeWidth="3"/>;
  const ink=(d:string)=><path d={d} fill="none" stroke={primary} strokeWidth="4"/>;
  let content:React.ReactNode;
  if(symbol==="parametric")content=<>{line("M150 450 H850 M150 450 V150")}{ink("M180 420 C320 420 340 150 500 150 S700 420 820 210")}{[280,500,720].map((x,i)=><g key={x}>{line("M"+x+" 490 V550")}<circle cx={x} cy={[520,500,535][i]} r="9" fill={primary}/></g>)}</>;
  else if(symbol==="bim")content=<>{line("M280 460 V210 L500 100 L720 210 V460 L500 570 Z M280 210 L500 320 L720 210 M500 320 V570 M280 335 L500 445 L720 335")}{ink("M390 155 V405 M610 155 V405 M280 460 L500 350 L720 460")}</>;
  else if(symbol==="grasshopper")content=<>{line("M300 210 C400 210 400 300 500 300 M300 440 C400 440 400 300 500 300 M620 300 C740 300 700 210 810 210")}{[[180,150,120,120],[180,380,120,120],[500,240,120,120],[810,150,120,120]].map(([x,y,w,h])=><rect key={x+":"+y} x={x} y={y} width={w} height={h} fill="none" stroke={primary} strokeWidth="4"/>)}</>;
  else if(symbol==="excel")content=<>{line("M220 120 H780 V500 H220 Z M220 215 H780 M220 310 H780 M220 405 H780 M360 120 V500 M500 120 V500 M640 120 V500")}{ink("M360 215 H640 V405 H360 Z M380 285 L410 245 M380 245 L410 285 M525 355 H610")}</>;
  else if(symbol==="optimization")content=<>{line("M170 470 H850 M170 470 V100")}{ink("M200 140 C350 500 480 470 540 260 S750 160 820 240")}<circle cx="415" cy="420" r="11" fill={primary}/>{line("M415 450 V500")}</>;
  else content=<>{line("M220 462 V222 L500 90 L780 222 V462 Z")}{["air","heat","cool"].includes(symbol)?[252,342,414].map(y=><g key={y}>{ink("M100 "+y+" H900 M870 "+(y-15)+" L900 "+y+" L870 "+(y+15))}</g>):symbol==="physics"?<g stroke={primary} fill="none" strokeWidth="4"><circle cx="500" cy="312" r="96"/>{[[320,192],[680,192],[320,420],[680,420]].map(([x,y])=><path key={x+":"+y} d={"M500 312 L"+x+" "+y}/>)}</g>:<g>{[420,600,300].map((v,i)=><rect key={i} x={360+120*i} y={456-v*.6} width="70" height={v*.6} fill={i===1?primary:muted}/>)}</g>}</>;
  return <svg viewBox="0 0 1000 600" preserveAspectRatio="xMidYMid meet" style={{width:"100%",height:"100%",overflow:"visible"}} aria-hidden="true">{content}</svg>;
}
function VideoPane({entry,frame,onClose}:{entry:any;frame:any;onClose:()=>void}) {
  const [pressed,setPressed]=React.useState(false),active=Boolean(frame.cursor?.pressed&&pressed),media=frame.media.find((value:any)=>value.id===entry.id),rect=config.contract.focus.rect;
  return <WindowChrome level="window" active stackDataAttrs={{"id":entry.id,"data-presentation-window":entry.id}} style={{position:"absolute",left:rect.x,top:rect.y-config.contract.chrome.navbarHeight,width:rect.width,height:rect.height}}
    titleChips={<div data-presentation-title-chip={entry.id} className={windowChromeTitleChipClass+(active?" "+interactiveActiveFillClass:"")} style={{paddingBlock:0}}><span id={entry.id+":tab"} className="min-w-0 truncate" title={title(entry)}>{title(entry)}</span><button id={entry.id+":fullscreen"} data-slot="fullscreen" type="button" aria-label={text().restore} className={windowChromeControlButtonClass+" shrink-0 outline-none focus-visible:ring-1 focus-visible:ring-inset focus-visible:ring-active-base"} onClick={onClose} onPointerDown={()=>setPressed(true)} onPointerUp={()=>setPressed(false)} onPointerCancel={()=>setPressed(false)}><Icon name="minimize-2"/></button></div>}
    body={<div data-presentation-body={entry.id} style={{position:"absolute",left:3.2,right:3.2,top:"calc(var(--window-silhouette-top-clearance) + 3.2px)",bottom:3.2,pointerEvents:"none"}}>{media?.body_image?<img src={media.body_image} alt="" style={{width:"100%",height:"100%",objectFit:"contain",opacity:media.body_opacity??1,background:"var(--base)"}}/>:<Thumbnail symbol={entry.symbol}/>}</div>}/>;
}
function Overview({entries,level,gridLabel,opened,revealed,interactive,camera,onCamera,onOpen,onReveal,renderPane}:any) {
  const cells=Object.fromEntries(entries.map((entry:any)=>[entry.id,entry.cell])),panes=entries.map((entry:any)=>({id:entry.id,label:title(entry),render:(state:any)=>renderPane(entry,state)}));
  const columns=Math.max(1,...entries.map((entry:any)=>entry.cell.column+1)),rows=Math.max(1,...entries.map((entry:any)=>entry.cell.row+1));
  return <div data-presentation-level={level} className="relative h-full min-h-0 w-full"><LayeredOverview panes={panes} cells={cells} mode="strip" pan="pointer" routing="none" interactive={interactive} openedId={opened} revealedId={revealed} revealedBy="pointer" camera={camera} onCameraChange={onCamera} onOpenedIdChange={onOpen} onRevealedIdChange={onReveal}
    lifecycle={{budget:entries.length,warmStartMs:0,warmIntervalMs:0}} overlayClassName="course-home-grid" overlayStyle={{"--course-home-columns":`repeat(${columns}, minmax(0, 1fr))`,"--course-home-rows":`repeat(${rows}, minmax(0, 1fr))`} as React.CSSProperties}
    labels={{grid:gridLabel,waiting:pane=>text().waiting+" "+pane.label,failed:pane=>text().failed+" "+pane.label,neighbour:pane=>pane.label}}
    renderCard={(pane,card)=>{const entry=entries.find((value:any)=>value.id===pane.id);return <div className="course-home-cell" style={{gridColumn:entry.cell.column+1,gridRow:entry.cell.row+1}}><div className="course-home-card"><OverviewCard as="button" slot="course-card" title={pane.label} titleAdornment={<span data-presentation-card-fullscreen={entry.id} data-layered-zoom="" className="inline-flex self-stretch items-center"><Icon name="maximize-2"/></span>} icon={entry.icon?<span aria-hidden="true">{entry.icon}</span>:null} onClick={card.open} lifted={card.revealed} data={{"data-presentation-card":entry.id}}
      footerRight={<OverviewCardOpenChip slot="course-card" icon={null}>{text().watch}</OverviewCardOpenChip>}><span style={{display:"block",height:160}}><Thumbnail symbol={entry.symbol??entry.id}/></span></OverviewCard></div></div>}}/></div>;
}
function Navigation({frame,navigation,onNavigate}:{frame:any;navigation:any;onNavigate:(path:string[])=>void}) {
  const branch=(entry:any,path:string[],depth:number):any=>{const next=[...path,entry.id],children=depth<3?entry[["topics","videos","chapters"][depth]]:null;return {id:"navbar:tree:"+entry.id,label:title(entry),defaultOpen:depth===0||navigation.opened_path.includes(entry.id),onClick:()=>onNavigate(next),items:children?.map((child:any)=>branch(child,next,depth+1))}};
  return <Popover open={Boolean(frame.menu_open)} onOpenChange={()=>{}}><PopoverTrigger asChild><button id="navbar:energy" className={windowChromeTitleChipClass}>{config.contract.identity.subject[config.language]}</button></PopoverTrigger><PopoverContent align="start" data-presentation-menu="navbar:tree" onOpenAutoFocus={event=>event.preventDefault()} onCloseAutoFocus={event=>event.preventDefault()}><Tree key={navigation.opened_path.join("/")} sections={[{id:"courses",items:config.contract.courses.map((course:any)=>branch(course,[],0))}]}/></PopoverContent></Popover>;
}
function Hierarchy({entries,path,frame,navigation,change,active=true}:{entries:any[];path:string[];frame:any;navigation:any;change:(value:any)=>void;active?:boolean}) {
  const depth=path.length,matches=(value:string[])=>path.every((id,index)=>value[index]===id),opened=matches(navigation.opened_path)?navigation.opened_path[depth]??null:null,revealed=matches(navigation.revealed_path)?navigation.revealed_path[depth]??null:null,interactive=active&&navigation.opened_path.length===depth;
  return entries.length?<Overview entries={entries} level={path.at(-1)??"root"} gridLabel={text()[["courses","topics","clips","chapters"][depth]]} opened={opened} revealed={revealed} interactive={interactive} camera={navigation.camera} onCamera={(camera:any)=>{if(interactive&&frame.cursor?.point)change({camera})}} onOpen={(id:string|null)=>change({opened_path:id?[...path,id]:path,revealed_path:[]})} onReveal={(id:string|null)=>{if(interactive)change({revealed_path:id?[...path,id]:path})}}
    renderPane={(entry:any,state:any)=>depth===3?<VideoPane entry={entry} frame={frame} onClose={()=>change({opened_path:path,revealed_path:[]})}/>:<Hierarchy entries={entry[["topics","videos","chapters"][depth]]} path={[...path,entry.id]} frame={frame} navigation={navigation} change={change} active={active&&state.opened}/>}/>:<div data-presentation-empty={path.at(-1)} className="flex h-full w-full flex-col items-center justify-center gap-double"><p>{text().empty}</p></div>;
}
function App({frame}:{frame:any}) {
  const [navigation,setNavigation]=React.useState(frame.hierarchy);
  React.useLayoutEffect(()=>setNavigation(frame.hierarchy),[frame]);
  const change=(value:any)=>setNavigation((current:any)=>({...current,...value})),navigate=(path:string[])=>change({opened_path:path,revealed_path:[]});
  const section=config.contract.identity.section[config.language],ancestors:any[]=[];
  let entries=config.contract.courses;
  for(const [depth,id] of navigation.opened_path.entries()){const entry=entries.find((value:any)=>value.id===id);if(!entry)throw Error("Unknown hierarchy path "+navigation.opened_path.join("/"));ancestors.push(entry);entries=depth<3?entry[["topics","videos","chapters"][depth]]:[]}
  return <div id="app" className="course-app" data-presentation-scene data-presentation-opened-path={JSON.stringify(navigation.opened_path)} data-presentation-revealed-path={JSON.stringify(navigation.revealed_path)} style={{position:"absolute",left:0,top:0,width:1600,height:900,background:"var(--base)",color:"var(--foreground)",overflow:"hidden"}}>
    <div style={{position:"absolute",left:0,top:0,width:1600}}><Navbar label="semio" showFullscreenToggle={false} items={[item(<SemioLogo className="size-medium"/>),item(<Navigation frame={frame} navigation={navigation} onNavigate={navigate}/>),navbarFillItem(),{content:label(section),centered:true}]}/></div>
    <main id="canvas" data-slot="overview-canvas" style={{position:"absolute",left:0,top:28.8,width:1600,height:842.4,background:"var(--base)"}}>
      <Hierarchy entries={config.contract.courses} path={[]} frame={frame} navigation={navigation} change={change}/>
    </main>
    <div style={{position:"absolute",left:0,top:871.2,width:1600}}><Footer items={[item(<button id="footer:root" className={windowChromeTitleChipClass} onClick={()=>navigate([])}>{text().courses}</button>),...ancestors.map((entry,index)=>item(<button id={"footer:path:"+entry.id} className={windowChromeTitleChipClass} onClick={()=>navigate(navigation.opened_path.slice(0,index+1))}>{title(entry)}</button>)),navbarFillItem(),item(<button id="footer:range" className={windowChromeTitleChipClass} onClick={()=>navigate(navigation.opened_path.slice(0,-1))}>{text()[["courses","topics","clips","chapters","chapters"][navigation.opened_path.length]]}</button>)]}/></div>
    {frame.cursor?.point&&frame.cursor.visible!==false?createPortal(<img data-presentation-cursor src={config.cursors[frame.cursor.kind??"arrow"]} style={{position:"fixed",left:frame.cursor.point[0],top:frame.cursor.point[1],width:config.cursorSizes[frame.cursor.kind??"arrow"][0],height:config.cursorSizes[frame.cursor.kind??"arrow"][1],pointerEvents:"none",zIndex:2147483647}}/>,document.body):null}
  </div>;
}
(window as any).semioPresentation={
  configure(value:any){config=value;document.documentElement.lang=value.language},
  render(frame:any,reset=false){if(reset)flushSync(()=>root.render(null));flushSync(()=>root.render(<App frame={frame}/>))},
  contentCamera(value:any){
    const app=document.getElementById("app");
    if(!app)return null;
    app.style.transform="";app.style.transformOrigin="0 0";
    const progress=value?.progress??0;
    if(!progress)return {progress:0,scale:1,translation:[0,0],target:null};
    const host=[...document.querySelectorAll("[data-presentation-body]")].find(element=>element.getAttribute("data-presentation-body")===value.video_id),image=host?.querySelector("img"),box=host?.getBoundingClientRect();
    if(!box)throw Error("Content camera requires an actual video body");
    const aspect=image?.naturalWidth&&image?.naturalHeight?image.naturalWidth/image.naturalHeight:1600/900;
    const width=Math.min(box.width,box.height*aspect),height=width/aspect,target={x:box.x+(box.width-width)/2,y:box.y+(box.height-height)/2,width,height};
    const scale=1+progress*(Math.min(1600/width,900/height)-1),translation=[progress*(800-(target.x+width/2)*Math.min(1600/width,900/height)),progress*(450-(target.y+height/2)*Math.min(1600/width,900/height))];
    app.style.transform=`matrix(${scale},0,0,${scale},${translation[0]},${translation[1]})`;
    return {progress,scale,translation,target};
  },
  debugGeometry(){return [...document.querySelectorAll("[data-presentation-window]")].map(stack=>{const measured=measureWindowSilhouetteMetrics(stack as HTMLElement),expected=createWindowSilhouetteGeometry(measured);return{id:stack.id,measured,expectedBorder:expected.borderPath,actualBorder:stack.querySelector("[data-window-silhouette-border] path")?.getAttribute("d"),expectedClip:expected.contentClipPath,actualClip:(stack.querySelector("[data-window-silhouette-content]") as HTMLElement)?.style.clipPath}})},
  settled(){return [...document.querySelectorAll("[data-presentation-window]")].every(stack=>{const measured=measureWindowSilhouetteMetrics(stack as HTMLElement);if(!measured)return false;const expected=createWindowSilhouetteGeometry(measured),normalized=document.createElement("div").style;normalized.clipPath=expected.contentClipPath;return sameGeometry(stack.querySelector("[data-window-silhouette-border] path")?.getAttribute("d"),expected.borderPath)&&sameGeometry((stack.querySelector("[data-window-silhouette-content]") as HTMLElement)?.style.clipPath,normalized.clipPath)})}
};
