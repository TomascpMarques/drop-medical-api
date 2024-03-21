#[derive(Debug, serde::Deserialize, serde::Serialize, getset::Getters, getset::Setters)]
#[getset(get = "pub", set)]
pub struct Dropper {
    id: i32,
    serial_id: uuid::Uuid,
    active: bool,
    owner_id: uuid::Uuid,
    machine_url: Option<String>,
    name: String,
}

impl Dropper {
    pub async fn new(
        pg_pool: &sqlx::postgres::PgPool,
        active: bool,
        owner_id: uuid::Uuid,
        machine_url: Option<String>,
        name: String,
    ) -> Result<Self, sqlx::Error> {
        let mut dropper = Self {
            id: 0,
            serial_id: uuid::Uuid::default(),
            active,
            owner_id,
            machine_url,
            name,
        };

        let query_res = sqlx::query!(
            r#"
            insert into "dropper" (active, owner_id, machine_url, name) 
            values ($1, $2, $3, $4) 
            returning id, serial_id;
            "#,
            dropper.active,
            dropper.owner_id,
            dropper.machine_url,
            dropper.name
        )
        .fetch_one(pg_pool)
        .await?;

        dropper.set_id(query_res.id);
        dropper.set_serial_id(query_res.serial_id);
        Ok(dropper)
    }
}
