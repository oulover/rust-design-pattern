// products
trait Product{
    fn getInfo(&self)->String{
        "Product".to_string()
    }
}

trait AirConditioner : Product{
    fn showAirInfo(&self);
}
trait Fan : Product{
    fn showFanInfo(&self);
}
trait Fridge : Product{
    fn showFridgeInfo(&self);
}

// A 公司的产品： 空调
struct A_AirConditioner{

}

impl Product for A_AirConditioner {
    fn getInfo(&self) -> String {
        "Im A_AirConditioner Product".to_string()
    }
}

impl AirConditioner for A_AirConditioner {
    fn showAirInfo(&self) {
        println!("{}", self.getInfo());
    }
}

// A 公司的产品： Fan
struct A_Fan{

}

impl Product for A_Fan {
    fn getInfo(&self) -> String {
        "Im A_Fan Product".to_string()
    }
}

impl Fan for A_Fan {
    fn showFanInfo(&self) {
        println!("{}", self.getInfo());
    }
}


trait AbsFactory<'a>{
        fn createAirConditioner(&self)-> Box<dyn AirConditioner + 'a>;
        fn createFan(&self)->Box<dyn Fan + 'a>;
}

struct A_Factory{}

impl <'a> AbsFactory<'a> for A_Factory {
    fn createAirConditioner(&self) -> Box<dyn  AirConditioner + 'a> {
         Box::new(A_AirConditioner{})
    }

    fn createFan(&self) -> Box<dyn  Fan + 'a> {
        Box::new(A_Fan{})
    }
}



fn main() {
    println!("Hello, world!---------");
    let a= A_AirConditioner{};
    a.showAirInfo();
    let  b = A_Fan{};
    b.showFanInfo();

    let af=A_Factory{};
    let aaAir= af.createAirConditioner();
    aaAir.showAirInfo();
    let aaFan=af.createFan();
    aaFan.showFanInfo();

}
