import {Base64DecodeCursor} from "@semio-tech/framework/io/base64";
import sourceFixture from "../../📥️source/🧪️tests/🔣️.json";
/** 🧪️ Moving focus to the tools must preserve an asynchronously computed pixel selection. */
import {act,cleanup,fireEvent,render,waitFor as waitForWithin} from "@semio-tech/ui-react/test";
import {afterEach,expect,test,vi} from "vitest";
import {getCurrentTest} from "vitest/suite";
import sharp from "sharp";
import {PixelEditingOverlay as ControlledPixelEditingOverlay} from "../../🟦️.tsx";
import {useState,type ComponentProps} from "react";
import type {Paint2dEditingOptions} from "@semio-tech/framework";
import editingFixture from "../../../../../../../../../../🔨️modules/🖱️ui/🎬️scene/🧪️tests/🧫️fixtures/🎚️paint2d-editing/🔣️.json";
const initialEditingOptions:Paint2dEditingOptions={...editingFixture.initial,filterKind:"brightness"};

/** ⏳ Waits for an asynchronous editing job under the test's own budget: a loaded machine delays the job, it never races a 1 s default. */
const waitFor:typeof waitForWithin=(check,timeoutMs)=>waitForWithin(check,Math.max(timeoutMs??0,getCurrentTest()!.timeout));

/** 🧊️ Holds the job's next zero-delay turn yield from `hold` until `release`, so the test observes the UI at an exact job step instead of racing the timer queue: the job asks for its yield right after the step that called `hold`, and nothing else schedules a zero-delay timer in between. */
function jobStepGate(){
  const real=globalThis.setTimeout;let armed=false,held:(()=>void)|null=null;
  vi.spyOn(globalThis,"setTimeout").mockImplementation(((handler:TimerHandler,delay?:number,...args:unknown[])=>{
    if(armed&&!delay){armed=false;held=()=>{real(handler as never,0,...args);};return 0 as unknown as ReturnType<typeof setTimeout>;}
    return real(handler as never,delay,...args);
  }) as unknown as typeof setTimeout);
  return {hold:()=>{armed=true;},release:()=>{const resume=held;held=null;resume?.();}};
}

/** 🎛️ Applies semantic config commands as the editor does before republishing its scene. */
function PixelEditingOverlay(props:Omit<ComponentProps<typeof ControlledPixelEditingOverlay>,"paintTarget"|"maskValue"|"fillTolerance"|"editingOptions">){
  const [paintTarget,setTarget]=useState<"pixels"|"mask">("pixels"),[maskValue,setValue]=useState(255),[fillTolerance,setFill]=useState(24),[pixelSelectionJson,setSelection]=useState(props.pixelSelectionJson);
  const [editingOptions,setOptions]=useState(initialEditingOptions);
  return <ControlledPixelEditingOverlay {...props} editingOptions={editingOptions} pixelSelectionJson={pixelSelectionJson} paintTarget={paintTarget} maskValue={maskValue} fillTolerance={fillTolerance} dispatch={(action,args)=>{
    if(action==="setPaintTarget"){if(args!.value!==paintTarget)setSelection(undefined);setTarget(args!.value as "pixels"|"mask");}
    if(action==="setMaskValue")setValue(args!.value as number);
    if(action==="setFillTolerance")setFill(args!.value as number);
    if(action==="setFilterKind"){const filterKind=args!.value as Paint2dEditingOptions["filterKind"],filterAmount=filterKind==="gamma"||filterKind==="sharpen"?1:filterKind==="threshold"?128:filterKind==="posterize"?4:filterKind==="blur"?2:0;setOptions(value=>({...value,filterKind,filterAmount}));}
    if(action==="setFilterAmount")setOptions(value=>({...value,filterAmount:args!.value as number}));
    if(action==="setImageGrid")setOptions(value=>({...value,imageGrid:{...value.imageGrid,[args!.field as string]:JSON.parse(args!.value as string)}}));
    const result=props.dispatch(action,args);
    const apply=async(outcome:unknown)=>{
      if(action!=="selectPixels"||(outcome&&typeof outcome==="object"&&"kind" in outcome&&["refused","superseded"].includes(String(outcome.kind))))return outcome;
      if(args!.method==="none"){setSelection(undefined);return outcome;}
      const width=args!.width as number,height=args!.height as number,method=args!.method;
      let spans:{start:number;length:number;coverage:number}[]=[];
      if(method==="all"||method==="invert")spans=[{start:0,length:width*height,coverage:255}];
      else if(method==="rectangle"){
        const [[x1,y1],[x2,y2]]=JSON.parse(args!.path as string) as [[number,number],[number,number]];
        const x=Math.min(x1,x2),y=Math.min(y1,y2),w=Math.abs(x2-x1),h=Math.abs(y2-y1);
        const {data}=await sharp(Buffer.from(`<svg width="${width}" height="${height}"><rect x="${x}" y="${y}" width="${w}" height="${h}" fill="white"/></svg>`)).ensureAlpha().raw().toBuffer({resolveWithObject:true});
        for(let i=0;i<width*height;i++)if(data[i*4+3])spans.push({start:i,length:1,coverage:data[i*4+3]!});
      }
      setSelection(JSON.stringify({layerId:args!.layerId,target:args!.target,width,height,spans}));return outcome;
    };
    if(result instanceof Promise)return result.then(apply);
    return apply(result);
  }}/>;
}
const editCalls=(dispatch:{mock:{calls:any[][]}})=>dispatch.mock.calls.filter(call=>["transformImage","fillSelection","maskFromSelection","paintStroke","applyFilter"].includes(call[0]));
import fixture from "../🧫️fixtures/🖱️selection-focus/🔣️.json";

