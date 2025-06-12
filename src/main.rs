mod random_enviroment;
mod engine;

fn main() {
    let mut rand_env = random_enviroment::create_primitive_enviroment(2, 3, 10, 50, 5);


    //Debug print for points
    for point in rand_env.points.iter(){
        //print!("{} \n", point.id);
    }

    //Debug print for roads
    for road in rand_env.roads.iter(){
        //                                                      Since road from and road to are both Point structures I need to do it like this
        print!("Road with ID: {} from: {}, road to: {}, road lenght: {} \n", road.id, road.from.id, road.to.id, road.lenght);

    }

    //Debug print for trucks
    for truck in rand_env.trucks.iter(){
        print!("Truck with ID: {} current_location: {}, to: {:?}, capacity: {}, stationary: {} \n", truck.id, truck.current_location.id, truck.destination, truck.capacity, truck.stationary)
    }

    //Debug print for cargo
    for cargo in rand_env.cargoes.iter(){
        //print!("Cargo with ID: {}, from: {}, to: {}, with capacity usage: {}, is_loaded?: {}, is_stationary?: {} \n", cargo.id, cargo.origin.id, cargo.destination.id, cargo.capacity_usage, cargo.is_loaded, cargo.stationary);
    }


    engine::min_max(&mut rand_env, 2);


}
