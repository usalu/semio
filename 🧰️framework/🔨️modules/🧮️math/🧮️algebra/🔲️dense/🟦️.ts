/** 🔲️ Fixed dense elimination cursor over caller-owned numeric spans. */
export interface DenseSolveBudget {items:number;copyBytes:number;depth:number;capacityBytes:number}
export type DenseSolveFault="dimensions"|"non-finite-input"|"singular"|"non-finite-arithmetic"|"invalid-snapshot";
export type DenseSolveStatus={kind:"pending"}|{kind:"complete"}|{kind:"fault";fault:DenseSolveFault}|{kind:"refused";demand:DenseSolveBudget};
export interface DenseSolveTurn {status:DenseSolveStatus;receipt:DenseSolveBudget}
enum Phase {Validate,Pivot,SwapMatrix,SwapRhs,Factor,Eliminate,UpdateRhs,BackBegin,BackSum,BackDivide,Complete,Fault}
const faults:DenseSolveFault[]=["dimensions","non-finite-input","singular","non-finite-arithmetic","invalid-snapshot"];
const zero=():DenseSolveBudget=>({items:0,copyBytes:0,depth:0,capacityBytes:0});
const bits=(value:number):bigint=>{const view=new DataView(new ArrayBuffer(8));view.setFloat64(0,value,true);return view.getBigUint64(0,true);};
const scalar=(word:bigint):number=>{const view=new DataView(new ArrayBuffer(8));view.setBigUint64(0,word,true);return view.getFloat64(0,true);};

