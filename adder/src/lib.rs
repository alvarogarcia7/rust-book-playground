#![feature(test)]

extern crate test;

pub fn add2(n: i32) -> i32 {
    n + n
}

#[cfg(test)]
mod tests {
    use test::Bencher;
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[bench]
    fn bench1(b: &mut Bencher) {
        let n = test::black_box(10000);

        let a = b.iter(|| {
            (0..n).fold(0, |old, new| add2(old) + add2(new))
        });
        println!("{:?}", a);
    }

    #[test]
    fn another() {
        panic!("Make this test fail!");      
    }
}
