import type{GenerationRowV1}from"../🟨️.mjs";
/** 📏️ The declared limits of the row log. */
export const ROW_LOG_POLICY_V1:Readonly<{flushBytes:number;batchRows:number;gapBytes:number;spanBytes:number;staleSeconds:number}>;
/** 📚️ The rows of one owner as ids into a row log: counted, keyed and read back in batches, never held. */
export interface RowLogListV1<Row=GenerationRowV1>extends Iterable<Row>{readonly length:number;readonly ids:Uint32Array;keys():IterableIterator<string>;paths():IterableIterator<string>;rows():IterableIterator<Row>;batchesSync():IterableIterator<readonly Row[]>;batches():AsyncIterable<readonly Row[]>;concat(...more:readonly (readonly Row[])[]):RowLogListV1<Row>}
/** 🧱️ Keeps the ids of one owner while its rows arrive. */
export interface RowLogBuilderV1<Row=GenerationRowV1>{add(rows:Iterable<Row>):void;addList(list:RowLogListV1<Row>):void;finish():RowLogListV1<Row>}
/** 🗃️ An append-only log of distinct observation rows on disk. */
export interface RowLogV1<Row=GenerationRowV1>{readonly path:string;append(rows:Iterable<Row>):RowLogListV1<Row>;builder():RowLogBuilderV1<Row>;list(ids:Iterable<number>):RowLogListV1<Row>;readonly empty:RowLogListV1<Row>;stats():{rows:number;bytes:number;reads:number;readBytes:number;pendingBytes:number};close():void}
/** 🗃️ Creates the log at `path` (a previous crashed build's logs next to it are swept). */
export function createRowLogV1<Row=GenerationRowV1>(options:{path:string;policy?:typeof ROW_LOG_POLICY_V1}):RowLogV1<Row>;
