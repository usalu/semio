/** 🔢️ Exact native unsigned64 Home generation, independent of occurrence identities. */
export function parseHomeCatalogGeneration(value:unknown,at="$"):bigint{
 if(typeof value!=="bigint"||value<0n||value>0xffffffffffffffffn)throw Error(at+": Home catalog generation is not an unsigned64 bigint");return value;
}
