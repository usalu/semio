//! 🔀️ Supersede epoch: a guest-wide, per app instance counter that a command bumps the moment it requests a whole-document replacement, so every job of that instance started under an older epoch cancels at its next step instead of waiting for the replacement to commit.
//! See [`super::root_cancel_token`] for the cancellation authority the observing jobs own.

use std::cell::Cell;

const SUPERSEDE_SLOTS: usize = 16;

thread_local! {
    static CLOCK: Cell<u64> = const { Cell::new(0) };
    static SLOTS: [Cell<(u32, u64)>; SUPERSEDE_SLOTS] = const { [const { Cell::new((u32::MAX, 0)) }; SUPERSEDE_SLOTS] };
}

/// 🔀️ Announces that `app_instance_id`'s live document is being replaced; returns its new epoch. A full table recycles the stalest instance, whose jobs then see a larger epoch and cancel, never a silently missed one.
pub fn begin_supersede(app_instance_id: u32) -> u64 {
    let next = CLOCK.with(|clock| {
        let next = clock.get().wrapping_add(1);
        clock.set(next);
        next
    });
    SLOTS.with(|slots| {
        let index = slots.iter().position(|slot| slot.get().0 == app_instance_id).or_else(|| slots.iter().position(|slot| slot.get().0 == u32::MAX)).unwrap_or_else(|| (0..SUPERSEDE_SLOTS).min_by_key(|index| slots[*index].get().1).unwrap_or(0));
        slots[index].set((app_instance_id, next));
    });
    next
}

/// 🔀️ The epoch a job of `app_instance_id` records at birth and compares at every step; a different epoch means it was superseded.
pub fn supersede_epoch(app_instance_id: u32) -> u64 {
    SLOTS.with(|slots| slots.iter().map(Cell::get).find(|slot| slot.0 == app_instance_id).map_or(0, |slot| slot.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 🔀️ LAW: an announcement changes only the announced instance's epoch, and every announcement is a new value.
    #[test]
    fn an_announced_supersede_changes_only_its_own_instances_epoch() {
        let (first, second) = (supersede_epoch(7), supersede_epoch(8));
        let announced = begin_supersede(7);
        assert_ne!(announced, first);
        assert_eq!(supersede_epoch(7), announced);
        assert_eq!(supersede_epoch(8), second);
        assert_ne!(begin_supersede(7), announced);
    }
}
