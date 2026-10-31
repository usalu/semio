import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{existsSync,readFileSync}from"node:fs";
import law from"./🧫️fixtures/🔣️.json";
import schema from"../🧬️schema/🔣️.json";
test("owned original async turn preserves its issuing wallet and receiver across suspension",()=>{
 const validate=new Ajv({strict:true}).compile(schema);expect(validate(law)).toBe(true);for(const key of["copiedBudgetDto","freshPollingRecipientDebit","newRootOrLedger","deadlineRenewal","duplicateReturn","foreignPollingTransfer"])expect(validate({...law,[key]:true})).toBe(false);
 const db=new Database(":memory:");try{db.exec("CREATE TABLE original_turn(operation INTEGER,generation INTEGER,epoch INTEGER,fuel INTEGER,admission INTEGER,guest INTEGER,remaining INTEGER)");db.query("INSERT INTO original_turn VALUES(?,?,?,?,?,?,?)").run(law.operation,law.generation,law.epoch,law.initialFuel,law.admissionReceipt[0]!,law.guestWork,law.remainingFuel);expect(db.query("SELECT operation,generation,epoch,fuel-admission-guest AS conserved,remaining FROM original_turn").get()).toEqual({operation:17,generation:9,epoch:19,conserved:4,remaining:4});}finally{db.close()}
 const path=new URL("../🦀️.rs",import.meta.url);const source=existsSync(path)?readFileSync(path,"utf8"):"";for(const name of["OriginalOwnedCallerTurn","OriginalOwnedCallerReturn","OriginalCallerSourceLoan","async fn","admit_loan","OriginalRetainedLoanIdentity","validate_polling_context","clock_observation","std::mem::take(cx.preview_sequence)"])expect(source).toContain(name);for(const forbidden of["StepContext::new","StepBudget::new","CancelToken::root_now","default_now_us"])expect(source).not.toContain(forbidden);
 const loan=readFileSync(new URL("../../🎟️source/🦀️.rs",import.meta.url),"utf8");expect(loan.slice(loan.indexOf("impl OriginalCallerSourceLoan"))).toContain("pub fn cancellation(&self)");
 const actor=new URL("../../../../🎭️actor/🎟️retained-turn/💼️job/⏸️park/🦀️.rs",import.meta.url);const issuer=existsSync(actor)?readFileSync(actor,"utf8"):"";for(const name of["OriginalActorParkedTurn","receive_guest_spent","return_actor_turn","take_original_issuer_on_caller"])expect(issuer).toContain(name);
});
