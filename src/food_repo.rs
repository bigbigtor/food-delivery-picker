use crate::models::{Food, NewFood};
use diesel::prelude::*;

pub fn get_foods() -> Vec<Food> {
    use crate::schema::food::dsl::*;
    let connection = crate::establish_connection();
    food.load::<Food>(&connection).expect("Error loading foods")
}

pub fn get_food_by_id(target_id: i32) -> Food {
    use crate::schema::food::dsl::*;
    let connection = crate::establish_connection();
    food.find(target_id)
        .get_result::<Food>(&connection)
        .expect("Error loading food")
}

pub fn save_food(new_food: &NewFood) -> Food {
    use crate::schema::food::dsl::*;
    let connection = crate::establish_connection();
    diesel::insert_into(food)
        .values(new_food)
        .get_result(&connection)
        .expect("Error saving food")
}
