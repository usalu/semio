/** 🔮️ Plain first-party geometry crosses the test-only Sharp interface. */
import type {PathSegment,Vec2} from "../../../../../../📐️drawing-contract/🟦️.ts";
export function originalCoverage(segments:RetainedWorkInputSequence<PathSegment>,transform:readonly number[],size:readonly number[],scale:number):Promise<Uint8Array>;
export function actualCoverage(contours:readonly {closed:boolean;points:readonly Vec2[]}[],transform:readonly number[],size:readonly number[],scale:number):Promise<Uint8Array>;
import type {RetainedWorkInputSequence} from "../../../../../../../🌱️value/📋️list/🔗️persistent/🎟️work/🟦️.ts";
export function originalPath(segments:RetainedWorkInputSequence<PathSegment>):string;
