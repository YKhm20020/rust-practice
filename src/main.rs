fn main() {
    let y = {
        let x = 3; // なんらかの動作をして値を返さない命令である「文」
        x + 1 // 結果値に評価される「式」、終端にセミコロンがない

        // let x = (let y = 6);  // let = y = 6 は値を返さないので、エラーになる
    };

    // The value of y is: 4
    println!("The value of y is: {}", y);
}
