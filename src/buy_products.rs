
use crate::{check_choise};
use std::{io};

pub fn buy_product(check_buy:String,mut buy:Vec<String>,spisok2:Vec<String>, shop_balance:u64, prices:Vec<u64>,ask_choise:String, spisok:Vec<String>,mut quantitys:Vec<u64>, korzina:Vec<String> ){
    if check_buy.trim() == "Купить" {
        println!("Введите номер товара, 1-ый товар начинается с '0'");
        let mut check2 = String::new();
        io::stdin().read_line(&mut check2).unwrap();
        let check:usize = check2.trim().parse().unwrap();
        let product12 = &spisok2[check];
        let product12 = String::from(product12);
        buy.insert(0, product12);
        println!("Введите количество товара, которое вы хотите");
        let mut check_quantity = String::new();
        io::stdin().read_line(&mut check_quantity).unwrap();
        let check_quantity2:u64 = check_quantity.trim().parse().unwrap();
        let shop_balance = shop_balance + (check_quantity2 * prices[check]);
        let new_quantity = quantitys[check] - check_quantity2;
        quantitys[check] = new_quantity;
        println!("Ваша корзина:");
        for products in &buy{
            println!("{}", products);
        }
        println!("Ваши покупки на сумму: {}", shop_balance); 
        println!("Введите то, что вы хотите сделать");
        return check_choise( shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
                }
        else {
            return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
        }
    }


pub fn delete_product(mut buy:Vec<String>, shop_balance:u64, spisok2:Vec<String>, prices:Vec<u64>, spisok:Vec<String>, quantitys:Vec<u64>, korzina:Vec<String>){
    println!("Введите номер товара, который хотите удалить");
    let mut check_delete = String::new();
    io::stdin().read_line(&mut check_delete).unwrap();
    let check_delete2:usize = check_delete.trim().parse().unwrap();
    if check_delete.trim() == "0"{
        buy.remove(check_delete2);
        println!("Вы удалили товар");
        for i in &buy{
            println!("{}",i);
        }
        return check_choise( shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
    }
    else {
        println!("Такого товара нету в вашей корзине");
        return check_choise( shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
    }
}
pub fn edit_catalog( buy:Vec<String>, shop_balance:u64, spisok2:Vec<String>, prices:Vec<u64>,mut spisok:Vec<String>, mut quantitys:Vec<u64>, mut korzina:Vec<String>) {
        println!("Введите то, что вы хотите сделать \n 'Добавить товар', \n 'Изменить количество товара'");
        let mut check_edit = String::new();
        io::stdin().read_line(&mut check_edit).unwrap();
        if check_edit.trim() == "Добавить товар" {
            println!("Введите товар, который вы хотите добавить в каталог");
            let mut new_products = String::new();
            io::stdin().read_line(&mut new_products).unwrap();
            spisok.insert(1, new_products);
            println!("Введите количество товара");
            let mut kol_products = String::new();
            io::stdin().read_line(&mut kol_products).unwrap();
            let kol_products:u64 = kol_products.trim().parse().unwrap();
            quantitys.insert(1, kol_products);
            for i in &spisok {
                println!("{}",i);
            }
            for i in &quantitys{
                println!("{}",i);
            }
            return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
        }
        else if check_edit.trim() == "Изменить количество товара" {
            println!("Введите номер товара, количество которого вы хотите отредактировать");
            let mut check_edits2 = String::new();
            io::stdin().read_line(&mut check_edits2).unwrap();
            let check_edits2: usize = check_edits2.trim().parse().unwrap();
            println!("Введите количество вашего товара");
            let mut edit_kol_tovara = String::new();
            io::stdin().read_line(&mut edit_kol_tovara).unwrap();
            let edit_kol_tovara: u64 = edit_kol_tovara.trim().parse().unwrap();
            quantitys[check_edits2] = edit_kol_tovara;
            return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
        }
        else {
            println!("Повторите попытку");
            return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
        }
}
pub fn delete_products(buy:Vec<String>, shop_balance:u64, spisok2:Vec<String>, prices:Vec<u64>,mut spisok:Vec<String>, mut quantitys:Vec<u64>, korzina:Vec<String>){
    println!("Введите номер товара, который вы хотите удалить из каталога");
    let mut check_delete_products = String::new();
    io::stdin().read_line(&mut check_delete_products).unwrap();
    let delete_tovar:usize = check_delete_products.trim().parse().unwrap();
    spisok.remove(delete_tovar);
    quantitys.remove(delete_tovar);
    for i in &buy{
        println!("{}", i);
    }
    return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina);
}