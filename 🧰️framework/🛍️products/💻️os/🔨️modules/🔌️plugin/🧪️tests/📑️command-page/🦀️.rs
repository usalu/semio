//! 📑️ Actual command-page reservation, refusal and terminal retirement integration laws.
use std::mem::size_of;
#[global_allocator]
static COMMAND_PAGE_HEAP_WITNESS:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

/// ⚖️ LAW: the prologue reserves EXACTLY the pages its command declares, and that reservation stays
    /// inside one guest-allocator growth unit.
    ///
    /// 🧊️ The measurement is the heap witness's own peak across `CommandPageSet::try_new`, so it reads
    /// the block the allocator was actually asked for. Before the fix the one-page reading is 262 272 B
    /// — four times the ceiling the guest can be relied on to serve — and after it is one page's worth.
    ///
    /// 🧵️ This thread's own allocator scope — a process-wide peak minus a process-wide baseline
    /// reads every concurrent law's allocations and frees as this reservation's own, which makes
    /// the reading drift below the block that was really requested (127 B for a 128 B authority).
    #[test]
    fn a_command_page_authority_reserves_only_the_pages_its_command_declares() {
        for declared in [1usize, 2, 8, semio_framework::kernel::COMMAND_MAXIMUM_PAGES] {
            let reserved = semio_framework::kernel::CommandPageSet::reservation_bytes(declared) as isize;
            assert_eq!(reserved, (declared * size_of::<semio_framework::kernel::FixedCommandPage>()) as isize);
            semio_framework_trace::reset_heap_peak_on_this_thread();
            let baseline = semio_framework_trace::retained_heap_bytes_on_this_thread();
            let pages = semio_framework::kernel::CommandPageSet::try_new(declared).expect("a declared page authority");
            let measured = semio_framework_trace::peak_heap_bytes_on_this_thread() - baseline;
            assert_eq!(pages.declared(), declared);
            assert!(measured >= reserved, "a {declared}-page authority must actually reserve its {reserved} B; the witness saw {measured} B");
            assert!(
                measured < semio_framework::kernel::CommandPageSet::reservation_bytes(semio_framework::kernel::COMMAND_MAXIMUM_PAGES) as isize || declared == semio_framework::kernel::COMMAND_MAXIMUM_PAGES,
                "a {declared}-page authority reserved {measured} B — the whole page ceiling, not its own declaration"
            );
            drop(pages);
        }
        let one_page = semio_framework::kernel::CommandPageSet::reservation_bytes(1);
        assert!(
            one_page <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
            "a one-page command's ingress reservation is {one_page} B, over the {} B a routine guest path may ask a fixed linear memory for",
            semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
        );
        let ceiling_spine = semio_framework::kernel::CommandPageSet::reservation_bytes(semio_framework::kernel::COMMAND_MAXIMUM_PAGES);
        assert!(
            ceiling_spine <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
            "a ceiling-sized page authority reserves {ceiling_spine} B — over the {} B bound, which is what a page held INLINE in its slot costs and why it forced a fixed page ceiling",
            semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES
        );
    }

/// ⚖️ LAW: a page authority refuses every over-declaration and every over-push with a `Fault` the
    /// host can display, and never panics. Exhaustion on this path reaches a user as a refused command,
    /// not as a guest that stops answering.
    #[test]
    fn a_saturated_command_page_authority_refuses_without_panicking() {
        assert_eq!(semio_framework::kernel::CommandPageSet::try_new(0).expect_err("a zero-page command has no authority").code.0, "plugin.command-page-count");
        assert_eq!(
            semio_framework::kernel::CommandPageSet::try_new(semio_framework::kernel::COMMAND_MAXIMUM_PAGES + 1).expect_err("a command may not declare more than the ceiling").code.0,
            "plugin.command-page-count"
        );
        let mut pages = semio_framework::kernel::CommandPageSet::try_new(2).expect("a two-page authority");
        for _ in 0..2 {
            pages.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(&[7; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]).expect("a full page")).expect("the declared pages are admitted");
        }
        let (fault, returned) = pages.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(b"third").expect("a page")).expect_err("a third page is over the declared authority");
        assert_eq!(fault.code.0, "plugin.command-page-count");
        assert_eq!(returned.as_slice(), b"third", "a refused page is handed back, never dropped");
        assert_eq!(pages.len(), 2);
        let mut saturated = semio_framework::kernel::CommandPageSet::try_new(semio_framework::kernel::COMMAND_MAXIMUM_PAGES).expect("the ceiling authority");
        for _ in 0..semio_framework::kernel::COMMAND_MAXIMUM_PAGES {
            saturated.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(&[3; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]).expect("a full page")).expect("64 full pages are the whole byte authority");
        }
        let (fault, _) = saturated.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(b"over").expect("a page")).expect_err("the byte authority is spent");
        assert_eq!(fault.code.0, "plugin.command-page-count");
        let mut released = 0;
        while !saturated.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).0 {
            released += 1;
            assert!(released <= semio_framework::kernel::COMMAND_MAXIMUM_PAGES, "closing a saturated authority must terminate");
        }
        assert!(saturated.is_empty());
        let wire = semio_framework_diagnostic::decode_fault_bytes(&semio_framework_diagnostic::encode_fault_bytes(&fault));
        assert_eq!(wire.code.0, fault.code.0, "the refusal survives the exact wire the ingress status carries it on");
        assert_eq!(wire.message, fault.message);
        assert!(!wire.retryable, "a refusal the guest cannot serve is not something the host should retry");
    }
