/** ✂️ First-party float32 edge classification shared with retained solid clipping. */
export function meshClipEdgeV1(input:Readonly<{a:readonly[number,number,number];b:readonly[number,number,number];normal:readonly[number,number,number];distance:number;epsilon:number}>){
 const f=Math.fround,dot=(p:readonly[number,number,number])=>f(f(f(p[0]*input.normal[0])+f(p[1]*input.normal[1]))+f(p[2]*input.normal[2]));
 const da=f(dot(input.a)-input.distance),db=f(dot(input.b)-input.distance),crossed=da>input.epsilon&&db< -input.epsilon||da< -input.epsilon&&db>input.epsilon;
 const parameter=crossed?f(da/f(da-db)):null;
 const coordinate=(axis:0|1|2,t:number)=>f(input.a[axis]+f(f(input.b[axis]-input.a[axis])*t));
 const position:[number,number,number]|null=parameter===null?null:[coordinate(0,parameter),coordinate(1,parameter),coordinate(2,parameter)];
 return {retained:da<=input.epsilon,parameter,position};
}
