// 函数
// fn (函数名)(<参数>)<函数体>

// 普通函数
fn function1() {
    println!("Hello, runoob!");
}

// 有参数的函数
fn function2(x: i32, y: i64)
{
    println!("x={}, y={}", x, y);
}


fn main() {
    function1();
    function2(12, 23);

    //语句与表达式
    let x = 5;
    let y = {
        let x = 3; //此时的x只作用于这个作用域
        x + 1 // 为返回给y的赋值
        // return x + 1; 为错误用法，此处为函数体表达式，区别于真正的函数
    };
    println!("x={}, y={}", x, y);

    // 函数的嵌套定义与使用，Rust不支持制动返回类型判断
    fn aaplusbb(aa: i32, bb: i32) -> i32  //此时必须要给出函数返回值的类型
    {
        // aa + bb
        return aa + bb;  //此时可以使用return实现数值的返回（return只适用于函数，不适用于函数体表达式）
    }

    println!("{}", aaplusbb(11, 22));

    return;
}