vi.mock("@semio-tech/ui-react",async original=>({...await original<typeof import("@semio-tech/ui-react")>(),useTranslation:()=>({i18n:{resolvedLanguage:"en"}})}));
vi.mock("../../../../📐️Canvas2dHost/📷️camera-math/🟦️.ts",()=>({canvasPinchCamera:()=>({x:0,y:0,zoom:1})}));
afterEach(()=>{cleanup();vi.restoreAllMocks();vi.unstubAllGlobals();});

test("filter and image grid controls consume published scene options",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  const props={documentJson:'{"layers":[{"kind":"pixel","id":"p","width":16,"height":16}]}',assetsJson:"{}",assetExtentsJson:"{}",selectionJson:'["p"]',activeUtility:"select",brushSize:1,brushOpacity:1,brushColor:"#000000",brushHardness:1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,camera:{current:{x:0,y:0,zoom:1}},container:{current:host},dispatch,onWheel:()=>{},onCameraChange:()=>{}};
  const view=render(<ControlledPixelEditingOverlay {...props} editingOptions={initialEditingOptions}/>);view.container.querySelector("details")!.open=true;
  fireEvent.change(view.getByRole("combobox",{name:"Adjustment"}),{target:{value:"gamma"}});
  expect((view.getByRole("combobox",{name:"Adjustment"}) as HTMLSelectElement).value).toBe("brightness");
  view.rerender(<ControlledPixelEditingOverlay {...props} editingOptions={{...initialEditingOptions,filterKind:"gamma",filterAmount:1}}/>);
  fireEvent.change(view.getByRole("spinbutton",{name:"Amount"}),{target:{value:"1.8"}});
  fireEvent.change(view.getByRole("spinbutton",{name:"Width",exact:true}),{target:{value:"5"}});
  fireEvent.click(view.getByRole("checkbox",{name:"Bilinear sampling"}));
  for(const row of editingFixture.commands)expect(dispatch).toHaveBeenCalledWith(row.action,row.args);
  expect((view.getByRole("spinbutton",{name:"Amount"}) as HTMLInputElement).value).toBe("1");
  expect((view.getByRole("spinbutton",{name:"Width",exact:true}) as HTMLInputElement).value).toBe("512");
  const published:Paint2dEditingOptions={...editingFixture.options,filterKind:"gamma"};
  view.rerender(<ControlledPixelEditingOverlay {...props} editingOptions={published}/>);
  expect((view.getByRole("combobox",{name:"Adjustment"}) as HTMLSelectElement).value).toBe("gamma");
  expect((view.getByRole("spinbutton",{name:"Amount"}) as HTMLInputElement).value).toBe("1.8");
  expect((view.getByRole("spinbutton",{name:"Crop x"}) as HTMLInputElement).value).toBe("2");
  expect((view.getByRole("spinbutton",{name:"Crop y"}) as HTMLInputElement).value).toBe("3");
  expect((view.getByRole("checkbox",{name:"Bilinear sampling"}) as HTMLInputElement).checked).toBe(false);
  fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("applyFilter",{layerId:"p",filter:"gamma",amount:1.8}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Resize layer"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(view.getByRole("button",{name:"Resize layer"}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("transformImage",{layerId:"p",operation:"resize",x:0,y:0,width:5,height:4,bilinear:false}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Crop layer",exact:true}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(view.getByRole("button",{name:"Crop layer",exact:true}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("transformImage",{layerId:"p",operation:"crop",x:2,y:3,width:5,height:4,bilinear:false}));
});

for(const tool of ["Color picker","Magic wand"])test(tool+(tool==="Color picker"?" prepares actual image bytes":" submits its seed without browser source preparation"),async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  const row=sourceFixture.browser,data=Uint8ClampedArray.from(row.pixels),close=vi.fn();
  const png=await sharp(Buffer.from(row.pixels),{raw:{width:row.width,height:row.height,channels:4}}).png().toBuffer();
  const bitmap=vi.fn(async(blob:Blob)=>{
    const bytes=await new Promise<ArrayBuffer>((resolve,reject)=>{const reader=new FileReader();reader.onload=()=>resolve(reader.result as ArrayBuffer);reader.onerror=()=>reject(reader.error);reader.readAsArrayBuffer(blob);});
    const decoded=await sharp(Buffer.from(bytes)).raw().toBuffer();
    expect([...decoded]).toEqual(row.pixels);return {width:row.width,height:row.height,close};
  });
  vi.stubGlobal("createImageBitmap",bitmap);
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockImplementation(function(this:HTMLCanvasElement){
    return this.width===row.width&&this.height===row.height?{drawImage:vi.fn(),getImageData:()=>({data}),createImageData:()=>({data:new Uint8ClampedArray(data.length)}),putImageData:vi.fn()} as unknown as CanvasRenderingContext2D:null;
  });
  const host=document.createElement("div"),dispatch=vi.fn();host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:row.width,height:row.height,imageKey:"source"}]})} assetsJson={JSON.stringify({source:{mime:"image/png",data:png.toString("base64")}})} assetExtentsJson={JSON.stringify({source:{width:row.width,height:row.height}})} selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#000000" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:tool,exact:true}));
  const canvas=view.getByRole("application",{name:"Image editor"});canvas.setPointerCapture=()=>{};
  act(()=>{canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));});
  const command=tool==="Color picker"?"setBrushColor":"selectPixels";
  await waitFor(()=>expect(dispatch.mock.calls.some(call=>call[0]===command)).toBe(true));
  expect(bitmap).toHaveBeenCalledTimes(tool==="Color picker"?1:0);expect(close).toHaveBeenCalledTimes(tool==="Color picker"?1:0);
  if(tool==="Color picker")expect(dispatch).toHaveBeenCalledWith(command,{value:row.foreground});
  else expect(dispatch).toHaveBeenCalledWith(command,{layerId:"p",target:"pixels",width:row.width,height:row.height,method:"wand",mode:"replace",path:JSON.stringify([[row.width/2,row.height/2]]),expectedImageKey:"source"});
});

