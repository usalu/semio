/** 📬️ Tracks the original source and permit-return owners independently of physical allocation. */
export class PublicationClaimReturn {
 constructor(public aliases:number,readonly frameBytes:number){}
 claimed=false;returned=false;cancelled=false;terminal=false;
 claim(){if(this.cancelled||this.claimed||this.terminal)return false;this.returned=false;this.claimed=true;return true;}
 returnPermit(){if(!this.claimed||this.returned)throw Error("sole original permit return");this.returned=true;this.claimed=false;}
 demand(){return this.claimed?null:this.returned?{copy:8,release:0}:{copy:16,release:this.frameBytes};}
 close(items:number,copy:number,release:number,depth:number){const demand=this.demand();if(this.terminal||demand===null||!items||copy<demand.copy||release<demand.release||!depth)return{ready:false,released:0};if(this.returned){this.returned=false;return{ready:true,released:0};}this.aliases--;this.terminal=true;return{ready:true,released:this.aliases?0:this.frameBytes};}
}
