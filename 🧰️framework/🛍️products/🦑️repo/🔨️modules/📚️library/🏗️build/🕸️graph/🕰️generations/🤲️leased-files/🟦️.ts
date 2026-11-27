import type{GenerationFilePortV1}from"../🟨️.mjs";
import type{NativeImageSharedReaderV1}from"../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts";
/** 🤲️ Wraps a generation file port so that every operation runs under the shared lease of the image the store lives on: a retirement, regrow or detach of the image never finds a generation file read, written or listed behind its lease, and a long upload takes the lease in bursts instead of holding the image for its whole run.
 * @see ../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🤲️shared-read/🟦️.ts */
export function leasedGenerationFilePortV1(inner:GenerationFilePortV1,reader:NativeImageSharedReaderV1):GenerationFilePortV1{
 const port:GenerationFilePortV1={
  read:path=>reader.run(()=>inner.read(path)),
  write:(path,bytes,options)=>reader.run(()=>inner.write(path,bytes,options)),
  names:directory=>reader.run(()=>inner.names(directory)),
  list:directory=>reader.run(()=>inner.list(directory)),
  remove:(path,options)=>reader.run(()=>inner.remove(path,options))
 };
 return Object.freeze(port);
}
