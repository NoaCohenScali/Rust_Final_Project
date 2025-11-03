use sqlx::sqlite::SqlitePool;

pub async fn init_db() -> Result<SqlitePool, sqlx::Error> {
    let database_url = "sqlite:src/zoo.db";
    let pool = SqlitePool::connect(database_url).await?;

    // הפעלת תמיכה במפתחות זרים
    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await?;

    // 🦁 טבלת מתחמים (Enclosures)
    println!("📋 Creating enclosures table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS enclosures (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            enclosure_type TEXT NOT NULL,
            capacity INTEGER NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;
    println!("✅ Enclosures table ready");

    // 🐾 טבלת עובדים (Staff)
    println!("📋 Creating staff table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS staff (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            first_name TEXT NOT NULL,
            last_name TEXT NOT NULL,
            role TEXT NOT NULL,
            phone TEXT NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;
    println!("✅ Staff table ready");

    // 🐘 טבלת חיות (Animals)
    println!("📋 Creating animals table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS animals (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            age INTEGER NOT NULL,
            enclosure_id INTEGER NOT NULL,
            species TEXT NOT NULL,
            FOREIGN KEY (enclosure_id) REFERENCES enclosures(id)
        )
        "#,
    )
    .execute(&pool)
    .await?;
    println!("✅ Animals table ready");

    // 🥕 טבלת האכלה (Feeding)
    println!("📋 Creating feedings table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS feedings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            animal_id INTEGER NOT NULL,
            staff_id INTEGER NOT NULL,
            feeding_time TEXT NOT NULL,
            FOREIGN KEY (animal_id) REFERENCES animals(id),
            FOREIGN KEY (staff_id) REFERENCES staff(id)
        )
        "#,
    )
    .execute(&pool)
    .await?;
    println!("✅ Feedings table ready");

    // 💊 טבלת טיפולים רפואיים (Medical_Treatments)
    println!("📋 Creating medical_treatments table if not exists...");
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS medical_treatments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            animal_id INTEGER NOT NULL,
            staff_id INTEGER NOT NULL,
            date TEXT NOT NULL,
            description TEXT NOT NULL,
            FOREIGN KEY (animal_id) REFERENCES animals(id),
            FOREIGN KEY (staff_id) REFERENCES staff(id)
        )
        "#,
    )
    .execute(&pool)
    .await?;
    println!("✅ Medical_Treatments table ready");

    println!("🎉 All zoo tables initialized successfully!");
    Ok(pool)
}