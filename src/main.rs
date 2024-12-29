fn main() {
    {
        // Integer types
        // f64の方がサイズが大きいにもかかわらず、f32とほぼ同等のスピードなので、f64を使うことを推奨
        let x = 2.0;
        let y: f32 = 3.0;
    
        // addition
        // 足し算
        let _sum = x + y;
        // println!("sum: {}", sum);
    
        // subtraction
        // 引き算
        let _difference = x - y;
        // println!("difference: {}", difference);
    
        // multiplication
        // 掛け算
        let _product = x * y;
        // println!("product: {}", product);
    
        // division
        // 割り算
        let _quotient = x / y;
        // println!("quotient: {}", quotient);
        let _floored = x / y; // Results in 0
                             // 結果は0
        // println!("floored: {}", floored);
    
        // remainder
        // 余り
        let _remainder = 43 % 5;
        // println!("remainder: {}", remainder);
    }
    {
        // char type
        let _c = 'z';
        // println!("c: {}", c);
        let _z = 'Z';
        // println!("z: {}", z);
        let _heart_eyed_cat = '😻'; 
        // println!("heart_eyed_cat: {:?}", heart_eyed_cat);
    }
    {
        // touple type
        // タプルは、1つの複合要素と考えられる。
        // タプルから個々の値にアクセスするには、パターンマッチングを使用する必要がある
        let tup: (i32, f64, u8, char) = (500, 6.4, 1, '😻');
        println!("tup: {:?}", tup);

        // パターンマッチング
        // TSの分割代入みたいで覚えやすい
        // Rustではこれを分配と呼んでいる
        let (x,y,z, c) = tup;
        println!("x is {}", x);
        println!("y is {}", y);
        println!("z is {}", z);
        println!("c is {}", c);
        println!(".を付けてアクセスする事もできます。例：tup.0 tup.1 tup.2 {}", tup.3);
    }
    {
        // array
        // タプルとは異なり、配列は全ての要素が同じ型でなければならない
        // Rustの配列の長さは固定
        let array = [1, 2, 3, 4, 5];
        println!("array: {:?}", array);
        // 固定ではない配列はベクタ型と呼ばれ、色々なデータに柔軟に対応できる。
        // 既に値の長さが決まっているものをデータとして格納したい時はarrayを使用し、値の長さが今後変更されると思われる場合はベクタ型を使用するべき。
        let _a = [3; 5]; 
        let _a = [3, 3, 3, 3, 3];
        // aとして定義されたこれらの変数は、どちらも同じ値を持つ。
        // 同じ値が連続して格納される場合は、1つ目の様に[データ; 長さ]の様にして書く方が簡潔
        // 配列の個々の値にアクセスするには
        println!("index 3 of array: {}", array[2]);
    }
}
