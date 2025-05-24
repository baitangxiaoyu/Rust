use std::collections::VecDeque;


// 双端队列是一种同时具有栈（先进后出）和队列（先进先出）特征的数据结构，适用于只能在队列两端进行添加或删除元素操作的应用场景
fn main(){
  // 第一种创建方式
  let mut _vec: VecDeque<u32> = VecDeque::new();
  // 第二种创建方式
  let mut _vec1: VecDeque<u32>  = VecDeque::with_capacity(10);
  // VecDeque 增删改查
  // 使用push_front方法在队列的头部添加新元素，使用push_back方法在队列的尾部添加新元素
  let mut _vec2: VecDeque<i32> = VecDeque::new();
  _vec2.push_front(1);
  _vec2.push_back(2);
  _vec2.push_front(3);
  _vec2.push_back(4);
  // 通过索引值支持修改队列中元素
  _vec2[3] = 10;
  println!("{:?}",_vec2);
  // 使用pop_front方法删除并返回队列的头部元素，使用pop_back方法删除并返回队列的尾部元素
  println!("{:?},{:?},{:?}",_vec2.pop_front(),_vec2.pop_back(),_vec2)

}