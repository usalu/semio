mod paged_command_ingress_tests {
    use super::*;

    fn two_page_command() -> semio_framework::kernel::PagedCommand {
        let mut pages = semio_framework::kernel::CommandPageSet::try_new(2).unwrap();
        pages.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(&[2; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]).unwrap()).unwrap();
        pages.try_push(semio_framework::kernel::FixedCommandPage::try_copy_from(b"tail").unwrap()).unwrap();
        semio_framework::kernel::PagedCommand::try_from_pages(pages).unwrap()
    }

    #[test]
    fn interrupted_two_page_command_closes_one_exact_page_per_step_then_faults() {
        let fault = plugin_internal_fault("cancelled");
        let first = PluginCommandIngress::Encoded(two_page_command()).cancel(fault);
        let second = match first.step() {
            PluginCommandIngressStep::Pending(next) => next,
            _ => panic!("first close step must retain the tail"),
        };
        assert!(matches!(second.step(), PluginCommandIngressStep::TerminalFault(fault) if fault.message == "cancelled"));
    }

    #[test]
    fn decoded_two_field_cancellation_never_publishes_partial_success() {
        let owner = protocol::DecodedAppCommandOwner::new(protocol::AppCommand::Command { seq: 3, command: vec![1; 4_096], view_state: vec![2; 4_096] });
        let first = PluginCommandIngress::Decoded(owner).cancel(plugin_internal_fault("cancelled"));
        let second = match first.step() {
            PluginCommandIngressStep::Pending(next) => next,
            _ => panic!("first decoded field close must remain pending"),
        };
        let third = match second.step() {
            PluginCommandIngressStep::Pending(next) => next,
            _ => panic!("second decoded field close must remain pending until the shell witness"),
        };
        assert!(matches!(third.step(), PluginCommandIngressStep::TerminalFault(fault) if fault.message == "cancelled"));
    }
}
