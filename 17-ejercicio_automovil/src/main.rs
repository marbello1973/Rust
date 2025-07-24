use std::collections::HashMap;
use std::usize;

#[derive(PartialEq, Debug)]
enum Transmission {
    Manual,
    SemiAuto,
    Automatic,  
}

#[derive(PartialEq, Debug)]
enum Age {
    New,
    Used
}

#[derive(PartialEq, Debug)]
struct Car {
    color: String,
    motor: Transmission,
    roof: bool,
    age: (Age, u32),
}


fn car_quality(miles: u32) -> (Age, u32){
   if miles == 0{ 
       (Age::New, 0)
   }else {
       (Age::Used, miles)
   }
}

fn car_factory(order: i32, miles: u32) -> Car {

    let colors = ["Blue", "Green", "Red", "Silver"];

    let mut color = order as usize;
    
    while color > 4 {
        color = color - 4;
    }

    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {
        motor = Transmission::Automatic;
    }else if order % 2 == 0 {
        motor = Transmission::SemiAuto;
        roof = false;
    }
    
    Car{
        color: String::from(colors[(color-1) as usize]),
        motor,
        roof,
        age: car_quality(miles),
    }
}

fn main() {

    let mut orders: HashMap<i32, Car> = HashMap::new();

    // se inicializa el contador 
    // let mut order = 1;
    
    // se declara mutable car 
    let mut car: Car;


    let mut miles = 0;

    for order in 1..50 {    
        // Order 6 cars, increment "order" for each request
        // Car order #1: Usado, Hard top    
        car = car_factory(order, miles);
        orders.insert(order, car);
        print!("Car order {}: {:?}\n", order, orders.get(&order));

        if miles == 2100 {
            miles = 0;
        }else {
            miles = miles + 700;
        }
    }

    // Car order #2 Usado, Convertible
   /* order = order + 1;
    car = car_factory(order, 2000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #3 Nuevo, Convertible
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #4 Nuevo, Hard top
    order = order + 1;
    car = car_factory(order, 0);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #5 Usado, Hard top
    order = order + 1;
    car = car_factory(order, 3000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));

    // Car order #2 Usado, Hard top
    order = order + 1;
    car = car_factory(order, 4000);
    orders.insert(order, car);
    println!("Car order {}: {:?}", order, orders.get(&order));
    */

}