for(const tool of ["Color picker"])test(tool+" cancels encoded source preparation before browser decode",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const bitmap=vi.fn();vi.stubGlobal("createImageBitmap",bitmap);
  const step=Base64DecodeCursor.prototype.step,gate=jobStepGate();
  vi.spyOn(Base64DecodeCursor.prototype,"step").mockImplementation(function(this:Base64DecodeCursor,...args:Parameters<typeof step>){const outcome=step.apply(this,args);if(outcome!=="complete")gate.hold();return outcome;});
  const host=document.createElement("div"),dispatch=vi.fn();host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const encoded=sourceFixture.large.pattern.repeat(sourceFixture.large.repeat);
  const view=render(<PixelEditingOverlay documentJson='{"layers":[{"kind":"pixel","id":"p","width":2,"height":1,"imageKey":"source"}]}' assetsJson={JSON.stringify({source:{mime:"image/png",data:encoded}})} assetExtentsJson='{"source":{"width":2,"height":1}}' selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#000000" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:tool,exact:true}));
  const canvas=view.getByRole("application",{name:"Image editor"});canvas.setPointerCapture=()=>{};
  act(()=>{canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));});
  await waitFor(()=>expect(view.container.querySelector("progress")!.value).toBeGreaterThan(0));
  fireEvent.click(view.getByRole("button",{name:"Cancel",exact:true}));gate.release();
  await waitFor(()=>expect(view.container.querySelector("progress")).toBeNull());
  expect(bitmap).not.toHaveBeenCalled();expect(dispatch).not.toHaveBeenCalled();
});

for(const action of fixture.actions) test("selection survives focus moving to "+action,async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div");
  host.getBoundingClientRect=()=>({left:0,top:0,width:fixture.viewportWidth,height:fixture.viewportHeight} as DOMRect);
  const dispatch=vi.fn();
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:fixture.width,height:fixture.height,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Rectangle selection"}));
  const canvas=view.getByRole("application",{name:"Image editor"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:fixture.start[0],clientY:fixture.start[1]}));
    canvas.dispatchEvent(new PointerEvent("pointermove",{bubbles:true,pointerId:1,button:0,clientX:fixture.end[0],clientY:fixture.end[1]}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:fixture.end[0],clientY:fixture.end[1]}));
    fireEvent.blur(canvas);
  });
  await waitFor(()=>expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false),10_000);
  const disclosure=view.container.querySelector("details")!;fireEvent.click(disclosure.querySelector("summary")!);disclosure.open=true;
  fireEvent.change(view.getByRole("combobox",{name:"Adjustment"}),{target:{value:"invert"}});
  fireEvent.click(view.getByRole("button",{name:action,exact:true}));
  const command=action==="Apply"?"applyFilter":"maskFromSelection";
  await waitFor(()=>expect(dispatch.mock.calls.some(call=>call[0]===command)).toBe(true),10_000);
  const call=dispatch.mock.calls.find(call=>call[0]===command);
  expect(call).toBeDefined();
  if(action==="Apply")expect(call![1]).toEqual({layerId:"p",filter:"invert",amount:0});
  const published=dispatch.mock.calls.filter(call=>call[0]==="selectPixels").at(-1)!;
  expect(published[1]).toEqual({layerId:"p",target:"pixels",width:fixture.width,height:fixture.height,expectedImageKey:null,method:"rectangle",mode:"replace",path:JSON.stringify([[fixture.pixelBounds.x,fixture.pixelBounds.y],[fixture.pixelBounds.x+fixture.pixelBounds.width,fixture.pixelBounds.y+fixture.pixelBounds.height]])});
  if(action!=="Apply")expect(call![1]).toEqual({layerId:"p",expectedImageKey:null});
  const b=fixture.pixelBounds;
  const {data}=await sharp(Buffer.from(`<svg width="${fixture.width}" height="${fixture.height}"><rect x="${b.x}" y="${b.y}" width="${b.width}" height="${b.height}" fill="white"/></svg>`)).ensureAlpha().raw().toBuffer({resolveWithObject:true});
  expect(Array.from({length:fixture.width*fixture.height},(_,i)=>data[i*4+3]).filter(Boolean)).toHaveLength(b.width*b.height);
});

