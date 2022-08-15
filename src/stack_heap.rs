pub fn run(){
  //let a1: [u8; 9000000] = [1; 9000000]; // u8　各要&&素が１バイトになる　Rustは8MBまでしかスタックを積めない。

  let mut v1 = vec![1,2,3,4];
  let v2 = vec![5,6,7,8,];
  let mut v3 = vec![9,10];
  println!("stack address of v1 is: {:p}",&v1);//stack address of v1 is: 0x16f26ec38
  println!("stack address of v2 is: {:p}",&v2);//stack address of v2 is: 0x16f26ec50
  println!("Heap address of v1 is: {}" )
  

}