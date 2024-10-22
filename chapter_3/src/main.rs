fn myprint<T: std::fmt::Display>(msg: &T) {
    // 借用する
    println!("{}", msg); // reference を使うとき一般には * を付けるが println! は付けなくても大丈夫
}

fn myclear(x: &mut String) {
    // リファレンスの引数を変更するので &String ではなく &mut String を使う
    x.clear(); // 文字列.clear でセットされてる文字列を全て消す
}

// fn retrun_hello() -> &String {
//     let s = "Hello".to_string();
//     &s // 関数内のローカル変数のリファレンスを返り値にできない．関数のスコープから抜けると破棄されるのでリファレンスが参照する先がない
// }

fn pick1(x: &[i32], end: usize) -> &[i32] {
    // 引数が 1 つのときは，リファレンスのライフタイムと返り値に表れる全てのリファレンスは同じライフタイムを持つと推定される
    &x[..end]
}

fn pick2<'a, 'b>(x: &'a [i32], y: &'b [i32], end: usize) -> (&'a [i32], &'b [i32]) {
    // 引数と返り値のライフタイムが同じであるこを示すためにライフタイムパラメータを追加する．'a だけでも動く
    (&x[..end], &y[..end])
}

fn main() {
    // コピーセマンティクス
    // let mut x = 1;
    // x = x + 1;
    // println!("x = {}", x);

    // let y = 2;
    // println!("y = {}", y);

    // let s = 1;
    // let t = s;
    // println!("{}", t);
    // println!("{}", t);

    // ムーブセマンティクス
    // let s = "Hello".to_string();
    // let t = s;
    // println!("{}", t);
    // println!("{}", s); // 所有権が移るのでコンパイルできない

    // let s = 1;  // int のときはコンパイルできる
    // let s = "Hello".to_string();
    // let ss = s.clone(); // s のコピーを作る
    // myprint(s); // consume: myprint の引数 s に所有権が移る → 引数のライフタイムが終わるのでメモリから解放される
    // myprint(&s); // reference を使う
    // myprint(ss);

    // let mut s = "Hello".to_string();
    // println!("s={}", s);

    // let s_ref = &mut s;
    // // let s_ref_2 = &mut s; // mutable なリファレンスは複数個作成できない(書き換えが重複して困ってしまう可能性がある)
    // myclear(s_ref);
    // println!("s={}", s);

    // let x = 1;
    // println!("{:p}", &x);

    // let mut s = "hello".to_string();
    // println!("s= {}", s);

    // let s_ref = &mut s;
    // myclear(s_ref);
    // println!("s= {}", s);

    // let s_ref_2 = &mut s; // スコープが異なるのでコンパイルできる
    // myclear(s_ref_2);
    // println!("s= {}", s);

    // let mut x = 1;
    // let x_ref = &x;

    // // x = 2; // x_ref のスコープ内(所有権が x にない)なので更新できない
    // println!("{}", x_ref);

    // let s = retrun_hello();
    // println!("{}", s);

    // let x;
    // {
    //     let y = 1;
    //     x = &y; // y のライタイムが x よりも短い．これよりも後で x が出てこないならエラーじゃない
    // }
    // println!("{}", x)

    // let v1 = [1, 2, 3, 4, 5];
    // let p = pick1(&v1, 2);
    // for ss in p {
    //     println!("{}", ss)
    // }

    // let v1 = [1, 2, 3, 4, 5];
    // let v2 = [6, 7, 8];

    // let p = pick2(&v1, &v2, 2);
    // for ss in p.0 {
    //     println!("{}", ss)
    // }
    // for ss in p.1 {
    //     println!("{}", ss)
    // }

    let x = 1;
    println!("{}", x);

    {
        let x = 2; // xという同名のラベルを付与してx=1を隠している
        println!("{}", x);
    }

    println!("{}", x);
}
