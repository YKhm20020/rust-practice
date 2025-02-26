fn main() {
    // x には plus_one 関数の返り値として i32 型が入ることを期待
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    // 式なのでセミコロンはいらない、セミコロンがあると文になる
    // 文にすると値を返さないので、let x の宣言で値が入らずエラーになる
    x + 1
}
