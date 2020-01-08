
use diesel;
use diesel::prelude::*;
use diesel::MysqlConnection;

use serde_derive::{Deserialize, Serialize};

use super::schema::locations;
use super::schema::locations::dsl::locations as all_locations;

#[derive(Queryable, PartialEq, Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: i32,
    pub name: String,
    pub location_type: String,
}

#[derive(Insertable)]
#[table_name = "locations"]
pub struct NewLocation {
    pub name: String,
    pub location_type: String,
}

impl Location {
    pub fn show(id: i32, conn: &MysqlConnection) -> Vec<Location> {
        all_locations
            .find(id)
            .load::<Location>(conn)
            .expect("Error loading book")

    }

    pub fn all(conn: &MysqlConnection) -> Vec<Location> {
        all_locations
            .order(locations::name.desc())
            .load::<Location>(conn)
            .expect("Error loading locations")
    }

    pub fn update_by_id(id: i32, conn: &MysqlConnection, location: NewLocation) -> bool {
        use super::schema::locations::dsl::{name as n, location_type as lt};

        let NewLocation {
            name,
            location_type,
        } = location;

        diesel::update(all_locations.find(id))
            .set(
                (
                    n.eq(name),
                    lt.eq(location_type)
                )
            )
            .execute(conn)
            .is_ok()

    }

    pub fn insert(location: NewLocation, conn: &MysqlConnection) -> bool {
        diesel::insert_into(locations::table)
            .values(&location)
            .execute(conn)
            .is_ok()
    }

    pub fn delete_by_id(id: i32, conn: &MysqlConnection) -> bool {
        if Location::show(id, conn).is_empty() {
            return false;
        }

        diesel::delete(all_locations.find(id)).execute(conn).is_ok()


    }

    pub fn all_by_location_type(location_type: String, conn: &MysqlConnection) -> Vec<Location> {

        all_locations
            .filter(locations::location_type.eq(location_type))
            .load::<Location>(conn)
            .expect("Error loading locations by type")
    }
}