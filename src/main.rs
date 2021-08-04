// Create a request class
#[derive(Debug)]
struct OrderDetail {
    name: String,
    quantity: u8,
}

impl Default for OrderDetail {
    fn default() -> Self {
        Self {
            name: "ABC".into(),
            quantity: 10,
        }
    }
}

impl OrderDetail {
    fn buy(&self) {
        println!(
            "Stock [ Name: {}, Quantity: {}] bought",
            self.name, self.quantity
        );
    }

    fn sell(&self) {
        println!(
            "Stock [ Name: {}, Quantity: {}] sold",
            self.name, self.quantity
        );
    }
}

// Create concrete classes to implement the Order interface
#[derive(Debug)]
struct BuyStock {
    order_detail: OrderDetail,
}

impl BuyStock {
    fn new(abc_stock: OrderDetail) -> Self {
        Self {order_detail: abc_stock}
    }
}

#[derive(Debug)]
struct SellStock {
    abc_stock: OrderDetail,
}

impl SellStock {
    fn new(abc_stock: OrderDetail) -> Self {
        Self { abc_stock}
    }
}

// Create the command interface
trait Order {
    fn execute(&self);
}

impl Order for BuyStock {
    fn execute(&self) {
        self.order_detail.buy();
    }
}

impl Order for SellStock {
    fn execute(&self) {
        self.abc_stock.sell();
    }
}

// Create command invoker class
struct Broker {
    orderlist: Vec<Box<dyn Order>>,
}

impl Broker {
    fn new() -> Self {
        Self {
            orderlist: Vec::new(),
        }
    }

    fn take_order(&mut self, order: Box<dyn Order>) {
	self.orderlist.push(order);
    }

    fn place_orders(&mut self) {
	for order in self.orderlist.iter() {
	    order.execute();
	}

	self.orderlist.clear();
    }
}


// Use the Broker class to take and execute commands.
fn main() {
    let abc_stock: OrderDetail = OrderDetail::default();
    let buy_stock_order: BuyStock = BuyStock::new(abc_stock);
    let sell_stock_order: SellStock = SellStock::new(OrderDetail::default());

    let mut broker: Broker = Broker::new();
    broker.take_order(Box::new(buy_stock_order));
    broker.take_order(Box::new(sell_stock_order));

    broker.place_orders();
}
