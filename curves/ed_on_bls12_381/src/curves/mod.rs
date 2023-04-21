use crate::{Fq, Fr};
use ark_ff::MontFp;
use ark_scale::hazmat::ArkScaleProjective;
use ark_std::{marker::PhantomData, vec::Vec};
use codec::{Decode, Encode};
use sp_ark_models::{
    short_weierstrass::{self, SWCurveConfig},
    twisted_edwards::{Affine, MontCurveConfig, Projective, TECurveConfig},
    CurveConfig,
};

const HOST_CALL: ark_scale::Usage = ark_scale::HOST_CALL;
type ArkScale<T> = ark_scale::ArkScale<T, HOST_CALL>;

#[cfg(test)]
mod tests;

pub type EdwardsAffine<H> = Affine<JubjubConfig<H>>;
pub type EdwardsProjective<H> = Projective<JubjubConfig<H>>;
pub type SWAffine<H> = short_weierstrass::Affine<JubjubConfig<H>>;
pub type SWProjective<H> = short_weierstrass::Projective<JubjubConfig<H>>;

/// `JubJub` is a twisted Edwards curve. These curves have equations of the
/// form: ax² + y² = 1 - dx²y².
/// over some base finite field Fq.
///
/// JubJub's curve equation: -x² + y² = 1 - (10240/10241)x²y²
///
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513.
///
/// a = -1.
/// d = -(10240/10241) mod q
///   = 19257038036680949359750312669786877991949435402254120286184196891950884077233.
///
/// Sage script to calculate these:
///
/// ```text
/// q = 52435875175126190479447740508185965837690552500527637822603658699938581184513
/// Fq = GF(q)
/// d = -(Fq(10240)/Fq(10241))
/// ```
/// These parameters and the sage script obtained from:
/// <https://github.com/zcash/zcash/issues/2230#issuecomment-317182190>
///
///
/// `jubjub` also has a short Weierstrass curve form, following the
/// form: y² = x³ + A * x + B
/// where
///
/// A = 52296097456646850916096512823759002727550416093741407922227928430486925478210
/// B = 48351165704696163914533707656614864561753505123260775585269522553028192119009
///
/// We can use the script available
/// [here](https://github.com/zhenfeizhang/bandersnatch/blob/main/bandersnatch/script/jubjub.sage)
/// to convert between the different representations.
///
/// #[derive(Clone, Default, PartialEq, Eq)]

#[derive(Clone, Default, PartialEq, Eq)]
pub struct JubjubConfig<H: HostFunctions>(PhantomData<fn() -> H>);
pub type EdwardsConfig<H> = JubjubConfig<H>;
pub type SWConfig<H> = JubjubConfig<H>;

pub trait HostFunctions: 'static {
    fn ed_on_bls12_381_te_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_381_sw_msm(bases: Vec<u8>, scalars: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_381_te_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
    fn ed_on_bls12_381_sw_mul_projective(base: Vec<u8>, scalar: Vec<u8>) -> Result<Vec<u8>, ()>;
}

impl<H: HostFunctions> CurveConfig for JubjubConfig<H> {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR = 8
    const COFACTOR: &'static [u64] = &[8];

    /// COFACTOR^(-1) mod r =
    /// 819310549611346726241370945440405716213240158234039660170669895299022906775
    const COFACTOR_INV: Fr =
        MontFp!("819310549611346726241370945440405716213240158234039660170669895299022906775");
}

impl<H: HostFunctions> TECurveConfig for JubjubConfig<H> {
    /// COEFF_A = -1
    const COEFF_A: Fq = MontFp!("-1");

    /// COEFF_D = -(10240/10241) mod q
    const COEFF_D: Fq =
        MontFp!("19257038036680949359750312669786877991949435402254120286184196891950884077233");

    /// AFFINE_GENERATOR_COEFFS = (GENERATOR_X, GENERATOR_Y)
    const GENERATOR: EdwardsAffine<H> = EdwardsAffine::<H>::new_unchecked(GENERATOR_X, GENERATOR_Y);

    type MontCurveConfig = JubjubConfig<H>;

