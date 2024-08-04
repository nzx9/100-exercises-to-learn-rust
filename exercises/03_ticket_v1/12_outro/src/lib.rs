// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 characters.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        Order::val_pn(&product_name);
        Order::val_qty(&quantity);
        Order::val_up(&unit_price);
        Order {
            product_name,
            quantity,
            unit_price,
        }
    }
    pub fn total(&self) -> u32 {
        self.quantity * self.unit_price
    }

    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    pub fn set_product_name(&mut self, name: String) {
        Order::val_pn(&name);
        self.product_name = name.into();
    }

    pub fn set_quantity(&mut self, qty: u32) {
        Order::val_qty(&qty);
        self.quantity = qty;
    }

    pub fn set_unit_price(&mut self, price: u32) {
        Order::val_up(&price);
        self.unit_price = price;
    }

    fn val_pn(name: &String) {
        if name.is_empty() {
            panic!("Name can't be empty")
        }
        if name.len() > 300 {
            panic!("Name must be less than 300!")
        }
    }
    fn val_qty(qty: &u32) {
        if !(*qty > 0) {
            panic!("Quantity must be strictly greater than zero")
        }
    }
    fn val_up(price: &u32) {
        if !(*price > 0) {
            panic!("Unit price must be strictly greater than zero")
        }
    }
}
