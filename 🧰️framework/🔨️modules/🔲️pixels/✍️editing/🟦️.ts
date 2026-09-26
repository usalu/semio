/** 🎨️ Deterministic RGBA8 editing shared by browser tools and language-neutral conformance cases. */
export type PixelImage = { width: number; height: number; pixels: Uint8Array };
export type PixelColor = readonly [number, number, number, number];
export type PixelPoint = readonly [number, number];
export type PixelOperation =
  | { kind: "invert" | "grayscale" | "clear" | "flipHorizontal" | "flipVertical" | "rotateClockwise" | "rotateCounterclockwise" }
  | { kind: "brightness" | "contrast" | "saturation" | "gamma" | "threshold" | "posterize" | "blur" | "sharpen"; value: number }
  | { kind: "resize"; width: number; height: number; sampling: "nearest" | "bilinear" }
  | { kind: "crop"; x: number; y: number; width: number; height: number }
  | { kind: "fill"; color: PixelColor }
  | ({ kind: "stroke"; points: readonly PixelPoint[]; erase: boolean } & PixelBrush);
export type SelectionShape =
  | { kind: "rectangle" | "ellipse"; x: number; y: number; width: number; height: number }
  | { kind: "polygon"; points: readonly PixelPoint[] };
export type SelectionMerge = "replace" | "add" | "subtract" | "intersect";
export type PixelProgress = { completed: number; total: number; done: boolean };
export type PixelEditOptions = { selection?: Uint8Array; signal?: AbortSignal; chunkPixels?: number; onProgress?: (progress: PixelProgress) => void };
export type PixelBrush = { size: number; opacity: number; hardness: number; color: PixelColor; erase?: boolean };
export const MAX_IMAGE_SIDE = 16384;
export const MAX_IMAGE_PIXELS = 16 * 1024 * 1024;

const byte = (value: number) => Math.round(Math.max(0, Math.min(255, value)));
const bounded = (value: number, min: number, max: number) => Number.isFinite(value) && value >= min && value <= max;
const invalid = (message: string): never => { throw new RangeError(message); };
const aborted = (): never => { throw new DOMException("Pixel operation cancelled", "AbortError"); };
const yieldTurn = () => new Promise<void>(resolve => setTimeout(resolve, 0));

export function validateExtent(width: number, height: number): number {
  if (!Number.isInteger(width) || !Number.isInteger(height) || !bounded(width, 1, MAX_IMAGE_SIDE) || !bounded(height, 1, MAX_IMAGE_SIDE) || width * height > MAX_IMAGE_PIXELS) invalid("Image extent exceeds the pixel budget");
  return width * height;
}

export function validateImage(image: PixelImage): void {
  if (!(image.pixels instanceof Uint8Array) || image.pixels.length !== validateExtent(image.width, image.height) * 4) invalid("RGBA8 length does not match image extent");
}

function validateMask(mask: Uint8Array | undefined, count: number): void {
  if (mask !== undefined && (!(mask instanceof Uint8Array) || mask.length !== count)) invalid("Selection extent does not match image");
}

function validateColor(color: PixelColor): void {
  if (color.length !== 4 || color.some(v => !Number.isInteger(v) || !bounded(v, 0, 255))) invalid("Color must contain four byte channels");
}

function sample(image: PixelImage, x: number, y: number): PixelColor {
  const index = (Math.max(0, Math.min(image.height - 1, y)) * image.width + Math.max(0, Math.min(image.width - 1, x))) * 4;
  return [image.pixels[index]!, image.pixels[index + 1]!, image.pixels[index + 2]!, image.pixels[index + 3]!];
}

