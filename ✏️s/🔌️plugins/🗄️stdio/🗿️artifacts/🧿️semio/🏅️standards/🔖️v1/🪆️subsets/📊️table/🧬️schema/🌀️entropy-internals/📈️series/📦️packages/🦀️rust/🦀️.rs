//! 📈️ Series complexity measures: regularity, ordinal and Markov entropies, multiscale and Lempel-Ziv complexity, spectra, wavelets and resampling inference.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

#[path = "."]
pub mod schema {
    #[path = "."]
    pub mod entropy_internals {
        #[path = "../../🔁️regularity/🦀️.rs"]
        pub mod regularity;
        #[path = "../../📶️ordinal/🦀️.rs"]
        pub mod ordinal;
        #[path = "../../⛓️markov/🦀️.rs"]
        pub mod markov;
        #[path = "../../🔍️multiscale/🦀️.rs"]
        pub mod multiscale;
        #[path = "../../🗜️lz/🦀️.rs"]
        pub mod lz;
        #[path = "../../🌊️fft/🦀️.rs"]
        pub mod fft;
        #[path = "../../🌈️spectral/🦀️.rs"]
        pub mod spectral;
        #[path = "../../〽️wavelet/🦀️.rs"]
        pub mod wavelet;
        #[path = "../../🧐️inference/🦀️.rs"]
        pub mod inference;
    }
}
