use crate::helpers::spawn_app;

#[tokio::test]

async fn health_check_works() {
    // Arrange
    let address = spawn_app().await.address;
    let client = reqwest::Client::new();
    // Act
    let response = client
        // Use the returned application address
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
// Launch our application in the background ~somehow~

// #[test]
// fn dummy_fail() {
//     let result: Result<&str, &str> = Err("The app crashed due to an IO error");
//     claim::assert_ok!(result);
// }
