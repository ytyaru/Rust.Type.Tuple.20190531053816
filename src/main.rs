/*
 * Rustの型（タプル）。
 * CreatedAt: 2019-05-31
 */
fn main() {
    let tuple: (i32, char) = (100, 'A');
//    println!("tuple {}", tuple); // error[E0277]: `(i32, char)` doesn't implement `std::fmt::Display`
    println!("tuple {:?}", tuple);

    // 要素を参照（インデックス）
    println!("tuple.0 {}", tuple.0);
    println!("tuple.1 {}", tuple.1);

    // 分解
    let (i1, c1) = tuple;
    println!("i1 {}", i1);
    println!("c1 {}", c1);

    // 要素が1つのときはカンマを付与することで演算子の括弧と区別する
    println!("tuple litteral {:?}", (200,));
    println!("value {:?}", (200));
}

