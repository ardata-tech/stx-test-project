use serde::Deserialize;
use w3io_partner_space_and_time::{
    SxT, SxTUser, SxTUserBuilder, SxTTable, TableAccessType, Error as SxTError,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct PolygonBlock {
    block_number: i32,
    block_hash: String,
    transaction_count: i32,
    reward: String,
}

#[derive(Debug, Deserialize)]
struct TestRecord {
    id: i32,
    name: String,
    value: f64,
    description: String,
}

type Result<T> = std::result::Result<T, SxTError>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    println!("Space and Time Test Examples\n");

    // Example 1: Query existing public data
    if let Err(e) = example_query_public_data().await {
        println!("Error in example 1: {:?}", e);
    }

    // Example 2: Create and authenticate a new user
    if let Err(e) = example_create_user().await {
        println!("Error in example 2: {:?}", e);
    }

    // Example 3: Work with tables
    if let Err(e) = example_table_operations().await {
        println!("Error in example 3: {:?}", e);
    }
}

async fn example_query_public_data() -> Result<()> {
    println!("=== Example 1: Query Public Data ===");
    
    // Load existing user from .env file
    let sxt = SxT::new()?;
    let sxt = sxt.authenticate().await?;
    
    // Query public Polygon blockchain data
    let blocks = sxt
        .execute_query::<PolygonBlock>("SELECT * FROM POLYGON.BLOCKS LIMIT 5".to_string())
        .await?;
    
    println!("Retrieved {} blocks:", blocks.len());
    for block in blocks {
        println!("  Block #{}: {} (tx count: {})", 
            block.block_number, 
            block.block_hash,
            block.transaction_count
        );
    }
    
    println!();
    Ok(())
}

async fn example_create_user() -> Result<()> {
    println!("=== Example 2: Create New User ===");
    
    // Create a new user with a generated keypair
    let new_user = SxTUserBuilder::new("test_user_123".to_string())
        .new_keypair()
        .build();
    
    // Save credentials to a file
    new_user.save("test_user.env")?;
    println!("Created and saved new user: test_user_123");
    
    // Note: To authenticate this user, you would need a valid join code
    // let authenticated_user = new_user.join_subscription("JOIN_CODE_HERE".to_string()).await?;
    
    println!();
    Ok(())
}

async fn example_table_operations() -> Result<()> {
    println!("=== Example 3: Table Operations ===");
    
    // Load authenticated user
    let user = SxTUser::load()?;
    let authenticated_user = user.authenticate().await?;
    
    // Create a new table
    let table = SxTTable::new(
        "alvin_test_schema",
        "alvin_test_table",
        None, // Will generate a new keypair
        authenticated_user.clone(),
        TableAccessType::PublicRead,
    );
    
    // Define table schema
    let schema = "id INT PRIMARY KEY, name VARCHAR(100), value DECIMAL(10,2), description VARCHAR";
    
    // Create the table
    match table.create(schema.to_string()).await {
        Ok(_) => println!("Table created successfully"),
        Err(e) => println!("Table creation failed (may already exist): {:?}", e),
    }
    
    // Insert data
    let fields = "id, name, value";
    let values = "(1, 'Test Record 1', 100.50), (2, 'Test Record 2', 200.75)";
    
    match table.insert(fields, values).await {
        Ok(result) => println!("Inserted {} records", result.len()),
        Err(e) => println!("Insert failed: {:?}", e),
    }
    
    // Query data
    match table.select::<TestRecord>("*", "ORDER BY id").await {
        Ok(records) => {
            println!("Retrieved {} records:", records.len());
            for record in records {
                println!("  ID: {}, Name: {}, Value: {}", 
                    record.id, 
                    record.name, 
                    record.value
                );
            }
        }
        Err(e) => println!("Query failed: {:?}", e),
    }
    
    // Delete data
    match table.delete("WHERE id = 1").await {
        Ok(result) => println!("Deleted {} records", result.len()),
        Err(e) => println!("Delete failed: {:?}", e),
    }
    
    println!();
    Ok(())
}