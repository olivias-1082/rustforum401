use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;
use models::user::User;

pub fn check_user_login(username: &str, password: &str) -> Option<String> {
    let mut result = SQL_POOL.prep_exec(r#"
                            SELECT
                            password, salt
                            FROM
                            user
                            WHERE username = ?
                            "#, (username, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (password_hashed, salt) = from_row::<(String, String)>(row);
    let password_with_salt = password.to_string() + &*salt;

    if password_hashed != gen_md5(&password_with_salt) {
        return None;
    }

    Some(username.to_string())
}

pub fn update_password(username: &str, password: &str) -> Option<String> {
    let salt = gen_salt();
    let password_with_salt = password.to_string() + &*salt;
    let password_hashed = gen_md5(&password_with_salt);
    let update_time = gen_datetime().to_string();

    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE user SET
                        password = ?,
                        salt = ?,
                        update_time = ?
                        WHERE username = ?
                        "#).unwrap();
    let result = stmt.execute((
        &*password_hashed,
        &*salt,
        &*update_time,
        username
    ));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(username.to_string())
}

pub fn update_retrieve(username: &str, retrieve_token: &str) -> Option<String> {

    let retrieve_time = gen_datetime().to_string();

    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE user SET
                        retrieve_token = ?,
                        retrieve_time = ?
                        WHERE username = ?
                        "#).unwrap();
    let result = stmt.execute((
        retrieve_token,
        &*retrieve_time,
        username
    ));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(username.to_string())
}

pub fn get_retrieve_time(username: &str, retrieve_token: &str) -> Option<NaiveDateTime> {
    let mut result = SQL_POOL.prep_exec("SELECT retrieve_time FROM user WHERE username = ? AND retrieve_token = ? ", (username, retrieve_token)).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (retrieve_time, ) = from_row::<(NaiveDateTime, )>(row);

    Some(retrieve_time)
}

pub fn update_user(username: &str, user: &Value) -> Option<String> {
    let update_time = gen_datetime().to_string();
    let new_username = user["username"].as_str().unwrap();

    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE user SET
                        username = ?,
                        qq = ?,
                        email = ?,
                        site = ?,
                        update_time = ?
                        WHERE username = ?
                        "#).unwrap();
    let result = stmt.execute((
        new_username,
        user["email"].as_str().unwrap(),
        user["site"].as_str().unwrap(),
        &*update_time,
        username
    ));

    if let Err(MySqlError(ref err)) = result {
        println!("{:?}", err.message);
        return None;
    }

    Some(new_username.to_string())
}

pub fn is_user_created(username: &str) -> bool {
    let mut result = SQL_POOL.prep_exec("SELECT count(id) FROM user WHERE username = ?", (username, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return false;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (count, ) = from_row::<(u8, )>(row);

    if count == 0 {
        false
    } else {
        true
    }
}

pub fn get_user_count() -> u16 {
    let mut result = SQL_POOL.prep_exec("SELECT count(id) FROM user", ()).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();

    let (count, ) = from_row::<(u16, )>(row);

    count
}

pub fn get_user_id(username: &str) -> u16 {
    let mut result = SQL_POOL.prep_exec("SELECT id FROM user WHERE username = ?", (username, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return 0;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (id, ) = from_row::<(u16, )>(row);

    id
}



pub fn get_username(id: u16) -> Option<String> {
    let mut result = SQL_POOL.prep_exec("SELECT username FROM user WHERE id = ?", (id, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (username, ) = from_row::<(String, )>(row);

    Some(username.to_string())
}

pub fn get_username_by_email(email: &str) -> Option<String> {
    let mut result = SQL_POOL.prep_exec("SELECT username FROM user WHERE email = ? AND register_source = 0", (email, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let row = row_wrapper.unwrap().unwrap();
    let (username, ) = from_row::<(String, )>(row);

    Some(username.to_string())
}

pub fn get_user(username: &str) -> Option<User> {
    let mut result = SQL_POOL.prep_exec(r#"
                          SELECT * FROM user WHERE username = ?
                          "#, (username, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let mut row = row_wrapper.unwrap().unwrap();

    Some(User {
        id: row.get::<u16, _>(0).unwrap(),
        username: row.get::<String, _>(1).unwrap(),
        email: row.get::<String, _>(7).unwrap(),        
        site: row.get::<String, _>(11).unwrap(),
        create_time: row.get::<NaiveDateTime, _>(15).unwrap(),
        update_time: row.get::<NaiveDateTime, _>(16).unwrap()
    })
}

pub fn create_user(user: &Value) -> Option<String> {
    let username = user["username"].as_str().unwrap();

    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO user
                        (username, email,  password, salt, site, create_time, update_time)
                        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                        "#).unwrap();

    let result = stmt.execute((
        username,
        user["email"].as_str().unwrap(),
        user["password_hashed"].as_str().unwrap(),
        user["salt"].as_str().unwrap(),
        &*check_and_get_string(&user["site"]),
        user["create_time"].as_str().unwrap(),
    ));

    if let Err(MySqlError(ref err)) = result {
        if err.code == 1062 {
            return None;
        } else {
            panic!("{:?}", err.message);
        }
    }

    Some(username.to_string())
}