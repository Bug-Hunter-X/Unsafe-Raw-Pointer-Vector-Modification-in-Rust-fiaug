fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Safe way to modify vector elements
    println!("The value is: {:?}", v);

    //Alternatively using iter_mut
    for i in v.iter_mut(){
        *i = *i * 2; 
    }
    println!("The doubled value is: {:?}", v);
} 