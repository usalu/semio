/** 🫴️ Each original receiving field moves once into its typed candidate while both owners remain retained. */
export class JackRecordReconstruction<T>{
 readonly #source:Record<string,T|undefined>;
 #keys:readonly string[]|undefined;
 readonly #candidate:Record<string,T|undefined>={};
 #cursor=0;
 #closed=false;
 constructor(source:Record<string,T|undefined>){this.#source=source;}
 step(maximumWork:number):boolean{if(this.#closed)throw Error("Original reconstruction is terminal");if(!Number.isSafeInteger(maximumWork)||maximumWork<1)return false;this.#keys??=Object.keys(this.#source);if(this.#cursor<this.#keys.length){const key=this.#keys[this.#cursor++];this.#candidate[key]=this.#source[key];this.#source[key]=undefined;}return this.#cursor===this.#keys.length;}
 moved():number{return this.#cursor;}
 candidate():Record<string,T|undefined>{if(this.#closed)throw Error("Original reconstruction is terminal");return this.#candidate;}
 close(retire:(source:Record<string,T|undefined>,candidate:Record<string,T|undefined>)=>boolean):boolean{if(this.#closed)return true;if(!retire(this.#source,this.#candidate))return false;this.#closed=true;return true;}
}
