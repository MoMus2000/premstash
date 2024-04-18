use std::os::unix::process::parent_id;

use rusqlite::{params, Connection, Result, OpenFlags};

#[derive(Debug)]
pub struct sqlite_db{
    pub conn: Connection
}

impl sqlite_db{
    pub fn new() -> Self{
        let conn = Connection::open_with_flags("/Users/mmuhammad/Desktop/projects/premstash/premstash/cred.vault",
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE
              )
              .unwrap();
        sqlite_db::build_schema(&conn);
        sqlite_db{conn}
    }

    pub fn build_schema(conn: &Connection){
        conn.execute(
            "CREATE TABLE IF NOT EXISTS PREMSTASH (
                  id INTEGER PRIMARY KEY,
                  credential TEXT NOT NULL,
                  store TEXT NOT NULL)",
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

    pub fn insert_cred(&self, values: Vec<&&str>){
        println!("VALUES: {:?}", values);
        assert!(values.len() == 2);
        self.conn.execute("
        INSERT INTO PREMSTASH (credential, store) 
        VALUES (?1, ?2)", params![values[0], values[1]]).unwrap();
    }

    pub fn fetch_cred(&self, values: &[String]) -> String{
        assert!(values.len() == 1);
        let mut fetch_statement = self.conn.prepare("
        SELECT store FROM PREMSTASH
        WHERE credential == ?1
        ").unwrap();
        let fetch_iter = fetch_statement.query_map(params![values[0]], |row| {
            Ok((
                row.get::<usize, String>(0)?,
            ))
        }).unwrap();
        for cred in fetch_iter{
            match cred{
                Ok(res) => {
                    let store = res;
                    println!("STORE: {:?}", store);
                    return format!("{}", store.0);
                }
                _ => {
                    return format!("Could not find your credential");
                }
            }
        }
        String::from("Could not find your credential")
    }
    
}

#[cfg(test)]
mod tests{
    use super::sqlite_db;

    #[test]
    pub fn test_db_creation(){
        sqlite_db::new();
    }
}