fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectange is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: i32, height: i32) -> i32 {
    width * height
}
