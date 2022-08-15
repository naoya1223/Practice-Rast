pub fn run(){
  //let a1: [u8; 9000000] = [1; 9000000]; //Rustは8MBまでしかスタックを積めない。

  let mut v1 = vec![1,2,3,4];
  let v2 = vec![5,6,7,8,];
  let mut v3 = vec![9,10];
  println!("stack address of v1 is: {:p}",&v1);//stack address of v1 is: 0x16f26ec38   56　スタック
  println!("stack address of v2 is: {:p}",&v2);//stack address of v2 is: 0x16f26ec50   80 v1が24バイト占めていることがわかる
  println!("Heap address of v1 is: {:?}", v1.as_ptr());//Heap address of v1 is: 0x12af04280　ヒープ
  println!("Len of v1 is: {}",v1.len());//Len of v1 is: 4　各要素の長さ
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

}