fn get_grade(s1: u16, s2: u16, s3: u16) -> char {
    // let average: u16 = (s1 + s2 + s3) / 3;
    // let grade: char = match average {
    //     a if (a <= 100 && a >= 90) => 'A',
    //     a if (a < 90 && a >= 80) => 'B',
    //     a if (a < 80 && a >= 70) => 'C',
    //     a if (a < 70 && a >= 60) => 'D',
    //     _ => 'F',
    // };

    // grade

    match (s1 + s2 + s3) / 3 {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => unreachable!()
    }
}

fn main() {
    println!("{:?}", get_grade(82, 85, 87));
}