for(const action of ["Crop to selection"]) test(action+" cancels selection preparation before dispatch",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:256,height:256} as DOMRect);
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:256,height:256,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  const disclosure=view.container.querySelector("details")!;disclosure.open=true;
  fireEvent.click(view.getByRole("button",{name:action,exact:true}));
  expect(view.container.querySelector("progress")?.value).toBe(0.5);
  fireEvent.click(view.getByRole("button",{name:"Cancel",exact:true}));
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(dispatch.mock.calls.filter(call=>["transformImage","maskFromSelection"].includes(call[0]))).toEqual([]);
  expect(view.container.querySelector("progress")).toBeNull();
  expect((view.getByRole("button",{name:action,exact:true}) as HTMLButtonElement).disabled).toBe(false);
});


for(const kind of ["pixel","group"]) test(kind+" mask target sends coverage strokes and clears the pixel selection",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind,id:"p",...(kind==="pixel"?{width:3,height:1,imageKey:"source"}:{children:[]}),mask:{enabled:true,linked:true,invert:false,width:3,height:1,imageKey:"mask",transform:{x:0,y:0,a:1,b:0,c:0,d:1}}}]})} assetsJson="{}" assetExtentsJson='{"source":{"width":3,"height":1},"mask":{"width":3,"height":1}}' selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={0.5} brushColor="#ff0000" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  if(kind==="pixel"){
    fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
    await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  }
  fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});
  expect(dispatch).toHaveBeenLastCalledWith("setPaintTarget",{value:"mask"});dispatch.mockClear();
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true);
  const prompt=(view.getByRole("combobox",{name:"Layer",exact:true}) as HTMLSelectElement).options[0]!;expect(prompt.text).toBe("Choose a layer with a mask");expect(prompt.disabled).toBe(true);
  fireEvent.click(view.getByRole("button",{name:"Eraser",exact:true}));
  const canvas=view.getByRole("application",{name:"Image editor"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
  });
  await waitFor(()=>expect(dispatch.mock.calls.some(call=>call[0]==="paintStroke")).toBe(true),10_000);
  const payload=dispatch.mock.calls.find(call=>call[0]==="paintStroke")![1];
  expect(payload).toEqual({layerId:"p",tool:"eraser",xs:[1.5,1.5],ys:[0.5,0.5]});
  expect(dispatch.mock.calls.some(call=>call[0]==="fillSelection")).toBe(false);
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.change(view.getByRole("spinbutton",{name:"Mask value"}),{target:{value:"64"}});
  expect(dispatch).toHaveBeenCalledWith("setMaskValue",{value:64});
  expect(dispatch.mock.calls.filter(call=>call[0]==="selectPixels").at(-1)![1]).toEqual({layerId:"p",target:"mask",width:3,height:1,expectedImageKey:"mask",method:"all",mode:"replace",path:"[]"});
  fireEvent.click(view.getByRole("button",{name:"Fill mask",exact:true}));
  await waitFor(()=>expect(dispatch.mock.calls.filter(call=>call[0]==="fillSelection")).toEqual([["fillSelection",{layerId:"p"}]]),10_000);
});

for(const action of fixture.completionActions)for(const completion of fixture.completionCases)test(action.label+" waits for "+completion.outcome.kind+" completion",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div");
  let settle!:(value:unknown)=>void;
  const pending=new Promise(resolve=>{settle=resolve;});
  const dispatch=vi.fn((action:string,_args?:Record<string,unknown>)=>action==="selectPixels"?Promise.resolve({kind:"applied"}):pending);
  const view=render(<PixelEditingOverlay documentJson='{"layers":[{"kind":"pixel","id":"p","width":3,"height":1,"mask":{"enabled":true,"linked":true,"invert":false,"width":3,"height":1,"transform":{"x":0,"y":0,"a":1,"b":0,"c":0,"d":1}}}]}' assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  view.container.querySelector("details")!.open=true;
  if(action.target==="mask"){fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});expect(dispatch).toHaveBeenLastCalledWith("setPaintTarget",{value:"mask"});dispatch.mockClear();}
  if(action.selection)fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  const apply=view.getByRole("button",{name:action.label,exact:true}) as HTMLButtonElement;
  act(()=>{fireEvent.click(apply);fireEvent.click(apply);});
  await waitFor(()=>expect(editCalls(dispatch)).toHaveLength(1));
  expect(editCalls(dispatch)[0]![0]).toBe(action.command);
  await act(async()=>{await Promise.resolve();});
  expect(apply.disabled).toBe(true);
  expect(view.getByText("Editing image; progress and cancellation are available in Tasks.")).toBeDefined();
  fireEvent.click(view.getByRole("button",{name:"Pan canvas",exact:true}));
  expect(apply.disabled).toBe(true);
  expect([...view.container.querySelectorAll("button")].some(button=>button.textContent==="Cancel")).toBe(false);
  await act(async()=>{settle(completion.outcome);await pending;});
  await waitFor(()=>expect(apply.disabled).toBe(false));
  if(completion.error)expect(view.getByRole("alert").textContent).toContain(completion.message);
  else expect(view.getByText(completion.message)).toBeDefined();
  expect(view.container.querySelector('[role="alert"]')!==null).toBe(completion.error);
});

