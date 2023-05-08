use ark_ff::{Field, MontFp};
use ark_scale::hazmat::ArkScaleProjective;
use ark_std::marker::PhantomData;
use codec::{Decode, Encode};
use sp_ark_models::{
    bw6,
    short_weierstrass::{Affine, Projective},
    {short_weierstrass::SWCurveConfig, CurveConfig},
};

use crate::{ArkScale, Fq, Fr, HostFunctions};

pub type G1Affine<H> = bw6::G1Affine<crate::Config<H>>;
pub type G1Projective<H> = bw6::G1Projective<crate::Config<H>>;

#[derive(Clone, Default, PartialEq, Eq)]

pub struct Config<H: HostFunctions>(PhantomData<fn() -> H>);

impl<H: HostFunctions> CurveConfig for Config<H> {
    type BaseField = Fq;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 26642435879335816683987677701488073867751118270052650655942102502312977592501693353047140953112195348280268661194876
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x3de580000000007c,
        0x832ba4061000003b,
        0xc61c554757551c0c,
        0xc856a0853c9db94c,
        0x2c77d5ac34cb12ef,
        0xad1972339049ce76,
    ];

    /// COFACTOR^(-1) mod r =
    /// 91141326767669940707819291241958318717982251277713150053234367522357946997763584490607453720072232540829942217804
    const COFACTOR_INV: Fr = MontFp!("91141326767669940707819291241958318717982251277713150053234367522357946997763584490607453720072232540829942217804");
}

impl<H: HostFunctions> SWCurveConfig for Config<H> {
    /// COEFF_A = 0
    const COEFF_A: Fq = Fq::ZERO;

    /// COEFF_B = -1
    const COEFF_B: Fq = MontFp!("-1");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const GENERATOR: Affine<Self> = Affine::<Self>::new_unchecked(G1_GENERATOR_X, G1_GENERATOR_Y);
    #[inline(always)]
    fn mul_by_a(_elem: Self::BaseField) -> Self::BaseField {
        use ark_ff::Zero;
        Self::BaseField::zero()
    }

    fn msm(
        bases: &[Affine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<Projective<Self>, usize> {
        let bases: ArkScale<&[Affine<Self>]> = bases.into();
        let scalars: ArkScale<&[<Self as CurveConfig>::ScalarField]> = scalars.into();

        let result = H::bw6_761_msm_g1(bases.encode(), scalars.encode()).unwrap();

        let result =
            <ArkScaleProjective<Projective<Self>> as Decode>::decode(&mut result.as_slice());
        result.map_err(|_| 0).map(|res| res.0)
    }

    fn mul_projective(base: &Projective<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: ArkScaleProjective<Projective<Self>> = (*base).into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result = H::bw6_761_mul_projective_g1(base.encode(), scalar.encode()).unwrap();

        let result =
            <ArkScaleProjective<Projective<Self>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }

    fn mul_affine(base: &Affine<Self>, scalar: &[u64]) -> Projective<Self> {
        let base: Projective<Self> = (*base).into();
        let base: ArkScaleProjective<Projective<Self>> = base.into();
        let scalar: ArkScale<&[u64]> = scalar.into();

        let result = H::bw6_761_mul_projective_g1(base.encode(), scalar.encode()).unwrap();

        let result =
            <ArkScaleProjective<Projective<Self>> as Decode>::decode(&mut result.as_slice());
        result.unwrap().0
    }
}

/// G1_GENERATOR_X =
/// 6238772257594679368032145693622812838779005809760824733138787810501188623461307351759238099287535516224314149266511977132140828635950940021790489507611754366317801811090811367945064510304504157188661901055903167026722666149426237
pub const G1_GENERATOR_X: Fq = MontFp!("6238772257594679368032145693622812838779005809760824733138787810501188623461307351759238099287535516224314149266511977132140828635950940021790489507611754366317801811090811367945064510304504157188661901055903167026722666149426237");

/// G1_GENERATOR_Y =
/// 2101735126520897423911504562215834951148127555913367997162789335052900271653517958562461315794228241561913734371411178226936527683203879553093934185950470971848972085321797958124416462268292467002957525517188485984766314758624099
pub const G1_GENERATOR_Y: Fq = MontFp!("2101735126520897423911504562215834951148127555913367997162789335052900271653517958562461315794228241561913734371411178226936527683203879553093934185950470971848972085321797958124416462268292467002957525517188485984766314758624099");
