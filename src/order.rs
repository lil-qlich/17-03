use std::io;

use crate::check_choise;

pub fn add_order(buy:Vec<String>, shop_balance:u64, spisok2:Vec<String>, prices:Vec<u64>, spisok:Vec<String>, quantitys:Vec<u64>, korzina:Vec<String>, balance:u128){
    for i in &buy{
        println!("{}", i);
    }
    println!("Ваша покупка на сумму: {}", shop_balance);
    println!("Введите адрес, куда нужно доставить товары");
    let mut locate = String::new();
    io::stdin().read_line(&mut locate).unwrap();
    println!("Спасибо за покупку, доставим в {}.Если хотите заказать что-то еще, то введите 'back'", locate);
    let mut back_check = String::new();
    io::stdin().read_line(&mut back_check).unwrap();
    if back_check.trim() == "back"{
    println!("Введите, то что вы хотите сделать");
    return check_choise(shop_balance, spisok2, buy, prices, spisok, quantitys, korzina, balance); 
    }
    else {
        // println!("До свидания, с уважением!!!1!1!!");
    }
}