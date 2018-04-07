extern crate test;

fn print_str(s: &str) {
    println!("{:p}", s);
}

fn print_string(s: &String) {
    println!("{:p}", s);
}

#[cfg(test)]
mod tests {
    use super::*;
    use self::test::Bencher;

    #[bench]
    fn bench_string_to_str(b: &mut Bencher) {
        let s = "hoge".to_string();
        b.iter(|| print_str(&s) );
    }

    #[bench]
    fn bench_string_to_string(b: &mut Bencher) {
        let s = "hoge".to_string();
        b.iter(|| print_string(&s) );
    }

    #[bench]
    fn bench_str_to_str(b: &mut Bencher) {
        let s = "hoge";
        b.iter(|| print_str(&s) );
    }

    #[bench]
    fn bench_str_to_string(b: &mut Bencher) {
        let s = "hoge";
        b.iter(|| print_string(&s.to_string()) );
    }
}
