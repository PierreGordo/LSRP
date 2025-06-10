use std::fmt::{write, Display};
//Using for random enviroment creation
use rand::prelude::*;



//Truck struct
pub struct Truck{
    //Not the original origin, but the latest stop
    pub origin: Point,
    pub destination: Option<Point>,
    //Not sure about the units yet, just a work in progress
    pub capacity: u32,
    pub stationary: bool,
}


//Point struct
#[derive(Debug, Clone, PartialEq)]
pub struct Point{
    pub name: u32,
}

//Road struct
pub struct Road{
    pub from: Point,
    pub to: Point,
    pub lenght: u32,
}

//Cargo struct?
pub struct Cargo{
    //Not the original origin, but the latest stop
    origin: Point,
    destination: Point,
    //How much of the trucks capacity does it use?
    capacity_usage: u32,
    stationary: bool,
}


//Enviroment structure -> it will store the enviroment (all of the vectors of the different stuff...)
pub struct Enviroment{
    pub trucks: Vec<Truck>,
    pub points: Vec<Point>,
    pub roads: Vec<Road>,
    pub cargoes: Vec<Cargo>,
}



//Implementing Truck functions
impl Truck{
    fn new(origin: Point, destination: Option<Point>, capacity: u32, stationary: bool) -> Truck{
        Truck { origin: origin, destination: destination, capacity: capacity, stationary: stationary }
    }
}

//Implementing Point functions
impl Point{
    fn new(name: u32) -> Point{
        Point { name: name }
    }
}

//Implementing Road functions
impl Road{
    fn new(from: Point, to: Point, lenght: u32) -> Road{
        Road {from: from, to: to, lenght: lenght}
    }
}

//Implementing Cargo functions
impl Cargo{
    fn new(origin: Point, destination: Point, capacity_usage: u32, stationary: bool) -> Cargo{
        Cargo {origin: origin, destination: destination, capacity_usage: capacity_usage, stationary: stationary}
    }
}

//Implementing Enviroment functions
impl Enviroment{
    fn new(trucks: Vec<Truck>, points: Vec<Point>, roads: Vec<Road>, cargoes: Vec<Cargo>) -> Enviroment{
        Enviroment { trucks: trucks, points: points, roads: roads, cargoes: cargoes }
    }
}


//Random enviroment creation to run tests on
pub fn create_enviroment(min_points: u32, max_points: u32, min_lenght: u32, max_lenght: u32) -> Enviroment{

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
        road_vec.push(Road::new(point_vec[x].clone(), point_vec[x+1].clone(), rng.random_range(min_lenght..max_lenght)))
    }

    //Generate trucks on points
    //ALGORITHM: 1) Every time place a truck on the first point
    //           2) After that flip a coin whether to place a truck on the next point

    
    for x in 1..(point_vec.len() - 1){
        if rng.random_bool(0.5){
            //Later down the line figure out how to properly calculate capacity, for now lets set it to 50
            truck_vec.push(Truck::new(point_vec[x].clone(), None, 50, true));
        }
    }
    
    Enviroment::new(truck_vec, point_vec, road_vec, cargo_vec)

}