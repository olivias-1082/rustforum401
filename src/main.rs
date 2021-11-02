use vial::prelude::*;

routes! {
 
    GET "/" => |_| r#"
<h3>vial form</h3>
<form method="POST" action="/">
    <p><label>Name: <input type="text" name="name"/></label></p>
    <p><label>Post:
    <input type="textarea" name="post"/>
    </label></p>
    <p><input type="submit"/></p>
</form>
    "#;
    POST "/" => |req| format!(r#"
    <h3>results</h3>
    <p><b>Name:</b> {}</p>
    <p><b>Post:</b> {}</p>
    "#,
            req.form("name").unwrap_or(""),
            req.form("post").unwrap_or(""),
        );
    
}



fn main() {
    run!("localhost:2000");
}