/** 🛑️ The parent pid is no longer the parent (the process was handed to another one when it ended), or the parent no longer exists; a parent we may not signal still exists. See https://man7.org/linux/man-pages/man2/kill.2.html */
export function parentGoneV1(parent:number,current:number=process.ppid):boolean{
 if(current!==parent)return true;
 try{process.kill(parent,0);return false;}catch(error){return(error as NodeJS.ErrnoException).code!=="EPERM";}
}
