//Used to differentiate between text when debugging
use ansi_term::Style;

//House all the functions to calculate how to "solve" the enviroment
use crate::random_enviroment::*;

//Enumeration of all the possible actions the truck can take
#[derive(Debug, PartialEq)]
pub enum TruckActions{
    Load,
    Move,
}

//Find a way to query all the possible "legal moves"

pub fn query_legal_moves(enviroment: &mut Enviroment) -> Vec<(u32, TruckActions, u32)>{


    //FORMAT IS: TRUCK ID, ACTION THAT TRUCK CAN TAKE (enum TruckActions), ID OF THE TARGET OF THE ACTION (either point to move to or cargo to load)
    let mut possible_moves: Vec<(u32, TruckActions, u32)> = Vec::new();


    
    //Query all the trucks
    for truck in enviroment.trucks.iter(){
        //To get all the possible roads from the point where the truck is currently located
        for road in enviroment.roads.iter(){
                
            //TODO: Put these two statements into one statement, just did this to make debugging easier
            if road.from.id == truck.current_location.id{
                possible_moves.push((truck.id, TruckActions::Move, road.to.id));
            }
            else if road.to.id == truck.current_location.id{
                possible_moves.push((truck.id, TruckActions::Move, road.from.id));
                //Recursively execute this possible move
            }
                
        }
        //Check whether a truck can load cargo
        for cargo in enviroment.cargoes.iter(){
            if cargo.is_loaded == false && cargo.current_location.id == truck.current_location.id && cargo.capacity_usage <= truck.capacity{
                possible_moves.push((truck.id, TruckActions::Load, cargo.id));
                //Recursively execute this possible move
            }
        }
    }

    println!("{}", Style::new().bold().paint("All possible moves in current position: "));
    for element in possible_moves.iter(){
        print!("{:?} \n", element);
    }

    possible_moves
    
    

}

//Actuall algorithm, used recursively to calculate all the possible outcomes (all possible enviroments)
//Depth is used to determine the depth of how deep i want to go (bruh worst comment ever) of the tree
pub fn min_max(enviroment: &mut Enviroment, depth: u32){

    //Query all the moves for the given enviroment
    let possible_moves = query_legal_moves(enviroment);
    //Clone the enviroment so I can modify it
    let mut new_env = enviroment.clone();
    //Check whether we are not at the end with depth
    if depth >= 1{
        //Apply the moves to the enviroment and calculate to the given depth
        for action in possible_moves{
            //Determine the move type
            if action.1 == TruckActions::Move{
                //First find the truck in the Vec trucks by id, then modify its variable
                if let Some(mut truck_with_id) = new_env.find_truck_by_id(action.0){
                    println!("Algorithm wants to move truck with ID {}", action.0);
                    //"play" the chosen move in the enviroment
                    truck_with_id.current_location.id = action.2;
                    //Finally perform the recursion
                    min_max(&mut new_env, depth-1);
                }

            }
            else if action.1 == TruckActions::Load{
                
                if let Some(mut truck_with_id) = new_env.find_truck_by_id(action.0){
                    if let Some(mut cargo_with_id) = new_env.find_cargo_by_id(action.2){
                        println!("Algorithm wants to load cargo with ID: {} truck with ID {}", action.0, action.2);
                        //"Load" the cargo
                        truck_with_id.load.push(cargo_with_id.clone());
                        cargo_with_id.is_loaded = true;
                        //Finally perform the recursion
                        min_max(&mut new_env, depth-1);
                    }
                    
                }

            }
        }
    }
    else{
        println!("{}", Style::new().bold().paint("All moves have been calculated, tree has been created."));
    }
    

}