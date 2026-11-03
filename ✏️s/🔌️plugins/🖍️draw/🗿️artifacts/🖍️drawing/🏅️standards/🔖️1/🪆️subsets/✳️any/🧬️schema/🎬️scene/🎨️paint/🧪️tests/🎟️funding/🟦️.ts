/** 🏦️ Test-only caller funding: each turn borrows a fresh finite ledger sized to its quantum. */
import {RetainedWorkLedger,type RetainedWorkRetirementCursor} from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
import {binary64} from "@semio-tech/framework/value/ieee754";
import {DrawingAssetsOwner} from "../../../../🟦️.ts";
import {DocumentSceneJob} from "../../../📋️prepare/🎟️owned/🟦️.ts";
import {DrawingSceneWorkController} from "../../../🎟️work/🟦️.ts";
import type {ScenePlanOwner} from "../../../🗺️plan-model/🎟️owned/🟦️.ts";
export const stock=(quantum:number):RetainedWorkLedger=>new RetainedWorkLedger({maximumItems:Number.isSafeInteger(quantum)&&quantum>=1?quantum:1,maximumCopyBytes:2**30,maximumCapacityBytes:2**30,maximumReleaseBytes:2**30,maximumDepth:64});
export function drain(cursor:RetainedWorkRetirementCursor,quantum=4096):void{for(let at=0;at<10_000_000&&!cursor.terminalIsEmpty();at++)cursor.advance(quantum,stock(quantum));if(!cursor.terminalIsEmpty())throw Error("Retirement exceeded finite fuel");}
/** 🏷️ Authored fixture assets enter preparation as the authentic cold asset owner. */
export const owned=<T extends {assets?:unknown}>(document:T):T=>({...document,assets:DrawingAssetsOwner.fromEntriesCold(Object.entries((document.assets??{}) as Record<string,any>))});
/** 🏃️ Runs one caller-funded producer to completion with a fresh finite ledger per turn. */
export function drive<P extends {done:boolean}>(job:{advance(items:number,work:RetainedWorkLedger):P},quantum=4096,observe?:(progress:P)=>void):P{for(let at=0;at<20_000_000;at++){const progress=job.advance(quantum,stock(quantum));observe?.(progress);if(progress.done)return progress;}throw Error("Producer exceeded finite fuel");}
/** 🔬️ Borrowed sequences project to plain arrays so independent oracles can compare them. */
export function plain(value:any):any{if(value===null||typeof value!=="object")return value;if(ArrayBuffer.isView(value))return value;if(Array.isArray(value))return value.map(plain);if(typeof value.at==="function"&&typeof value.length==="number")return Array.from({length:value.length},(_,at)=>plain(value.at(at)));return Object.fromEntries(Object.entries(value).map(([key,item])=>[key,plain(item)]));}
/** 🏗️ Authored fixture layers prepare into an owned scene plan under ample funding. */
const lifted=(value:any):any=>typeof value==='number'?binary64(value):Array.isArray(value)?value.map(lifted):value&&typeof value==='object'&&!ArrayBuffer.isView(value)&&Object.getPrototypeOf(value)===Object.prototype?Object.fromEntries(Object.entries(value).map(([key,item])=>[key,lifted(item)])):value;
export function sceneOwner(document:any,limits:any,maximumWork=1e9):ScenePlanOwner{const job=new DocumentSceneJob(owned({...document,layers:lifted(document.layers)}) as any,limits,maximumWork);drive(job);const moved=job.intoRetirement();drain(moved.job);return moved.output!;}

/** 🎛️ A caller controller that yields to the event loop on every offered turn. */
export function controllerFixture(source:object,quantum:number,signal?:AbortSignal){const operation={},big=2**50,grant=()=>({maximumItems:quantum,maximumCopyBytes:2**30,maximumCapacityBytes:2**30,maximumReleaseBytes:2**30,maximumDepth:64});const work=new DrawingSceneWorkController({operation,source,stock:{maximumItems:big,maximumCopyBytes:big,maximumCapacityBytes:big,maximumReleaseBytes:big,maximumDepth:64},fuel:big,provideTurn:async()=>{await new Promise<void>(resolve=>setTimeout(resolve,0));return{grant:grant(),fuel:quantum};},wait:async()=>{await new Promise<void>(resolve=>setTimeout(resolve,0));},signal});return{control:{work,operation,source}};}
