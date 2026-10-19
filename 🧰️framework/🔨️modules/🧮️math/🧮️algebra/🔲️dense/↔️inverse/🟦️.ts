import type {DenseSolveBudget,DenseSolveFault,DenseSolveStatus,DenseSolveTurn} from "../🟦️.ts";
enum Phase{Validate,Identity,Pivot,SwapMatrix,SwapInverse,ReadPivot,NormalizeMatrix,NormalizeInverse,SelectRow,EliminateMatrix,EliminateInverse,AdvancePivot,Complete,Fault}
const faults:DenseSolveFault[]=["dimensions","non-finite-input","singular","non-finite-arithmetic","invalid-snapshot"];
const zero=():DenseSolveBudget=>({items:0,copyBytes:0,depth:0,capacityBytes:0});
const bits=(value:number):bigint=>{const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value,true);return view.getBigUint64(0,true);};
const scalar=(word:bigint):number=>{const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,word,true);return view.getFloat64(0,true);};

/** ↔️ Every accepted turn acts on original matrix/inverse scalars with no numeric scratch array. */
export class DenseInverseCursor{
 private pivot=0;private row=0;private column=0;private best=0;private scan=0;private phase:Phase;private denominator=0;private factor=0;private magnitude=0;private fault?:DenseSolveFault;
 constructor(private n:number,matrixLength:number,inverseLength:number){const valid=Number.isSafeInteger(n)&&n>=0&&Number.isSafeInteger(n*n)&&matrixLength===n*n&&inverseLength===n*n;this.phase=!valid?Phase.Fault:n===0?Phase.Complete:Phase.Validate;if(!valid)this.fault="dimensions";}
 private accesses():[number,number]{switch(this.phase){case Phase.Validate:case Phase.ReadPivot:return[1,0];case Phase.Identity:return[0,1];case Phase.Pivot:return this.row<this.n?[1,0]:[0,0];case Phase.SwapMatrix:case Phase.SwapInverse:return[2,2];case Phase.NormalizeMatrix:case Phase.NormalizeInverse:return[1,1];case Phase.SelectRow:return this.row<this.n&&this.row!==this.pivot?[1,0]:[0,0];case Phase.EliminateMatrix:case Phase.EliminateInverse:return[2,1];default:return[0,0];}}
 demand():DenseSolveBudget{if(this.phase===Phase.Complete||this.phase===Phase.Fault)return zero();const[reads,writes]=this.accesses();return{items:1,copyBytes:(reads+writes)*8,depth:1,capacityBytes:0};}
 private fail(fault:DenseSolveFault,receipt:DenseSolveBudget):DenseSolveTurn{this.phase=Phase.Fault;this.fault=fault;return{status:{kind:"fault",fault},receipt};}
 step(matrix:number[],inverse:number[],budget:DenseSolveBudget):DenseSolveTurn{
  if(this.phase===Phase.Fault)return{status:{kind:"fault",fault:this.fault!},receipt:zero()};if(matrix.length!==this.n*this.n||matrix.length!==inverse.length)return this.fail("dimensions",zero());if(this.phase===Phase.Complete)return{status:{kind:"complete"},receipt:zero()};
  const demand=this.demand();if(budget.items<demand.items||budget.copyBytes<demand.copyBytes||budget.depth<demand.depth)return{status:{kind:"refused",demand},receipt:zero()};
  const read={items:1,copyBytes:this.accesses()[0]*8,depth:1,capacityBytes:0};
  switch(this.phase){
   case Phase.Validate:if(!Number.isFinite(matrix[this.scan]))return this.fail("non-finite-input",read);this.scan++;if(this.scan===matrix.length){this.scan=0;this.phase=Phase.Identity;}break;
   case Phase.Identity:inverse[this.scan]=Math.floor(this.scan/this.n)===this.scan%this.n?1:0;this.scan++;if(this.scan===inverse.length){this.scan=0;this.row=0;this.phase=Phase.Pivot;}break;
   case Phase.Pivot:if(this.row<this.n){const magnitude=Math.abs(matrix[this.row*this.n+this.pivot]!);if(magnitude>this.magnitude){this.magnitude=magnitude;this.best=this.row;}this.row++;}else{if(this.magnitude===0)return this.fail("singular",read);this.column=0;this.phase=this.best===this.pivot?Phase.ReadPivot:Phase.SwapMatrix;}break;
   case Phase.SwapMatrix:case Phase.SwapInverse:{const values=this.phase===Phase.SwapMatrix?matrix:inverse,left=this.pivot*this.n+this.column,right=this.best*this.n+this.column;[values[left],values[right]]=[values[right]!,values[left]!];this.column++;if(this.column===this.n){this.column=0;this.phase=this.phase===Phase.SwapMatrix?Phase.SwapInverse:Phase.ReadPivot;}break;}
   case Phase.ReadPivot:this.denominator=matrix[this.pivot*this.n+this.pivot]!;if(!Number.isFinite(this.denominator))return this.fail("non-finite-arithmetic",read);if(this.denominator===0)return this.fail("singular",read);this.column=0;this.phase=Phase.NormalizeMatrix;break;
   case Phase.NormalizeMatrix:case Phase.NormalizeInverse:{const values=this.phase===Phase.NormalizeMatrix?matrix:inverse,index=this.pivot*this.n+this.column,value=values[index]!/this.denominator;if(!Number.isFinite(value))return this.fail("non-finite-arithmetic",read);values[index]=value;this.column++;if(this.column===this.n){this.column=0;if(this.phase===Phase.NormalizeMatrix)this.phase=Phase.NormalizeInverse;else{this.row=0;this.phase=Phase.SelectRow;}}break;}
   case Phase.SelectRow:if(this.row===this.n)this.phase=Phase.AdvancePivot;else if(this.row===this.pivot)this.row++;else{this.factor=matrix[this.row*this.n+this.pivot]!;this.column=0;this.phase=Phase.EliminateMatrix;}break;
   case Phase.EliminateMatrix:case Phase.EliminateInverse:{const values=this.phase===Phase.EliminateMatrix?matrix:inverse,index=this.row*this.n+this.column,other=this.pivot*this.n+this.column,value=values[index]!-this.factor*values[other]!;if(!Number.isFinite(value))return this.fail("non-finite-arithmetic",read);values[index]=value;this.column++;if(this.column===this.n){this.column=0;if(this.phase===Phase.EliminateMatrix)this.phase=Phase.EliminateInverse;else{this.row++;this.phase=Phase.SelectRow;}}break;}
   case Phase.AdvancePivot:this.pivot++;if(this.pivot===this.n)this.phase=Phase.Complete;else{this.row=this.pivot;this.best=this.pivot;this.column=0;this.magnitude=0;this.phase=Phase.Pivot;}break;
  }
  return{status:{kind:this.phase===Phase.Complete?"complete":"pending"},receipt:demand};
 }
 snapshot():bigint[]{return[1n,BigInt(this.n),BigInt(this.pivot),BigInt(this.row),BigInt(this.column),BigInt(this.best),BigInt(this.scan),BigInt(this.phase),bits(this.denominator),bits(this.factor),bits(this.magnitude),BigInt(this.fault===undefined?0:faults.indexOf(this.fault)+1),0n,0n];}
 static fromSnapshot(words:readonly bigint[]):DenseInverseCursor{
  const reject=():never=>{throw Error("invalid-snapshot");};if(words.length!==14||words[0]!==1n||words[12]!==0n||words[13]!==0n)return reject();
  const counter=(index:number,limit:number):number=>{const word=words[index]!;if(word<0n||word>BigInt(limit))return reject();return Number(word);};const n=counter(1,Number.MAX_SAFE_INTEGER);if(!Number.isSafeInteger(n*n))return reject();
  const phase=counter(7,Phase.Fault),pivot=counter(2,n),row=counter(3,n),column=counter(4,n),best=counter(5,Math.max(0,n-1)),scan=counter(6,n*n),denominator=scalar(words[8]!),factor=scalar(words[9]!),magnitude=scalar(words[10]!),faultCode=counter(11,5);
  if(![denominator,factor,magnitude].every(Number.isFinite)||magnitude<0||(phase===Phase.Fault)!==(faultCode!==0))return reject();if(phase!==Phase.Complete&&phase!==Phase.Fault&&(n===0||pivot>=n))return reject();
  if((phase===Phase.Validate||phase===Phase.Identity)&&scan>=n*n)return reject();if([Phase.SwapMatrix,Phase.SwapInverse,Phase.NormalizeMatrix,Phase.NormalizeInverse,Phase.EliminateMatrix,Phase.EliminateInverse].includes(phase)&&column>=n)return reject();if((phase===Phase.EliminateMatrix||phase===Phase.EliminateInverse)&&row>=n)return reject();
  if(phase===Phase.Complete&&pivot!==n)return reject();if((phase===Phase.NormalizeMatrix||phase===Phase.NormalizeInverse)&&denominator===0)return reject();if((phase===Phase.SwapMatrix||phase===Phase.SwapInverse)&&best===pivot)return reject();if((phase===Phase.EliminateMatrix||phase===Phase.EliminateInverse)&&row===pivot)return reject();
  const cursor=new DenseInverseCursor(n,n*n,n*n);Object.assign(cursor,{phase,pivot,row,column,best,scan,denominator,factor,magnitude,fault:faultCode===0?undefined:faults[faultCode-1]});return cursor;
 }
}
