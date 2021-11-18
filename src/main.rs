trait PaymentStrategy {
    fn pay(&self, price: f64) -> f64;
}

enum PaymentType {
    Alipay,
    WechatPay,
}

struct Order {
    price: f64,
    pay_entity: Box<dyn PaymentStrategy>,
}

impl Order {
    fn new(price: f64, kind: PaymentType) -> Self {
        Order {
            price,
            pay_entity: match kind {
                PaymentType::Alipay => Box::new(Alipay {}),
                PaymentType::WechatPay => Box::new(WechatPay {}),
            },
        }
    }

    fn get_price(&self) -> f64 {
        self.price
    }
}

struct Alipay;
impl PaymentStrategy for Alipay {
    fn pay(&self, price: f64) -> f64 {
        let price = price - (price * 0.5);
        return price;
    }
}

struct WechatPay;
impl PaymentStrategy for WechatPay {
    fn pay(&self, price: f64) -> f64 {
        let price = price - (price * 0.66);
        return price;
    }
}

fn main() {
    let order = Order::new(100.0, PaymentType::Alipay);
    let payment_amount = order.pay_entity.pay(order.get_price());
    assert_eq!(payment_amount, 100.0 - (100.0 * 0.5));

    let order = Order::new(100.0, PaymentType::WechatPay);
    let num = order.pay_entity.pay(order.get_price());
    assert_eq!(num, 100.0 - (100.0 * 0.66));
}
