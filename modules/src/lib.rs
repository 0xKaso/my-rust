mod front{
    pub mod hosting{
        pub fn test1(){
            println!("I am running!");
        }
        pub fn test2(){
            println!("I am running in crate");
        }
    }

    // 不加pub默认是私有的，外部无法直接访问
    mod serving{
        // 默认方法是私有的
        fn good1(){}
        fn good2(){}
        fn good3(){}
    }
}

pub fn eat(){
    // 绝对路径
    crate::front::hosting::test1();

    // 相对路径
    front::hosting::test2();
}