export type StoreReceiptEntryV1={readonly path:string;readonly kind:"file"|"directory";readonly bytes:number;readonly files:number;readonly sha256:string};
export type StoreReceiptLinkV1={readonly path:string;readonly target:string};
export type StoreReceiptV1={readonly version:1;readonly project:string;readonly target:string;readonly entries:readonly StoreReceiptEntryV1[];readonly links:readonly StoreReceiptLinkV1[]};
export type StoreReceiptFindingV1={readonly path:string;readonly reason:"absent"|"kind"|"size"|"digest"|"unreadable"|"link"};
export type StoreReceiptVerdictV1={readonly state:"valid"|"invalid"|"unwritten"|"malformed";readonly findings:readonly StoreReceiptFindingV1[]};
export type StoreReceiptDeclarationV1={readonly workspaceRoot:string;readonly projectRoot:string;readonly target:string;readonly outputs:readonly string[]|undefined};
export type StoreReceiptModeV1="stat"|"digest";
/** 🧾️ Names the declared receipt output of one target, relative to its project root. */
export function storeReceiptOutputV1(target:string):string;
/** 🔎️ Resolves the absolute receipt path a task declares among its outputs, or nothing when it declares none. */
export function declaredStoreReceiptV1(declaration:StoreReceiptDeclarationV1):string|undefined;
/** 🎟️ Starts a task that declares a receipt from no receipt and tells its handler where to write it; any other task is told of none, so a nested task never records into its parent's receipt. */
export function issueStoreReceiptV1(declaration:StoreReceiptDeclarationV1):Record<string,string|undefined>;
/** 🔧️ Admits the single-task uncached Nx child launch that repairs a receipt whose store content is gone. */
export function isStoreRepairSelectionV1(selection:readonly string[]):boolean;
/** 🛠️ Projects the Nx arguments that re-execute exactly one task uncached. */
export function storeRepairNxArgumentsV1(task:string,forwarded?:readonly string[]):string[];
/** 🧮️ Digests a directory by its sorted relative paths, sizes and file digests. */
export function storeTreeDigestV1(directory:string,hashes?:boolean):Promise<{sha256:string;bytes:number;files:number}>;
/** 🔭️ Observes one published file or directory into a receipt entry. */
export function observeStoreContentV1(workspaceRoot:string,path:string):Promise<StoreReceiptEntryV1>;
/** 📜️ Admits exactly the closed receipt shape the schema declares. */
export function requireStoreReceiptV1(value:unknown):StoreReceiptV1;
/** 📖️ Reads one receipt from its text. */
export function parseStoreReceiptV1(source:string):StoreReceiptV1;
/** 🖋️ Encodes a receipt canonically: entries sorted by path, two-space indent, trailing newline. */
export function encodeStoreReceiptV1(receipt:StoreReceiptV1):string;
/** 🧷️ Records published store content and the links bound to it into the receipt the running task declared; a task without a declared receipt records nothing. */
export function recordStoreContentV1(options:{readonly environment?:Readonly<Record<string,string|undefined>>;readonly workspaceRoot:string;readonly paths?:readonly string[];readonly links?:readonly StoreReceiptLinkV1[]}):Promise<void>;
/** ✅️ Checks every receipt entry against the store: `stat` compares kind and size, `digest` also compares content. */
export function verifyStoreReceiptV1(workspaceRoot:string,receipt:StoreReceiptV1,mode?:StoreReceiptModeV1):Promise<StoreReceiptVerdictV1>;
/** 🗂️ Verifies the receipt file one task left behind: missing is `unwritten`, unreadable is `malformed`. */
export function checkStoreReceiptFileV1(workspaceRoot:string,file:string,mode?:StoreReceiptModeV1):Promise<StoreReceiptVerdictV1>;
