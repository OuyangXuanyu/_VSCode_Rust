pub fn testaaa(a: i32)
{
    println!("{}", a);
}

fn main() { 
    let a = 12;
    let b = -12;
    let c = "{}";
    println!("a is {}", a);
    println!("a is {}, and a is {}.", a, a);
    println!("a is {0}, a is {0} also.", a);
    println!("a is {0}, a is {0} also, but b is {1}", a, b);
    // 用于打印大括号
    println!("{{}}");println!("{}", c);
    testaaa(a);
}


