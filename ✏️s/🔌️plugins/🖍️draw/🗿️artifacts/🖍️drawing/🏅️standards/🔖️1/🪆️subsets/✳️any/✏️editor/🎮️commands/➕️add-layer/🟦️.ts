/** 🎨️ Visible initial paint for newly authored artwork. */
export interface InitialAppearance {
  fill:{kind:"solid";color:[number,number,number,number]}|null;
  stroke:{color:[number,number,number,number];width:number;cap:string;join:string;dash:number[]|null}|null;
}
export function initialAppearance(kind:string):InitialAppearance {
  const filled=["shape:rect","shape:ellipse","shape:polygon","boolean","trace"].includes(kind);
  const outlined=kind==="path" || kind.startsWith("shape:");
  return {
    fill:filled?{kind:"solid",color:[0.2,0.7,0.65,1]}:kind==="text"?{kind:"solid",color:[0,0,0,1]}:null,
    stroke:outlined?{color:[0.1,0.15,0.2,1],width:2,cap:"round",join:"round",dash:null}:null,
  };
}
