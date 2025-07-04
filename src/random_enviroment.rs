//Display
use std::fmt::{write, Display};
//Using for random enviroment creation
use rand::prelude::*;
//Used to differentiate between text when debugging
use ansi_term::Style;
use ansi_term::Colour::Red;




//Truck struct
#[derive(Clone)]
pub struct Truck{
    //Adding ID so I can tell them apart
    pub id: u32,
    //Not the original origin, but the latest stop
    pub current_location: Point,
    pub destination: Option<Point>,
    //Not sure about the units yet, just a work in progress
    pub capacity: u32,
    pub stationary: bool,
    //Figure out how to connect
    pub load: Vec<Cargo>,
}


//Point struct
#[derive(Debug, Clone, PartialEq)]
pub struct Point{
    pub id: u32,
}

//Road struct
#[derive(Clone)]
pub struct Road{
    //Adding ID so I can tell them apart
    pub id: u32,
    pub from: Point,
    pub to: Point,
    pub lenght: u32,
}

//Cargo struct?
#[derive(Clone)]
pub struct Cargo{
    //Adding ID so I can tell them apart
    pub id: u32,
    //Not the original origin, but the latest stop
    pub current_location: Point,
    pub destination: Point,
    //How much of the trucks capacity does it use?
    pub capacity_usage: u32,
    pub stationary: bool,
    //Whether the cargo is loaded on a truck or not
    pub is_loaded: bool,
}


//Enviroment structure -> it will store the enviroment (all of the vectors of the different stuff...)
#[derive(Clone)]
pub struct Enviroment{
    pub trucks: Vec<Truck>,
    pub points: Vec<Point>,
    pub roads: Vec<Road>,
    pub cargoes: Vec<Cargo>,
}



//Implementing Truck functions
impl Truck{
    fn new(id: u32, current_location: Point, destination: Option<Point>, capacity: u32, stationary: bool, load: Vec<Cargo>) -> Truck{
        Truck { id: id, current_location: current_location, destination: destination, capacity: capacity, stationary: stationary , load: load}
    }
}

//Implementing Point functions
impl Point{
    fn new(id: u32) -> Point{
        Point { id: id }
    }
}

//Implementing Road functions
impl Road{
    fn new(id: u32, from: Point, to: Point, lenght: u32) -> Road{
        Road {id: id, from: from, to: to, lenght: lenght}
    }
}

//Implementing Cargo functions
impl Cargo{
    fn new(id: u32, current_location: Point, destination: Point, capacity_usage: u32, stationary: bool, is_loaded: bool) -> Cargo{
        Cargo {id: id, current_location: current_location, destination: destination, capacity_usage: capacity_usage, stationary: stationary, is_loaded: is_loaded}
    }
}

//Implementing Enviroment functions
impl Enviroment{
    fn new(trucks: Vec<Truck>, points: Vec<Point>, roads: Vec<Road>, cargoes: Vec<Cargo>) -> Enviroment{
        Enviroment { trucks: trucks, points: points, roads: roads, cargoes: cargoes }
    }
    //Function to find a truck by id
    pub fn find_truck_by_id(&self, id: u32) -> Option<Truck>{
        self.trucks.iter().find(|truck| truck.id == id).cloned()
    }
    //Function to find cargo by id
    pub fn find_cargo_by_id(&self, id: u32) -> Option<Cargo>{
        self.cargoes.iter().find(|cargo| cargo.id == id).cloned()
    }
}

//Implementing Display for Enviroment because I need it for the debugging
impl Display for Enviroment{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //Bold start of debug
        writeln!(f, "{}", Red.bold().paint("ENVIROMENT DEBUG PRINT"));
        //Write out all the points in the enviroment
        writeln!(f, "{}", Style::new().bold().paint("Points in enviroment:"));

        for point in self.points.iter(){
            writeln!(f, "Point with ID: {}", point.id);
        }

        //Write out all the roads in the enviroment
        writeln!(f, "{}", Style::new().bold().paint("Roads in enviroment:"));

        for road in self.roads.iter(){
            writeln!(f, "Road with ID: {}, from: {}, to: {}, with lenght: {}", road.id, road.from.id, road.to.id, road.lenght);
        }

        //Write out all the trucks in the enviroment
        writeln!(f, "{}", Style::new().bold().paint("Trucks in enviroment:"));

        for truck in self.trucks.iter(){
            //There are more values to debug with truck but this will do for now
            writeln!(f, "Truck with ID: {}, current location: {}, ", truck.id, truck.current_location.id);
        }
        
        //Write out all the cargo in the enviroment
        writeln!(f, "{}", Style::new().bold().paint("Cargo in enviroment:"));

        for cargo in self.cargoes.iter(){
            //Again more values in cargo, but this should suffice for now
            writeln!(f, "Cargo with ID: {}, with destination: {}, current location: {}, loaded: {}", cargo.id, cargo.destination.id, cargo.current_location.id, cargo.is_loaded);
        }

        Ok(())
    }
}


//Random enviroment creation to run tests on
pub fn create_primitive_enviroment(min_points: u32, max_points: u32, min_lenght: u32, max_lenght: u32, cargo_count: u32) -> Enviroment{

    //First generate points -> random number of points
    let mut rng = rand::rng();


    //Vectors to store all the structures
    let mut truck_vec: Vec<Truck> = Vec::new();
    let mut point_vec: Vec<Point> = Vec::new();
    let mut road_vec: Vec<Road> = Vec::new();
    let mut cargo_vec: Vec<Cargo> = Vec::new();


    //Generates a random number of points, the name is a number
    for num in 0..rng.random_range(min_points..max_points){
        point_vec.push(Point::new(num));
    }

    //Generate roads
    //Every point has to have atleast one road either going from it or going to it so it is connected with the others.
    //ALGORITHM: 1) Iterate over all the points.
    //           2) Connect a road from the current point to the next point, except for the last point

    for x in 0..(point_vec.len() - 1){
        //Generate the lenght randomly, since its a random enviroment
        road_vec.push(Road::new(x as u32, point_vec[x].clone(), point_vec[x+1].clone(), rng.random_range(min_lenght..max_lenght)))
    }

    //Generate trucks on points
    //ALGORITHM: 1) Every time place a truck on the first point
    //           2) After that flip a coin whether to place a truck on the next point

    //Generate first guaranteed truck
    truck_vec.push(Truck::new(0,  point_vec[0].clone(), None, 50, true, Vec::new()));
    
    //Generate all subsequent trucs
    let mut last_id: u32 = 0;
    for x in 1..(point_vec.len() - 1){
        if rng.random_bool(0.5){
            last_id += 1;
            //Later down the line figure out how to properly calculate capacity, for now lets set it to 50                           Hope it can infer the type (Its Vec<Cargo>)
            truck_vec.push(Truck::new(last_id, point_vec[x].clone(), None, 50, true, Vec::new()));
        }
    }


    //Place random cargo on points with random points to go to
    //ALGORITHM: 1) Pick how much cargo I want to place in the enviroment.
    //           2) Spread this cargo randomly amongst the points.

    for x in 0..cargo_count{
        cargo_vec.push(Cargo::new(x, point_vec.choose(&mut rng).unwrap().clone(), point_vec.choose(&mut rng).unwrap().clone(), 10, true, false));
    }

    


    Enviroment::new(truck_vec, point_vec, road_vec, cargo_vec)

}