/**
 * 抽象工厂
*/
trait AbstractFactory<'a> {
    fn create_product_x(&self) -> Box<ProductX + 'a>;
    fn create_Product_y(&self) -> Box<ProductY + 'a>;
}


trait ProductX {
    fn get_value(&self) -> String;
}

trait ProductY {
    fn get_value(&self) -> String;
}


struct ConcreateProductX(String);
impl ConcreateProductX {
    fn new(msg: String) -> ConcreateProductX {
        ConcreateProductX(msg + &" ProductX".to_string())
    }
}

impl ProductX for ConcreateProductX {
    fn get_value(&self) -> String {
        self.0.clone()
    }
}

struct ConcreateProductY(String);
impl ConcreateProductY {
    fn new(msg: String) -> ConcreateProductY {
        ConcreateProductY(msg + &" ProductY".to_string())
    }
}

impl ProductY for ConcreateProductY {
    fn get_value(&self) -> String {
        self.0.clone()
    }
}

struct ConcreateFactoryA;
impl<'a> AbstractFactory<'a> for ConcreateFactoryA {
    fn create_product_x(&self) -> Box<ProductX + 'a>{
        Box::new(ConcreateProductX::new("FactoryA".to_string())) as Box(ProductX)
    }

    fn create_product_y(&self) -> Box<ProductY + 'a> {
        Box::new(ConcreateProductY::new("FactoryA".to_string())) as Box(ProductY)
    }
}

struct ConcreateFactoryB;
impl<'a> AbstractFactory<'a> for ConcreateFactoryB {
    fn create_product_x(&self) -> Box<ProductX + 'a>{
        Box::new(ConcreateProductX::new("FactoryB".to_string())) as Box(ProductX)
    }

    fn create_product_y(&self) -> Box<ProductY + 'a> {
        Box::new(ConcreateProductY::new("FactoryB".to_string())) as Box(ProductY)
    }
}

#[derive(Debug)]
enum FactoryID {
    A,
    B,
}

fn create_factory<'a>(id: FactoryID) -> Box<AbstractFactory<'a> + 'a> {
    match id {
        FactoryID::A => Box::new(ConcreateFactoryA)
        FactoryID::B => Box::new(ConcreateFactoryB)
    }
}