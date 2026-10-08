//! ✂️ Positional splice script — the order-exact diff of one ordered list. A script removes
//! entries by BASE index (`cuts`) and inserts entries at AFTER index (`puts`); everything else
//! keeps its relative order. Both halves are strictly ascending, which makes the script the unique
//! normal form of its edit, so composing, inverting and comparing scripts is plain index arithmetic.
//! Pure `std`: no snapshot, no codec, no framework type.

//#region 🔖️Script
/// ✂️ One list edit: `cuts` name removed entries by base index and key, `puts` name inserted entries by after index.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Splice<K, T> {
    cuts: Vec<(usize, K)>,
    puts: Vec<(usize, T)>,
}

impl<K, T> Default for Splice<K, T> {
    fn default() -> Self {
        Self { cuts: Vec::new(), puts: Vec::new() }
    }
}

/// 🚫️ Why a script does not apply to a list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpliceFault {
    CutOutOfRange { index: usize, len: usize },
    CutMismatch { index: usize },
    Unordered,
    PutOutOfRange { index: usize, len: usize },
}

impl std::fmt::Display for SpliceFault {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CutOutOfRange { index, len } => write!(formatter, "removal index {index} is outside a list of {len} entries"),
            Self::CutMismatch { index } => write!(formatter, "the entry at index {index} is not the one the removal names"),
            Self::Unordered => write!(formatter, "removals and insertions must ascend strictly by index"),
            Self::PutOutOfRange { index, len } => write!(formatter, "insertion index {index} is outside a result of {len} entries"),
        }
    }
}
//#endregion 🔖️Script

//#region 🔖️Build
impl<K, T> Splice<K, T> {
    /// 🏗️ Builds a script from unordered cuts and puts, sorting both halves by index.
    pub fn new(mut cuts: Vec<(usize, K)>, mut puts: Vec<(usize, T)>) -> Self {
        cuts.sort_by_key(|cut| cut.0);
        puts.sort_by_key(|put| put.0);
        Self { cuts, puts }
    }

    /// ✂️ Removes the entry at base `index`.
    pub fn cutting(index: usize, key: K) -> Self {
        Self { cuts: vec![(index, key)], puts: Vec::new() }
    }

    /// ➕️ Inserts `row` so it lands at after `index`.
    pub fn putting(index: usize, row: T) -> Self {
        Self { cuts: Vec::new(), puts: vec![(index, row)] }
    }

    /// 🧩️ Splits the script into its cuts and puts.
    pub fn into_parts(self) -> (Vec<(usize, K)>, Vec<(usize, T)>) {
        (self.cuts, self.puts)
    }

    pub fn cuts(&self) -> &[(usize, K)] {
        &self.cuts
    }

    pub fn puts(&self) -> &[(usize, T)] {
        &self.puts
    }

    pub fn is_empty(&self) -> bool {
        self.cuts.is_empty() && self.puts.is_empty()
    }
}
//#endregion 🔖️Build

//#region 🔖️Apply
impl<K, T: Clone> Splice<K, T> {
    /// ▶️ Applies the script to `list`; `names` decides whether a list entry is the one a removal names.
    pub fn apply(&self, list: Vec<T>, names: impl Fn(&T, &K) -> bool) -> Result<Vec<T>, SpliceFault> {
        let len = list.len();
        if self.cuts.windows(2).any(|pair| pair[0].0 >= pair[1].0) || self.puts.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
            return Err(SpliceFault::Unordered);
        }
        for (index, key) in &self.cuts {
            match list.get(*index) {
                None => return Err(SpliceFault::CutOutOfRange { index: *index, len }),
                Some(entry) if !names(entry, key) => return Err(SpliceFault::CutMismatch { index: *index }),
                Some(_) => {}
            }
        }
        let total = len - self.cuts.len() + self.puts.len();
        if let Some((index, _)) = self.puts.last().filter(|(index, _)| *index >= total) {
            return Err(SpliceFault::PutOutOfRange { index: *index, len: total });
        }
        let mut cuts = self.cuts.iter().map(|cut| cut.0).peekable();
        let mut survivors = list.into_iter().enumerate().filter(|(index, _)| {
            if cuts.peek() == Some(index) {
                cuts.next();
                false
            } else {
                true
            }
        });
        let mut puts = self.puts.iter().peekable();
        let mut result = Vec::with_capacity(total);
        for position in 0..total {
            match puts.peek() {
                Some((index, row)) if *index == position => {
                    result.push(row.clone());
                    puts.next();
                }
                _ => match survivors.next() {
                    Some((_, entry)) => result.push(entry),
                    None => return Err(SpliceFault::PutOutOfRange { index: position, len: total }),
                },
            }
        }
        Ok(result)
    }
}
//#endregion 🔖️Apply

