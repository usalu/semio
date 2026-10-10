/** ♻️ Keeps original initialized bytes separate from actual backing retirement. */
export class RetainedCopyClose{
 constructor(public capacity:number,public reserved:boolean,public complete:boolean){}
 get demand(){return{copy:0,capacity:0,release:this.capacity,depth:Number(this.capacity!==0||this.reserved||this.complete)};}
 step(grant:{items:number;copy:number;capacity:number;release:number;depth:number}){
  const demand=this.demand;if(demand.depth===0)return{terminal:true,released:0};if(grant.items<1||grant.release<demand.release||grant.depth<demand.depth)return{terminal:false,released:0};if(this.capacity!==0){this.capacity=0;return{terminal:false,released:demand.release};}this.reserved=false;this.complete=false;return{terminal:false,released:0};
 }
}