for(const action of fixture.selectionControls.actions)for(const completion of fixture.completionCases)test(action+" retains published coverage until "+completion.outcome.kind,async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const {width,height}=fixture.selectionControls;
  let settle!:(value:unknown)=>void;const receipt=new Promise(resolve=>{settle=resolve;}),dispatch=vi.fn((_action:string,_args?:Record<string,unknown>)=>receipt);
  const initial={layerId:"p",target:"pixels",width,height,spans:[{start:0,length:1,coverage:128}]};
  const props={documentJson:JSON.stringify({layers:[{kind:"pixel",id:"p",width,height}]}),assetsJson:"{}",assetExtentsJson:"{}",selectionJson:'["p"]',pixelSelectionJson:JSON.stringify(initial),activeUtility:"select",brushSize:24,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,editingOptions:initialEditingOptions,camera:{current:{x:0,y:0,zoom:1}},container:{current:document.createElement("div")},dispatch,onWheel:()=>{},onCameraChange:()=>{}};
  const view=render(<ControlledPixelEditingOverlay {...props}/>);
  await waitFor(()=>expect((view.getByRole("button",{name:action,exact:true}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.click(view.getByRole("button",{name:action,exact:true}));
  expect(dispatch).toHaveBeenCalledWith("selectPixels",{layerId:"p",target:"pixels",width,height,expectedImageKey:null,method:action==="Select all pixels"?"all":"invert",mode:"replace",path:"[]"});
  expect(view.getByText("Editing image; progress and cancellation are available in Tasks.")).toBeDefined();
  expect(view.container.querySelector("progress")).toBeNull();
  fireEvent.click(view.getByRole("button",{name:"Pan canvas",exact:true}));
  fireEvent.keyDown(view.getByRole("application",{name:"Image editor"}),{key:"Escape"});
  expect(dispatch).toHaveBeenCalledTimes(1);
  await act(async()=>{settle(completion.outcome);await receipt;});
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false);
  if(completion.error)expect(view.getByRole("alert").textContent).toContain(completion.message);
  else expect(view.getByText(completion.message)).toBeDefined();
  view.rerender(<ControlledPixelEditingOverlay {...props} pixelSelectionJson={undefined}/>);
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true);
});

for(const modifier of fixture.selectionControls.keyboardModifiers)test(modifier+" selection shortcuts wait for authoritative receipt",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const {width,height}=fixture.selectionControls;
  let settle!:(value:unknown)=>void;const receipt=new Promise(resolve=>{settle=resolve;}),dispatch=vi.fn((_action:string,_args?:Record<string,unknown>)=>receipt);
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width,height}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="paintBrush" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:document.createElement("div")}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  const canvas=view.getByRole("application",{name:"Image editor"});
  fireEvent.keyDown(canvas,{key:"a",code:"KeyA",[modifier]:true});fireEvent.keyDown(canvas,{key:"d",code:"KeyD",[modifier]:true});
  expect(dispatch).toHaveBeenCalledTimes(1);expect(dispatch.mock.calls[0]![0]).toBe("selectPixels");
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true);
  await act(async()=>{settle({kind:"applied"});await receipt;});
  await waitFor(()=>expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false));
  fireEvent.keyDown(canvas,{key:"d",code:"KeyD",[modifier]:true});
  await waitFor(()=>expect(dispatch).toHaveBeenLastCalledWith("selectPixels",{layerId:"p",target:"pixels",width,height,expectedImageKey:null,method:"none",mode:"replace",path:"[]"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true));
});

for(const row of fixture.selectionIntents)for(const cancellation of [null,...fixture.selectionCancellation])test(row.method+" "+row.mode+" "+(cancellation??"submits intrinsic intent"),async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const props={documentJson:'{"layers":[{"kind":"pixel","id":"p","width":64,"height":64}]}',assetsJson:"{}",assetExtentsJson:"{}",selectionJson:'["p"]',activeUtility:"select",brushSize:1,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,camera:{current:{x:0,y:0,zoom:1}},container:{current:host},dispatch,onWheel:()=>{},onCameraChange:()=>{}};
  const view=render(<PixelEditingOverlay {...props}/>),canvas=view.getByRole("application",{name:"Image editor"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  fireEvent.click(view.getByRole("button",{name:row.label,exact:true}));
  fireEvent.change(view.getByRole("combobox",{name:"Selection",exact:true}),{target:{value:row.mode}});
  fireEvent.pointerDown(canvas,{pointerId:1,button:0,clientX:row.start[0],clientY:row.start[1]});
  if("middle" in row&&row.middle)fireEvent.pointerMove(canvas,{pointerId:1,button:0,clientX:row.middle[0],clientY:row.middle[1]});
  if(cancellation==="pointercancel")fireEvent.pointerCancel(canvas,{pointerId:1});
  else if(cancellation==="Escape")fireEvent.keyDown(canvas,{key:"Escape"});
  else if(cancellation==="Pan canvas")fireEvent.click(view.getByRole("button",{name:"Pan canvas",exact:true}));
  else if(cancellation==="Switch utility")view.rerender(<PixelEditingOverlay {...props} activeUtility="paintBrush"/>);
  fireEvent.pointerUp(canvas,{pointerId:1,button:0,clientX:row.end[0],clientY:row.end[1]});
  if(cancellation){expect(dispatch).not.toHaveBeenCalled();return;}
  await waitFor(()=>expect(dispatch).toHaveBeenCalledTimes(1));
  expect(dispatch).toHaveBeenCalledWith("selectPixels",{layerId:"p",target:"pixels",width:64,height:64,expectedImageKey:null,method:row.method,mode:row.mode,path:JSON.stringify(row.path)});
});

for(const inherited of [false,true])test((inherited?"Inherited":"Own")+" layer protection preserves selection and refuses pixel and mask edits",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const layer={kind:"pixel",id:"p",width:3,height:1,locked:!inherited,mask:{enabled:true,linked:true,width:3,height:1,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}};
  const layers=inherited?[{kind:"group",id:"g",locked:true,children:[layer]}]:[layer];
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="paintBrush" brushSize={1} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  expect(view.getByText("Unlock this layer or its parent in Inspection to edit pixels or masks.")).toBeDefined();
  const canvas=view.getByRole("application",{name:"Image editor"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));
    fireEvent.keyDown(canvas,{key:"Delete"});
  });
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false));
  view.container.querySelector("details")!.open=true;
  for(const name of ["Apply","Crop to selection","Mask from selection","Resize layer","Clear selected pixels"])expect((view.getByRole("button",{name,exact:true}) as HTMLButtonElement).disabled).toBe(true);
  fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});
  expect(dispatch).toHaveBeenLastCalledWith("setPaintTarget",{value:"mask"});dispatch.mockClear();
  expect((view.getByRole("button",{name:"Fill mask"}) as HTMLButtonElement).disabled).toBe(true);
  expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false);
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(editCalls(dispatch)).toEqual([]);
});

