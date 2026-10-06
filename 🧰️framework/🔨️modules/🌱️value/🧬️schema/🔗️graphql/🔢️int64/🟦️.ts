/** 🔗️ An owned literal descriptor keeps the scalar API independent of GraphQL libraries. */
export interface Int64Literal {readonly kind:string;readonly value?:unknown;}
const minimum=-9223372036854775808n,maximum=9223372036854775807n;
function decimal(value:unknown):bigint{
 if(typeof value!=="string")throw new TypeError("Int64 requires canonical decimal text");
 if(value.length>(value[0]==="-"?20:19)||!/^(0|-?[1-9][0-9]*)$/.test(value))throw new TypeError("Int64 decimal differs");
 const number=BigInt(value);
 if(number<minimum||number>maximum)throw new RangeError("Int64 is outside signed64");
 return number;
}
/** ➖️ Admits only the normative canonical decimal variable transport. */
export function parseInt64Variable(value:unknown):bigint{return decimal(value);}
/** 🧾️ Admits exact integer and quoted decimal literals without numeric widening. */
export function parseInt64Literal(literal:Int64Literal):bigint{
 if(literal.kind!=="IntValue"&&literal.kind!=="StringValue")throw new TypeError("Int64 literal kind differs");
 return decimal(literal.value);
}
/** 📤️ Serializes the exact intrinsic signed word through its canonical transport. */
export function serializeInt64(value:bigint):string{
 if(typeof value!=="bigint")throw new TypeError("Int64 output requires an intrinsic signed word");
 if(value<minimum||value>maximum)throw new RangeError("Int64 is outside signed64");
 return value.toString();
}

