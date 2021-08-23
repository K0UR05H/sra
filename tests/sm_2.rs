use sra::sm_2::{Quality, SM2};

#[test]
fn review() {
    let item = SM2::new()
        .set_repetitions(3)
        .set_interval(5)
        .set_ease_factor(2.0)
        .review(Quality::Grade2);

    assert_eq!(0, item.repetitions());
    assert_eq!(1, item.interval());
    assert_eq!(2.0, item.ease_factor());
}
