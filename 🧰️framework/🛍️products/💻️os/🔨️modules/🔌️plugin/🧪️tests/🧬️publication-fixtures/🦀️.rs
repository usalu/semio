//! 🧪️ Source-owned presence and transient leaves for publication-pipeline laws.

#[path = "👥️presence/🦀️.rs"]
pub mod presence;
#[path = "🫧️transient/🦀️.rs"]
pub mod transient;

pub use presence::{ChangePublicationPresence, PublicationPresence, PublicationPresenceDiff, PublicationPresenceMutation};
pub use transient::{ChangePublicationTransient, PublicationTransient, PublicationTransientDiff, PublicationTransientMutation};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{NoConfig, NoConfigMutation, NoPresence, NoPresenceMutation, NoTransient, NoTransientMutation};
    use protocol::{Mutation, MutationDiff, MutationKind, MutationLeaf, OpBinary, OpText};

    #[test]
    fn no_state_mutations_have_empty_rosters_and_reject_all_codec_input() {
        assert!(<NoConfigMutation as Mutation<NoConfig>>::DESCRIPTORS.is_empty());
        assert!(<NoPresenceMutation as Mutation<NoPresence>>::DESCRIPTORS.is_empty());
        assert!(<NoTransientMutation as Mutation<NoTransient>>::DESCRIPTORS.is_empty());
        for line in ["", "noop", "change revision 1"] {
            assert!(NoConfigMutation::parse_op(line).is_err());
            assert!(NoPresenceMutation::parse_op(line).is_err());
            assert!(NoTransientMutation::parse_op(line).is_err());
        }
        for bytes in [Vec::new(), vec![0], vec![255]] {
            assert!(NoConfigMutation::decode_op(&bytes).is_err());
            assert!(NoPresenceMutation::decode_op(&bytes).is_err());
            assert!(NoTransientMutation::decode_op(&bytes).is_err());
        }
    }

    #[test]
    fn publication_leaves_apply_inverse_preserve_identity_diff_and_expose_full_rosters() {
        let presence = ChangePublicationPresence { revision: 7 };
        let presence_before = PublicationPresence { revision: 3 };
        assert_eq!(PublicationPresenceDiff::default().apply(&presence_before).unwrap(), presence_before);
        let mut presence_diff = PublicationPresenceDiff::default();
        presence_diff.absorb(PublicationPresenceDiff { revision: Some(7) });
        assert_eq!(presence_diff.apply(&presence_before).unwrap(), PublicationPresence { revision: 7 });
        let presence_after = presence.diff(&presence_before).diff().apply(&presence_before).unwrap();
        assert_eq!(presence_after, PublicationPresence { revision: 7 });
        let [presence_inverse] = <[PublicationPresenceMutation; 1]>::try_from(presence.inverse(&presence_before)).unwrap();
        assert_eq!(presence_inverse.diff(&presence_after).diff().apply(&presence_after).unwrap(), presence_before);

        let transient = ChangePublicationTransient { revision: 11 };
        let transient_before = PublicationTransient { revision: 5 };
        assert_eq!(PublicationTransientDiff::default().apply(&transient_before).unwrap(), transient_before);
        let mut transient_diff = PublicationTransientDiff::default();
        transient_diff.absorb(PublicationTransientDiff { revision: Some(11) });
        assert_eq!(transient_diff.apply(&transient_before).unwrap(), PublicationTransient { revision: 11 });
        let transient_after = transient.diff(&transient_before).diff().apply(&transient_before).unwrap();
        assert_eq!(transient_after, PublicationTransient { revision: 11 });
        let [transient_inverse] = <[PublicationTransientMutation; 1]>::try_from(transient.inverse(&transient_before)).unwrap();
        assert_eq!(transient_inverse.diff(&transient_after).diff().apply(&transient_after).unwrap(), transient_before);

        assert_eq!(<PublicationPresenceMutation as Mutation<PublicationPresence>>::DESCRIPTORS, &[ChangePublicationPresence::DESCRIPTOR]);
        assert_eq!(<PublicationTransientMutation as Mutation<PublicationTransient>>::DESCRIPTORS, &[ChangePublicationTransient::DESCRIPTOR]);
    }

    #[test]
    fn publication_leaf_and_aggregate_codecs_are_exact_and_u64_serde_rejects_invalid_numbers() {
        let presence = ChangePublicationPresence { revision: u64::MAX };
        let transient = ChangePublicationTransient { revision: 9 };
        assert_eq!(ChangePublicationPresence::parse_op(&presence.print_op()).unwrap(), presence);
        assert_eq!(ChangePublicationTransient::parse_op(&transient.print_op()).unwrap(), transient);
        assert_eq!(PublicationPresenceMutation::parse_op(&presence.print_op()).unwrap(), PublicationPresenceMutation::from(presence.clone()));
        assert_eq!(PublicationTransientMutation::parse_op(&transient.print_op()).unwrap(), PublicationTransientMutation::from(transient.clone()));
        assert_eq!(ChangePublicationPresence::decode_op(&presence.encode_op().unwrap()).unwrap(), presence);
        assert_eq!(ChangePublicationTransient::decode_op(&transient.encode_op().unwrap()).unwrap(), transient);
        assert_eq!(PublicationPresenceMutation::decode_op(&presence.encode_op().unwrap()).unwrap(), PublicationPresenceMutation::from(presence.clone()));
        assert_eq!(PublicationTransientMutation::decode_op(&transient.encode_op().unwrap()).unwrap(), PublicationTransientMutation::from(transient.clone()));
        for line in ["change-publication-presence", "change-publication-presence -1", "change-publication-presence 1 2", "change-publication-transient 1"] {
            assert!(ChangePublicationPresence::parse_op(line).is_err());
            assert!(PublicationPresenceMutation::parse_op(line).is_err());
        }
        for line in ["change-publication-transient", "change-publication-transient -1", "change-publication-transient 1 2", "change-publication-presence 1"] {
            assert!(ChangePublicationTransient::parse_op(line).is_err());
            assert!(PublicationTransientMutation::parse_op(line).is_err());
        }
        for bytes in [Vec::new(), vec![ChangePublicationPresence::BINARY_TAG], vec![ChangePublicationTransient::BINARY_TAG, 0, 0, 0, 0, 0, 0, 0, 0], vec![ChangePublicationPresence::BINARY_TAG; 10]] {
            assert!(ChangePublicationPresence::decode_op(&bytes).is_err());
            assert!(PublicationPresenceMutation::decode_op(&bytes).is_err());
        }
        for bytes in [Vec::new(), vec![ChangePublicationTransient::BINARY_TAG], vec![ChangePublicationPresence::BINARY_TAG, 0, 0, 0, 0, 0, 0, 0, 0], vec![ChangePublicationTransient::BINARY_TAG; 10]] {
            assert!(ChangePublicationTransient::decode_op(&bytes).is_err());
            assert!(PublicationTransientMutation::decode_op(&bytes).is_err());
        }
        for payload in ["{\"revision\":0}", "{\"revision\":18446744073709551615}"] {
            assert!(serde_json::from_str::<PublicationPresence>(payload).is_ok());
            assert!(serde_json::from_str::<PublicationTransient>(payload).is_ok());
        }
        for payload in ["{\"revision\":18446744073709551616}", "{\"revision\":-1}", "{\"revision\":1.5}", "{\"revision\":\"1\"}", "{\"revision\":1,\"other\":2}"] {
            assert!(serde_json::from_str::<PublicationPresence>(payload).is_err());
            assert!(serde_json::from_str::<PublicationTransient>(payload).is_err());
        }
    }
}
