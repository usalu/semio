/** 🧫️ Explicit owned words constructed from the neutral native numeric wire corpus. */
import {binary64,binary32} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
function word64(value:unknown):unknown{return typeof value==="number"?binary64(value):value;}
function word32(value:unknown):unknown{return typeof value==="number"?binary32(value):value;}
export function point3Fixture(value:any):any{return value!==null&&typeof value==="object"?{...value,x:word64(value.x),y:word64(value.y),z:word64(value.z)}:value;}
export function point2Fixture(value:any):any{return value!==null&&typeof value==="object"?{...value,x:word64(value.x),y:word64(value.y)}:value;}
export function uvFixture(value:any):any{return value!==null&&typeof value==="object"?{...value,u:word64(value.u),v:word64(value.v)}:value;}
export function rgbaFixture(value:any):any{return value!==null&&typeof value==="object"?{...value,r:word32(value.r),g:word32(value.g),b:word32(value.b),a:word32(value.a)}:value;}
export function quaternionFixture(value:any):any{return value!==null&&typeof value==="object"?{...value,x:word64(value.x),y:word64(value.y),z:word64(value.z),w:word64(value.w)}:value;}
export function transformFixture(value:any):any{return value!==null&&typeof value==="object"?{...value,translation:point3Fixture(value.translation),rotation:quaternionFixture(value.rotation),scale:point3Fixture(value.scale)}:value;}
