use std::cell::RefCell;
fn main() {
    let data = RefCell::new(1);
    {
        //获得可变数据
        let mut v = data.borrow_mut();
        *v +=1;
    }
    println!("data: {:?}", data);
}