function weightedSample(image: PixelImage, x: number, y: number): PixelColor {
  const x0 = Math.floor(x), y0 = Math.floor(y), fx = x - x0, fy = y - y0;
  const samples = [sample(image,x0,y0),sample(image,x0+1,y0),sample(image,x0,y0+1),sample(image,x0+1,y0+1)];
  const weights = [(1-fx)*(1-fy),fx*(1-fy),(1-fx)*fy,fx*fy];
  let alpha = 0;
  const rgb = [0,0,0];
  for (let i = 0; i < 4; i++) {
    const a = samples[i]![3] * weights[i]!;
    alpha += a;
    for (let c = 0; c < 3; c++) rgb[c]! += samples[i]![c]! * a;
  }
  return alpha > 0 ? [byte(rgb[0]! / alpha),byte(rgb[1]! / alpha),byte(rgb[2]! / alpha),byte(alpha)] : [0,0,0,0];
}

export function sourceOver(destination: PixelColor, source: PixelColor, opacity = 1): PixelColor {
  const sa = source[3] / 255 * opacity, da = destination[3] / 255, alpha = sa + da * (1-sa);
  if (alpha === 0) return [0,0,0,0];
  return [byte((source[0]*sa+destination[0]*da*(1-sa))/alpha),byte((source[1]*sa+destination[1]*da*(1-sa))/alpha),byte((source[2]*sa+destination[2]*da*(1-sa))/alpha),byte(alpha*255)];
}

function dimensions(image: PixelImage, operation: PixelOperation): readonly [number, number] {
  if (operation.kind === "rotateClockwise" || operation.kind === "rotateCounterclockwise") return [image.height, image.width];
  if (operation.kind === "resize" || operation.kind === "crop") return [operation.width, operation.height];
  return [image.width, image.height];
}

function validateOperation(image: PixelImage, operation: PixelOperation, selected: boolean): void {
  if (!operation || typeof operation !== "object") invalid("Pixel operation is required");
  const numeric: Record<string, readonly [number, number]> = { brightness:[-1,1],contrast:[-1,1],saturation:[-1,1],gamma:[0.01,10],threshold:[0,255],posterize:[2,256],blur:[0,16],sharpen:[0,5] };
  if ("value" in operation) {
    const range = numeric[operation.kind];
    if (!range || !bounded(operation.value,range[0],range[1])) invalid("Invalid filter parameter");
    if (["blur","posterize"].includes(operation.kind) && !Number.isInteger(operation.value)) invalid("Filter parameter must be an integer");
  } else if (operation.kind === "stroke") {
    validateBrush(operation.points,operation);
  } else if (operation.kind === "fill") validateColor(operation.color);
  else if (operation.kind === "crop") {
    if (![operation.x,operation.y].every(v => Number.isInteger(v) && v >= 0) || operation.x + operation.width > image.width || operation.y + operation.height > image.height) invalid("Crop lies outside image");
  } else if (operation.kind === "resize") {
    if (!["nearest","bilinear"].includes(operation.sampling)) invalid("Unknown resize sampling");
  } else if (!["invert","grayscale","clear","flipHorizontal","flipVertical","rotateClockwise","rotateCounterclockwise"].includes(operation.kind)) invalid("Unknown pixel operation");
  const [width,height] = dimensions(image,operation);
  validateExtent(width,height);
  if (selected && ["crop","resize","flipHorizontal","flipVertical","rotateClockwise","rotateCounterclockwise"].includes(operation.kind)) invalid("Geometry operations require an unrestricted image");
}

