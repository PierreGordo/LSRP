use crate::random_enviroment::Road;

mod random_enviroment;

fn main() {
    let rand_env = random_enviroment::create_enviroment(1, 100, 10, 50);


    //Debug print for points
    for point in rand_env.points{
        print!("{} \n", point.name);
    }

    //Debug print for roads
    for road in rand_env.roads{
        //                                                      Since road from and road to are both Point structures I need to do it like this
        print!("Road from: {}, road to: {}, road lenght: {} \n", road.from.name, road.to.name, road.lenght);

    }

    //Debug print for trucks
    for truck in rand_env.trucks{
        print!("Truck from: {}, to: {:?}, capacity: {}, stationary: {} \n", truck.origin.name, truck.destination, truck.capacity, truck.stationary)
    }


}
