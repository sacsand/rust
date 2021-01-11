fn main() {
    let a;
    a = 5;
    println!("Hello, world is {}",a);

    let (x, y) = (1, 2); // x = 1 and y = 2

    let z = {x+y};

    let z = {

        let x =4;
        let y =2;

        x+y

    };

    println!("{}",z);


}
