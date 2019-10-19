use std::io;
fn main(){
    println!("Enter Actual Price :");

    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let actual_price: u32 = input1.trim().parse().unwrap();

    println!("Enter Discount Price :");

    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let disc_price: u32 = input2.trim().parse().unwrap();

    println!("Enter Customer Payment Amount :");

    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read line");
    let paid_price: u32 = input3.trim().parse().unwrap();

    let x = shopping(actual_price,disc_price,paid_price);

    println!("Discounted amount is : {}PKR",x.0);
    println!("Discount is : {}%",x.1);
    println!("Balance amount is : {}PKR",x.2);

    if x.1 <= 10.0{
        println!("Azadi Offer");
    } else if x.1 >10.0 && x.1 <=20.0 {
        println!("Eid Offer");
    } else if x.1 >20.0{
        println!("Clearance Sale");
    }
}
fn shopping(actual_price:u32,disc_price:u32,paid_price:u32)->(u32,f32,u32){
    let disc_amt= actual_price-disc_price;
    let disc_avail: f32= disc_amt as f32*100.0 / actual_price as f32;
    let balance= paid_price-disc_price;
    let tup=(disc_amt,disc_avail,balance);
    tup
    }