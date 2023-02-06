#![cfg_attr(not(feature = "std"), no_std)]
use crate::HostFunctions;
use ark_algebra_test_templates::*;
use ark_std::vec::Vec;

pub struct Host;

impl HostFunctions for Host {
    fn bls12_377_multi_miller_loop(a: Vec<Vec<u8>>, b: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_multi_miller_loop(a, b)
    }
    fn bls12_377_final_exponentiation(f12: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_final_exponentiation(f12)
    }
    fn bls12_377_msm_g1(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_msm_g1(bases, bigints)
    }
    fn bls12_377_mul_projective_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_mul_projective_g1(base, scalar)
    }
    fn bls12_377_mul_affine_g1(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_mul_affine_g1(base, scalar)
    }
    fn bls12_377_msm_g2(bases: Vec<Vec<u8>>, bigints: Vec<Vec<u8>>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_msm_g2(bases, bigints)
    }
    fn bls12_377_mul_projective_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_mul_projective_g2(base, scalar)
    }
    fn bls12_377_mul_affine_g2(base: Vec<u8>, scalar: Vec<u8>) -> Vec<u8> {
        sp_io::elliptic_curves::bls12_377_mul_affine_g2(base, scalar)
    }
}

test_group!(g1; crate::G1Projective<super::Host>; sw);
test_group!(g2; crate::G2Projective<super::Host>; sw);
test_group!(pairing_output; ark_ec::pairing::PairingOutput<crate::Bls12_377<super::Host>>; msm);
test_pairing!(pairing; crate::Bls12_377<super::Host>);
