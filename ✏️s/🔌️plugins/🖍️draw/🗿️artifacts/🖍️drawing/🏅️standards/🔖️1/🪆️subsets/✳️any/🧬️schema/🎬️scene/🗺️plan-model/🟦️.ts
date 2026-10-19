/** 🗺️ The prepared drawing scene plan every scene job reads: paint, node content, node and plan types and the source address validation. */

import type { DrawingFontFamily } from "../../📝️text/🔤️family/🟦️.ts";
import type { RasterSceneAsset, RasterSceneGroup, RasterSceneNode, RasterSceneContent } from "../📷️raster/🟦️.ts";
import type { PathRasterStroke } from "../../🧮️geometry/📷️raster/🟦️.ts";
import type { Fill } from "../../🎨️fill/🖌️paint-model/🟦️.ts";
import type { PathSegment } from "@semio-tech/framework/2d";

export type Paint={fillRule:"nonzero"|"evenodd";fill:Fill|null;stroke:PathRasterStroke|null};

export type DocumentSceneContent=({kind:"path";segments:PathSegment[]}&Paint)|({kind:"glyphs";content:string;x:number;y:number;size:number;fontFamily:DrawingFontFamily;segments:PathSegment[]}&Paint)|Extract<RasterSceneContent,{kind:"image"}>|{kind:"group";children:string[];isolation:boolean}|({kind:"text";content:string;x:number;y:number;size:number;fontFamily:DrawingFontFamily}&Paint)|({kind:"boolean";operation:"union"|"difference"|"intersection"|"xor";children:string[];referenceTransform:Matrix}&Paint)|({kind:"trace";source:string;threshold:number;simplifyEpsilon:number}&Paint);

export type DocumentSceneNode=Omit<RasterSceneNode,"content"|"groups">&{sourcePath:number[];lockedAncestors:number;groups:RasterSceneGroup[];content:DocumentSceneContent};

export type DocumentScenePlan={assets:RasterSceneAsset[];nodes:DocumentSceneNode[]};

/** 🧭️ Validate at most 32 authored indices and the exact unsigned lock-mask width. */
export function validateSceneSourceAddress(node:{readonly sourcePath:readonly number[];readonly lockedAncestors:number}):void{
 const path=node.sourcePath,locks=node.lockedAncestors;if(!Array.isArray(path)||path.length<1||path.length>32||!path.every(index=>Number.isSafeInteger(index)&&index>=0&&index<=65535)||!Number.isSafeInteger(locks)||locks<0||locks>=2**path.length)throw Error("Invalid scene source ancestry");
}

export type Matrix=[number,number,number,number,number,number];