for(const inherited of [false,true])test((inherited?"Parent":"Layer")+" locking cancels pixel edit preparation before dispatch",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const {width,height}=fixture.selectionControls,host=document.createElement("div"),dispatch=vi.fn();
  const editor=(locked:boolean)=>{
    const layer={kind:"pixel",id:"p",width,height,locked:!inherited&&locked};
    return <PixelEditingOverlay documentJson={JSON.stringify({layers:inherited?[{kind:"group",id:"g",locked,children:[layer]}]:[layer]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={24} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>;
  };
  const view=render(editor(false));
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect((view.getByRole("button",{name:"Select all pixels"}) as HTMLButtonElement).disabled).toBe(false));
  view.container.querySelector("details")!.open=true;
  fireEvent.click(view.getByRole("button",{name:"Crop to selection",exact:true}));
  expect(view.container.querySelector("progress")).not.toBeNull();
  view.rerender(editor(true));
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(view.container.querySelector("progress")).toBeNull();expect(editCalls(dispatch)).toEqual([]);
  expect((view.getByRole("button",{name:"Crop to selection",exact:true}) as HTMLButtonElement).disabled).toBe(true);
  view.rerender(editor(false));
  fireEvent.click(view.getByRole("button",{name:"Crop to selection",exact:true}));
  await waitFor(()=>expect(editCalls(dispatch)).toHaveLength(1));
  expect(editCalls(dispatch)[0]).toEqual(["transformImage",{layerId:"p",operation:"crop",x:0,y:0,width,height,bilinear:false}]);
});

for(const kind of ["pixel","group"])for(const sample of fixture.affineMaskTargets)test(kind+" strokes preserve "+sample.name+" coordinates",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const mask={enabled:true,linked:sample.linked,invert:false,width:4,height:4,transform:sample.mask};
  const layer={kind,id:"p",width:4,height:4,children:[],transform:sample.layer,mask};
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[layer]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="paintBrush" brushSize={1} brushOpacity={1} brushColor="#ffffff" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.change(view.getByRole("combobox",{name:"Edit target"}),{target:{value:"mask"}});
  expect(dispatch).toHaveBeenLastCalledWith("setPaintTarget",{value:"mask"});dispatch.mockClear();
  const canvas=view.getByRole("application",{name:"Image editor"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{for(const type of ["pointerdown","pointerup"])canvas.dispatchEvent(new PointerEvent(type,{bubbles:true,pointerId:1,button:0,clientX:sample.client[0],clientY:sample.client[1]}));});
  await waitFor(()=>expect(editCalls(dispatch)).toHaveLength(1));
  const [command,payload]=editCalls(dispatch)[0]!;
  expect(command).toBe("paintStroke");
  expect(payload).toEqual({layerId:"p",tool:"brush",xs:[sample.point[0],sample.point[0]],ys:[sample.point[1],sample.point[1]]});
});

import patch,{type Operation} from "fast-json-patch";
import { paintStrokeRevisionFixture as revisions } from "@semio-tech/framework-surface-ts/testing/fixtures";
for(const row of revisions.cases)test("active stroke handles target revision "+row.name,async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const props={documentJson:JSON.stringify(revisions.document),assetsJson:"{}",assetExtentsJson:"{}",selectionJson:'["paint"]',activeUtility:"paintBrush",brushSize:1,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,camera:{current:{x:0,y:0,zoom:1}},container:{current:host},dispatch,onWheel:()=>{},onCameraChange:()=>{}};
  const view=render(<PixelEditingOverlay {...props}/>);
  const canvas=view.getByRole("application",{name:"Image editor"});canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  fireEvent.pointerDown(canvas,{pointerId:1,button:0,clientX:50,clientY:50});
  const changed=patch.applyPatch(revisions.document,row.patch as Operation[],true,false).newDocument;
  view.rerender(<PixelEditingOverlay {...props} documentJson={JSON.stringify(changed)} selectionJson={JSON.stringify(row.selected)} activeUtility={row.utility}/>);
  fireEvent.pointerUp(canvas,{pointerId:1,button:0,clientX:50,clientY:50});
  if(row.cancel){await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});expect(dispatch.mock.calls.filter(call=>call[0]==="paintStroke")).toEqual([]);}
  else await waitFor(()=>expect(dispatch.mock.calls.filter(call=>call[0]==="paintStroke")).toHaveLength(1));
});

