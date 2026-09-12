
use super::*;

#[test]
fn loss_flag_round_trips_every_reason() {
    assert_eq!(decode_loss_flag(LOSS_NONE), None);
    assert_eq!(decode_loss_flag(LOSS_SURFACE), Some(LossReason::Surface));
    assert_eq!(decode_loss_flag(LOSS_DEVICE), Some(LossReason::Device));
    assert_eq!(decode_loss_flag(LOSS_TIMEOUT), Some(LossReason::Timeout));
}

#[cfg(feature = "backend-testing")]
#[test]
fn padded_row_rounds_up_to_alignment() {
    assert_eq!(padded_bytes_per_row(1), 256);
    assert_eq!(padded_bytes_per_row(64), 256);
    assert_eq!(padded_bytes_per_row(65), 512);
    assert_eq!(padded_bytes_per_row(256), 1024);
}
