use rusqlite::{params, Connection, OpenFlags};

#[derive(Debug)]
pub struct SqliteDb{
    pub conn: Connection
}

impl SqliteDb{
    pub fn new() -> Self{
        let conn = Connection::open_with_flags("/Users/mmuhammad/Desktop/projects/premstash/premstash/cred.vault",
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE
              )
              .unwrap();
        SqliteDb::build_schema(&conn);
        SqliteDb{conn}
    }

    pub fn build_schema(conn: &Connection){
        conn.execute(
            "CREATE TABLE IF NOT EXISTS PREMSTASH (
                  id INTEGER PRIMARY KEY,
                  credential TEXT UNIQUE NOT NULL,
                  store TEXT NOT NULL,
                  user TEXT UNIQUE 
                )",
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
        assert!(values.len() == 2);
        self.conn.execute("
        INSERT INTO PREMSTASH (credential, store) 
        VALUES (?1, ?2)", params![values[0], values[1]]).unwrap();
    }

    pub fn up_cred(&self, values: Vec<&&str>) -> usize{
        assert!(values.len() == 2);
        let count = self.conn.execute("
        UPDATE PREMSTASH SET store = ?2
        WHERE credential = ?1", 
        params![values[0], values[1]]).unwrap();
        count
    }

    pub fn delete_cred(&self, values: &[String]) -> usize{
        assert!(values.len() == 1);
        let count = self.conn.execute("
        DELETE FROM PREMSTASH WHERE credential = ?1", 
        params![values[0]]).unwrap();
        count
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

    pub fn list_creds(&self) -> Vec<String>{
        let mut res_arr = Vec::<String>::new();
        res_arr.push("--Values--".to_string());
        let mut fetch_statement = self.conn.prepare("
        SELECT credential FROM PREMSTASH
        ").unwrap();
        let fetch_iter = fetch_statement.query_map(params![], |row|{
            Ok((
                row.get::<usize, String>(0)?,
            ))
        }).unwrap();
        for cred in fetch_iter{
            match cred{
                Ok(res) => {
                    let credential = res;
                    println!("Credential: {:?}", credential.0);
                    res_arr.push(credential.0);
                }
                _ => {
                    return Vec::<String>::new();
                }
            }
        }
        res_arr.push("--End--".to_string());
        res_arr
    }
    
}

#[cfg(test)]
mod tests{
    use super::SqliteDb;

    #[test]
    pub fn test_db_creation(){
        SqliteDb::new();
    }
}