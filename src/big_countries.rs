/// @number 595
/// @title Big Countries
/// @url https://leetcode.com/problems/big-countries/
/// @difficulty easy


struct Solution;


fn main() {
    let sql = "select name, population, area from World where area > 3000000 or population > 25000000;";
    println!("{}", sql);
}
