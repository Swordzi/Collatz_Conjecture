fn multiply(mut num1: u128) -> Option<u128>{
    for i in 0.. {
        match num1 {
           0 => break, // If the number is already 0 which we cant divide then return nothing
           1 => return Some(i), // If the number has entered the loop then return the current value of i
           even if even % 2 == 0 => num1 /= 2, // If the number is even then divide by 2, 
            _ => num1 = num1.checked_mul(3)?.checked_add(1)?, // If the value is odd then multiply by 3 and add 1
        }
    }
    None 
}
fn main() {
    let num1 = 5345345; // You can enter whatever here
    println!("{:?}", multiply(num1));
}