    /// Multiplication by `a` is simply negation here.
    #[inline(always)]
    fn mul_by_a(elem: Self::BaseField) -> Self::BaseField {
        -elem
    }

    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[Affine<Self>]> = bases.into();
        let scalars: ArkScale<&[<Self as CurveConfig>::ScalarField]> = scalars.into();

        let result = H::ed_on_bls12_381_te_msm(bases.encode(), scalars.encode()).unwrap();

        let result = <ArkScaleProjective<Projective<JubjubConfig<H>>> as Decode>::decode(
            &mut result.as_slice(),
        );
        result.map_err(|_| 0).map(|res| res.0)
    }

    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result = H::ed_on_bls12_381_te_mul_projective(base.encode(), scalar.encode()).unwrap();

        let result =
            <ArkScaleProjective<Projective<Self>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }

    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: Projective<Self> = (*base).into();
        let base: ArkScaleProjective<Projective<Self>> = base.into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result = H::ed_on_bls12_381_te_mul_projective(base.encode(), scalar.encode()).unwrap();

        let result =
            <ArkScaleProjective<Projective<Self>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }
}

impl<H: HostFunctions> MontCurveConfig for JubjubConfig<H> {
    /// COEFF_A = 40962
    const COEFF_A: Fq = MontFp!("40962");

    /// COEFF_B = -40964
    const COEFF_B: Fq = MontFp!("-40964");

    type TECurveConfig = JubjubConfig<H>;
}

const GENERATOR_X: Fq =
    MontFp!("8076246640662884909881801758704306714034609987455869804520522091855516602923");

const GENERATOR_Y: Fq =
    MontFp!("13262374693698910701929044844600465831413122818447359594527400194675274060458");

impl<H: HostFunctions> SWCurveConfig for JubjubConfig<H> {
    /// COEFF_A = 52296097456646850916096512823759002727550416093741407922227928430486925478210
    const COEFF_A: Self::BaseField =
        MontFp!("52296097456646850916096512823759002727550416093741407922227928430486925478210");

    /// COEFF_B = 48351165704696163914533707656614864561753505123260775585269522553028192119009
    const COEFF_B: Self::BaseField =
        MontFp!("48351165704696163914533707656614864561753505123260775585269522553028192119009");

    /// generators
    const GENERATOR: SWAffine<H> = SWAffine::<H>::new_unchecked(SW_GENERATOR_X, SW_GENERATOR_Y);

    fn msm(
        bases: &[SWAffine<H>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<SWProjective<H>, usize> {
        let bases: ArkScale<&[SWAffine<H>]> = bases.into();
        let scalars: ArkScale<&[<Self as CurveConfig>::ScalarField]> = scalars.into();

        let result = H::ed_on_bls12_381_sw_msm(bases.encode(), scalars.encode()).unwrap();

        let result = <ArkScaleProjective<
            sp_ark_models::short_weierstrass::Projective<JubjubConfig<H>>,
        > as Decode>::decode(&mut result.as_slice());
        result.map_err(|_| 0).map(|res| res.0)
    }

    fn mul_projective(base: &SWProjective<H>, scalar: &[u64]) -> SWProjective<H> {
        let base: ArkScaleProjective<SWProjective<H>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result = H::ed_on_bls12_381_sw_mul_projective(base.encode(), scalar.encode()).unwrap();

        let result =
            <ArkScaleProjective<SWProjective<H>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }

    fn mul_affine(base: &SWAffine<H>, scalar: &[u64]) -> SWProjective<H> {
        let base: SWProjective<H> = (*base).into();
        let base: ArkScaleProjective<SWProjective<H>> = base.into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result = H::ed_on_bls12_381_sw_mul_projective(base.encode(), scalar.encode()).unwrap();

        let result =
            <ArkScaleProjective<SWProjective<H>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }
}

/// x coordinate for SW curve generator
const SW_GENERATOR_X: Fq =
    MontFp!("33835869156188682335217394949746694649676633840125476177319971163079011318731");

/// y coordinate for SW curve generator
const SW_GENERATOR_Y: Fq =
    MontFp!("43777270878440091394432848052353307184915192688165709016756678962558652055320");
