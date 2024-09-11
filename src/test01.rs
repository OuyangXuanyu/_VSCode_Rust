// use std::any::Any;

fn main()
{
    // 强类型语言，自动识别变量类型
    let a = 123; // 确定为整形数字
    println!("{}", a);
    /* 不可：
    (1. 类型不能变更)
    a = "abc";
    a = 1.23;
    (!!!2. a为不可变变量)
    a = 456; // 不被允许，a为不可变变量
    但是可以再次声明
    let a = 456; //允许，称之为“重新绑定”

    // let aa = 10; -> let mut aa = 100; 是可以的
    */
    let a = 456;
    println!("{}", a);
    // 使用mut关键字即可实现2的操作
    let mut b = 123;
    println!("{}", b);
    b = 456;
    println!("{}", b);

    // 常量的声明
    const c:i32 = 123;

    //重影：变量名称可以被重新利用
    let d = 5;
    let d = d + 10;
    let d = d * d;
    println!("{}", d);

    // 不可变变量重影时，类型|可变属性|值 都可以发生变化
    let d = "牛逼";
    println!("{}", d);

    // 但可变变量重影只能发生值的变化，不能发生类型变化
    let mut e = "123";
    // e = e.len(); //不可

    return;
}