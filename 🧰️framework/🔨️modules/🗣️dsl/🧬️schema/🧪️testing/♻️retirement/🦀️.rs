//! 🧪️ Original Record laws retain payload grants while admitting exact owned allocation events.
use semio_framework_value::{RetainedCloneGrant, RetainedCloneStep, RetainedWorkPoll, ValueError, retirement::{RetireOwned, controlled::ControlledRetirement}};
use semio_framework_value_testing_admission::{EffectBank, Wallet, demand_grant, driven};

pub fn retain<T: RetireOwned>(value: T) -> ControlledRetirement<T> {
    ControlledRetirement::new(value).unwrap_or_else(|_| panic!("Record test owner lacks controlled retirement authority"))
}

pub fn assert_unfunded<T: RetireOwned>(owner: &mut ControlledRetirement<T>) {
    let demand = owner.next_demand(0).expect("retirement demand");
    let step = funded_step(owner, 0, RetainedCloneGrant::default()).expect("unfunded retirement");
    assert!(matches!(step, RetainedCloneStep::Progress(progress) if progress == Default::default()));
    assert_eq!(owner.last_turn_work(), 0);
    assert_eq!(owner.next_demand(0).expect("retirement demand"), demand);
}

pub fn advance<T: RetireOwned>(owner: &mut ControlledRetirement<T>, body: usize) -> Result<RetainedCloneStep, ValueError> {
    let grant = demand_grant(owner.next_demand(body)?);
    funded_step(owner, body, grant)
}

fn funded_step<T: RetireOwned>(owner: &mut ControlledRetirement<T>, body: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    let work = owner.next_computational_work(body)?;
    let stock = RetainedCloneGrant {
        maximum_items: 4,
        maximum_copy_bytes: grant.maximum_copy_bytes + 1024,
        maximum_capacity_bytes: grant.maximum_capacity_bytes + 1024,
        maximum_release_bytes: grant.maximum_release_bytes + 1024,
        maximum_depth: grant.maximum_depth.max(4),
    };
    let mut bank = EffectBank::new(Wallet::new(stock, work + 4));
    let mut allocated = None;
    let (RetainedWorkPoll::Complete(turn), _) = driven(&mut bank, grant, work, |effect, reply, recovering| {
        let answer = if recovering {
            owner.recover_step_into(body, effect, reply)
        } else {
            owner.step_into(body, effect, reply)
        };
        allocated = (*effect.actual()).or(allocated);
        answer
    }) else {
        panic!("Record test retirement must complete its funded step");
    };
    assert!(turn.work <= u128::from(work));
    let step = turn.result.map_err(|failure| failure.primary().cloned().unwrap_or_else(|| ValueError::invariant("Record test retirement failed without a primary cause")))?;
    assert!(step.progress().fits(grant));
    assert_eq!(allocated.unwrap_or_default(), step.progress().retained_capacity_bytes);
    Ok(step)
}
