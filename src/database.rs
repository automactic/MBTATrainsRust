use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;

use super::entities::Vehicle;
use super::schema;

pub struct Database {

}

impl Database {
    fn establish_connection(&self) -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
    }

    pub fn update_vehicles(&self, vehicles: Vec<Vehicle>) {
        let connection = self.establish_connection();
        for vehicle in vehicles {
            diesel::insert_into(schema::vehicles::table)
                .values(&vehicle)
                .on_conflict(schema::vehicles::id)
                .do_update()
                .set(&vehicle)
                .execute(&connection)
                .ok();
        }
    }

    pub fn get_vehicle_count(&self) {
        let connection = self.establish_connection();
        let results = schema::vehicles::table.load::<Vehicle>(&connection).expect("Error loading vehicles");
        println!("Found {} vehicles", results.len());
    }
}