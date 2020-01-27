use std::env;

use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;

use super::entities::Vehicle;
use super::schema;

pub struct Database {
    connection: PgConnection
}

impl Database {
    pub fn from_dot_env() -> Database {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let connection = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));
        Database{connection}
    }

    pub fn update_vehicles(&self, vehicles: Vec<Vehicle>) {
        for vehicle in vehicles {
            diesel::insert_into(schema::vehicles::table)
                .values(&vehicle)
                .on_conflict(schema::vehicles::id)
                .do_update()
                .set(&vehicle)
                .execute(&self.connection)
                .ok();
        }
    }

    pub fn get_vehicle_count(&self) {
        let results = schema::vehicles::table.load::<Vehicle>(&self.connection).expect("Error loading vehicles");
        println!("Found {} vehicles", results.len());
    }
}