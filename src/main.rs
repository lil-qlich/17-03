mod sclad;
use std::fs;

use std::vec;
use akaunt::reg;
use crate::fs::OpenOptions;
use psih::psih_help;
use akaunt::shop_help;
use buy_products::buy_product;
use buy_products::delete_products;
use buy_products::edit_catalog;
use order::add_order;
use crate::buy_products::delete_product;
use sclad::Product;
mod buy_products;
use std::io;
mod akaunt;
use akaunt::entrance;
use akaunt::exit;
mod order;
mod psih;
use crate::akaunt::balance;

pub struct Admin{
    pub name: String,
    pub balance: u16,
    }
    
pub struct Client{
    pub name: String,
    pub balance: u16,
    }

fn main() {   

    let admin1 = Admin{name:"Blesk".to_string(), balance: 0};
    let admin2 = Admin{name:"Sajaba".to_string(), balance: 0};
    let client1 = Client{name:"DeadInside".to_string(), balance: 0};
    let client2 = Client{name:"Timur".to_string(), balance: 0};
    let client3 = Client{name:"Max".to_string(), balance: 0};
    let client4 = Client{name:"Artem".to_string(), balance: 0};
    let client5 = Client{name:"DeadInside".to_string(), balance: 0};
    let new_client1 = client1.name;
    let new_client2 = client2.name;
    let new_client3 = client3.name;
    let new_client4 = client4.name;
    let new_client5 = client5.name;
    let new_admin = admin1.name;
    let new_admin2 = admin2.name;

    reg();

        let i = 1;
        if i == 1{
            entrance(new_client1.clone(), new_admin.clone(), new_admin2.clone(), new_client2.clone(), new_client3.clone(), new_client4.clone(), new_client5.clone());
        }   

    let product1 = Product{name:"Апельсины".to_string(), price: 60, quantity: 20};
    let product2 = Product{name:"Дмитрий Ножкин".to_string(), price: 10000000, quantity: 99};
    let product3 = Product{name:"Куриная грудка".to_string(), price: 120, quantity: 10};
    let product4 = Product{name:"Бутылка воды".to_string(), price:30, quantity:15};
    let tovar1 = product1.name;
    let tovar2 = product2.name;
    let tovar3 = product3.name;
    let tovar4 = product4.name;

        let mony = 1;
        if mony == 1{
            balance();
        }

    let quantitys = vec![product1.quantity, product2.quantity, product3.quantity, product4.quantity];
    let prices:Vec<u64> = vec![product1.price, product2.price, product3.price, product4.price];
    let shop_balance:u64 = 0;
    let spisok = vec![tovar1.to_string(), tovar2.to_string(), tovar3.to_string(), tovar4.to_string()];
    let spisok2= spisok.clone();
    let mut balance:u128 = 0;

    let buy: Vec<String>= vec![];
    let korzina: Vec<String> = vec![];
    let mut ask_choice = String::new();
    io::stdin().read_line(&mut ask_choice).unwrap();
    
    check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina.clone(), balance);

        let i1 = 1;
        if i1 == 1{
            exit(new_client1, new_admin, new_admin2, new_client2, new_client3, new_client4, new_client5, korzina);
        } 
}
pub fn check_choise( shop_balance:u64, spisok2:Vec<String>, buy:Vec<String>, prices:Vec<u64>, spisok:Vec<String>, quantitys:Vec<u64>, korzina:Vec<String>, balance:u128){
    let mut ask_choice = String::new();
    io::stdin().read_line(&mut ask_choice).unwrap();
    while ask_choice.trim() != "Exit" {
    if ask_choice.trim() == "Просмотр" {
        println!("В нашем магазине есть следующие товары:");
        for tovar in &spisok.clone() {
            println!("{}", tovar);
        }
        for i in &quantitys.clone(){
            println!("количество: {}", i);
        }
        println!("Если вы хотите купить товар, то напишите 'Купить'");
        let mut check_buy = String::new();
        io::stdin().read_line(&mut check_buy).unwrap();
        
        return buy_product(check_buy, buy.clone(), spisok2.clone(), shop_balance, prices, ask_choice, spisok, quantitys.clone(), korzina, balance);
    }
    else if ask_choice.trim() == "Корзина" {
        for i in &buy {
            println!("{}",i);
        }
        for l in  &korzina {
        return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina, balance);
        } 
    }
    else if ask_choice.trim() == "Заказать" {
        add_order(buy, shop_balance, spisok2, prices, spisok, quantitys, korzina, balance);
        return;
    }
    else if ask_choice.trim() == "Редактор корзины" {
        delete_product(buy, shop_balance, spisok2, prices, spisok, quantitys, korzina, balance);
        return;
    }

    else if ask_choice.trim() == "Редактор каталога" {
        edit_catalog(buy.clone(), shop_balance.clone(),spisok2.clone(), prices.clone(), spisok.clone(),quantitys.clone(), korzina.clone(), balance);
    }
    else if ask_choice.trim() == "Редактор товара"{
        delete_products(buy.clone(), shop_balance, spisok2.clone(), prices.clone(), spisok.clone(), quantitys.clone(), korzina.clone(), balance);
    }
    else if ask_choice.trim() == "Псих.помощь" {
        psih_help(buy.clone(), shop_balance, spisok2.clone(), prices.clone(), spisok.clone(), quantitys.clone(), korzina.clone(), balance.clone());
    }
    else if ask_choice.trim() == "Помощь по магазину" {
        shop_help(buy.clone(), shop_balance, spisok2.clone(), prices.clone(), spisok.clone(), quantitys.clone(), korzina.clone(), balance);
    }
    else {
        println!("Вам доступны следующие функции: \n Просмотр товаров: Просмотр \n Просмотр вашей корзины: Корзина \n Пополнить и просмотреть ваш баланс: Баланс \n Перейти к оформлению заказа: Заказать \n Удаление товаров из корзины: Редактор корзины - \n Если же Вам нужна будет психологическая помощь наших операторов, то напишите: Псих.помощь \n Exit - выход");
        return;
    }
}
}
