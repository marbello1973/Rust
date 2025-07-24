fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("mi nombre"),
        email: String::from("arroba@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another@gmail.com");

    // ceando instancia de actualizacion
    let mut user2 = User {
        email: String::from("otroemail@gmail.com"),
        ..user1
    };

    user2.username = String::from("username");
    user2.active = true;
    user2.sign_in_count = 2;

    // usando las extructiras de tupls y point
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let always = AlwaysEquals;
    println!("Color: {:?}, {:?}, {:?}", black, origin, always);

    

}

// Definiendo el struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

// defineindo el metodo para la estructura
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// definiendo estructuras de tuplas
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

// definir extructuras sin campos
#[derive(Debug)]
struct AlwaysEquals;
