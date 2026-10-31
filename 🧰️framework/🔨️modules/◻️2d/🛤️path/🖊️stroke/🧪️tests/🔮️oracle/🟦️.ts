/** 🔮️ Independent original Sharp128x SVG stroke reference behind a checked first-party read-sequence test interface. */
import sharp from "sharp";
import type {StrokeOutlineInput} from "../../🟦️.ts";
export async function reference(source:StrokeOutlineInput,extent:readonly number[]):Promise<number[]> {
 const scale=128,w=extent[0]!,h=extent[1]!,s=source.style,contours=Array.from({length:source.contours.length},(_,at)=>{const contour=source.contours.at(at);if(contour===undefined)throw Error("Original SVG stroke contour absent");return{closed:contour.closed,points:Array.from({length:contour.points.length},(_,index)=>{const point=contour.points.at(index);if(point===undefined)throw Error("Original SVG stroke point absent");return point;})};}),transform=Array.from({length:source.transform.length},(_,at)=>{const value=source.transform.at(at);if(value===undefined)throw Error("Original SVG stroke matrix absent");return value;}),dash=Array.from({length:s.dash.length},(_,at)=>{const value=s.dash.at(at);if(value===undefined)throw Error("Original SVG stroke dash absent");return value;});
 const d=contours.filter(c=>c.points.length>1||c.closed).map(c=>c.points.length?`M${c.points.map(p=>p.join(",")).join("L")}${c.closed?"Z":""}`:"").join("");
 const svg=`<svg xmlns="http://www.w3.org/2000/svg" width="${w*scale}" height="${h*scale}" viewBox="0 0 ${w} ${h}"><path d="${d}" transform="matrix(${transform.join(" ")})" fill="none" stroke="red" stroke-width="${s.width}" stroke-linecap="${s.cap}" stroke-linejoin="${s.join}" stroke-miterlimit="${s.miterLimit}" stroke-dasharray="${s.dash.length?dash.join(" "):"none"}" stroke-dashoffset="${s.dashOffset}"/></svg>`;
 const pixels=await sharp(Buffer.from(svg)).ensureAlpha().raw().toBuffer(),out:number[]=[];
 for(let at=0;at<w*h;at++){const x=at%w,y=Math.floor(at/w);let sum=0;for(let dy=0;dy<scale;dy++)for(let dx=0;dx<scale;dx++)sum+=pixels[((y*scale+dy)*w*scale+x*scale+dx)*4+3]!;out.push(Math.round(sum/(scale*scale)));}
 return out;
}
