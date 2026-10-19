/** 📸️ Captures one immutable original presentation body without reconstructing any field. */
export class ToolRunPresentation<T> {
 private value:T|undefined;
 constructor(original:T){this.value=original;}
 get original(){if(!this.retained)throw Error("Original presentation has already returned");return this.value!;}
 get retained(){return this.value!==undefined;}
 capture(items:number){if(items<=0||!this.retained)return undefined;return new ToolRunPresentation(this.original);}
 close(items:number){if(items<=0||!this.retained)return 0;this.value=undefined;return 1;}
}
