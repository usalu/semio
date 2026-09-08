
use crate::{ActorSystem, Command, InvokeId, Machine, NoMigrations, TestHost, TimerId, TraceInspector};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CheckoutContext {
    pub attempts: u32,
    pub method_set: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Receipt {
    pub attempts: u32,
}

fn build_context(_input: ()) -> CheckoutContext {
    CheckoutContext::default()
}

fn allow_select(ctx: &CheckoutContext, _event: Option<&checkout::Event>) -> bool {
    ctx.attempts < 3
}

fn set_method(ctx: &mut CheckoutContext, _event: Option<&checkout::Event>, _sink: &mut Vec<Command<checkout::Checkout>>) {
    ctx.method_set = true;
}

fn note_timeout(ctx: &mut CheckoutContext, _event: Option<&checkout::Event>, _sink: &mut Vec<Command<checkout::Checkout>>) {
    ctx.attempts += 1;
}

fn make_receipt(ctx: &CheckoutContext) -> Receipt {
    Receipt { attempts: ctx.attempts }
}

crate::statechart! {
    machine checkout {
        context: CheckoutContext;
        event Event { Confirm, SelectMethod, PaymentSucceeded, PaymentFailed, Retry, Cancel, Resume, ShipDone, InvoiceDone }
        input: ();
        output: Receipt;
        effect: ();
        context_from_input: build_context;
        output_from_context: make_receipt;
        initial: cart;

        state cart {
            on Confirm => payment;
            on Resume => payment_history;
        }
        state payment {
            initial: selecting;
            history payment_history shallow;
            state selecting {
                on SelectMethod if allow_select => processing do set_method;
            }
            state processing {
                invoke charge;
                after 5000 => failed do note_timeout;
                on PaymentSucceeded => fulfilment;
                on PaymentFailed => failed;
                on Cancel => cart;
            }
            state failed {
                on Retry => processing;
            }
        }
        parallel fulfilment {
            state shipping {
                initial: ship_pending;
                state ship_pending { on ShipDone => ship_done; }
                final ship_done;
            }
            state invoicing {
                initial: invoice_pending;
                state invoice_pending { on InvoiceDone => invoice_done; }
                final invoice_done;
            }
            on_done => done;
        }
        final done;
    }
}

#[test]
fn dsl_machine_walks_cart_to_receipt() {
    let mut system: ActorSystem<checkout::Checkout, TestHost<checkout::Checkout>> = ActorSystem::new(TestHost::new());
    let root = system.spawn_root(());
    assert!(system.snapshot(root).unwrap().matches("cart"));

    system.send(root, checkout::Event::Confirm);
    system.drain();
    assert!(system.snapshot(root).unwrap().matches("selecting"));

    system.send(root, checkout::Event::SelectMethod);
    system.drain();
    assert!(system.snapshot(root).unwrap().matches("processing"));
    assert!(system.snapshot(root).unwrap().context.method_set);
    assert_eq!(system.host.started_tasks(), &[(root, InvokeId(0))]);

    system.send(root, checkout::Event::PaymentSucceeded);
    system.drain();
    assert!(system.snapshot(root).unwrap().matches("ship_pending"));
    assert!(system.snapshot(root).unwrap().matches("invoice_pending"));
    assert_eq!(system.host.cancelled_tasks(), &[(root, InvokeId(0))], "leaving processing must stop its invoke");

    system.send(root, checkout::Event::ShipDone);
    system.drain();
    assert!(system.snapshot(root).unwrap().matches("ship_done"));
    assert!(system.snapshot(root).unwrap().matches("invoice_pending"), "invoicing region must still be pending");
    system.send(root, checkout::Event::InvoiceDone);
    system.drain();

    assert!(matches!(system.snapshot(root).unwrap().status, crate::Status::Done(_)));
    if let crate::Status::Done(receipt) = &system.snapshot(root).unwrap().status {
        assert_eq!(receipt.attempts, 0);
    }
}

#[test]
fn dsl_machine_cancel_resume_round_trips_via_shallow_history() {
    let mut sink: Vec<Command<checkout::Checkout>> = Vec::new();
    let mut snapshot = crate::init::<checkout::Checkout>((), &mut sink);
    let mut inspector = TraceInspector::<checkout::Checkout>::default();

    crate::macrostep(&mut snapshot, checkout::Event::Confirm, &mut sink, &mut inspector);
    crate::macrostep(&mut snapshot, checkout::Event::SelectMethod, &mut sink, &mut inspector);
    assert!(snapshot.matches("processing"));

    // Cancelling from `processing` exits `payment` entirely (recording shallow
    // history), landing back in `cart`.
    crate::macrostep(&mut snapshot, checkout::Event::Cancel, &mut sink, &mut inspector);
    assert!(snapshot.matches("cart"));
    assert!(!snapshot.matches("payment"));

    // Resuming must restore `processing`, not `payment`'s default `selecting`.
    crate::macrostep(&mut snapshot, checkout::Event::Resume, &mut sink, &mut inspector);
    assert!(snapshot.matches("processing"), "shallow history must restore into processing, not the default selecting");
    assert!(!snapshot.matches("selecting"));
    assert!(!inspector.entries.is_empty());

    let fired = crate::timer_elapsed(&mut snapshot, TimerId(0), &mut sink, &mut inspector);
    assert_eq!(fired.microsteps, 1);
    assert!(snapshot.matches("failed"));
    assert_eq!(snapshot.context.attempts, 1);

    crate::macrostep(&mut snapshot, checkout::Event::Retry, &mut sink, &mut inspector);
    assert!(snapshot.matches("processing"));

    let persisted = super::persist(&snapshot);
    assert_eq!(persisted.fingerprint, checkout::Checkout::definition().fingerprint);
    let restored = crate::restore::<checkout::Checkout, NoMigrations>(&persisted, snapshot.context.clone(), &[]).expect("restore should succeed");
    assert!(restored.matches("processing"));
}

#[test]
fn dsl_machine_coverage_reaches_every_declared_state() {
    let model = crate::Model::<checkout::Checkout>::new(vec![
        checkout::Event::Confirm,
        checkout::Event::SelectMethod,
        checkout::Event::PaymentSucceeded,
        checkout::Event::PaymentFailed,
        checkout::Event::Retry,
        checkout::Event::Cancel,
        checkout::Event::Resume,
        checkout::Event::ShipDone,
        checkout::Event::InvoiceDone,
    ]);
    let coverage = crate::explore(&model, ());
    for expected in ["cart", "selecting", "processing", "failed", "ship_pending", "ship_done", "invoice_pending", "invoice_done", "done"] {
        assert!(coverage.reached_stable_ids.contains(&expected), "expected model exploration to reach `{expected}`, got {:?}", coverage.reached_stable_ids);
    }
}

//#region 🔖️StepTests

/// 🌱️ `start` yields a persistable initial configuration without the caller ever holding a `Snapshot`.
#[test]
fn start_produces_a_persistable_initial_configuration() {
    let step = crate::start::<checkout::Checkout>(());
    assert!(step.is_active("cart"), "expected the initial configuration to be `cart`, got {:?}", step.active);
    assert!(step.entered.is_empty(), "start reports no transition, so nothing was entered by one");
    assert_eq!(step.persisted.fingerprint, checkout::Checkout::definition().fingerprint);
}

/// 🔁️ A whole read-transition-write cycle threads only `PersistedSnapshot` values across calls —
/// the property that lets a host keep machine state in a data lane instead of a durable field.
#[test]
fn step_round_trips_through_persisted_state_only() {
    let mut carried = crate::start::<checkout::Checkout>(()).persisted;
    let mut context = CheckoutContext::default();

    for (event, expected) in [(checkout::Event::Confirm, "selecting"), (checkout::Event::SelectMethod, "processing")] {
        let step = crate::step::<checkout::Checkout, NoMigrations>(&carried, context.clone(), event, &[]).expect("restore should succeed");
        assert!(step.is_active(expected), "expected `{expected}` after the transition, got {:?}", step.active);
        context.method_set = true;
        carried = step.persisted;
    }
}

/// 🔬️ `entered`/`exited` report the states a transition actually crossed, which is what a host
/// projects from — a `cart -> payment` confirm enters both the compound and its initial child.
#[test]
fn step_reports_entered_and_exited_states() {
    let initial = crate::start::<checkout::Checkout>(()).persisted;
    let step = crate::step::<checkout::Checkout, NoMigrations>(&initial, CheckoutContext::default(), checkout::Event::Confirm, &[]).expect("restore should succeed");

    assert!(step.exited.contains(&"cart"), "expected `cart` to be exited, got {:?}", step.exited);
    assert!(step.entered.contains(&"payment"), "expected the compound `payment` to be entered, got {:?}", step.entered);
    assert!(step.entered.contains(&"selecting"), "expected `payment`'s initial child to be entered, got {:?}", step.entered);
    assert!(!step.is_active("cart"), "`cart` must be gone from the settled configuration");
}

/// 🧯️ A guard that rejects still settles: the configuration is unchanged and nothing is reported
/// as entered, rather than the step failing.
#[test]
fn step_with_a_blocked_guard_leaves_the_configuration_untouched() {
    let initial = crate::start::<checkout::Checkout>(()).persisted;
    let confirmed = crate::step::<checkout::Checkout, NoMigrations>(&initial, CheckoutContext::default(), checkout::Event::Confirm, &[]).expect("restore should succeed");

    let exhausted = CheckoutContext { attempts: 3, method_set: false };
    let blocked = crate::step::<checkout::Checkout, NoMigrations>(&confirmed.persisted, exhausted, checkout::Event::SelectMethod, &[]).expect("restore should succeed");

    assert!(blocked.is_active("selecting"), "the blocked guard must leave `selecting` active, got {:?}", blocked.active);
    assert!(blocked.entered.is_empty(), "a rejected transition enters nothing, got {:?}", blocked.entered);
}

/// 🚫️ A fingerprint from a different machine shape is refused rather than silently reinterpreted.
#[test]
fn step_rejects_a_persisted_snapshot_from_another_machine_shape() {
    let mut foreign = crate::start::<checkout::Checkout>(()).persisted;
    foreign.fingerprint ^= 0xFFFF_FFFF;
    let outcome = crate::step::<checkout::Checkout, NoMigrations>(&foreign, CheckoutContext::default(), checkout::Event::Confirm, &[]);
    assert!(outcome.is_err(), "a mismatched fingerprint with no migration must not restore");
}

//#endregion 🔖️StepTests
