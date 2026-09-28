/** 📦️ One bounded standard-base64 page for segmented PNG output. */
export function base64OutputPage(bytes:Uint8Array):Uint8Array {
  if(bytes.length>3072)throw new Error("raster.export-page-limit");
  const alphabet="ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/",output=new Uint8Array(Math.ceil(bytes.length/3)*4);
  for(let i=0,j=0;i<bytes.length;i+=3,j+=4){const a=bytes[i]!,b=bytes[i+1]??0,c=bytes[i+2]??0;output[j]=alphabet.charCodeAt(a>>2);output[j+1]=alphabet.charCodeAt(((a&3)<<4)|(b>>4));output[j+2]=i+1<bytes.length?alphabet.charCodeAt(((b&15)<<2)|(c>>6)):61;output[j+3]=i+2<bytes.length?alphabet.charCodeAt(c&63):61;}
  return output;
}
