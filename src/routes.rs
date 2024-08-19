use crate::handlers::customers;

pub fn get_routes() -> Vec<rocket::Route> {
    let mut routes = Vec::new();
    routes.append(&mut customers::get_routes());
    
    routes
}
