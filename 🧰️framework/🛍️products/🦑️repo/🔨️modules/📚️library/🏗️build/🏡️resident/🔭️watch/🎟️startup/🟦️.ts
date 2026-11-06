import {withRequestedIssuedResourceControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {buildPreparationBudgetMs} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

/** 🎟️ Borrows the declared preparation budget without extending the resident's original control. */
export function withResidentWatcherStartupV1<T>(parent:ResourceLeaseControl,body:(control:ResourceLeaseControl)=>Promise<T>,borrow:typeof withRequestedIssuedResourceControlV1=withRequestedIssuedResourceControlV1):Promise<T>{return borrow(parent,buildPreparationBudgetMs(process.env),body);}
