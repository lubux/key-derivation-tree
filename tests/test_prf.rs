
use constrained_prf::prf::ConstrainedPrf;

#[test]
fn test_constrained_key_der1() {
    let key = [0u8; 16];
    let prf = ConstrainedPrf::init(4, key);
    let key_out1 = prf.apply(2).unwrap();
    let cons = prf.constrain(0, 16).unwrap();
    let prf2 = ConstrainedPrf::new(4, cons);
    let key_out2 = prf2.apply(2).unwrap();
    assert_eq!(&key_out1[..], &key_out2[..], "Keys are not equal");
}

#[test]
fn test_constrained_key_der2() {
    let key = [0u8; 16];
    let prf = ConstrainedPrf::init(4, key);
    let cons = prf.constrain(1, 15).unwrap();
    let prf2 = ConstrainedPrf::new(4, cons);

    for i in 1..15 {
        let key_out1 = prf.apply(i as u64).unwrap();
        let key_out2 = prf2.apply(i as u64).unwrap();
        assert_eq!(&key_out1[..], &key_out2[..], "Keys are not equal");
    } 
    
}

#[test]
fn test_single_aes_der() {
    let key = [0u8; 16];
    let sol : [u8; 16] = [102, 233, 75, 212, 239, 138, 44, 59, 136, 76, 250, 89, 202, 52, 43, 46];
    let prf = ConstrainedPrf::init(1, key);
    let key_out1 = prf.apply(0).unwrap();
    assert_eq!(&key_out1[..], &sol[..], "Keys are not equal");
}


#[test]
fn test_constrain1() {
    let key = [0u8; 16];
    let prf = ConstrainedPrf::init(4, key);
    let cons = prf.constrain(1, 15).unwrap();
    assert_eq!(cons.len(), 6, "Not enough nodes extracted");
}

#[test]
fn test_constrain2() {
    let key = [0u8; 16];
    let prf = ConstrainedPrf::init(4, key);
    let cons = prf.constrain(1, 15).unwrap();
    let prf2 = ConstrainedPrf::new(4, cons);
    let key_out = prf2.apply(0);
    match key_out {
        Ok(_) => assert!(false, "Key should not be derivable"),
        Err(_) => assert!(true),
    }
}
