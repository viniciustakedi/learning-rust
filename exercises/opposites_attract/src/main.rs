fn lovefunc(flower1: u16, flower2: u16) -> bool {
    let flower1_is_odd: bool = flower1 % 2 != 0;
    let flower2_is_odd: bool = flower2 % 2 != 0;

    if flower1_is_odd != flower2_is_odd {
        return true;
    } else {
        return false;
    }
}

fn main() {
    println!("{}", lovefunc(1, 4));
}
