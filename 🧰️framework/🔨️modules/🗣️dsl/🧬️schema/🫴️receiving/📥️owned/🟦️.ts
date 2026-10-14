/** 🪜️ Fixed declaration inspection preserves the caller-owned field source across finite turns. */
export class DeclaredRecordShape {
 private declaration=0;
 private field=0;
 ready=false;
 refused=false;
 step(source:{readonly length:number;at(index:number):readonly [number,boolean]|undefined},declared:readonly (readonly [number,boolean])[],maximumWork:number):number {
  if(!Number.isSafeInteger(maximumWork)||maximumWork<0)throw Error("declared record work requires a finite nonnegative integer");
  if(maximumWork===0||this.ready||this.refused)return 0;
  if(source.length>declared.length){this.refused=true;return 1;}
  const declaration=declared[this.declaration];
  if(declaration){const [id,required]=declaration;const value=source.at(this.field);if(value&&value[0]<id||required&&(!value||value[0]!==id||!value[1])){this.refused=true;return 1;}if(value?.[0]===id)this.field++;this.declaration++;}
  if(this.declaration===declared.length){this.refused=this.field!==source.length;this.ready=!this.refused;}
  return 1;
 }
}
