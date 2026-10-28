export interface PostCustodyOracle{snapshot<T>(value:T):T;equal(a:unknown,b:unknown):boolean;retire(value:unknown,original:unknown):void}
export function postCustodyOracle(schema:unknown,fixture:unknown):PostCustodyOracle;
