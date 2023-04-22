use sqlb::HasFields;

use super::db::DB;

#[derive(Debug, sqlx::FromRow, Clone)]
pub struct Log{
    pub id: i64,

    pub log: String,
    pub error: bool,
    pub priority: i32,
    pub send: bool,
    pub extra: Option<String>,

    pub subdomain: String,
    pub appname: String
}

#[derive(sqlb::Fields, Debug, Default, Clone)]
pub struct LogPatch {
    pub log: String,
    pub error: bool,
    pub priority: i32,
    pub send: bool,
    pub extra: Option<String>,

    pub subdomain: String,
    pub appname: String
}

pub struct LogMac;

impl LogMac{
    const TABLE: &'static str = "log";
    const COLUMNS: &'static [&'static str] = &["id", "log", "error", "priority", "send", "extra", "subdomain", "appname"];
}

impl LogMac{
    pub async fn create(db: &DB, data: LogPatch) -> Result<Log, sqlx::Error> {
        let fields = data.fields();
        let sb = sqlb::insert().table(Self::TABLE).data(fields).returning(Self::COLUMNS);
        let log = sb.fetch_one(db).await?;

        Ok(log)
    }
}
