//! 🪪️ A local authoring identity for native runtime fixtures.
/// 🎟️ The caller's Work every fixture identity hop commits under: each observed effect is admitted, bounded by the laws' grants and never by a job loan.
struct FixtureWork;
impl semio_framework_value::native_encoding::OriginalWorkRecipient for FixtureWork {
    type Context<'t, 's> = () where 's: 't;
    fn validate_context<'t, 's>(&self, _: &()) -> Result<(), semio_framework_value::ValueError> where 's: 't { Ok(()) }
    fn source_turn<'t, 's>(&self, _: &()) -> Option<semio_framework_value::native_encoding::OriginalSourceTurnIdentity> where 's: 't { None }
    fn admits<'r, 't, 's>(&mut self, _: &mut (), quote: semio_framework_value::native_encoding::OriginalWorkQuote<'r>, observed: u64, additional: u64) -> semio_framework_value::native_encoding::OriginalWorkQuoteComplete<'r> where 's: 't { quote.complete(Ok(observed.checked_add(additional).is_some())) }
    fn settle_partial<'r, 't, 's>(&mut self, _: &mut (), check: semio_framework_value::native_encoding::OriginalWorkPartialCheck<'r>) -> semio_framework_value::native_encoding::OriginalWorkPartialCheckComplete<'r> where 's: 't { check.complete(Ok(())) }
    fn recover_partial<'r, 't, 's>(&mut self, _: &mut (), check: semio_framework_value::native_encoding::OriginalWorkPartialCheck<'r>) -> semio_framework_value::native_encoding::OriginalWorkPartialCheckComplete<'r> where 's: 't { check.pending() }
}

/// 🪢️ Lends the fixture's one Work authority to every native hop of a fixture identity.
struct FixtureIdentityWork(semio_framework_value::native_encoding::OriginalWorkAuthority<FixtureWork>);
impl crate::control::IdentityWorkLender for FixtureIdentityWork {
    fn lend(&mut self, control: &mut semio_framework_value::NativeEncodeControl<'_>, operation: &mut dyn FnMut(&mut semio_framework_value::NativeEncodeControl<'_>) -> Result<(), semio_framework_value::ValueError>) -> Result<(), semio_framework_value::ValueError> {
        self.0.with_loan(&mut (), |loan| control.with_work_recipient(loan, |control| operation(control)))?
    }
}

/// 🪪️ One local native authoring identity for a fixture call: `&mut fixture_identity()` lives for the whole statement, awaits included.
/// Its no-op observer is a zero-sized closure, so leaking it allocates nothing; every hop commits under the fixture's own Work loan.
pub fn fixture_identity() -> crate::control::EntityIdentityAuthority<'static> {
    let observer: &'static mut crate::control::Observer<'static> = Box::leak(Box::new(|_: semio_framework_value::native_encoding::NativeEncodeProgress| true));
    crate::control::EntityIdentityAuthority::new(1usize << 24, observer).expect("fixture authoring identity admission")
        .with_work_lender(Box::new(FixtureIdentityWork(semio_framework_value::native_encoding::OriginalWorkAuthority::new(FixtureWork))))
}
