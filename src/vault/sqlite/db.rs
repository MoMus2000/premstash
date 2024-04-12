use std::os::unix::process::parent_id;

use rusqlite::{params, Connection, Result, OpenFlags};

pub struct sqlite_db{
    pub conn: Connection
}

impl sqlite_db{
    fn new() -> Self{
        let conn = Connection::open_with_flags("/Users/mmuhammad/Desktop/projects/premstash/premstash/cred.vault",
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE
              )
              .unwrap();
        sqlite_db::build_schema(&conn);
        sqlite_db{conn}
    }

    fn build_schema(conn: &Connection){
        conn.execute(
            "CREATE TABLE IF NOT EXISTS PREMSTASH (
                  id INTEGER PRIMARY KEY,
                  credential TEXT NOT NULL,
                  store INTEGER NOT NULL)",
            [],
        ).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS META_PREMSTASH (
                  id INTEGER PRIMARY KEY,
                  premstash_id INTEGER NOT NULL,
                  Location TEXT,
                  CREATED TEXT,
                  UPDATED TEXT,
                  STORED_AT TEXT,
                  NUM_ACCESS TEXT,
                  LATEST_ACCESS TEXT,
                  ACCESSED_BY TEXT,
                  FOREIGN KEY(premstash_id) REFERENCES PREMSTASH(id))",
            [],
        ).unwrap();
    }

    fn insert_cred(&self, values: &[String]){
        assert!(values.len() == 2);
        self.conn.execute("
        INSERT INTO PREMSTASH (credential, store) 
        VALUES (?1, ?2)", params![values[0], values[1]]).unwrap();
    }

    fn fetch_cred(&self, values: &[String]){
        assert!(values.len() == 1);
        let mut fetch_statement = self.conn.prepare("
        SELECT store FROM PREMSTASH
        WHERE credential == ?1 
        ").unwrap();
        fetch_statement.execute(params![values[0]]).unwrap();
    }
    
}

#[cfg(test)]
mod tests{
    use super::sqlite_db;

    #[test]
    fn test_db_creation(){
        sqlite_db::new();
    }
}