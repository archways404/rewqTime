use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize}; // For serialization and deserialization of JSON
use rand::Rng; // For generating random numbers
use tokio::time::{sleep, Duration}; // For the sleep function
use std::time::Instant; // For timing the request processing

// Struct for deserializing input JSON
#[derive(Deserialize)]
struct InputData {
    name: String,
    age: u32,
}

// Struct for serializing output JSON
#[derive(Serialize)]
struct OutputData {
    message: String,
    name: String,
    age: u32,
    processing_time_ms: u128,  // Field to store processing time
}

// A simple handler function for the index route
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix!")
}

// A handler function for processing JSON input and returning modified JSON output
async fn process_data(data: web::Json<InputData>) -> impl Responder {
    // Start the timer
    let start_time = Instant::now();

    // Generate a random duration between 1.1 and 2.3 seconds
    let mut rng = rand::thread_rng();
    let delay_seconds = rng.gen_range(1.1..2.3);

    // Convert the random seconds to milliseconds and sleep
    let delay_duration = Duration::from_secs_f64(delay_seconds);
    sleep(delay_duration).await; // Asynchronously wait for the random delay

    // Extract data from the input JSON
    let name = &data.name;
    let age = data.age;

    // Modify data (for example, adding a message or making changes)
    let response_message = format!("Hello, {}! We've processed your data.", name);

    // Calculate the processing time in milliseconds
    let processing_time = start_time.elapsed().as_millis();

    // Create a new struct for the response
    let output = OutputData {
        message: response_message,
        name: name.clone(),
        age: age + 1, // Increment age for demonstration
        processing_time_ms: processing_time,  // Include processing time
    };

    // Send back the modified JSON response
    HttpResponse::Ok().json(output)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create and run the server
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/process", web::post().to(process_data))  // POST route for processing JSON
    })
    .bind(("127.0.0.1", 8080))?  // Bind the server to 127.0.0.1:8080
    .run()
    .await
}
