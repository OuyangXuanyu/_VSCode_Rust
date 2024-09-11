// 条件语句 & 循环语句
fn main()
{
    // 条件语句
    let num = 3;
    // Rust语言if后面的语句返回类型必须是bool（没有python或C++的非0即1的操作）
    if num < 5{
        println!("num小于5");
    } else if num == 5{
        println!("num等于5");
    } else{
        println!("num大于5");
    }

    // 联系上一节的函数体语句
    let a = 3;
    let num = {
        if a > 0 {10} else {-10}  //Rust语言的特性，此外两个返回值类型必须一样，必须要有else语句块
    };

    println!("{}", num);

    // 循环语句
    // while
    let mut while_num = 0;
    while while_num < 5{
        print!("{} ", while_num);
        while_num += 1;
    }
    println!("");

    // for（注意Rust语言没有for(i=0;i<10;i++)(Python/C++)的操作，只能通过while实现，或最后介绍的循环类似实现
    let for_list = [10, 20, 30, 40, 50];
    for i in for_list.iter(){  // 注意此处如何遍历元素
        print!("{} ", i);
    }
    println!("");
    for i in 0..5{  // i走一个list[0, 1, 2, 3, 4]，对于0..5为[0, 5)左闭右开
        println!("a[{}] = {}", i, for_list[i]);
    }
    // println!("{}", 0..5[2]);  //但本质上0..5并不是一个数组，列表，切片

    // (!)loop，用于在循环开头或结束无法判断是否应当退出循环的情况
    let loop_list = ["R", "U", "N", "O", "O", "B"];
    let mut i = 0;
    loop{
        let ch = loop_list[i];
        if ch == "O"{
            break;  // 实现即时退出循环
        }
        print!("\'{}\' ", ch);
        i += 1;
    }
    



    return;
}