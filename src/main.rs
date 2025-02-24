fn main() {
    let mut x = 5; // 同変数に再代入を許すため、mut をつける
    println!("The value of x is: {}", x);
    x = 6; // x が束縛している値を 5 から 6 に変更する
    println!("The value of x is: {}", x);
}
