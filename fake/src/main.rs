use fake::{Dummy, Fake, Faker};


pub struct Foo {

    name: String,
    id: u32,
}

fn main() {


    let mut i = 0;
    while i < 10{
        let mut counters = 0;
        let mut user = vec![];
        let random: Foo = Foo{
            name: Faker.fake(),
            id: Faker.fake(),
        };
        user.push(random);

        println!("{i}. name :{0}\nid:{1}\n", user[counters].name,user[counters].id);
        counters+=1;
        i+=1;
    }

       
    
   
    

    //println!("{:?}", random.name);
}