/** 🧭️ Every accepted turn reads or writes a bounded number of original scalars. */
export class DenseSolveCursor {
 private pivot=0;private row=0;private column=0;private best=0;private scan=0;private phase:Phase;private magnitude=0;private factor=0;private sum=0;private fault?:DenseSolveFault;
 constructor(private n:number,matrixLength:number,rhsLength:number){const valid=Number.isSafeInteger(n)&&n>=0&&Number.isSafeInteger(n*n)&&n*n===matrixLength&&rhsLength===n;this.phase=!valid?Phase.Fault:n===0?Phase.Complete:Phase.Validate;if(!valid)this.fault="dimensions";}
 order():number{return this.n;}
 private accesses():[number,number]{switch(this.phase){case Phase.Validate:case Phase.BackBegin:return[1,0];case Phase.Pivot:return this.row<this.n?[1,0]:[0,0];case Phase.SwapMatrix:case Phase.SwapRhs:return[2,2];case Phase.Factor:return this.row<this.n?[2,1]:[0,0];case Phase.Eliminate:case Phase.UpdateRhs:return[2,1];case Phase.BackSum:return this.column<this.n?[2,0]:[0,0];case Phase.BackDivide:return[1,1];default:return[0,0];}}
 demand():DenseSolveBudget{if(this.phase===Phase.Complete||this.phase===Phase.Fault)return zero();const[reads,writes]=this.accesses();return{items:1,copyBytes:8*(reads+writes),depth:1,capacityBytes:0};}
 private fail(fault:DenseSolveFault,receipt:DenseSolveBudget):DenseSolveTurn{this.phase=Phase.Fault;this.fault=fault;return{status:{kind:"fault",fault},receipt};}
 step(matrix:number[],rhs:number[],budget:DenseSolveBudget):DenseSolveTurn{
  if(this.phase===Phase.Fault)return{status:{kind:"fault",fault:this.fault!},receipt:zero()};
  if(matrix.length!==this.n*this.n||rhs.length!==this.n)return this.fail("dimensions",zero());
  if(this.phase===Phase.Complete)return{status:{kind:"complete"},receipt:zero()};
  const demand=this.demand();if(budget.items<demand.items||budget.copyBytes<demand.copyBytes||budget.depth<demand.depth)return{status:{kind:"refused",demand},receipt:zero()};
  const reads=this.accesses()[0],readReceipt={items:1,copyBytes:8*reads,depth:1,capacityBytes:0};
  switch(this.phase){
   case Phase.Validate:{const value=this.scan<matrix.length?matrix[this.scan]!:rhs[this.scan-matrix.length]!;if(!Number.isFinite(value))return this.fail("non-finite-input",readReceipt);this.scan++;if(this.scan===matrix.length+rhs.length){this.phase=Phase.Pivot;this.row=0;}break;}
   case Phase.Pivot:if(this.row<this.n){const magnitude=Math.abs(matrix[this.row*this.n+this.pivot]!);if(magnitude>this.magnitude){this.magnitude=magnitude;this.best=this.row;}this.row++;}else{if(this.magnitude===0)return this.fail("singular",readReceipt);this.column=0;if(this.best!==this.pivot)this.phase=Phase.SwapMatrix;else{this.row=this.pivot+1;this.phase=Phase.Factor;}}break;
   case Phase.SwapMatrix:{const left=this.pivot*this.n+this.column,right=this.best*this.n+this.column;[matrix[left],matrix[right]]=[matrix[right]!,matrix[left]!];this.column++;if(this.column===this.n)this.phase=Phase.SwapRhs;break;}
   case Phase.SwapRhs:[rhs[this.pivot],rhs[this.best]]=[rhs[this.best]!,rhs[this.pivot]!];this.row=this.pivot+1;this.phase=Phase.Factor;break;
   case Phase.Factor:if(this.row<this.n){const factor=matrix[this.row*this.n+this.pivot]!/matrix[this.pivot*this.n+this.pivot]!;if(!Number.isFinite(factor))return this.fail("non-finite-arithmetic",readReceipt);this.factor=factor;matrix[this.row*this.n+this.pivot]=0;this.column=this.pivot+1;this.phase=this.column<this.n?Phase.Eliminate:Phase.UpdateRhs;}else{this.pivot++;if(this.pivot<this.n){this.row=this.pivot;this.best=this.pivot;this.magnitude=0;this.phase=Phase.Pivot;}else{this.row=this.n-1;this.phase=Phase.BackBegin;}}break;
   case Phase.Eliminate:{const index=this.row*this.n+this.column,value=matrix[index]!-this.factor*matrix[this.pivot*this.n+this.column]!;if(!Number.isFinite(value))return this.fail("non-finite-arithmetic",readReceipt);matrix[index]=value;this.column++;if(this.column===this.n)this.phase=Phase.UpdateRhs;break;}
   case Phase.UpdateRhs:{const value=rhs[this.row]!-this.factor*rhs[this.pivot]!;if(!Number.isFinite(value))return this.fail("non-finite-arithmetic",readReceipt);rhs[this.row]=value;this.row++;this.phase=Phase.Factor;break;}
   case Phase.BackBegin:this.sum=rhs[this.row]!;this.column=this.row+1;this.phase=Phase.BackSum;break;
   case Phase.BackSum:if(this.column<this.n){const sum=this.sum-matrix[this.row*this.n+this.column]!*rhs[this.column]!;if(!Number.isFinite(sum))return this.fail("non-finite-arithmetic",readReceipt);this.sum=sum;this.column++;}else this.phase=Phase.BackDivide;break;
   case Phase.BackDivide:{const value=this.sum/matrix[this.row*this.n+this.row]!;if(!Number.isFinite(value))return this.fail("non-finite-arithmetic",readReceipt);rhs[this.row]=value;if(this.row===0)this.phase=Phase.Complete;else{this.row--;this.phase=Phase.BackBegin;}break;}
   default:throw Error("invalid dense phase");
  }return{status:{kind:this.phase===Phase.Complete?"complete":"pending"},receipt:demand};
 }
 /** 📸️ Scalar words preserve identical IEEE-754 observations in both implementations. */
 snapshot():bigint[]{return[1n,BigInt(this.n),BigInt(this.pivot),BigInt(this.row),BigInt(this.column),BigInt(this.best),BigInt(this.scan),BigInt(this.phase),bits(this.magnitude),bits(this.factor),bits(this.sum),BigInt(this.fault?faults.indexOf(this.fault)+1:0)];}
 /** 🪞️ Invalid source indices are rejected before any original span is observed. */
 static fromSnapshot(words:readonly bigint[]):DenseSolveCursor{
  const invalid=()=>{throw Error("invalid-snapshot");};if(words.length!==12||words.some(x=>typeof x!=="bigint"||x<0n||x>0xffffffffffffffffn)||words[0]!==1n)invalid();
  const value=(index:number):number=>{const x=Number(words[index]);if(!Number.isSafeInteger(x))invalid();return x;};
  const n=value(1),cursor=new DenseSolveCursor(n,n*n,n);cursor.pivot=value(2);cursor.row=value(3);cursor.column=value(4);cursor.best=value(5);cursor.scan=value(6);cursor.phase=value(7);cursor.magnitude=scalar(words[8]!);cursor.factor=scalar(words[9]!);cursor.sum=scalar(words[10]!);const fault=value(11);cursor.fault=fault?faults[fault-1]:undefined;
  if(cursor.phase>Phase.Fault||fault>faults.length||!Number.isSafeInteger(n*n+n)||cursor.pivot>n||cursor.row>n||cursor.column>n||cursor.best>=Math.max(n,1)||cursor.scan>n*n+n||!Number.isFinite(cursor.magnitude)||cursor.magnitude<0||!Number.isFinite(cursor.factor)||!Number.isFinite(cursor.sum)||(cursor.phase===Phase.Fault)!==!!cursor.fault)invalid();
  if(cursor.phase!==Phase.Complete&&cursor.phase!==Phase.Fault&&(n===0||cursor.phase<=Phase.UpdateRhs&&cursor.pivot>=n||cursor.phase===Phase.Validate&&cursor.scan>=n*n+n))invalid();
  if((cursor.phase===Phase.SwapMatrix||cursor.phase===Phase.Eliminate)&&(cursor.column>=n||cursor.phase===Phase.Eliminate&&cursor.row>=n))invalid();
  if([Phase.UpdateRhs,Phase.BackBegin,Phase.BackSum,Phase.BackDivide].includes(cursor.phase)&&cursor.row>=n)invalid();return cursor;
 }
}
