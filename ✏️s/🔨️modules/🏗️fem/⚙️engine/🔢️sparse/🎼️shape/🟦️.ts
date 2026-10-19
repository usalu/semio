/** 📏️ Logical admission checks exact native index/layout arithmetic; backing is admitted one real page at a time. */
export function subspaceShapeAdmitted(order:number,modes:number,wordBits:32|64):boolean{
 if(!Number.isSafeInteger(order)||!Number.isSafeInteger(modes)||order<=0||modes<=0||modes>order||order>0xffffffff)return false;
 return BigInt(order)**2n*8n<(1n<<BigInt(wordBits));
}
