use crate::domain::clip::field;
use rocket::FromForm;
use serde::Serialize;

#[derive(Debug,Serialize,FromForm)]
pub struct NewClip{
    pub content : field::Content,
    pub title : field::Title,
    pub expires : field::Expires,
    pub password : field::Password,
}

#[derive(Debug,Serialize,FromForm)]
pub struct GetPasswordProtectedClip {
    pub password : field::Password,
}

// The from form macro allows us to use a struct within a url route in rocket
// If the user enters invalid argument either the route won't be called 
// or form will not be made available inside the route 