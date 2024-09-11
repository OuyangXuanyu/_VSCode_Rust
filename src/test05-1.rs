fn main()
{
    let mut a:i32 = 1;
    const C:i32 = 6;
    
    let mut vec_mut = [1, 2, 3, -4, -5];
    let iter_mut_ = vec_mut.iter();
    
    println!("{:?}", vec_mut);
    println!("{:?}", iter_mut_);

    return;
}