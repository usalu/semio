import budget from "../../📏️budget/🧬️schema/🔣️.json" with { type: "json" };

function scalarChunks(bytes: Uint8Array, decoder: TextDecoder): string[] {
  const width = budget.const.textMaxBytes;
  const chunks: string[] = [];
  let start = 0;
  do {
    let end = Math.min(start + width, bytes.length);
    while (end < bytes.length && (bytes[end]! & 0xc0) === 0x80) end--;
    chunks.push(decoder.decode(bytes.subarray(start, end)));
    start = end;
  } while (start < bytes.length);
  return chunks;
}

/** 🧶️ Preserves exact UTF-8 text in existing bounded scalar pages. */
export function splitUiTextChunks(source: string, maximumBytes: number): string[] {
  const {textMaxBytes: width, valueMaxItems: maximum} = budget.const;
  const bytes = new TextEncoder().encode(source);
  const decoder = new TextDecoder("utf-8", {fatal: true, ignoreBOM: true});
  if (!Number.isSafeInteger(maximumBytes) || maximumBytes < 0 || maximumBytes > width * maximum || bytes.length > maximumBytes || decoder.decode(bytes) !== source) throw Error("UI text source byte limit exceeded");
  const chunks = scalarChunks(bytes, decoder);
  if (chunks.length > maximum) throw Error("UI text chunk collection capacity exceeded");
  return chunks;
}

/** 🪡️ Validates bounded wire chunks before reconstructing an owned text source. */
export function joinUiTextChunks(chunks:unknown,maximumBytes:number):string{
 const width=budget.const.textMaxBytes,maximum=budget.const.valueMaxItems,encoder=new TextEncoder(),decoder=new TextDecoder("utf-8",{fatal:true,ignoreBOM:true});
 if(!Number.isSafeInteger(maximumBytes)||maximumBytes<0||maximumBytes>width*maximum||!Array.isArray(chunks)||chunks.length===0||chunks.length>maximum)throw Error("invalid UI text chunks");
 let total=0;
 for(const chunk of chunks){
  if(typeof chunk!=="string")throw Error("invalid UI text chunk");
  const bytes=encoder.encode(chunk);
  total+=bytes.length;
  if(bytes.length>width||total>maximumBytes||decoder.decode(bytes)!==chunk)throw Error("UI text chunk byte limit exceeded");
 }
 return chunks.join("");
}

/** 🗃️ Divides a semantic input draft into pages of bounded UTF-8 scalars. */
export function retainedUiTextInput(source: string): string | { value: string[][] } {
  const {textMaxBytes: width, valueMaxItems: columns} = budget.const;
  const bytes = new TextEncoder().encode(source);
  const decoder = new TextDecoder("utf-8", {fatal: true, ignoreBOM: true});
  if (decoder.decode(bytes) !== source) throw Error("Invalid input text");
  if (bytes.length <= width) return source;
  const chunks = scalarChunks(bytes, decoder);
  const pages: string[][] = [];
  for (let start = 0; start < chunks.length; start += columns) pages.push(chunks.slice(start, start + columns));
  return {value: pages};
}
