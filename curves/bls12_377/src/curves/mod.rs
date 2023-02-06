use ark_bls12_377::Config as OrgConfig;
use ark_ec::{
    bls12::{Bls12, Bls12Config, G1Prepared, G2Prepared, TwistType},
    pairing::{MillerLoopOutput, Pairing, PairingOutput},
};
use ark_ff::Fp12;
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use ark_std::{io::Cursor, marker::PhantomData, vec::Vec};
use sp_ark_utils::serialize_argument;

pub mod g1;
pub mod g2;

#[cfg(test)]
mod tests;

pub use self::{
    g1::{G1Affine, G1Projective},
    g2::{G2Affine, G2Projective},
};

pub trait HostFunctions: Send + Sync + 'static {
    fn bls12_377_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Vec<u8>;
    fn bls12_377_msm_g1(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bls12_377_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bls12_377_msm_g2(bases: Vec<Vec<u8>>, scalars: Vec<Vec<u8>>) -> Vec<u8>;
    fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
    fn bls12_377_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8>;
}

pub struct Config<H: HostFunctions>(PhantomData<H>);

pub type Bls12_377<H> = Bls12<Config<H>>;

impl<H: HostFunctions> Bls12Config for Config<H> {
    type Fp = <OrgConfig as Bls12Config>::Fp;
    type Fp2Config = <OrgConfig as Bls12Config>::Fp2Config;
    type Fp6Config = <OrgConfig as Bls12Config>::Fp6Config;
    type Fp12Config = <OrgConfig as Bls12Config>::Fp12Config;

    type G1Config = g1::Config<H>;
    type G2Config = g2::Config<H>;

    const X: &'static [u64] = <OrgConfig as Bls12Config>::X;
    const X_IS_NEGATIVE: bool = <OrgConfig as Bls12Config>::X_IS_NEGATIVE;
    const TWIST_TYPE: TwistType = <OrgConfig as Bls12Config>::TWIST_TYPE;

    fn multi_miller_loop(
        a: impl IntoIterator<Item = impl Into<G1Prepared<Self>>>,
        b: impl IntoIterator<Item = impl Into<G2Prepared<Self>>>,
    ) -> MillerLoopOutput<Bls12<Self>> {
        let a: Vec<Vec<u8>> = a
            .into_iter()
            .map(|elem| {
                let elem: <Bls12<Self> as Pairing>::G1Prepared = elem.into();
                serialize_argument(elem)
            })
            .collect();

        let b: Vec<Vec<u8>> = b
            .into_iter()
            .map(|elem| Self::serialize_as_affine(elem))
            .collect();

        let result = H::bls12_377_multi_miller_loop(a, b);

        let cursor = Cursor::new(&result[..]);
        let f: <Bls12<Self> as Pairing>::TargetField =
            Fp12::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
        MillerLoopOutput(f)
    }

    fn final_exponentiation(
        f: MillerLoopOutput<Bls12<Self>>,
    ) -> Option<PairingOutput<Bls12<Self>>> {
        let serialized_target = serialize_argument(f.0);

        let result = H::bls12_377_final_exponentiation(serialized_target);

        let cursor = Cursor::new(&result[..]);
        let result = PairingOutput::<Bls12<Self>>::deserialize_with_mode(
            cursor,
            Compress::Yes,
            Validate::No,
        )
        .unwrap();

        Some(result)
    }
}

impl<H: HostFunctions> Config<H> {
    // Hack to serialize all the expected types into affine.
    // `G2Prepared` conversion is performed into the host function for efficiency.
    fn serialize_as_affine(elem: impl Into<G2Prepared<Self>>) -> Vec<u8> {
        use ark_ec::CurveGroup;
        use core::any;

        fn type_name_of<T>(_: &T) -> &str {
            core::any::type_name::<T>()
        }

        // Hack to catch one of the expected types
        // TODO: is there a better way?
        let type_name = type_name_of(&elem);
        let affine = if type_name == any::type_name::<G2Projective<H>>() {
            let proj: &G2Projective<H> = unsafe { core::mem::transmute(&elem) };
            proj.into_affine()
        } else if type_name == any::type_name::<&G2Projective<H>>() {
            let proj: &&G2Projective<H> = unsafe { core::mem::transmute(&elem) };
            proj.into_affine()
        } else if type_name == any::type_name::<G2Affine<H>>() {
            let affine: &G2Affine<H> = unsafe { core::mem::transmute(&elem) };
            *affine
        } else if type_name == any::type_name::<&G2Affine<H>>() {
            let affine: &&G2Affine<H> = unsafe { core::mem::transmute(&elem) };
            **affine
        } else {
            panic!("Unhandled type: {}", type_name);
        };

        serialize_argument(affine)
    }
}
