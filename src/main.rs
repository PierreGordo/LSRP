mod random_enviroment;

fn main() {
    let rand_env = random_enviroment::create_enviroment(1, 100, 10, 50);


    for point in rand_env.points{
        print!("{} \n", point);
    }


}
