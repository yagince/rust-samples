// #![feature(conservative_impl_trait)]
    // let boxed = boxed_fn(3);
    // println!("{}", boxed(10));
    // println!("{}", boxed(100));
    // println!("{}", boxed(20));

    // let trait_fn = impl_trait_fn(3);
    // println!("{}", trait_fn(10));
    // println!("{}", trait_fn(100));
    // println!("{}", trait_fn(20));
    // let a = format!("{}", "hoge");
    // hoge(&a);
    // hoge(a.as_ref());
    // hoge(&a as &str);

    // let r: Result<i32, i32> = Err(100);

    // match r {
    //     Err(e) => println!("死んでしまえ: {}", e),
    //     _ => {},
    // }

fn main() {
    {
        println!("-- 配列の参照(スライス)を別の変数に束縛して、ポインタを確認してみる");

        let a: [i32; 2] = [1,2];
        let b = &a;

        // これは同じ
        println!("{:p}", &a);
        println!("{:p}", b);
    }

    {
        println!("-- 配列を別の変数にそのまま代入してみる");

        let a: [i32; 2] = [1,2];
        let b = a;

        // これは違う コピーされる
        println!("{:p}", &a);
        println!("{:p}", &b);
    }

    {
        println!("-- 配列を書き換える");

        let mut a = [1,2,3];
        let b = a; // ここでコピーされる
        a[0] = 0;

        println!("{:?}", a); // #=> [0, 2, 3]
        println!("{:?}", b); // #=> [1, 2, 3]
    }

    {
        println!("-- スライスを書き換える");

        let a = &mut [1,2,3];
        // let b = a;
        a[0] = 0;

        println!("{:?}", a); // #=> [0, 2, 3]
        // println!("{:?}", b); // #=> [1, 2, 3]
    }

    // {
    //     let mut a = 1;
    //     a = 2;
    //     println!("{:?}", a);
    // }

    {
        println!("-- スライスを書き換える");

        let a = &mut [1];
        a[0] = 0; // これはできる

        // a = &mut [2]; // これはできない

        println!("{:?}", a);
    }

    {
        println!("-- slice or string");

        let a = "hoge".to_string();
        println!("{:p}", &a);

        puts_str(&a);
        puts_string(&a);

        println!("{:p}", &a);
    }

    {
        println!("-- print pointer");

        let a = "hoge".to_string();
        puts_str(&a);
        puts_string(&a);
    }
}

fn puts_str(s: &str) {
    println!("{:p}", s);
}

fn puts_string(s: &String) {
    println!("{:p}", s.as_ptr());
}

// fn hoge() -> &str {
//     let a = "hoge".to_string();
//     &a
// }

// fn hoge(s: &str) {
//     println!("{}", s);
// }

// fn boxed_fn(i: i64) -> Box<Fn(i64) -> i64> {
//     Box::new(move |j| i * j)
// }

// fn impl_trait_fn(i: i64) -> impl Fn(i64) -> i64 {
//     move |j| i * j
// }

