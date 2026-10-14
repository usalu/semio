//! 🧪️ Shared test vocabulary and step views for the token-sampling crates' unit tests (dev-dependency only).

use semio_framework_math_sampling_vocabulary::{SequenceId, StepIndex, TokenId, Vocabulary};
use semio_framework_math_sampling_workspace::{StepView};

pub async fn small_vocab() -> Vocabulary {
    Vocabulary::new(8).await.with_eos(vec![TokenId::new(7)]).await
}

pub async fn step_view<'a>(vocab: &'a Vocabulary, prompt: &'a [TokenId], generated: &'a [TokenId]) -> StepView<'a> {
    StepView { sequence: SequenceId::new(1), step: StepIndex::new(generated.len() as u32), prompt, generated, vocab, adapter: None, last_entropy: None }
}
