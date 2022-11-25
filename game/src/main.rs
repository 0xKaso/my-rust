use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜猜数字!");
    println!("预言家，输入你的预测(1～10)：");
    let secret_number: u32 = rand::thread_rng().gen_range(1..=10);

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取输入内容");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("你猜的数字是: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("咋这么大胆呢？"),
            Ordering::Equal => {
                println!("猜中啦！");
                break;
            }
        }
    }
}