function filtered(image: PixelImage, x: number, y: number, operation: PixelOperation, coverage = 1): PixelColor {
  const original = sample(image,x,y);
  if (operation.kind === "clear") return [0,0,0,0];
  if (operation.kind === "fill") return sourceOver(original,operation.color,coverage);
  if (operation.kind === "stroke") {
    const radius=operation.size/2,core=radius*operation.hardness;
    let amount=0;
    for(let i=0;i<operation.points.length;i++) {
      const from=operation.points[Math.max(0,i-1)]!,to=operation.points[i]!;
      const distance=segmentDistance(x+0.5,y+0.5,from,to);
      amount=Math.max(amount,distance>radius?0:distance<=core?255:byte(255*(radius-distance)/(radius-core)));
    }
    const opacity=amount/255*operation.opacity*coverage;
    if(opacity===0) return original;
    if(!operation.erase) return sourceOver(original,operation.color,opacity);
    const alpha=byte(original[3]*(1-opacity));
    return alpha>0?[original[0],original[1],original[2],alpha]:[0,0,0,0];
  }
  if (operation.kind === "flipHorizontal") return sample(image,image.width-x-1,y);
  if (operation.kind === "flipVertical") return sample(image,x,image.height-y-1);
  if (operation.kind === "rotateClockwise") return sample(image,y,image.height-x-1);
  if (operation.kind === "rotateCounterclockwise") return sample(image,image.width-y-1,x);
  if (operation.kind === "crop") return sample(image,x+operation.x,y+operation.y);
  if (operation.kind === "resize") {
    if (operation.sampling === "nearest") return sample(image,Math.floor((x+0.5)*image.width/operation.width),Math.floor((y+0.5)*image.height/operation.height));
    return weightedSample(image,(x+0.5)*image.width/operation.width-0.5,(y+0.5)*image.height/operation.height-0.5);
  }
  if (operation.kind === "blur" || operation.kind === "sharpen") {
    if (operation.value === 0) return original;
    const radius = operation.kind === "blur" ? operation.value : 1;
    const rgb = [0,0,0];
    let alpha = 0, count = 0;
    for (let dy = -radius; dy <= radius; dy++) for (let dx = -radius; dx <= radius; dx++) {
      const neighbor = sample(image,x+dx,y+dy);
      alpha += neighbor[3];
      for (let c = 0; c < 3; c++) rgb[c]! += neighbor[c]! * neighbor[3];
      count++;
    }
    const blur = alpha > 0 ? rgb.map(v => v/alpha) : [0,0,0];
    return operation.kind === "blur"
      ? [byte(blur[0]!),byte(blur[1]!),byte(blur[2]!),byte(alpha/count)]
      : [byte(original[0]+operation.value*(original[0]-blur[0]!)),byte(original[1]+operation.value*(original[1]-blur[1]!)),byte(original[2]+operation.value*(original[2]-blur[2]!)),original[3]];
  }
  const luma = 0.2126*original[0]+0.7152*original[1]+0.0722*original[2];
  const map = (v: number): number => {
    switch (operation.kind) {
      case "invert": return 255-v;
      case "grayscale": return luma;
      case "brightness": return v+operation.value*255;
      case "contrast": return (v-127.5)*Math.pow(2,operation.value*4)+127.5;
      case "saturation": return luma+(v-luma)*(operation.value+1);
      case "gamma": return 255*Math.pow(v/255,1/operation.value);
      case "threshold": return luma >= operation.value ? 255 : 0;
      case "posterize": return Math.round(v/255*(operation.value-1))*255/(operation.value-1);
    }
    return invalid("Unknown color operation");
  };
  return [byte(map(original[0])),byte(map(original[1])),byte(map(original[2])),original[3]];
}

