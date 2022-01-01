
fn main(){
    let prod = assembly_line::production_rate_per_hour(5);
    let items = assembly_line::working_items_per_minute(5);
    println!("The items are {} and count is {}", items, prod);
}