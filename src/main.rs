// prelude にない型を使いたいときは、use でライブラリを明示的に指定
// Option や Copy, String は prelude にあるので、use は不要
use rand::Rng;
use std::cmp::Ordering; // cmp モジュールの Ordering 型をスコープに導入
use std::io; // Rng トレイトが乱数生成器が実装するメソッドを定義している

fn main() {
    // println! はマクロ、! がつくらしい
    println!("Guess the number!");

    // 1から100までの乱数を生成、101 は含まない
    // 1..=100 と書いてもよい
    // 0.9.0 から thread_rng が非推奨になり、rand::rng() を使うようになった
    // 同様に gen_range も非推奨になり、rand::rng().random_range に
    let secret_number = rand::rng().random_range(1..101);

    println!("The secret number is: {}", secret_number);

    // loop で無限ループ
    loop {
        println!("Please input your guess.");

        // mut は mutable (可変) な変数を宣言するときに使う
        // guess に String 型の新しいインスタンスを束縛
        // new 関数: String の関連関数、空の文字列を生成
        let mut guess = String::new();

        // メソッドが重なるときは、ドットごとに改行することが推奨されているらしい
        // .expect がないと、read_line が返す Result 型の値を処理していない旨の警告が出る
        io::stdin()
            .read_line(&mut guess) // read_line メソッドでユーザーの入力を受け付ける。入力した文字を guess を参照して受け取る
            .expect("Failed to read line"); // read_line メソッドが列挙型である Result 型を返す。Result 型のインスタンスが expect メソッドを呼び出す。

        // guess を u32 の数値型に変換する
        // シャドーイング: 同じ変数名を再利用することで、新しい値を変数に束縛することができる
        // シャドーイングは変数の型を変換する際によく使われる
        // String::trim メソッドは文字列の両端の空白と改行を削除する
        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // parse メソッドで変換できない場合

        // {} はプレースホルダー、C 言語で言うところの型指定がないフォーマット指定子みたいなもの？
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // ループを抜ける
            }
        }
    }
}
