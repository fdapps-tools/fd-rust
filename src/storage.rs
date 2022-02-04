
use rusqlite::{params, Connection, Result};
use serde::{Serialize, Deserialize};
use serde_rusqlite::{from_rows};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Node {
    id: i32,
    host: String,
    lastcheck: String,
    hash: String,
}

pub fn setup() -> Result<()> {
    let path = "./my_db.sqlite";
    let conn = Connection::open(&path)?;
    println!("is_autocommit: {}", conn.is_autocommit());

    conn.execute(
        "CREATE TABLE IF NOT EXISTS nodes (
                  id              INTEGER PRIMARY KEY,
                  host            TEXT NOT NULL,
                  lastcheck       TEXT NOT NULL,
                  hash            TEXT NOT NULL
                  )",
        [],
    )?;

    Ok(())
}


pub fn node_add(node: Node) -> Result<()> {
    let path = "./my_db.sqlite";
    let conn = Connection::open(&path)?;

    conn.execute(
        "INSERT INTO nodes (host, lastcheck, hash)
       VALUES (?1, ?2, ?3)",
        params![node.host, node.lastcheck, node.hash],
    )?;

    Ok(())
}

pub fn node_list() -> Result<Node, rusqlite::Error> {
    let path = "./my_db.sqlite";
    let conn = Connection::open(&path)?;

    let mut stmt = conn.prepare("SELECT id, hash, host, lastcheck FROM nodes").unwrap();
    let mut res = from_rows::<Node>(stmt.query([]).unwrap());

    Ok(res.next().unwrap().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_creation() {
        assert_eq!(setup(), Ok(()));
    }

    #[test]
    fn test_add_node() {
        let node = Node {
            id: 0,
            host: "Steven".to_string(),
            lastcheck: "12".to_string(),
            hash: "12".to_string(),
        };
        assert_eq!(node_add(node), Ok(()));
    }

    #[test]
    fn test_list_nodes() {
        assert_eq!(node_list(), Ok(()));
    }
}
