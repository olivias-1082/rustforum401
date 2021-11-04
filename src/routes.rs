use router::Router;



pub fn gen_router() -> Router {

    let mut router = Router::new();

    router.get("/", topic_list::render_default_topic_list, "render_default_topic_list");
    router.get("/:username/comments", topic_list::render_user_comments, "render_user_comments");


    router.get("/login", login::render_login, "render_login");
    router.post("/login", login::login, "login");

    router.get("/register", register::render_register, "render_register");
    router.post("/register", register::register, "register");



    router.get("/logout", logout::logout, "logout");


    router.post("/create-comment", authorize(comment::create_comment, true, false), "create_comment");
    router.get("/edit-comment/:comment_id", authorize(comment::render_edit_comment, true, false), "render_edit_comment");
    router.put("/edit-comment/:comment_id", authorize(comment::edit_comment, true, false), "edit_comment");
    router.delete("/delete-comment/:comment_id", authorize(comment::delete_comment, true, false), "delete_comment");

    router.get("/:username/message/unread", authorize(message::render_unread_message, true, false), "render_unread_message");
    router.get("/read-message/:message_id", authorize(message::read_message, true, false), "read_message");
    router.get("/read-all-message", authorize(message::read_all_message, true, false), "read_all_message");

    router.get("/user/:username", user::render_user, "render_user");
    router.put("/user/update", authorize(user::update_user_info, true, false), "update_user_info");
    router.put("/user/change-password", authorize(user::change_password, true, false), "change_password");

    router.get("/reset-password", reset_password::render_reset_password, "render_find_password");
    router.post("/reset-password", reset_password::send_reset_password_email, "send_reset_password_email");
    router.get("/set-new-password", reset_password::render_set_new_password, "render_set_new_password");
    router.post("/set-new-password", reset_password::set_new_password, "set_new_password");


    router.get("/*", error::render_not_found, "render_not_found");

    router
}
