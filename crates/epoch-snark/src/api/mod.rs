mod prover;
pub use prover::prove;

mod setup;
pub use setup::{trusted_setup, Parameters};

mod verifier;
pub use verifier::{verify, VerificationError};

// Instantiate certain types to avoid confusion
use algebra::{bls12_377, bw6_761};
pub type BLSCurve = bls12_377::Bls12_377;
type CPField = bw6_761::Fr;
pub type CPCurve = bw6_761::BW6_761;
type CPFrParams = bw6_761::FrParameters;
