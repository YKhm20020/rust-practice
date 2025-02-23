// prelude にない型を使いたいときは、use でライブラリを明示的に指定
// Option や Copy, String は prelude にあるので、use は不要
use std::io;

fn main() {
    // println! はマクロ、! がつくらしい
    println!("Guess the number!");

    println!("Please input your guess.");

    // mut は mutable (可変) な変数を宣言するときに使う
    // guess に String 型の新しいインスタンスを束縛
    // new 関数: String の関連関数、空の文字列を生成
    let mut guess = String::new();

    // メソッドが重なるときは、ドットごとに改行することが推奨されているらしい
    io::stdin()
        .read_line(&mut guess) // read_line メソッドでユーザーの入力を受け付ける。入力した文字を guess を参照して受け取る
        .expect("Failed to read line"); // read_line メソッドが列挙型である Result 型を返す。Result 型のインスタンスが expect メソッドを呼び出す。

    // .expect がないと、read_line が返す Result 型の値を処理していない旨の警告が出る

    // {} はプレースホルダー、C 言語で言うところの型指定がないフォーマット指定子みたいなもの？
    println!("You guessed: {}", guess);
}
