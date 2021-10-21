use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub struct DbConfig {
    pub connection_count: u32,
    pub url: String
}

impl DbConfig {
    pub fn create_pool(&self) -> Pool<ConnectionManager<PgConnection>>{
        let manager = ConnectionManager::<PgConnection>::new(&self.url);
        return Pool::builder().max_size(self.connection_count).build(manager).expect("Poolの作成に失敗");
    }
}