/** 🧵️ An owned unpublished candidate; callers retain the source until completion. */
export class PixelEditJob {
  private source: PixelImage;
  private output: PixelImage;
  private operation: PixelOperation;
  private selection?: Uint8Array;
  private cursor = 0;
  private cancelled = false;
  constructor(image: PixelImage, operation: PixelOperation, selection?: Uint8Array) {
    validateImage(image);
    validateMask(selection,image.width*image.height);
    validateOperation(image,operation,selection !== undefined);
    this.source = { ...image, pixels: image.pixels.slice() };
    this.operation = structuredClone(operation);
    this.selection = selection?.slice();
    const [width,height] = dimensions(image,operation);
    this.output = {width,height,pixels:new Uint8Array(width*height*4)};
  }
  advance(pixelBudget = 4096): PixelProgress {
    if (this.cancelled) aborted();
    if (!Number.isInteger(pixelBudget) || !bounded(pixelBudget,1,65536)) invalid("Pixel budget must be 1–65536");
    const total = this.output.width*this.output.height;
    const end = Math.min(total,this.cursor+pixelBudget);
    for (; this.cursor < end; this.cursor++) {
      const x = this.cursor%this.output.width, y = Math.floor(this.cursor/this.output.width);
      const coverage = (this.selection?.[this.cursor] ?? 255)/255;
      const before = sample(this.source,x,y);
      const after = coverage === 0 ? before : filtered(this.source,x,y,this.operation,coverage);
      const mix = this.operation.kind === "fill" || this.operation.kind === "stroke" ? 1 : coverage;
      for (let c = 0; c < 4; c++) this.output.pixels[this.cursor*4+c] = byte(before[c]!+(after[c]!-before[c]!)*mix);
      if (this.operation.kind === "clear" && coverage > 0 && coverage < 1) {
        for (let c = 0; c < 3; c++) this.output.pixels[this.cursor*4+c] = this.output.pixels[this.cursor*4+3] ? before[c]! : 0;
      }
    }
    return {completed:this.cursor,total,done:this.cursor === total};
  }
  cancel(): void {
    this.cancelled = true;
    this.output.pixels = new Uint8Array();
    this.source.pixels = new Uint8Array();
    this.selection = undefined;
  }
  result(): PixelImage {
    if (this.cancelled) aborted();
    if (this.cursor !== this.output.width*this.output.height) throw new Error("Pixel operation is incomplete");
    return {...this.output,pixels:this.output.pixels.slice()};
  }
}

export async function editImage(image: PixelImage, operation: PixelOperation, options: PixelEditOptions = {}): Promise<PixelImage> {
  if (options.signal?.aborted) aborted();
  if(operation.kind === "stroke") return paintStroke(image,operation.points,operation,options);
  const job = new PixelEditJob(image,operation,options.selection);
  try {
    while (true) {
      if (options.signal?.aborted) aborted();
      const progress = job.advance(options.chunkPixels ?? (operation.kind === "blur" ? 128 : 4096));
      options.onProgress?.(progress);
      if (options.signal?.aborted) aborted();
      if (progress.done) return job.result();
      await yieldTurn();
    }
  } catch (error) {
    job.cancel();
    throw error;
  }
}

function validateShape(shape:SelectionShape):void {
  if (shape.kind === "polygon") {
    if (shape.points.length < 3 || shape.points.length > 4096 || shape.points.some(p => p.length !== 2 || !p.every(Number.isFinite))) invalid("Polygon needs 3–4096 finite points");
  } else if (!["rectangle","ellipse"].includes(shape.kind) || ![shape.x,shape.y,shape.width,shape.height].every(Number.isFinite)) invalid("Invalid selection shape");
}

/** 🪢️ Scanline selection rasterization with bounded row grants. */
export class PixelSelectionJob {
  private mask:Uint8Array;
  private row=0;
  private cancelled=false;
  private shape:SelectionShape;
  constructor(private width:number,private height:number,shape:SelectionShape) {
    validateShape(shape);
    this.mask=new Uint8Array(validateExtent(width,height));this.shape=structuredClone(shape);
  }
  advance(rows=16):PixelProgress {
    if(this.cancelled) aborted();
    if(!Number.isInteger(rows)||!bounded(rows,1,64)) invalid("Selection grant must be 1–64 rows");
    const end=Math.min(this.height,this.row+rows),shape=this.shape;
    for(;this.row<end;this.row++) {
      const y=this.row+0.5;
      const fill=(left:number,right:number,inclusive=false)=>{
        const from=Math.max(0,Math.min(this.width,Math.ceil(left-0.5)));
        const to=Math.max(from,Math.min(this.width,inclusive?Math.floor(right-0.5)+1:Math.ceil(right-0.5)));
        this.mask.fill(255,this.row*this.width+from,this.row*this.width+to);
      };
      if(shape.kind==="polygon") {
        const intersections:number[]=[];
        for(let i=0,j=shape.points.length-1;i<shape.points.length;j=i++) {
          const a=shape.points[i]!,b=shape.points[j]!;
          if((a[1]>y)!==(b[1]>y)) intersections.push((b[0]-a[0])*(y-a[1])/(b[1]-a[1])+a[0]);
        }
        intersections.sort((a,b)=>a-b);
        for(let i=0;i+1<intersections.length;i+=2) fill(intersections[i]!,intersections[i+1]!);
      } else {
        const left=Math.min(shape.x,shape.x+shape.width),top=Math.min(shape.y,shape.y+shape.height),w=Math.abs(shape.width),h=Math.abs(shape.height);
        if(w===0||h===0||y<top||y>top+h) continue;
        if(shape.kind==="rectangle") {if(y<top+h)fill(left,left+w);}
        else {
          const radius=w/2*Math.sqrt(Math.max(0,1-((y-top-h/2)/(h/2))**2));
          fill(left+w/2-radius,left+w/2+radius,true);
        }
      }
    }
    return {completed:this.row,total:this.height,done:this.row===this.height};
  }
  result():Uint8Array {
    if(this.cancelled) aborted();
    if(this.row!==this.height) throw new Error("Selection is incomplete");
    return this.mask.slice();
  }
  cancel():void {this.cancelled=true;this.mask=new Uint8Array();}
}

