/** 🆔️ Preserves canonical full-width decimal handle identity without numeric coercion. */
export function canonicalDecimalHandle(value:unknown):string|null{
 if(typeof value!=="string"||value.length<1||value.length>39||value.length>1&&value[0]==="0")return null;
 for(let index=0;index<value.length;index++){const digit=value.charCodeAt(index);if(digit<48||digit>57)return null;}
 return value.length===39&&value>"340282366920938463463374607431768211455"?null:value;
}
