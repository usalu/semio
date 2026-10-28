/// <reference path="./🔤️reference.d.ts" />
/** 📚️ Test-only Unicode16 oracle exposes first-party UTF-8 boundary observations. */
import {Rules} from '@cto.af/linebreak';
import type {LineBreakPoint} from '../../🟦️.ts';
export interface LineBreakReference{breaks(text:string):Iterable<{position:number;required:boolean}>}
export type LineBreakOracle={textBreaks(text:string):LineBreakPoint[]};
export function createLineBreakOracle():LineBreakOracle{const rules=new Rules();return{textBreaks:text=>[...rules.breaks(text)].map(point=>({at:Buffer.byteLength(text.slice(0,point.position)),mandatory:point.required}))};}
