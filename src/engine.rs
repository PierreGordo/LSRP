use std::collections::HashMap;

//House all the functions to calculate how to "solve" the enviroment
use crate::random_enviroment::*;
use crate::random_enviroment;

//Enumeration of all the possible actions the truck can take
pub enum TruckActions{
    Load,
    Move,
}

//Find a way to query all the possible "legal moves"


pub fn query_legal_moves(enviroment: &Enviroment, actions: &HashMap<random_enviroment::Truck, (TruckActions, u32)>){


    //All possible destinations for a given truck
    let mut possible_destinations: Vec<u32> = Vec::new();
    //All possible cargo to be loaded on a point !!! FUTURE CONFLICT (say there is 6 cargo on a given point, my algorithm gives the green light but it can't all fit in the truck)
    let mut possible_loads: Vec<u32> = Vec::new();


    //Query all the trucks
    for truck in enviroment.trucks.iter(){
        //To get all the possible roads from the point where the truck is currently located
        for road in enviroment.roads.iter(){
            
            //TODO: Put these two statements into one statement, just did this to make debugging easier
            if road.from.id == truck.current_location.id{
                possible_destinations.push(road.to.id);
            }
            else if road.to.id == truck.current_location.id{
                possible_destinations.push(road.from.id);
            }
            
        }
        //Check whether a truck can load cargo
        for cargo in enviroment.cargoes.iter(){
            if cargo.is_loaded == false && cargo.current_location.id == truck.current_location.id && cargo.capacity_usage <= truck.capacity{
                possible_loads.push(cargo.id);
            }
        }
        //Clear the vector to get it ready for the next truck
        possible_destinations.clear();
        possible_loads.clear();
    }

}