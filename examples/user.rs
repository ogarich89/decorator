use decorator::dynamic_decorator::{Info, NicknameDecorator, SurnameDecorator, User};

fn main() {
    let with_surname = SurnameDecorator::new(Box::new(User), "Ivanov");
    let with_nickname = NicknameDecorator::new(Box::new(with_surname), "ivan1973");

    let name = "Ivan";
    let user = with_nickname.info(name);
    println!("User: {}", user)
}
