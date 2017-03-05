fn main() {
    let a = 2.5;
    for i in 1..11 {
        let t = (i * 5) as f32;
        // let R = a * ((1.0 + (3.0 * t / (3.1415 * a * a) / 1.2)  (2.0 / 3.0)))  0.5;
        let x1 = 3.0 * t / (3.1415 * a * a) / 1.2;
        let x2 = x1 * x1;
        let x3 = mi(x2, 3) + 1.0;
        let R = a * mi(x3, 2);
        println!("{:?}", R);
    }
}

fn mi(di: f32, n: u32) -> f32 {
    let mut a = 0.001;
    loop {
        a += 0.001;
        let mut ji = 1.0;
        for _ in 0..n {
            ji *= a;
        }
        if (ji - di <= 0.01 && ji - di >= -0.01) {
            return a;
        }
    }
}
