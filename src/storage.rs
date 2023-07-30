use rusqlite::{Connection, Result};
use new_dice_roller::misc::Error;


pub struct Storage
{
    database: Connection
}




impl Storage
{
    fn unix_timestamp() -> i64
    {
        let start = std::time::SystemTime::now();


        start
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards").as_secs() as i64
    }
    pub fn open() -> Result<Self, Error>
    {
        let path = std::env::var("DATABASE_PATH").expect("Expected a path to a mysql database.");
        let database = Connection::open(path)?;
        database.execute(
        "CREATE TABLE IF NOT EXISTS registered_commands (
             id INTEGER PRIMARY KEY,
             userid INTEGER,
             command_name TEXT NOT NULL,
             command TEXT NOT NULL,
             date INTEGER,
             UNIQUE(userid, command_name)
         )", ()
        )?;
        
        Ok(Self{database})
    }

    pub fn store_command(&self, user: i64, key: &str, cmd: &str) -> Result<(), Error>
    {
        self.database.execute(
            "INSERT OR REPLACE INTO registered_commands (userid, command_name, command, date)
                     VALUES (?1, ?2, ?3, ?4)",
            (user, key, cmd, &format!("{}", Self::unix_timestamp())),
        )?;
        Ok(())
    }
    
    pub fn load_command(&self, user: i64, key: &str) -> Result<Option<String>, Error>
    {


        let mut stmt = self.database.prepare(
            "SELECT userid, command_name, command, date FROM registered_commands
             WHERE userid=(?1) AND command_name=(?2)
             ORDER BY date DESC;"
        )?;
        
        let cmd: Option<String> = stmt.query_map((user, key), |row| {
            row.get(2)
        })?.filter_map(|result| result.ok()).next();

        Ok(cmd)
    }
    pub fn load_all_command(&self, user: i64) -> Result<Vec<(String, String)>, Error>
    {


        let mut stmt = self.database.prepare(
            "SELECT userid, command_name, command, date FROM registered_commands
             WHERE userid=(?1)
             ORDER BY date DESC;"
        )?;
        
        let cmds: Vec<(String, String)>
            = stmt.query_map([user],
                             |row|
                             {
                                 let maybe_key: Result<String, _> = row.get(1);
                                 let maybe_cmd: Result<String, _> = row.get(2);
                                 match (maybe_key, maybe_cmd)
                                 {
                                     (Ok(key), Ok(cmd)) => Ok((key, cmd)),
                                     (Err(e), _) | (_, Err(e)) => Err(e)
                                 }
                             })?
            .filter_map(|tuple| tuple.ok()).collect(); 
        
        Ok(cmds)
    }
}





