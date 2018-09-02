use diesel::insert_into;
use djangohashers::make_password;
use repository::establish_connection;
use schema::users;
use time::get_time;
use time::Duration;
use uuid::{NAMESPACE_DNS, Uuid};

const UUID_NAMESPACE: &str = "REALWORD_WARP";
const JWT_ISSUER: &str = "REALWORD_WARP";
const JWT_AUDIENCE: &str = "AUDIENCE";
const JWT_ROLE: &str = "user";

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
#[serde(remote = "Self")]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
    #[serde(skip)]
    passhash: String,
    #[serde(skip)]
    user_id: String
}
deserialize_with_root!("user": NewUser);

#[derive(Debug, Serialize)]
#[serde(remote = "Self")]
pub struct User {
    #[serde(skip)]
    id: u64,
    user_id: String,
    #[serde(skip)]
    passhash: String,
    email: String,
    token: String,
    username: String,
    bio: String,
    image: String
}
serialize_with_root!("user": User);

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    jti: String,
    sub: String,
    iss: String,
    aud: String,
    name: String,
    role: String,
    iat: i64,
    nbf: i64,
    exp: i64,
}
 
impl Token {
    fn new(user_id: String, user_name: String) -> Token {
        let jti = Uuid::new_v5(&NAMESPACE_DNS, UUID_NAMESPACE).to_string();
        let iat = get_time();
        let exp = iat + Duration::hours(1);
        Token {
            jti: jti,
            sub: user_id,
            iss: String::from(JWT_ISSUER),
            aud: String::from(JWT_AUDIENCE),
            name: user_name,
            role: String::from(JWT_ROLE),
            iat: iat.sec,
            nbf: iat.sec,
            exp: exp.sec
        }
    }
}

impl NewUser {
    pub fn register(new_user: NewUser) -> User {
        let conn = establish_connection();
        let user_id = Uuid::new_v5(&NAMESPACE_DNS, UUID_NAMESPACE).to_string();
        let hashed_password = make_password(&new_user.password);
        insert_into(users::table)
            .values(new_user.with_hashed_password(hashed_password)
                    .with_user_id(user_id))
            .get_result(conn)
            .expect("Error saving new post");
            User {
            email: String::from(""),
            token: String::from(""),
            username: String::from(""),
            bio: String::from(""),
            image: String::from("")
        }
    }

    fn with_hashed_password(self: NewUser, hashed_password: String) -> NewUser {
        self.hashed_password = hashed_password;
        self
    }

    fn with_user_id(self: NewUser, user_id: Uuid) -> NewUser {
        self.user_id = user_id.to_string();
        self
    }
}
