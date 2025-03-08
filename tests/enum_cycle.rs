use deki::*;

#[derive(Cycle,PartialEq,Debug)]
enum Number {One, Two, Three}

#[test]
fn test_enum_cycle(){
    let one = Number::One;
    assert_eq!(Number::Two,one.cycle_next());
    assert_eq!(Number::Three,one.cycle_prev());
    let two = one.cycle_next();
    assert_eq!(Number::Three,two.cycle_next());
}
