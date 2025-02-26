fn main() {
    let x = 5;

    // シャドーイング、x に 5 が束縛
    // let x = を繰り返し、x の値を変更
    let x = x + 1;

    {
        // シャドーイング、スコープ範囲内のみ x の値が 2 倍に
        let x = x * 2;

        // The value of x in the inner scope is: 12
        println!("The value of x in the inner scope is: {}", x);
    }

    // シャドーイング、 x = x * 2 のスコープ外なので、x の値は 6
    // The value of x is: 6
    println!("The value of x is: {}", x);
}
