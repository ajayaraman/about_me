use rocket::local::asynchronous::Client;
use rocket::http::Status;
use about_me::index; // Import functions from your app

#[rocket::async_test]
async fn test_index_page() {
    // Build the Rocket instance
    let rocket = rocket::build()
        .mount("/", rocket::routes![index]);

    // Create a test client
    let client = Client::tracked(rocket).await.expect("valid rocket instance");

    // Make a GET request to the root endpoint
    let response = client.get("/").dispatch().await;

    // Verify the response
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.expect("response body");
    assert!(body.contains("About Me"));  // Check for expected content
}