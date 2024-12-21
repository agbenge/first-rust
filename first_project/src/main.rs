 

use actix_web::{get, HttpResponse, Responder, App, HttpServer};

// Define the lb module
mod lb {
    pub fn greeting() -> String {
        "Hello from lb module!".to_string()
    }
}

// Define a route
#[get("/greet")]
async fn greet() -> impl Responder {

    HttpResponse::Ok().body(lb::greeting())
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet) // Register the /greet service
            .service(index) // Register the / service
    })
    .bind("0.0.0.0:8080")? // Bind to local address
    .run()
    .await
}
