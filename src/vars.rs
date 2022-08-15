//mod sub_a;
//mod sub_b;

const MAX_POINTS:u32 = 100_000; 
//constで&定義すると任意の関数内で使用できる

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();

    let mut x = 5; //letで定義される変数は関数内でしか使えない。
    println!("The value of x is: {}",x);
    x = 9;
    println!("The value of x is: {}",x);
    let _i1 = 0.1; //f64　不動小数点型

    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    let i2:i64 = 1;
    let i3:i64 = 2;

    println!("Stack adress of i2 is: {:p}", &i2);
    println!("Stack adress of i3 is: {:p}", &i3);

    let y = 5;
    println!("Stack address of y is: {:p}", &y);
    let y = y + 1; //Rustにおけるシャドーインング__同じ変数名で再度定義できる。最後に定義したのが有効になり前に定義したものは隠れる。
    println!("Stack address of y is: {:p}", &y);
    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is:{}", y);
    //letで再定義した変数は書き換えられるのではなく新たな領域に書き込まれる
    {
        let y = 0;
        println!("The new value of y is:{}",y);
        //ローカルスコープ内で同じ変数で定義してもこのスコープを抜ければまた同じ値で変数名が使える。
    }
    println!("The value of y is:{}", y);

    let t1 = (500, 6.4,"dummy"); //タプル型
    let (_x,_y,_z) = t1; //x y zとして指定する
    println!("The value of t1 is:{} {} {}",t1.0,t1.1,t1.2); //インデックスを使って呼び出す

    let mut t2 = ((0, 1),(2, 3));//入れ子型のタプル //mut mutable 可変
    let ((ref mut x1_ptr, ref mut y1_ptr),_) = t2;//「 _ 」は何も受け取らない
    *x1_ptr = 5;//　*変数名 とすることで直接値を編集できる(参照外し)
    *y1_ptr = -5;
    println!("{:?}",t2);//複雑なデータ型を出力するときは{:?}を使う

    let a1 = [1,2,3,4,5];
    let a2 = [0; 10];// 0を10コ
    println!("{:?} {:?} {} {}",a1 ,a2,a1[2],a1[3]);

    let s1 = "hello こんにちわ　你好"; //&str  utf-8  hello is 1byte , japanese is 3byte 実データはスタックに保存されない
    let s2 ="hello";
    println!("Stack address os s1 is:{:p}",&s1);
    println!("Stack address os s2 is:{:p}",&s2);
}