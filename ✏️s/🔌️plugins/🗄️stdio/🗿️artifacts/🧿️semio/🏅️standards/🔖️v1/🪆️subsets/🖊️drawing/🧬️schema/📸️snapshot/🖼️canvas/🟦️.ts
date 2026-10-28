/** 🖼️ Canvas edits allocate only the two shallow owner records and retain all unrelated original leaves. */
import type {SemioDrawingSnapshot} from '../🟦️.ts';
import {binary64Value,parseBinary64,type Binary64} from '@semio-tech/framework/value/ieee754';
export function withCanvasSize(content:SemioDrawingSnapshot,width:Binary64,height:Binary64):SemioDrawingSnapshot{width=parseBinary64(width);height=parseBinary64(height);if(!Number.isFinite(binary64Value(width))||!Number.isFinite(binary64Value(height))||binary64Value(width)<0||binary64Value(height)<0||content.schema!=='stdio.semio.drawing')throw Error('drawing.canvas-size');return{...content,canvas:{...content.canvas,width,height}};}
