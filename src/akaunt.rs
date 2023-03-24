use crate::{Admin, Client};
use std::io;
use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::ptr::read;
use std::{fs::{OpenOptions}, io::{stdin, Read}};
use std::os::windows::prelude::FileExt;
use crate::check_choise;
use std::fs;

pub fn entrance(entrance1:String, admin1: String, admin2:  String, client2: String, client3: String,
client4: String, client5: String){
println!("What is your name?");
// let mut entrance: String = String::new();
// io::stdin().read_line(&mut entrance).unwrap();
// let mut entrance:String = entrance.trim().parse().unwrap();
let mut check_logins = String::new();
io::stdin().read_line(&mut check_logins).unwrap();
let mut all_logins = File::open(r"C:\Users\sajan\OneDrive\Рабочий стол\Колледж\АР\17-03\src\account.txt").unwrap();
all_logins.read_to_string(&mut check_logins).unwrap();
// запрос строки

if check_logins.contains("Blesk")&& check_logins == "Blesk"{
println!("Welcome, {}", admin1);
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Добавление товара в каталог и изменение количества товара: Редактор корзины + \n Удаление товара из каталога: Редактор каталога \n Изменить цену товара: Редактор цен \n Помощь по магазину \n Exit - выход ");
}
else if check_logins.contains("Sajaba") && check_logins == "Sajaba" {
println!("Welcome, {}", admin2);
println!("Приветствую");
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Добавление товара в каталог и изменение количества товара: Редактор корзины + \n Удаление товара из каталога: Редактор каталога \n Изменить цену товара: Редактор цен \n Помощь по магазину \n Exit - выход ");
}
else if check_logins.contains(&check_logins){
    println!("Welcome");
    println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Если же Вам нужна будет психологическая помощь наших операторов, то напишите: Псих.помощь \n Exit - выход ");
}
else {
    println!("Вы не зарегистрированы в нашем магазине, пожалуйста пройдите регистрацию");
    return reg();
}
}
// else if entrance == "DeadInside" {
// println!("Welcome, {}", client5);
// println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Если же Вам нужна будет психологическая помощь наших операторов, то напишите: Псих.помощь \n Exit - выход ");
// }



pub fn balance(){
let mut balance: String = String::new();
io::stdin().read_line(&mut balance).unwrap();
let balance:String = balance.trim().parse().unwrap();
if balance == "Баланс" {
println!("Введите на какую сумму хотите пополнить ваш баланс");
let mut new_balance: String = String::new();
io::stdin().read_line(&mut new_balance).unwrap();
println!("Ваш баланс равен: {}", new_balance);
}
}

pub fn exit(entrance1:String, admin1: String, admin2: String, client2: String, client3: String,
    client4: String, client5: String, korzina:Vec<String>){
let mut exit: String = String::new();
io::stdin().read_line(&mut exit).unwrap();
let exit:String = exit.trim().parse().unwrap();

if exit == "Exit" {
println!("До свидания, с уважением!");
return entrance(entrance1, admin1, admin2, client2, client3, client4, client5);
}
}


pub fn shop_help(buy:Vec<String>, shop_balance:u64, spisok2:Vec<String>, prices:Vec<u64>,spisok:Vec<String>,quantitys:Vec<u64>, korzina:Vec<String>, balance:u128){
    println!("Мы удивлены, что вы обратили к нам за помощью касательно нашей торговой точки. \n Мы в вас разочарованы. Ладно уж, что вы хотите узнать. Выберите категорию вопроса: ");
    println!("Где я? \n Как я сюда попал? \n Ты глупый или что-то?");
    let mut answers_help = String::new();
    io::stdin().read_line(&mut answers_help).unwrap();
    if answers_help.trim() == "Где я?"{
        println!("Вы в магазине AQ, тут есть много чего интересного. Даже психологическая помощь, вот так то");
        return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina, balance);
    }
    else if answers_help.trim() == "Как я сюда попал?" {
        println!("Мы сами не знаем как сюда попали, а вы просто запустили наш код");
        return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina, balance);
    }
    else if answers_help.trim() == "Ты глупый или что-то?" {
        println!("Наверное, может быть, не знаю...");
        return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina, balance);
    }
    else {
        println!("Я не знаю ответа на этот вопрос");
        return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina, balance);
    }
}

pub fn reg(){
    println!("Приветствуем с уважением в нашем магазине! Введите логин вашего аккаунта");
    let mut f = OpenOptions::new().read(true).write(true).create(true).append(true).open(r"C:\Users\sajan\OneDrive\Рабочий стол\Колледж\АР\17-03\src\account.txt").expect("msg");
    let mut new_login = String::new();
    let mut news = new_login.as_bytes();
    io::stdin().read_line(&mut new_login).expect("error");
    // f.write_all(new_login.as_bytes()).expect("msg");
    f.write(new_login.as_bytes()).expect("msg");
    
}