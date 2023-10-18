fn main() {
    let guess: u32 = "42".parse().expect("not a number");
    println!("number: {}", guess);

    let a: i8 = 0o77;
    let b: u8 = b'A';
    let c: isize = 0b1111_0000;
    let d: usize = 5;
    println!("i8: {}", a);
    println!("u8: {}", b);
    println!("isize: {}", c);
    println!("usize: {}", d);

    let e: f32 = 2.0;
    let f: f64 = -9.8;
    println!("f32: {}", e);
    println!("f64: {}", f);

    let z1 = 'z';
    let z2 = 'ℤ';
    let heart_eyed_cat = '😻';    //ハート目の猫
    println!("z1: {}", z1);
    println!("z2: {}", z2);
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    let tup: (i32, f64, char) = (500, 6.4, '壱');
    let (x, y, z) = tup;
    println!("x, y, z: {}, {}, {}", x, y, z);
    println!("x, y, z: {}, {}, {}", tup.0, tup.1, tup.2);

    let a = [3; 5];
    println!("array: {}, {}, {}, {}, {}", a[0], a[1], a[2], a[3], a[4]);
}
