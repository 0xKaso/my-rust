use std::io;     // 引入标准库中的IO库
use rand::Rng;   // 引入rand中的Rng库
use std::cmp::Ordering;  // 引入标准库中的cmp::Ordering库，Ordering n.排序

fn main(){
    println!("Guess the number!"); // 打印用println!

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Please input your guess.");

    loop {
        // mut表示为可变值，类似let与const的区别
        // ::表明string是一个关联函数
        // 创建一个可变字符串，绑定到guess上
        let mut guess = String::new();

        // 打开io，接收控制台输入
        // 也可以写作std::io::stdin()他将返回一个std::io:Stdin实例
        io::stdin()
            .read_line(&mut guess) // 读取为guess变量，注意这里读取到的是一个字符串
            .expect("Failed to read line");

        // u32是uint32
        // 用match来匹配附和条件的
        let guess: u32 = match guess
            .trim()  // 去除首位空格
            .parse(){     // 解析出符合类型的值
                Ok(num) => num,    //OK是成员函数
                Err(_) => continue // Err是成员函数
            };

        println!("You guessed: {guess}");
        
        // cmp compare比较
        match guess.cmp(&secret_number) {
            // Ordering是一个枚举类型，成员是Less、Greater 和 Equal
            Ordering::Less => println!("To small!"),
            Ordering::Greater => println!("To big!"),
            Ordering::Equal => print_win(),
        }
    }
}

fn print_win(){
    println!("You win!");
}