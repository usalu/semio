/** 📐️ Real browser measurements select only a sufficiently long fall and cards in the current grid. */
export interface DropGeometryV1{readonly width:number;readonly reach:number;readonly top:number;readonly height:number;readonly minimum:number;readonly edges:readonly Readonly<{left:number;right:number;top:number}>[];readonly others:readonly Readonly<{left:number;right:number}>[]}
/** 🪂️ Refuses a short scene so the browser can navigate to another genuine measured page before pickup. */
export function dropColumnV1(input:DropGeometryV1):Readonly<{x:number;floor:number;y:number}>|undefined{
 const all=Array.from({length:Math.max(0,Math.ceil((input.width-160)/8))},(_,index)=>80+index*8).map(x=>{const floor=Math.min(...input.edges.filter(edge=>edge.left<=x&&edge.right>=x&&edge.top>input.top+40).map(edge=>edge.top));return{x,floor,y:Math.max(input.top,floor-input.height)};}).filter(row=>Number.isFinite(row.floor)&&row.floor-row.y>input.minimum),clear=all.filter(row=>input.others.every(body=>row.x<body.left-input.reach||row.x>body.right+input.reach)),pool=clear.length?clear:all;
 return pool.reduce<Readonly<{x:number;floor:number;y:number}>|undefined>((best,row)=>!best||row.floor>best.floor?row:best,undefined);
}
/** 🗂️ Only the nonempty unique authored roster actually mounted in the active grid may be measured. */
export function activeQuizIdsV1(observed:readonly string[],authored:readonly string[]):readonly string[]{
 if(!observed.length||new Set(observed).size!==observed.length||observed.some(id=>!authored.includes(id)))throw Error("Actual current authored quiz grid required");return observed;
}
