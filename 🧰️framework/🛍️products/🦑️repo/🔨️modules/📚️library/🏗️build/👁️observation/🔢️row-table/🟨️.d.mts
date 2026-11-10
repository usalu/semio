/** 🔤️ The text identity of one fact inside an observation row. */
export function factKeyV1(fact:unknown):string;
/** 🧶️ A table of texts: equal texts are one string and one number, dense from zero until `clear()`. */
export type StringTableV1=Readonly<{id(text:string):number;text(id:number):string;size():number;characters():number;clear():void}>;
/** 🏗️ Creates a table of texts. */
export function createStringTableV1():StringTableV1;
/** 🔢️ The compact store of canonical observation rows (string table plus typed-array columns): equal rows share one number, a row object is a weakly kept view made on demand, numbers are never reused. */
export type RowTableV1<Row extends{kind:string;path:string;fact:unknown}>=Readonly<{
 /** 🗺️ The row alive for an equal fact, else the row `make(path)` returns, where `path` is the shared instance of the path text. */
 canonical(kind:string,path:string,fact:unknown,make:(path:string)=>Row):Row;
 /** 📊️ The number of a row: that of its canonical row. */
 id(row:Row):number;
 /** 🪄️ The live canonical row of a number, or a row made from the columns; undefined for a number of a cleared store. */
 row(id:number):Row|undefined;
 /** 🧬️ A row made from the columns whether or not a canonical row is alive. */
 materialize(id:number):Row|undefined;
 /** 📦️ The rows held by the columns. */
 size():number;
 /** 📏️ The rows, strings, characters and the estimated bytes of the store. */
 stats():Readonly<{rows:number;strings:number;characters:number;bytes:number}>;
 /** 🧹️ Empties the storage; numbers keep counting. */
 clear():void;
}>;
/** 🏗️ Creates the compact row store; over `maximumRows` or `maximumCharacters` it empties itself before the next row. */
export function createRowTableV1<Row extends{kind:string;path:string;fact:unknown}=any>(options?:{maximumRows?:number;maximumCharacters?:number;hash?:(kind:number,path:number,shape:number,reference:number)=>number}):RowTableV1<Row>;
