use crate::helpers::TestApi;
use app::models::FlashCard;
use app::routes::flashcards::FlashCardResponse;
use jiff_sqlx::ToSqlx;
use pavex::http::StatusCode;
use pavex::time::Timestamp as PavexTimestamp;
use uuid::Uuid;

#[tokio::test]
async fn list_flashcards_returns_200_and_list_of_flash_cards() {
    // Arrange
    let api = TestApi::spawn().await;
    let flash_cards = vec![
        FlashCard {
            id: Uuid::new_v4(),
            question: "test question".to_string(),
            answer: "test answer".to_string(),
            topic: "test topic".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
            difficulty: 1,
            created_at: PavexTimestamp::now().to_sqlx(),
            updated_at: None,
        },
        FlashCard {
            id: Uuid::new_v4(),
            question: "test question 2".to_string(),
            answer: "test answer 2".to_string(),
            topic: "test topic 2".to_string(),
            tags: vec!["tag2".to_string(), "tag3".to_string()],
            difficulty: 1,
            created_at: PavexTimestamp::now().to_sqlx(),
            updated_at: None,
        },
    ];

    for item in flash_cards.iter() {
        sqlx::query("INSERT INTO flashcards (id, question, answer, topic, tags, difficulty, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(item.id)
            .bind(&item.question)
            .bind(&item.answer)
            .bind(&item.topic)
            .bind(&item.tags)
            .bind(item.difficulty)
            .bind(item.created_at)
            .bind(item.updated_at)
            .execute(&api.api_db_pool)
            .await
            .unwrap();
    }

    // Act
    let response = api.get_flashcards().await;

    // Assert
    assert_eq!(response.status(), StatusCode::OK);

    let response_body: Vec<FlashCardResponse> = response.json().await.unwrap();

    let expected_body: Vec<FlashCardResponse> = flash_cards
        .into_iter()
        .rev()
        .map(FlashCardResponse::from)
        .collect();
    assert_eq!(response_body, expected_body);
}
