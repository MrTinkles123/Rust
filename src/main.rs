use std::io;    //prelude
use rand::Rng;  //trait
use std::cmp::Ordering;
fn main() {
    println!("猜数游戏!");

    let rng = rand::thread_rng().gen_range(1..=100);

    // println!("神秘数字是：{}", rng);

    loop {
            println!("猜测一个数");

    let mut guess  = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue
    };  //隐藏（shadow）之前的guess变量，trim方法去掉前后空白，parse把字符串转换成某种数字类型i32，u32，i64，在前面声明就可以

    println!("你猜测的数是：{}", guess);

    match guess.cmp(&rng) {
            Ordering::Less => println!("Too small!"),   //arm
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
    }
    }


}
