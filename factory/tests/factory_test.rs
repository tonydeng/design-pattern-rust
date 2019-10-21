pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}


//fn create_product_a() {
//    let factory_a = ConcreateProductX();
//}

// fn create_product_a() {
//     let factory_a = create_factory(FactoryID::A);
//     let a_x = factory_a.create_product_x();
//     let a_y = factory_a.create_product_y();

//     println!("{}",a_x.get_value());
// }