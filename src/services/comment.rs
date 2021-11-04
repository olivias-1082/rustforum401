use mysql::from_row;
use mysql::error::Error::MySqlError;
use serde_json::Value;
use chrono::NaiveDateTime;

use common::utils::*;
use common::lazy_static::SQL_POOL;
use models::comment::Comment;

pub fn create_comment(comment: &Value) -> Option<String> {

    let create_time = gen_datetime().to_string();
    let topic_id = comment["topic_id"].as_str().unwrap();
    let comment_id = gen_md5(&*(topic_id.to_string() + &*create_time));

    let mut stmt = SQL_POOL.prepare(r#"
                        INSERT INTO comment
                        (id, user_id, topic_id, content, create_time, update_time)
                        VALUES (?, ?, ?, ?, ?, ?);
                        "#).unwrap();

    let result = stmt.execute((
        &*comment_id,
        comment["user_id"].as_str().unwrap(),
        topic_id,
        comment["content"].as_str().unwrap(),
        &*create_time,
        &*create_time
    ));

    if let Err(MySqlError(ref err)) = result {

        println!("{:?}", err.message);
        return None;
    }

    Some(comment_id)
}

pub fn update_comment(comment_id: &str, comment: &Value) -> Option<String> {

    let update_time = gen_datetime().to_string();

    let mut stmt = SQL_POOL.prepare(r#"
                        UPDATE comment SET
                        content = ?,
                        update_time = ?
                        WHERE id = ?
                        "#).unwrap();
    let result = stmt.execute((
        comment["content"].as_str().unwrap(),
        &*update_time,
        comment_id
    ));

    if let Err(MySqlError(ref err)) = result {

        println!("{:?}", err.message);
        return None;
    }

    Some(comment_id.to_string())
}

pub fn delete_comment(comment_id: &str) -> Option<String> {

    let result = SQL_POOL.prep_exec("DELETE FROM comment WHERE id = ?", (comment_id, ));

    if let Err(MySqlError(ref err)) = result {

        println!("{:?}", err.message);
        return None;
    }

    Some(comment_id.to_string())
}

pub fn is_comment_created(comment_id: &str) -> bool {

    let mut result = SQL_POOL.prep_exec("SELECT count(id) FROM comment WHERE id = ?", (comment_id, )).unwrap();
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

pub fn get_comment(comment_id: &str) -> Option<Comment> {

    let mut result = SQL_POOL.prep_exec(r#"
                          SELECT
                          c.id, user_id, username, avatar_url, topic_id, content,
                          (SELECT count(id) FROM comment_vote WHERE state = 1 AND comment_id = c.id) AS agree_count,
                          (SELECT count(id) FROM comment_vote WHERE state = -1 AND comment_id = c.id) AS disagree_count,
                          c.status, c.create_time, c.update_time
                          FROM comment AS c
                          LEFT JOIN
                          user AS u
                          ON c.user_id = u.id
                          WHERE c.id = ?
                          "#, (comment_id, )).unwrap();
    let row_wrapper = result.next();

    if row_wrapper.is_none() {
        return None;
    }

    let mut row = row_wrapper.unwrap().unwrap();

    Some(Comment {
        id: row.get::<String, _>(0).unwrap(),
        user_id: row.get::<u16, _>(1).unwrap(),
        username: row.get::<String, _>(2).unwrap(),
        avatar_url: row.get::<String, _>(3).unwrap(),
        topic_id: row.get::<String, _>(4).unwrap(),
        content: parse_to_html(&*row.get::<String, _>(5).unwrap()),
        agree_count: row.get::<u16, _>(6).unwrap(),
        disagree_count: row.get::<u16, _>(7).unwrap(),
        status: row.get::<u8, _>(8).unwrap(),
        create_time: row.get::<NaiveDateTime, _>(9).unwrap(),
        update_time: row.get::<NaiveDateTime, _>(10).unwrap()
    })
}

pub fn get_comment_content(comment_id: &str) -> Option<String> {

    let mut result = SQL_POOL.prep_exec("SELECT content FROM comment WHERE id = ?", (comment_id, )).unwrap();
    let row_wrapper = result.next();
    if row_wrapper.is_none() {
        return None;
    }
    let mut row = row_wrapper.unwrap().unwrap();
    Some(row.get::<String, _>(0).unwrap())
}

pub fn get_comments_by_topic_id(topic_id: &str) -> Vec<Comment> {
    let result = SQL_POOL.prep_exec(r#"
                          SELECT
                          c.id, user_id, username, avatar_url, topic_id, content,
                          (SELECT count(id) FROM comment_vote WHERE state = 1 AND comment_id = c.id) AS agree_count,
                          (SELECT count(id) FROM comment_vote WHERE state = -1 AND comment_id = c.id) AS disagree_count,
                          c.status, c.create_time, c.update_time
                          FROM comment AS c
                          LEFT JOIN
                          user AS u
                          ON c.user_id = u.id
                          WHERE topic_id = ?
                          ORDER BY create_time ASC
                          "#, (topic_id, )).unwrap();

    result.map(|row_wrapper| row_wrapper.unwrap())
        .map(|mut row| {
            Comment {
                id: row.get::<String, _>(0).unwrap(),
                user_id: row.get::<u16, _>(1).unwrap(),
                username: row.get::<String, _>(2).unwrap(),
                avatar_url: row.get::<String, _>(3).unwrap(),
                topic_id: row.get::<String, _>(4).unwrap(),
                content: parse_to_html(&*row.get::<String, _>(5).unwrap()),
                agree_count: row.get::<u16, _>(6).unwrap(),
                disagree_count: row.get::<u16, _>(7).unwrap(),
                status: row.get::<u8, _>(8).unwrap(),
                create_time: row.get::<NaiveDateTime, _>(9).unwrap(),
                update_time: row.get::<NaiveDateTime, _>(10).unwrap()
            }
        })
        .collect()
}
pub fn get_comment_count() -> u64 {

    let mut result = SQL_POOL.prep_exec("SELECT count(id) FROM comment", ()).unwrap();
    let row_wrapper = result.next();
    if row_wrapper.is_none() {
        return 0;
    }
    let row = row_wrapper.unwrap().unwrap();
    let (count, ) = from_row::<(u64, )>(row);
    count
}
pub fn get_last_comment_by_topic_id(topic_id: &str) -> Option<Value> {
    let mut result = SQL_POOL.prep_exec(r#"
                          SELECT
                          c.id, user_id, username, avatar_url, c.create_time
                          FROM comment AS c
                          LEFT JOIN
                          user AS u
                          ON c.user_id = u.id
                          WHERE topic_id = ?
                          ORDER BY create_time DESC LIMIT 1
                          "#, (topic_id, )).unwrap();
    let row_wrapper = result.next();
    if row_wrapper.is_none() {
        return None;
    }
    let mut row = row_wrapper.unwrap().unwrap();
    Some(json!({
        "id": row.get::<String, _>(0).unwrap(),
        "user_id": row.get::<u16, _>(1).unwrap(),
        "username": row.get::<String, _>(2).unwrap(),
        "avatar_url": row.get::<String, _>(3).unwrap(),
        "create_time": row.get::<NaiveDateTime, _>(4).unwrap(),
    }))
}