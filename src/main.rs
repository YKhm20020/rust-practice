fn main() {
    let condition = true;

    // let 文内で if 式による条件分岐で代入する値を変えている
    let number = if condition { 5 } else { 6 };

    // この場合は if, else のそれぞれの型に互換性がない旨のエラー発生
    // let number = if condition { 5 } else { "six" };

    // The value of number is: 5
    println!("The value of number is: {}", number);
}
