import{aborted}from"../🧱️guards/🟨️.mjs";

/** 🧺️ Runs `visit` over `items` in bounded batches of `batch` at once, one batch after the other: before every batch `admit` (the footprint gate of the process) may slow the run down or give memory back, after every batch `after(done,total)` may give the event loop a turn, and a cancelled `signal` stops before the next batch. The store phase of a generation uses it so that a read near its memory grant never allocates the packed text of every owner on top of what it holds; the answers keep the order of the items.
 * @see ../🧪️tests/🧺️store-batches the contract */
export async function mapBatchesV1(items,{batch,visit,admit,after,signal}){
 const out=new Array(items.length),size=Math.max(1,Math.floor(batch));
 for(let from=0;from<items.length;from+=size){
  aborted(signal);
  if(admit!==undefined)await admit(signal);
  const end=Math.min(items.length,from+size);
  await Promise.all(Array.from({length:end-from},async(_unused,offset)=>{out[from+offset]=await visit(items[from+offset],from+offset);}));
  if(after!==undefined)await after(end,items.length);
 }
 return out;
}
