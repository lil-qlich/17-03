use crate::{Admin, Client};
use std::io;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::{fs::{File, OpenOptions}, io::{Write, stdin, Read}};
use std::os::windows::prelude::FileExt;
use crate::check_choise;

pub fn entrance(entrance1:String, admin1: String, admin2:  String, client2: String, client3: String,
client4: String, client5: String){
println!("What is your name?");
let mut entrance: String = String::new();
io::stdin().read_line(&mut entrance).unwrap();
let entrance:String = entrance.trim().parse().unwrap();
// запрос строки
if entrance == "Blesk" {
println!("Welcome, {}", admin1);
println!("изменения дадад Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Добавление товара в каталог и изменение количества товара: Редактор корзины + \n Удаление товара из каталога: Редактор каталога \n Изменить цену товара: Редактор цен \n Помощь по магазину \n Exit - выход ");
}
else if entrance == "Sajaba" {
println!("Welcome, {}", admin2);
println!("Приветствую");
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Добавление товара в каталог и изменение количества товара: Редактор корзины + \n Удаление товара из каталога: Редактор каталога \n Изменить цену товара: Редактор цен \n Помощь по магазину \n Exit - выход ");
}
else if entrance == "Timur" {
println!("Welcome, {}", client2);
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины \n Помощь по магазину \n Exit - выход ");
}
else if entrance == "Max" {
println!("Welcome, {}", client3);
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины  \n Помощь по магазину \n Exit - выход ");
}
else if entrance == "Artem" {
println!("Welcome, {}", client4);
println!("попа Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины \n Exit - выход ");
}
else if entrance == "DeadInside" {
println!("Welcome, {}", client5);
println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Если же Вам нужна будет психологическая помощь наших операторов, то напишите: Псих.помощь \n Exit - выход ");
}

}

pub fn balance(){
let mut balance: String = String::new();
io::stdin().read_line(&mut balance).unwrap();
let balance:String = balance.trim().parse().unwrap();
if balance == "3" {
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
// pub fn psih_help(buy:Vec<String>, shop_balance:u64, spisok2:Vec<String>, prices:Vec<u64>,spisok:Vec<String>,quantitys:Vec<u64>, korzina:Vec<String>){
//     println!("Нам очень жаль, что вам пришлось обратиться за помощью к психологу. Оцените своё состаяние от 1 до 10, где 1-это очень плохо, а 10-это очень хорошо.  ");
//     let mut psihh = String::new();
//     io::stdin().read_line(&mut psihh).unwrap();
//     while psihh.trim() != "Спасибо" {
//         if psihh.trim() == "1" {
//             println!("Ваше состояние говорит о том, что все очень печально. НО ЗАТО, вы уже на самом дне, хуже не будет! Дальнейший путь только вперед, если вы это конечно хотите! Будьте здоровы и любите маму!");
//             return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
//         }  
//         else if psihh.trim() == "2" {
//             println!("Ваше состояние говорит о том, что вы не на самом дне, и если постараться, вы сможете изменить свою жизнь, главное верить в себя и свои силы. Также не мало важно позволять себе отдыхать и ошибаться.");
//             return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
//         }
//         else if psihh.trim() == "3" {
//             println!("Ваше состояние говорит о том, что вы не на самом дне, и если постараться, вы сможете изменить свою жизнь, главное верить в себя и свои силы. Также не мало важно позволять себе отдыхать и ошибаться.");
//             return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
//         }
//         else if psihh.trim() == "4" {
//             println!("Ваше состояние говорит о том, что все не так плохо, и если постараться, вы сможете изменить свою жизнь, главное верить в себя и свои силы. Также не мало важно позволять себе отдыхать и ошибаться.");
//             return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
//         }
//         else if psihh.trim() == "5" {
//             println!("Ваше состояние является среднем, это неплохо. Возможно для полного счастья и умиротворения вам не хватает отдыха или новых эмоций, пробуйте!");
//             return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
//         }
//         else if psihh.trim() == "6" {
//             println!("Возможно для полного счастья и умиротворения вам не хватает отдыха или новых эмоций, пробуйте!");
//             return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
//         }
//         else if psihh.trim() == "7" {
//             println!("Возможно для полного счастья и умиротворения вам не хватает отдыха или новых эмоций, пробуйте!");
//             return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
//         }
//         else if psihh.trim() == "8" {
//             println!("Это крутой результат, все понятно, обратились к нам, чтобы похвастаться, кек! Мы за вас рады!");
//             return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
//         }
//         else if psihh.trim() == "9" {
//             println!("Это крутой результат, все понятно, обратились к нам, чтобы похвастаться, кек! Мы за вас рады!");
//             return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
//         }
//         else if psihh.trim() == "10" {
//             println!("Честно, наша команда вам даже завидует ахахха) А вообще, это очень здорово!)");
//             check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina)
//         }
//         else {
//             println!("Нам очень жаль, но здесь даже мы бессильны... Извините, прощайте, с уважением, команда AQ!");
//             break;
        
//         }
//         // break;
//     }
// }
pub fn shop_help(buy:Vec<String>, shop_balance:u64, spisok2:Vec<String>, prices:Vec<u64>,spisok:Vec<String>,quantitys:Vec<u64>, korzina:Vec<String>){
    println!("Мы удивлены, что вы обратили к нам за помощью касательно нашей торговой точки. \n Мы в вас разочарованы. Ладно уж, что вы хотите узнать. Выберите категорию вопроса: ");
    println!("Где я? - 100 бонусов|Как я сюда попал? - 200 бонусов|Ты глупый или что-то? - 300 бонусов");
    let mut answers_help = String::new();
    io::stdin().read_line(&mut answers_help).unwrap();
    if answers_help.trim() == "Где я?"{
        println!("Вы в магазине AQ, тут есть много чего интересного. Даже психологическая помощь, вот так то");
    }
    else if answers_help.trim() == "Как я сюда попал?" {
        println!("Мы сами не знаем как сюда попали, а вы просто запустили наш код");
    }
    else if answers_help.trim() == "Ты глупый или что-то?" {
        println!("Наверное, может быть, не знаю...");
    }
    else {
        println!("Я не знаю ответа на этот вопрос");
        return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
    }
}
pub fn reg(){
    println!("Приветсвуем с уважением в нашем магазине! Введите логин вашего аккаунта");
    let mut new_login = String::new();
    io::stdin().read_line(&mut new_login);
    let logins = "account.txt"
}