mod pizza_order {

    pub struct Pizza  {
        pub dough: String,
        pub cheese: String,
        pub topping: String,

    }

    impl Pizza {
        pub fn lunch( topping: &str) -> Pizza {
            Pizza {
                dough:  String::from("The regular"),
                cheese: "Mozzarella".to_string(),
                topping: String::from(topping ),
            }
        }
        pub fn meet( topping: &str, cheese: &str) -> Pizza {
            Pizza {
                dough:  String::from("The regular"),
                cheese: String::from(cheese),
                topping: String::from(topping ),
            }
        }
    }


 pub mod help_customer {


    fn seat_at_table() {
        println!("Customer seated at table ");
    }

    pub fn take_order(){
        seat_at_table(); 
        let cust_pizza :super::Pizza = super::Pizza::lunch("Pepperoni");
        // Todo: creo la pizza personalziada de merienda que es meet
        let cust_pizza_meet :super::Pizza = super::Pizza::meet("Queso","Mozzarella"   );

        serve_customer(cust_pizza);
         //Todo: es la funciòn para pedir una   pizza ( personalizad personalziada) 
        //serve_customer(cust_pizza_meet);

    }
    
    fn serve_customer(pizza: super::Pizza) {
        println!("Here is your order: ");
        println!("Dough: {}", pizza.dough);
        println!("Cheese: {}", pizza.cheese);
        println!("Topping: {}", pizza.topping);
    }
 }


}


//Todo: permite que sea publcia y se pueda llamar desde otro archivo

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}