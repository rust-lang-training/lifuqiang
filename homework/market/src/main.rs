fn main() {
    let mut products = vec![
        Product {
            id: 1,
            name: "商品 A".to_string(),
            description: "商品 A 描述".to_string(),
            price: 19.99,
            inventory: 10,
            category: "商品1".to_string(),
        },
    ];

    let mut users = vec![
        User {
            id: 1,
            name: "用户 1".to_string(),
            balance: 100.0,
        },
    ];

    let mut orders = vec![];

    // 浏览商品
    let electronics_products: Vec<&Product> = browse_products_by_category(&products, "商品");
    println!("商品:");
    for product in &electronics_products {
        println!("- {}", product.name);
    }

    // 根据名称查找商品
    let found_products: Vec<&Product> = search_products_by_name(&products, "商品 A");
    println!("\n查找商品:");
    for product in &found_products {
        println!("- {}", product.name);
    }

    // 创建订单
    let mut order = Order {
        id: orders.len() + 1,
        user_id: 1, // 假设用户 ID 为 1
        products: vec![&products[0], &products[1]],
        total_amount: 0.0,
    };

    // 下单
    let user = &mut users[0];
    match place_order(user, &mut products, &mut order) {
        Ok(_) => println!("\n已成功下单"),
        Err(err) => println!("\n下单失败: {}", err),
    }

    orders.push(order);

    // 统计订单总金额
    let total_order_amount = calculate_total_order_amount(&orders);
    println!("\n订单总数: {:.2}", total_order_amount);
}

// 根据分类浏览商品
fn browse_products_by_category(products: &[Product], category: &str) -> Vec<&Product> {
    products.iter().filter(|p| p.category == category).collect()
}

// 根据名称查询商品
fn search_products_by_name(products: &[Product], name: &str) -> Vec<&Product> {
    products.iter().filter(|p| p.name.contains(name)).collect()
}

// 下单
fn place_order(user: &mut User, products: &mut [Product], order: &mut Order) -> Result<(), String> {
    let mut total_amount = 0.0;

    for product in &order.products {
        if product.inventory < 1 {
            return Err("库存不足了".to_string());
        }
        product.inventory -= 1;
        total_amount += product.price;
    }

    order.total_amount = total_amount;

    if user.balance < total_amount {
        return Err("余额不足了".to_string());
    }

    user.balance -= total_amount;
    
    Ok(())
}

// 统计订单总金额
fn calculate_total_order_amount(orders: &[Order]) -> f64 {
    orders.iter().map(|o| o.total_amount).sum()
}

struct Product {
    id: usize,
    name: String,
    description: String,
    price: f64,
    inventory: usize,
    category: String,
}

struct Order {
    id: usize,
    user_id: usize,
    products: Vec<Product>,
    total_amount: f64,
}

struct User {
    id: usize,
    name: String,
    balance: f64,
}