for(const spans of [[],[{start:1,length:2,coverage:128}]])test("restores shared selection "+spans+" across image content updates",async()=>{
  vi.stubGlobal("ResizeObserver",class{observe(){}disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  const props={documentJson:JSON.stringify({layers:[{kind:"pixel",id:"p",width:3,height:2}]}),assetsJson:"{}",assetExtentsJson:"{}",selectionJson:'["p"]',pixelSelectionJson:JSON.stringify({layerId:"p",target:"pixels",width:3,height:2,spans}),activeUtility:"select",brushSize:1,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,editingOptions:initialEditingOptions,camera:{current:{x:0,y:0,zoom:1}},container:{current:host},dispatch,onWheel:()=>{},onCameraChange:()=>{}};
  const view=render(<ControlledPixelEditingOverlay {...props}/>);
  await waitFor(()=>expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false));
  view.rerender(<ControlledPixelEditingOverlay {...props} documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:3,height:2,imageKey:"new-image"}]})} assetExtentsJson='{"new-image":{"width":3,"height":2}}'/>);
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(false);
  view.container.querySelector("details")!.open=true;fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("applyFilter",{layerId:"p",filter:"brightness",amount:0}));
  expect(dispatch.mock.calls.some(call=>call[0]==="transformImage")).toBe(false);
});

test("menu filters and grid changes each dispatch ONE parametric verb",async()=>{
  vi.stubGlobal("ResizeObserver",class{observe(){}disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  const view=render(<PixelEditingOverlay documentJson='{"layers":[{"kind":"pixel","id":"p","width":3,"height":2}]}' assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  const disclosure=view.container.querySelector("details")!;disclosure.open=true;
  fireEvent.change(view.getByRole("combobox",{name:"Adjustment"}),{target:{value:"blur"}});
  fireEvent.change(view.getByRole("spinbutton",{name:"Amount"}),{target:{value:"3"}});
  fireEvent.click(view.getByRole("button",{name:"Apply",exact:true}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("applyFilter",{layerId:"p",filter:"blur",amount:3}));
  dispatch.mockClear();
  fireEvent.click(view.getByRole("button",{name:"Flip horizontally",exact:true}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("applyFilter",{layerId:"p",filter:"flipHorizontal",amount:0}));
  dispatch.mockClear();
  fireEvent.click(view.getByRole("button",{name:"Rotate right",exact:true}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("transformImage",{layerId:"p",operation:"rotateClockwise",x:0,y:0,width:0,height:0,bilinear:false}));
  dispatch.mockClear();
  fireEvent.change(view.getByRole("spinbutton",{name:"Width"}),{target:{value:"6"}});
  fireEvent.change(view.getByRole("spinbutton",{name:"Height"}),{target:{value:"4"}});
  fireEvent.click(view.getByRole("button",{name:"Resize layer",exact:true}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("transformImage",{layerId:"p",operation:"resize",x:0,y:0,width:6,height:4,bilinear:true}));
});
test("refused completed selection does not replace authoritative coverage",{timeout:20000},async()=>{
  vi.stubGlobal("ResizeObserver",class{observe(){}disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn(async()=>({kind:"refused"}));
  const view=render(<ControlledPixelEditingOverlay documentJson='{"layers":[{"kind":"pixel","id":"p","width":3,"height":2}]}' assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#2878dc" brushHardness={1} paintTarget="pixels" maskValue={255} fillTolerance={24} editingOptions={initialEditingOptions} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Select all pixels"}));
  await waitFor(()=>expect(dispatch).toHaveBeenCalledWith("selectPixels",{layerId:"p",target:"pixels",width:3,height:2,method:"all",mode:"replace",path:"[]",expectedImageKey:null}));
  expect((view.getByRole("button",{name:"Deselect"}) as HTMLButtonElement).disabled).toBe(true);
  expect(view.getByRole("alert").textContent).toContain("could not be applied");
});

test("incoming shared coverage cancels an in-flight brush stroke",async()=>{
  vi.stubGlobal("ResizeObserver",class{observe(){}disconnect(){}});vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const props={documentJson:'{"layers":[{"kind":"pixel","id":"p","width":3,"height":2}]}',assetsJson:"{}",assetExtentsJson:"{}",selectionJson:'["p"]',activeUtility:"paintBrush",brushSize:1,brushOpacity:1,brushColor:"#2878dc",brushHardness:1,paintTarget:"pixels" as const,maskValue:255,fillTolerance:24,editingOptions:initialEditingOptions,camera:{current:{x:0,y:0,zoom:1}},container:{current:host},dispatch,onWheel:()=>{},onCameraChange:()=>{}};
  const view=render(<ControlledPixelEditingOverlay {...props}/>),canvas=view.getByRole("application",{name:"Image editor"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));});
  view.rerender(<ControlledPixelEditingOverlay {...props} pixelSelectionJson='{"layerId":"p","target":"pixels","width":3,"height":2,"spans":[]}'/>);
  act(()=>{canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));});
  await act(async()=>{await new Promise(resolve=>setTimeout(resolve,20));});
  expect(editCalls(dispatch)).toEqual([]);
});

test("the bucket dispatches ONE parametric fillRegion click and never floods or fills on the host",async()=>{
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div"),dispatch=vi.fn();
  host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const view=render(<PixelEditingOverlay documentJson={JSON.stringify({layers:[{kind:"pixel",id:"p",width:3,height:1,transform:{x:0,y:0,a:1,b:0,c:0,d:1}}]})} assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="select" brushSize={1} brushOpacity={1} brushColor="#ff0000" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  fireEvent.click(view.getByRole("button",{name:"Fill",exact:true}));
  const canvas=view.getByRole("application",{name:"Image editor"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  act(()=>{canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:50,clientY:50}));});
  await waitFor(()=>expect(dispatch.mock.calls.some(call=>call[0]==="fillRegion")).toBe(true),10_000);
  expect(dispatch.mock.calls.filter(call=>call[0]==="fillRegion")).toEqual([["fillRegion",{layerId:"p",x:1.5,y:0.5}]]);
  expect(editCalls(dispatch)).toEqual([]);
});

function strokeCanvas(dispatch:ComponentProps<typeof ControlledPixelEditingOverlay>["dispatch"]){
  vi.stubGlobal("ResizeObserver",class {observe(){} disconnect(){}});
  vi.spyOn(HTMLCanvasElement.prototype,"getContext").mockReturnValue(null);
  const host=document.createElement("div");host.getBoundingClientRect=()=>({left:0,top:0,width:100,height:100} as DOMRect);
  const view=render(<PixelEditingOverlay documentJson='{"layers":[{"kind":"pixel","id":"p","width":64,"height":64}]}' assetsJson="{}" assetExtentsJson="{}" selectionJson='["p"]' activeUtility="paintBrush" brushSize={1} brushOpacity={1} brushColor="#2878dc" brushHardness={1} camera={{current:{x:0,y:0,zoom:1}}} container={{current:host}} dispatch={dispatch} onWheel={()=>{}} onCameraChange={()=>{}}/>);
  const canvas=view.getByRole("application",{name:"Image editor"});
  canvas.setPointerCapture=()=>{};canvas.releasePointerCapture=()=>{};canvas.hasPointerCapture=()=>true;
  return canvas;
}
const strokeCalls=(dispatch:ReturnType<typeof vi.fn>)=>dispatch.mock.calls.filter(call=>call[0]==="paintStroke").map(call=>call[1] as {phase?:string;gesture?:string;reason?:string;xs:number[];ys:number[]});

test("a long brush stroke streams batches under one press and commits the rest",async()=>{
  const dispatch=vi.fn(),canvas=strokeCanvas(dispatch);
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:20,clientY:50}));
    for(let step=1;step<=40;step++)canvas.dispatchEvent(new PointerEvent("pointermove",{bubbles:true,pointerId:1,button:0,clientX:20+step,clientY:50}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:1,button:0,clientX:61,clientY:50}));
  });
  await waitFor(()=>expect(strokeCalls(dispatch)).toHaveLength(3),10_000);
  const strokes=strokeCalls(dispatch);
  expect(strokes.map(args=>args.phase)).toEqual(["stream","stream","commit"]);
  expect(new Set(strokes.map(args=>args.gesture)).size).toBe(1);
  expect(strokes[0]!.gesture).toMatch(/^paint:/);
  expect(strokes.map(args=>args.xs.length)).toEqual([16,16,10]);
  const xs=strokes.flatMap(args=>args.xs);
  expect(xs).toEqual([...xs].sort((a,b)=>a-b));
  expect(new Set(xs).size).toBe(42);
});

