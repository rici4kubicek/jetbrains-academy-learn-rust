pub fn calculate_price(count: u32) ->u32 {
    if count < 40 {
        count * 2
    } else {
        count
    }
}
