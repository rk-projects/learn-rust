
struct User {
    username: String,
    email: String,
    isactive: bool,
    sign_in_count: u32,
}

fn main() {

    let rk = User {
        username: String::from("rkgade"),
        email: String::from("rkgade@email.com"),
        isactive: true,
        sign_in_count: 1,
    };

    let someone_like_rk = User {
        username: String::from("someone"),
        email: String::from("someone@something.com"),
        ..rk
    };

    println!("{}", rk.email );
    println!("",someone_like_rk );
    
    // Don't use references as values for struct fields. 
    // This can be done using lifetimes, but that comes later

}