test("a cancelled streamed stroke aborts its press and a short stroke stays one dispatch",async()=>{
  const dispatch=vi.fn(),canvas=strokeCanvas(dispatch);
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:1,button:0,clientX:20,clientY:50}));
    for(let step=1;step<=20;step++)canvas.dispatchEvent(new PointerEvent("pointermove",{bubbles:true,pointerId:1,button:0,clientX:20+step,clientY:50}));
    canvas.dispatchEvent(new PointerEvent("pointercancel",{bubbles:true,pointerId:1,button:0,clientX:41,clientY:50}));
  });
  await waitFor(()=>expect(strokeCalls(dispatch)).toHaveLength(2),10_000);
  const [stream,abort]=strokeCalls(dispatch);
  expect([stream!.phase,abort!.phase,abort!.reason]).toEqual(["stream","abort","captureLost"]);
  expect(abort!.gesture).toBe(stream!.gesture);
  expect(abort!.xs).toEqual([]);
  dispatch.mockClear();
  act(()=>{
    canvas.dispatchEvent(new PointerEvent("pointerdown",{bubbles:true,pointerId:2,button:0,clientX:20,clientY:50}));
    canvas.dispatchEvent(new PointerEvent("pointermove",{bubbles:true,pointerId:2,button:0,clientX:24,clientY:50}));
    canvas.dispatchEvent(new PointerEvent("pointerup",{bubbles:true,pointerId:2,button:0,clientX:28,clientY:50}));
  });
  await waitFor(()=>expect(strokeCalls(dispatch)).toHaveLength(1),10_000);
  expect(strokeCalls(dispatch)[0]).toEqual({layerId:"p",tool:"brush",xs:[2,6,10],ys:[32,32,32]});
});
