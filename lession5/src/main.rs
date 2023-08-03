// 一个求输入值的平方的宏
macro_rules! square {
    ($x:expr) => {{
        let val = $x;
        val * val
    }};
}

// 求0到n-1分别传入这个参数宏之后结果的和
macro_rules! macro_sum {
    ($f:ident, $n:expr) => {{
        let mut result = 0;
        for i in 0..$n {
            result += $f!(i); 
        }
        result
    }};
}


// 参考资料：
// rustc官方文档：https://rustc-dev-guide.rust-lang.org/macro-expansion.html
// rust宏小册：https://zjp-cn.github.io/tlborm/introduction.html
fn main() {
    // 使用Rust实现一个简单的声明宏
    let s = square!(2);
    println!("2的平方为：{}", s);

    // 传入一个宏，计算0-10分别传入这个宏之后结果的和
    let x = macro_sum!(square, 5); 
    println!("0-4的平方和为{}", x); 

    // 注：可以使用cargo expand来查看宏展开后的代码，编译器也提供了--pretty=expanded参数可以展开宏代码
    // cargo-expand:https://github.com/dtolnay/cargo-expand

    

    // 在编译时,Rust编译器会对宏执行如下一些步骤:
    // 1. 预处理：在文本层面建立抽象AST语法树
    // 2. 解析展开宏：遍历AST，定位所有宏调用，然后进行展开。拿上边的square举例，square!(x)会被替换成宏体代码,变成let x = x * x; x。如果宏里边还包含了其他的宏，会递归去展开
    // 3. 语义分析：在AST上进行静态分析，如借用，类型检查等。
    // 4. LLVM IR生成：将AST转换成LLVM IR。
    // 5. 代码生成：根据目标平台，生成最终的可执行机器码。
    // 宏展开发生在预处理阶段，之后就是普通代码的编译。

    // rust中宏的优点：
    // 1. Rust宏并没有像C/C++那样使用很多括号来保护，也能够排除一些括号过多产生错误的概率，而且Rust宏并不是简单的文本替换。是有专门的宏解析器，在语法解析层面进行的宏展开。
    // 2. 宏展开是在编译期进行的，没有任何运行时的开销
    // 3. 宏可以方便的接受可变参数，这给了它一些类似js等语言的灵活性
    // rust中宏的缺点：
    // 写复杂的逻辑的确还是会让可读性下降，而且调试可能会困难一些，对于作用域的使用也需要小心，避免出现一些潜在问题
}
