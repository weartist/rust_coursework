
fn string_push(s: &mut String, new_str: &str) {
    s.push_str(new_str);
}

fn main() {

    {
        // 所有权:
        // 每一个值同一时间只能有一个拥有者，也叫所有权，如果拥有者离开作用域，这个值将被丢弃，不能够再次访问。

        // 所有权转移
        let x = 1;
        let y = x;
        // println!("{}", x); //错误，x已经被转移。
    }

    {
        // 不可变引用：
        // 不可变引用可以存在多个，因为不可变引用主要是查询作用，存在多个也不会有安全问题。
        let x = 1;
        let y = &x;
        
        println!("{}",x); // 1
        println!("{}",y); // 1
    }

    {
        // 可变引用
        let mut x = 1;
        let y = &mut x; 
        *y = 100;
        println!("{}",y); //输出100, 
        println!("{}",x); //输出100, 可以修改原值

        // 字符串可变引用
        let mut s = String::from("first");
        
        {
            let s1 = &mut s;
            s1.push_str(" second");
            println!("{}", s1);  //输出 first second
        }

        // 在上边括号作用域结束后，s1被丢弃，此时s再次可以被获取
        let s2 = &mut s; 
        s2.push_str(" third");
        println!("{}", s2);  //输出 first second third

        // 不能同时存在多个可变引用，但如果前一个引用已经用不到了，后边就可以在使用一个新的引用了
        let s3 = &mut s;
        s3.push_str(" fourth");
        println!("{}", s3);  //输出 first second third

        // 不能同时存在多个可变引用，例如如果s2后续还有使用，即s3创建的时候，s2还存在，会报错
        // 且是编译器就会报错，不会等到运行时才报错
        let s4 = &mut s;
        println!("{}", s3); //注意这句会导致上一行报错，因为s2直到这行还是需要存在的
    }

    {
        // 同样的规则，对于参数为可变借用的方法也是同理，如果方法需要修改一个参数，那么这个参数就必须是可变借用：
        let mut s1 = String::from("first");
        string_push(&mut s1, " second");
        println!("{}", s1);  //输出 first second

        // 如果参数是不可变借用，那么方法内部不能修改这个参数
        let s2 = String::from("first");
        string_push(s2, " second"); //编译期间会报错，编译器会替我们做一些基础的检测
        println!("{}", s1);  //输出 first second
    }

    

}