export function selectionMask(width:number,height:number,shape:SelectionShape):Uint8Array {
  const job=new PixelSelectionJob(width,height,shape);
  while(!job.advance(64).done) {}
  return job.result();
}

export async function selectPixels(width:number,height:number,shape:SelectionShape,options:PixelEditOptions={}):Promise<Uint8Array> {
  if(options.signal?.aborted) aborted();
  const job=new PixelSelectionJob(width,height,shape);
  try {
    while(true) {
      if(options.signal?.aborted) aborted();
      const progress=job.advance();options.onProgress?.(progress);
      if(options.signal?.aborted) aborted();
      if(progress.done) return job.result();
      await yieldTurn();
    }
  } catch(error) {job.cancel();throw error;}
}

export function combineSelections(current: Uint8Array, next: Uint8Array, mode: SelectionMerge): Uint8Array {
  if (current.length !== next.length) invalid("Selection extents differ");
  if (!["replace","add","subtract","intersect"].includes(mode)) invalid("Unknown selection merge");
  return next.map((value,i) => mode === "replace" ? value : mode === "add" ? Math.max(value,current[i]!) : mode === "subtract" ? Math.max(0,current[i]!-value) : Math.min(value,current[i]!));
}

export async function floodSelection(image: PixelImage, x: number, y: number, tolerance: number, options: PixelEditOptions = {}): Promise<Uint8Array> {
  validateImage(image);
  validateMask(options.selection,image.width*image.height);
  if (!Number.isInteger(x) || !Number.isInteger(y) || !bounded(x,0,image.width-1) || !bounded(y,0,image.height-1) || !bounded(tolerance,0,255)) invalid("Invalid flood selection seed or tolerance");
  if (options.signal?.aborted) aborted();
  image={...image,pixels:image.pixels.slice()};
  options={...options,selection:options.selection?.slice()};
  const total = image.width*image.height, mask = new Uint8Array(total), seen = new Uint8Array(total), queue = new Uint32Array(total);
  const target = sample(image,x,y), seed = y*image.width+x;
  let head = 0, tail = 1;
  queue[0] = seed;
  seen[seed] = 1;
  const admit = (index: number) => {
    if (!seen[index]) { seen[index] = 1; queue[tail++] = index; }
  };
  while (head < tail) {
    const end = Math.min(tail,head+4096);
    for (; head < end; head++) {
      const index = queue[head]!, px = index%image.width, py = Math.floor(index/image.width);
      const color = sample(image,px,py);
      if (options.selection?.[index] === 0 || Math.max(...color.map((v,c) => Math.abs(v-target[c]!))) > tolerance) continue;
      mask[index] = options.selection?.[index] ?? 255;
      if (px > 0) admit(index-1);
      if (px+1 < image.width) admit(index+1);
      if (py > 0) admit(index-image.width);
      if (py+1 < image.height) admit(index+image.width);
    }
    options.onProgress?.({completed:head,total,done:head === tail});
    if (options.signal?.aborted) aborted();
    if (head < tail) await yieldTurn();
  }
  options.onProgress?.({completed:total,total,done:true});
  return mask;
}

