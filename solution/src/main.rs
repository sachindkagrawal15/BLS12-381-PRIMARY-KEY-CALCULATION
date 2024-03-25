use lambdaworks_math::{
    elliptic_curve::{
        short_weierstrass::curves::bls12_381::curve::BLS12381Curve,
        traits::IsEllipticCurve,
    },
    msm::naive::msm, 
    traits::ByteConversion,
};

fn main() {

    let private_key: u128 = u128::from_str_radix("6C616D6264617370", 16).unwrap();
    let g = BLS12381Curve::generator();
    let scalars = vec![private_key];
    let base_points = vec![g];
    let public_key_points = msm(&scalars, &base_points);
    let affine_point = public_key_points.unwrap().to_affine();
    let x_bytes = affine_point.x().to_bytes_be();
    let y_bytes = affine_point.y().to_bytes_be();

    let mut new_public_key_bytes = Vec::new();
    new_public_key_bytes.extend_from_slice(&x_bytes);
    new_public_key_bytes.extend_from_slice(&y_bytes);

    let public_key_bytes = hex::encode(new_public_key_bytes);

    println!("Public Key is {:?} for Private Key {:?} for BLS12-381 curve ", public_key_bytes, private_key); 

}