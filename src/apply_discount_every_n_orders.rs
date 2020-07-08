use std::collections::HashMap;

/// @number 1357
/// @title Apply Discount Every n Orders
/// @url https://leetcode.com/problems/apply-discount-every-n-orders
/// @difficulty medium
//There is a sale in a supermarket, there will be a discount every n customer. 
//There are some products in the supermarket where the id of the i-th product is
// products[i] and the price per unit of this product is prices[i]. 
//The system will count the number of customers and when the n-th customer arriv
//e he/she will have a discount on the bill. (i.e if the cost is x the new cost is
// x - (discount * x) / 100). Then the system will start counting customers again.
// 
//The customer orders a certain amount of each product where product[i] is the i
//d of the i-th product the customer ordered and amount[i] is the number of units 
//the customer ordered of that product. 
//
// Implement the Cashier class: 
//
// 
// Cashier(int n, int discount, int[] products, int[] prices) Initializes the ob
//ject with n, the discount, the products and their prices. 
// double getBill(int[] product, int[] amount) returns the value of the bill and
// apply the discount if needed. Answers within 10^-5 of the actual value will be 
//accepted as correct. 
// 
//
// 
// Example 1: 
//
// 
//Input
//["Cashier","getBill","getBill","getBill","getBill","getBill","getBill","getBil
//l"]
//[[3,50,[1,2,3,4,5,6,7],[100,200,300,400,300,200,100]],[[1,2],[1,2]],[[3,7],[10
//,10]],[[1,2,3,4,5,6,7],[1,1,1,1,1,1,1]],[[4],[10]],[[7,3],[10,10]],[[7,5,3,1,6,4
//,2],[10,10,10,9,9,9,7]],[[2,3,5],[5,3,2]]]
//Output
//[null,500.0,4000.0,800.0,4000.0,4000.0,7350.0,2500.0]
//Explanation
//Cashier cashier = new Cashier(3,50,[1,2,3,4,5,6,7],[100,200,300,400,300,200,10
//0]);
//cashier.getBill([1,2],[1,2]);                        // return 500.0, bill = 1
// * 100 + 2 * 200 = 500.
//cashier.getBill([3,7],[10,10]);                      // return 4000.0
//cashier.getBill([1,2,3,4,5,6,7],[1,1,1,1,1,1,1]);    // return 800.0, The bill
// was 1600.0 but as this is the third customer, he has a discount of 50% which me
//ans his bill is only 1600 - 1600 * (50 / 100) = 800.
//cashier.getBill([4],[10]);                           // return 4000.0
//cashier.getBill([7,3],[10,10]);                      // return 4000.0
//cashier.getBill([7,5,3,1,6,4,2],[10,10,10,9,9,9,7]); // return 7350.0, Bill wa
//s 14700.0 but as the system counted three more customers, he will have a 50% dis
//count and the bill becomes 7350.0
//cashier.getBill([2,3,5],[5,3,2]);                    // return 2500.0
// 
//
// 
// Constraints: 
//
// 
// 1 <= n <= 10^4 
// 0 <= discount <= 100 
// 1 <= products.length <= 200 
// 1 <= products[i] <= 200 
// There are not repeated elements in the array products. 
// prices.length == products.length 
// 1 <= prices[i] <= 1000 
// 1 <= product.length <= products.length 
// product[i] exists in products. 
// amount.length == product.length 
// 1 <= amount[i] <= 1000 
// At most 1000 calls will be made to getBill. 
// Answers within 10^-5 of the actual value will be accepted as correct. 
// Related Topics Design 
// 👍 48 👎 62


//leetcode submit region begin(Prohibit modification and deletion)
struct Cashier {
    discount_position: i32,
    counter: i32,
    discount_rate: f64,
    product_prices: std::collections::HashMap<i32, i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Cashier {
    fn new(n: i32, discount: i32, products: Vec<i32>, prices: Vec<i32>) -> Self {
        let map = products.into_iter().zip(prices.into_iter()).collect();
        Self {
            discount_position: n,
            counter: 0,
            discount_rate: discount as f64 / 100.0,
            product_prices: map,
        }
    }

    fn get_bill(&mut self, product: Vec<i32>, amount: Vec<i32>) -> f64 {
        self.counter += 1;

        let x: f64 = product.iter().zip(amount.iter()).map(|(product_id, amount)| {
            let price = *self.product_prices.get(product_id).unwrap();

            price as f64 * *amount as f64
        }).sum();
        if self.counter == self.discount_position {
            self.counter = 0;
            x * (1.0 - self.discount_rate)
        } else {
            x
        }
    }
}

/**
 * Your Cashier object will be instantiated and called as such:
 * let obj = Cashier::new(n, discount, products, prices);
 * let ret_1: f64 = obj.get_bill(product, amount);
 */
//leetcode submit region end(Prohibit modification and deletion)
#[cfg(test)]
mod test {
    use crate::apply_discount_every_n_orders::Cashier;

    #[test]
    fn test1() {
        let mut cashier = Cashier::new(3, 50, vec![1, 2, 3, 4, 5, 6, 7], vec![100, 200, 300, 400, 300, 200, 100]);

        assert_eq!(500.0, cashier.get_bill(vec![1, 2], vec![1, 2]));
        assert_eq!(4000.0, cashier.get_bill(vec![3, 7], vec![10, 10]));
        assert_eq!(800.0, cashier.get_bill(vec![1, 2, 3, 4, 5, 6, 7], vec![1, 1, 1, 1, 1, 1, 1]));

        assert_eq!(4000.0, cashier.get_bill(vec![4], vec![10]));
        assert_eq!(4000.0, cashier.get_bill(vec![7, 3], vec![10, 10]));
        assert_eq!(7350.0, cashier.get_bill(vec![7, 5, 3, 1, 6, 4, 2], vec![10, 10, 10, 9, 9, 9, 7]));
    }
}