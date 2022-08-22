enum List{ //boxポインタにしなかった時　recursive type has infinite size
  Node(i32, Box<List>),//コンパイル時に領域を確保する enumのnodeのListサイズが無限になる　→ Boxポインタ化することでスタックに積むサイズが８バイトで確定する
  Nil,
}


pub fn run(){
  //let a1: [u8; 9000000] = [1; 9000000]; //Rustは&8&MBまでしかスタックを積めない。

  let mut v1 = vec![1,2,3,4];
  let v2 = vec![5,6,7,8,];
  let mut v3 = vec![9,10];
  println!("stack address of v1 is: {:p}",&v1);//stack address of v1 is: 0x16f26ec38   56　スタック
  println!("stack address of v2 is: {:p}",&v2);//stack address of v2 is: 0x16f26ec50   80 v1が24バイト占めていることがわかる
  println!("Heap address of v1 is: {:?}", v1.as_ptr());//Heap address of v1 is: 0x12af04280　ヒープ
  println!("Len of v1 is: {}",v1.len());//Len of v1 is: 4　実態の長さ
  println!("Capacity of v1 is: {}",v1.capacity());//Capacity of v1 is: 4　要素数
  
  //要素の追加と削除
  v1.insert(1, 10);
  println!("{:?}", v1);
  v1.remove(0);
  println!("{:?}", v1);

  //配列の連結
  v1.append(&mut v3);
  println!("{:?}", v1);
  println!("{:?}", v3);


  //box pointer
  //ヒープデータへのポインタデータをスタックに積む(ヒープのポインタの所有権をもつ)
  //ヒープに必要なデータ(実データへのポインタ、長さ、要素数)持つ(実データの所有権をもつ)
  //別に実データ

  let t1 = (10,String::from("hello"));
  println!("Stack address of t1 is: {:p}",&t1);//Stack address of t1 is: 0x16bccec90
  println!("Heap memory address of t1.1 is: {:?}",t1.1.as_ptr());//Heap memory address of t1.1 is: 0x14af040a0
  println!("Len of t1.1 is: {}",t1.1.len());//Len of t1.1 is: 5
  println!("Capacity of t1.1 is: {}",t1.1.capacity());//Capacity of t1.1 is: 5

  let mut b1 = Box::new(t1);
  (*b1).1 += "world";
  println!("{:?}",b1);

}