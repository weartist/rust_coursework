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



fn main() {
    // 使用Rust实现一个简单的声明宏
    let s = square!(2);
    println!("2的平方为：{}", s);

    // 传入一个宏，计算0-10分别传入这个宏之后结果的和
    let x = macro_sum!(square, 5); 
    println!("0-4的平方和为{}", x); 

    // 在编译时,Rust编译器会对宏执行如下一些步骤:
    // 1. 预处理：在文本上展开宏。拿上边的square举例，square!(x)会被替换成宏体代码,变成let x = x * x; x。如果宏里边还包含了其他的宏，会递归去展开
    // 2. 解析：解析展开后的代码生成AST抽象语法树。
    // 3. 语义分析：在AST上进行静态分析，如借用，类型检查等。
    // 4. LLVM IR生成：将AST转换成LLVM IR。
    // 5. 代码生成：根据目标平台，生成最终的可执行机器码。
    // 宏展开发生在预处理阶段，之后就是普通代码的编译。
}
