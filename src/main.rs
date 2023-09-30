extern crate stocks_services_graphql;

use std::env;
extern crate serde_json;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;

use stocks_services_graphql::persistence::connection::create_connection_pool;
use stocks_services_graphql::{configure_service, create_schema_with_context};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = create_connection_pool();

    let schema = web::Data::new(create_schema_with_context(pool));

    #[allow(unused_assignments)]
    let mut server_port = "".to_string();
    match env::var("SERVER_PORT") {
        Ok(stream) => {
            server_port = format!("{}", stream);
        }
        Err(_e) => {
            server_port = "8001".to_string();
        }
    };

    HttpServer::new(move || {
        App::new()
            .configure(configure_service)
            .app_data(schema.clone())
    })
    .bind(format!("0.0.0.0:{}", server_port))?
    .run()
    .await
}