function segmentDistance(x: number, y: number, from: PixelPoint, to: PixelPoint): number {
  const dx = to[0]-from[0], dy = to[1]-from[1], length = dx*dx+dy*dy;
  const t = length === 0 ? 0 : Math.max(0,Math.min(1,((x-from[0])*dx+(y-from[1])*dy)/length));
  return Math.hypot(x-from[0]-t*dx,y-from[1]-t*dy);
}

function validateBrush(points:readonly PixelPoint[],brush:PixelBrush):void {
  validateColor(brush.color);
  if (!bounded(brush.size,0.1,4096) || !bounded(brush.opacity,0,1) || !bounded(brush.hardness,0,1) || !points.length || points.length > 2048 || points.some(p => p.length !== 2 || !p.every(Number.isFinite))) invalid("Invalid brush stroke");
}

export async function paintStroke(image: PixelImage, points: readonly PixelPoint[], brush: PixelBrush, options: PixelEditOptions = {}): Promise<PixelImage> {
  validateImage(image);
  validateMask(options.selection,image.width*image.height);
  validateBrush(points,brush);
  if (options.signal?.aborted) aborted();
  image={...image,pixels:image.pixels.slice()};
  points=structuredClone(points);brush=structuredClone(brush);
  options={...options,selection:options.selection?.slice()};
  const pixels = image.pixels.slice(), coverage = new Uint8Array(image.width*image.height), radius = brush.size/2;
  for (let segment = 0; segment < points.length; segment++) {
    const from = points[Math.max(0,segment-1)]!, to = points[segment]!;
    const minX = Math.max(0,Math.floor(Math.min(from[0],to[0])-radius)), maxX = Math.min(image.width-1,Math.ceil(Math.max(from[0],to[0])+radius));
    const minY = Math.max(0,Math.floor(Math.min(from[1],to[1])-radius)), maxY = Math.min(image.height-1,Math.ceil(Math.max(from[1],to[1])+radius));
    for (let y = minY; y <= maxY; y++) {
      for (let x = minX; x <= maxX; x++) {
        const d = segmentDistance(x+0.5,y+0.5,from,to), core = radius*brush.hardness;
        const amount = d > radius ? 0 : d <= core ? 255 : byte(255*(radius-d)/(radius-core));
        const index = y*image.width+x;
        coverage[index] = Math.max(coverage[index]!,amount);
      }
      if ((y-minY)%16 === 15) {
        if (options.signal?.aborted) aborted();
        await yieldTurn();
      }
    }
    options.onProgress?.({completed:segment+1,total:points.length+1,done:false});
    if (options.signal?.aborted) aborted();
    if (segment%32 === 31) await yieldTurn();
  }
  for (let index = 0; index < coverage.length; index++) {
    if (index%65536 === 65535) {
      if (options.signal?.aborted) aborted();
      await yieldTurn();
    }
    const amount = coverage[index]!/255*(options.selection?.[index] ?? 255)/255*brush.opacity;
    if (amount === 0) continue;
    const before = sample(image,index%image.width,Math.floor(index/image.width));
    const alpha = byte(before[3]*(1-amount));
    const result: PixelColor = brush.erase ? alpha > 0 ? [before[0],before[1],before[2],alpha] : [0,0,0,0] : sourceOver(before,brush.color,amount);
    pixels.set(result,index*4);
  }
  options.onProgress?.({completed:points.length+1,total:points.length+1,done:true});
  if (options.signal?.aborted) aborted();
  return {width:image.width,height:image.height,pixels};
}
