fn get_full_string() {
    println!("Hello, world!");
    une_autre_fonction(12, 'c');
}

fn une_autre_fonction(x : i64, unite: char){
    println!("La valeur de x est {}{}",x, unite)
}

fn five() -> i32 {
    6
}

fn return_type() {
    let x = five();

    println!("La valeur de x est : {}", x);
}

fn add_one(x:i32)->i32{
    x+1
    // return an i32 data
    //dont add a semicolon else it will give an error
}

fn main() {
    return_type();
}