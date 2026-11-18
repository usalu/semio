import{mkdtempSync,realpathSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{stagedWithinV1}from"../../👷️service/🟨️.mjs";
import{repositoryBuildGraphWriterDirectoryV1}from"../../../../../../📥️inputs/💾️storage/🟨️.mjs";

const root=realpathSync(mkdtempSync(join(tmpdir(),"child-custody-"))),report:Record<string,string>={};
const attempt=(label:string,body:()=>string)=>{try{report[label]="ok:"+body();}catch(error){report[label]="refused:"+String((error as Error).message);}};
attempt("bare",()=>repositoryBuildGraphWriterDirectoryV1(root,"inference"));
attempt("job",()=>(stagedWithinV1(join(root,"staging","observe"),()=>repositoryBuildGraphWriterDirectoryV1(root,"inference")) as unknown as string));
console.log("REPORT "+JSON.stringify({report,staging:join(root,"staging","observe")}));
