// File: src/db/mod.rs
use serde::Deserialize;
use std::env;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error as SurrealError, RecordId, Surreal};
use tokio::sync::OnceCell;

pub mod models;

static DB: OnceCell<Surreal<Client>> = OnceCell::const_new();

#[derive(Debug, Deserialize)]
pub struct Record {
    id: RecordId,
}

pub async fn init_surrealdb() -> Result<&'static Surreal<Client>, SurrealError> {
    DB.get_or_try_init(|| async {
        let url = env::var("SURREALDB_URL").unwrap_or_else(|_| "127.0.0.1:8000".to_string());
        let username = env::var("SURREALDB_USER").unwrap_or_else(|_| "root".to_string());
        let password = env::var("SURREALDB_PASS").unwrap_or_else(|_| "root".to_string());

        let db = Surreal::new::<Ws>(url).await?;
        db.signin(Root {
            username: &username,
            password: &password,
        })
        .await?;

        db.use_ns("healthtech").use_db("helpdocs").await?;
        Ok(db)
    })
    .await
}

pub async fn setup_schema() -> Result<(), SurrealError> {
    let db = init_surrealdb().await?;

    // Define table for articles
    db.query("DEFINE TABLE articles SCHEMAFULL").await?;

    // Define fields for articles
    db.query(
        r#"
      -- Define the articles table
      DEFINE TABLE articles SCHEMAFULL;
      
      -- Define fields
      DEFINE FIELD title ON TABLE articles TYPE string;
      DEFINE FIELD content ON TABLE articles TYPE string;
      DEFINE FIELD slug ON TABLE articles TYPE string;
      DEFINE FIELD categories ON TABLE articles TYPE array;
      DEFINE FIELD created_at ON TABLE articles TYPE datetime DEFAULT time::now();
      DEFINE FIELD updated_at ON TABLE articles TYPE datetime DEFAULT time::now();
  "#,
    )
    .await?;

    // Define table for processed articles
    db.query("DEFINE TABLE processed_articles SCHEMAFULL")
        .await?;

    // Define fields for processed articles
    db.query(
        r#"
        DEFINE FIELD article ON TABLE processed_articles TYPE record ASSERT $value != NONE;
        DEFINE FIELD summary ON TABLE processed_articles TYPE string;
        DEFINE FIELD bullet_points ON TABLE processed_articles TYPE array;
        DEFINE FIELD keywords ON TABLE processed_articles TYPE array;
        DEFINE FIELD semantic_chunks ON TABLE processed_articles TYPE array;
        DEFINE FIELD summary_embedding ON TABLE processed_articles TYPE array;
        DEFINE FIELD bullet_point_embeddings ON TABLE processed_articles TYPE array;
        DEFINE FIELD categories ON TABLE processed_articles TYPE array;
        DEFINE FIELD created_at ON TABLE processed_articles TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON TABLE processed_articles TYPE datetime DEFAULT time::now();

         -- Create vector indexes
        DEFINE INDEX idx_summary_embedding ON TABLE processed_articles FIELD summary_embedding VECTOR 384 COSINE;
        DEFINE INDEX idx_keyword_embedding ON TABLE processed_articles FIELD keyword_embedding VECTOR 384 COSINE;
    "#,
    )
    .await?;

    // Define table for bullet point embeddings
    db.query(
        "DEFINE TABLE bullet_point_embeddings SCHEMAFULL;
         DEFINE FIELD article ON TABLE bullet_point_embeddings TYPE record ASSERT $value != NONE;
         DEFINE FIELD bullet_point_index ON TABLE bullet_point_embeddings TYPE number;
         DEFINE FIELD embedding ON TABLE bullet_point_embeddings TYPE array;
         
         -- Create vector index on the embedding field
         DEFINE INDEX idx_bulletpoint_embedding 
         ON TABLE bullet_point_embeddings 
         FIELD embedding 
         VECTOR 384 COSINE;
         
         -- Create regular index on article_id for fast lookups
         DEFINE INDEX idx_article_id 
         ON TABLE bullet_point_embeddings 
         FIELD article;",
    )
    .await?;

    // Define table for collections
    db.query("DEFINE TABLE collections SCHEMAFULL").await?;

    // Define fields for collections
    db.query(
        r#"
      -- Define the collections table
      DEFINE TABLE collections SCHEMAFULL;
      
      -- Define fields
      DEFINE FIELD name ON TABLE collections TYPE string;
      DEFINE FIELD description ON TABLE collections TYPE string;
      DEFINE FIELD slug ON TABLE collections TYPE string;
      DEFINE FIELD helpscout_collection_id ON TABLE collections TYPE string;
      DEFINE FIELD paragraph_description ON TABLE collections TYPE option<string>;
      DEFINE FIELD bullet_points ON TABLE collections TYPE option<string>;
      DEFINE FIELD keywords ON TABLE collections TYPE option<string>;
      DEFINE FIELD created_at ON TABLE collections TYPE datetime DEFAULT time::now();
      DEFINE FIELD updated_at ON TABLE collections TYPE datetime DEFAULT time::now();
  "#,
    )
    .await?;

    Ok(())
}
