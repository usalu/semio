export type DoorHolderV1=Readonly<{pid:number;command:string;startedMs:number;digest:string}>;
export type DoorProgressV1=Readonly<{phase:string;done:number;total:number;advancedMs:number}>;
export type DoorAskV1=Readonly<{version:1;reason:"stalled"|"stale-code";atMs:number;by:Readonly<{pid:number;digest:string}>}>;
export type DoorClaimViewV1=Readonly<{state:string;holder?:DoorHolderV1;progress?:DoorProgressV1;ask?:DoorAskV1}>;
/** 🪧 Remembers the directory of the claim documents this process queues in. */
export function rememberDoorV1(directory:string):void;
/** 🪧 The directory remembered by the last custody queue of this process. */
export function rememberedDoorV1():string|undefined;
/** 🪪 Admits a closed holder. */
export function readHolderV1(value:unknown):DoorHolderV1;
/** 📈 Admits a closed progress document. */
export function readProgressV1(value:unknown):DoorProgressV1;
/** 🙏 Admits a closed request to hand over. */
export function readAskV1(value:unknown):DoorAskV1;
/** 🪪 The holder document of this process. */
export function currentHolderV1(options?:{digest?:string;pid?:number;argv?:readonly string[];uptimeSeconds?:number;now?:()=>number}):DoorHolderV1;
/** 📈 The progress after one event; it advances when the phase changed or more owners are done. */
export function advanceProgressV1(previous:DoorProgressV1|undefined,event:{phase?:string;done?:number;total?:number},now:number):DoorProgressV1;
/** ⏱ The milliseconds since the last advance of a claim; undefined when it never reported. */
export function idleMillisecondsV1(claim:{progress?:DoorProgressV1},now:number):number|undefined;
/** ⚖ Why a held claim is asked to hand over. */
export function askReasonV1(options:{claim:DoorClaimViewV1;now:number;current?:string|undefined}):"stalled"|"stale-code"|undefined;
/** 🚪 Whether a claim lost the door. */
export function isDeposedV1(claim:{ask?:DoorAskV1},now:number):boolean;
/** 🧾 The text about one holder in a language. */
export function holderTextV1(claim:DoorClaimViewV1,now:number,language:string):string;
/** 🧾 The same as a suffix of a waiting line. */
export function holderSuffixV1(claim:DoorClaimViewV1,now:number,language:string):string;
/** 👀 The claims that hold the door of a directory, read without the custody. */
export function readDoorV1(directory:string,options?:{isAlive?:(pid:number)=>boolean}):readonly (DoorClaimViewV1&{token:string;ticket:number;structural:boolean;owners:readonly string[]})[];
/** 🪧 What the front door says about the holders of a directory. */
export function doorSuffixV1(directory:string|undefined,options?:{now?:number;language?:string;limit?:number;isAlive?:(pid:number)=>boolean}):string;