//#region 🔖️Inverse
impl<K, T: Clone> Splice<K, T> {
    /// ↩️ The negative script: what the script put is cut again, what it cut is put back from `base`; `key_of` names an inserted entry.
    pub fn inverse(&self, base: &[T], key_of: impl Fn(&T) -> K) -> Self {
        Self {
            cuts: self.puts.iter().map(|(index, row)| (*index, key_of(row))).collect(),
            puts: self.cuts.iter().filter_map(|(index, _)| base.get(*index).map(|row| (*index, row.clone()))).collect(),
        }
    }
}
//#endregion 🔖️Inverse

//#region 🔖️Compose
/// 🔢️ The `rank`-th (zero based) index that is not in the ascending `cuts`.
fn nth_surviving(rank: usize, cuts: impl Iterator<Item = usize>) -> usize {
    let mut index = rank;
    for cut in cuts {
        if cut <= index {
            index += 1;
        } else {
            break;
        }
    }
    index
}

/// 🔢️ Where the `rank`-th entry kept from a list lands once `puts` (ascending after-indices) are spliced around it.
fn landing(rank: usize, puts: impl Iterator<Item = usize>) -> usize {
    let mut before = 0;
    for put in puts {
        if put <= rank + before {
            before += 1;
        } else {
            break;
        }
    }
    rank + before
}

impl<K, T> Splice<K, T> {
    /// ➕️ Composes `self` (base to mid) with `later` (mid to after) into one script (base to after).
    /// An insertion that `later` removes again cancels with that removal; every other `later` removal is
    /// re-addressed to the base entry it removes; surviving insertions are re-addressed to their after index.
    pub fn absorb(&mut self, later: Self) {
        let Self { cuts: later_cuts, puts: later_puts } = later;
        let own_cuts: Vec<usize> = self.cuts.iter().map(|cut| cut.0).collect();
        let later_cut_indices: Vec<usize> = later_cuts.iter().map(|cut| cut.0).collect();
        let own_puts = std::mem::take(&mut self.puts);
        let mut cancelled = vec![false; own_puts.len()];
        for (index, key) in later_cuts {
            match own_puts.binary_search_by_key(&index, |put| put.0) {
                Ok(position) => cancelled[position] = true,
                Err(before) => self.cuts.push((nth_surviving(index - before, own_cuts.iter().copied()), key)),
            }
        }
        let mut puts: Vec<(usize, T)> = own_puts
            .into_iter()
            .zip(cancelled)
            .filter(|(_, cancelled)| !cancelled)
            .map(|((index, row), _)| (landing(index - later_cut_indices.partition_point(|cut| *cut < index), later_puts.iter().map(|put| put.0)), row))
            .collect();
        puts.extend(later_puts);
        puts.sort_by_key(|put| put.0);
        self.cuts.sort_by_key(|cut| cut.0);
        self.puts = puts;
    }
}
//#endregion 🔖️Compose

//#region 🔖️Settle
impl<T: PartialEq> Splice<T, T> {
    /// ⚖️ Cancels every removal and insertion of an equal value that leaves the list exactly where it was,
    /// which is the normal form of a value list (a keyed row cannot be compared without its base, so it never settles).
    pub fn settle(&mut self) {
        loop {
            let pair = self.cuts.iter().enumerate().find_map(|(cut, (base, value))| {
                let rank = base - self.cuts.partition_point(|other| other.0 < *base);
                self.puts.iter().position(|(after, row)| row == value && landing(rank, self.puts.iter().filter(|other| other.0 != *after).map(|other| other.0)) == *after).map(|put| (cut, put))
            });
            match pair {
                Some((cut, put)) => {
                    self.cuts.remove(cut);
                    self.puts.remove(put);
                }
                None => return,
            }
        }
    }
}
//#endregion 🔖️Settle

//#region 🔖️Replace
impl<T: Clone + PartialEq> Splice<T, T> {
    /// 🔁️ The script that makes `was` read `now` by cutting and re-putting exactly the positions whose entry differs.
    pub fn replacing(was: &[T], now: &[T]) -> Self {
        let differs = |index: usize| was.get(index).zip(now.get(index)).is_none_or(|(before, after)| before != after);
        Self {
            cuts: (0..was.len()).filter(|index| differs(*index)).map(|index| (index, was[index].clone())).collect(),
            puts: (0..now.len()).filter(|index| differs(*index)).map(|index| (index, now[index].clone())).collect(),
        }
    }
}
//#endregion 🔖️Replace

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
