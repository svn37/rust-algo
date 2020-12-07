pub fn is_prime(num: usize) -> bool {
    if num < 2 {
        return false;
    }
    if num < 4 {
        return true;
    }
    if num % 2 == 0 {
        return false;
    }
    for i in (3..)
        .step_by(2)
        .take_while(|i| *i <= (num as f64).sqrt().floor() as usize)
    {
        if num % i == 0 {
            return false;
        }
    }
    true
}

#[test]
fn test_is_prime() {
    vec![
        2usize,
        3,
        5,
        7,
        11,
        13,
        17,
        19,
        23,
        29,
        31,
        37,
        41,
        43,
        47,
        53,
        59,
        61,
        67,
        71,
        73,
        79,
        83,
        89,
        97,
        101,
        103,
        107,
        109,
        113,
        127,
        131,
        137,
        139,
        149,
        151,
        157,
        163,
        167,
        173,
        179,
        181,
        191,
        193,
        197,
        199,
        9901,
        9907,
        9923,
        9929,
        9931,
        9941,
        9949,
        9967,
        9973,
        693008464254389,
        717157641478073,
        104406121001953,
    ]
    .iter()
    .for_each(|&num| assert!(is_prime(num)))
}
