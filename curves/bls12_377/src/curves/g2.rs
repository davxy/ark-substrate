use ark_ec::{
    models::{bls12, CurveConfig},
    short_weierstrass::{Affine as SWAffine, Projective as SWProjective, SWCurveConfig},
};
use ark_ff::{Field, MontFp};
use ark_serialize::{Compress, Validate};
use ark_std::{io::Cursor, marker::PhantomData, vec::Vec};
use sp_ark_utils::serialize_argument;

use crate::{g1, Fq, Fq2, Fr, HostFunctions};

pub type G2Affine<H> = bls12::G2Affine<crate::Config<H>>;
pub type G2Projective<H> = bls12::G2Projective<crate::Config<H>>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Config<H: HostFunctions>(PhantomData<H>);

impl<H: HostFunctions> CurveConfig for Config<H> {
    type BaseField = Fq2;
    type ScalarField = Fr;

    /// COFACTOR =
    /// 7923214915284317143930293550643874566881017850177945424769256759165301436616933228209277966774092486467289478618404761412630691835764674559376407658497
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x0000000000000001,
        0x452217cc90000000,
        0xa0f3622fba094800,
        0xd693e8c36676bd09,
        0x8c505634fae2e189,
        0xfbb36b00e1dcc40c,
        0xddd88d99a6f6a829,
        0x26ba558ae9562a,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 6764900296503390671038341982857278410319949526107311149686707033187604810669
    const COFACTOR_INV: Fr =
        MontFp!("6764900296503390671038341982857278410319949526107311149686707033187604810669");
}

impl<H: HostFunctions> SWCurveConfig for Config<H> {
    /// COEFF_A = [0, 0]
    const COEFF_A: Fq2 = Fq2::new(g1::Config::<H>::COEFF_A, g1::Config::<H>::COEFF_A);

    // As per https://eprint.iacr.org/2012/072.pdf,
    // this curve has b' = b/i, where b is the COEFF_B of G1, and x^6 -i is
    // the irreducible poly used to extend from Fp2 to Fp12.
    // In our case, i = u (App A.3, T_6).
    /// COEFF_B = [0,
    /// 155198655607781456406391640216936120121836107652948796323930557600032281009004493664981332883744016074664192874906]
    const COEFF_B: Fq2 = Fq2::new(
        Fq::ZERO,
        MontFp!("155198655607781456406391640216936120121836107652948796323930557600032281009004493664981332883744016074664192874906"),
    );

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const GENERATOR: G2Affine<H> = G2Affine::<H>::new_unchecked(G2_GENERATOR_X, G2_GENERATOR_Y);

    // #[inline(always)]
    // fn mul_by_a(_: Self::BaseField) -> Self::BaseField {
    //     Self::BaseField::zero()
    // }

    fn msm(
        bases: &[SWAffine<Self>],
        scalars: &[<Self as CurveConfig>::ScalarField],
    ) -> Result<SWProjective<Self>, usize> {
        let bases: Vec<Vec<u8>> = bases.iter().map(|elem| serialize_argument(*elem)).collect();
        let scalars: Vec<Vec<u8>> = scalars
            .iter()
            .map(|elem| serialize_argument(*elem))
            .collect();

        let result = H::bls12_377_msm_g2(bases, scalars);

        let cursor = Cursor::new(&result[..]);
        let result = <Config<H> as SWCurveConfig>::deserialize_with_mode(
            cursor,
            Compress::Yes,
            Validate::No,
        )
        .unwrap();
        Ok(result.into())
    }

    fn mul_projective(base: &SWProjective<Self>, scalar: &[u64]) -> SWProjective<Self> {
        let serialized_base = serialize_argument(*base);
        let serialized_scalar = serialize_argument(scalar);

        let result = H::bls12_377_mul_projective_g2(serialized_base, serialized_scalar);

        let cursor = Cursor::new(&result[..]);
        let result = Self::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
        result.into()
    }

    fn mul_affine(base: &SWAffine<Self>, scalar: &[u64]) -> SWProjective<Self> {
        let serialized_base = serialize_argument(*base);
        let serialized_scalar = serialize_argument(scalar);

        let result = H::bls12_377_mul_affine_g2(serialized_base, serialized_scalar);

        let cursor = Cursor::new(&result[..]);
        let result = Self::deserialize_with_mode(cursor, Compress::Yes, Validate::No).unwrap();
        result.into()
    }
}

pub const G2_GENERATOR_X: Fq2 = Fq2::new(G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
pub const G2_GENERATOR_Y: Fq2 = Fq2::new(G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

/// G2_GENERATOR_X_C0 =
/// 233578398248691099356572568220835526895379068987715365179118596935057653620464273615301663571204657964920925606294
pub const G2_GENERATOR_X_C0: Fq = MontFp!("233578398248691099356572568220835526895379068987715365179118596935057653620464273615301663571204657964920925606294");

/// G2_GENERATOR_X_C1 =
/// 140913150380207355837477652521042157274541796891053068589147167627541651775299824604154852141315666357241556069118
pub const G2_GENERATOR_X_C1: Fq = MontFp!("140913150380207355837477652521042157274541796891053068589147167627541651775299824604154852141315666357241556069118");

/// G2_GENERATOR_Y_C0 =
/// 63160294768292073209381361943935198908131692476676907196754037919244929611450776219210369229519898517858833747423
pub const G2_GENERATOR_Y_C0: Fq = MontFp!("63160294768292073209381361943935198908131692476676907196754037919244929611450776219210369229519898517858833747423");

/// G2_GENERATOR_Y_C1 =
/// 149157405641012693445398062341192467754805999074082136895788947234480009303640899064710353187729182149407503257491
pub const G2_GENERATOR_Y_C1: Fq = MontFp!("149157405641012693445398062341192467754805999074082136895788947234480009303640899064710353187729182149407503257491");
