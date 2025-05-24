

fn main(){
  // 动态数组 三种创建方式
  let mut _vec: Vec<i32> = Vec::new();
  // 只是为 Vec 预留了 10 个元素的空间，但此时 Vec 的长度（ len() ）仍然是 0
  let mut _vec1: Vec<i32> = Vec::with_capacity(10);
  let mut _vec2: Vec<i32> = vec![];
  let mut _vec3 = vec![1,2,3,4,5];
  let mut _vec4 = vec![0;10];
  println!("{:?},{:?},{:?},{:?},{:?}",_vec,_vec1,_vec2,_vec3,_vec4);
  // 动态数组 增删改查
  // 使用push方法在动态数组的尾部添加新元素
  _vec.push(0);
  _vec.push(1);
  _vec.push(2);
  // 通过索引修改数组中元素
  _vec[2] = 10;
  _vec4[9] = 10;
  // 使用pop方法删除并返回动态数组的最后一个元素，如果数组为空则返回None。
  println!("{:?},{:?}", _vec.pop(),_vec);
  // 使用remove方法删除并返回动态数组指定索引的元素，同时将其后面的所有元素向左移动一位
  println!("{:?},{:?}",_vec4.remove(9),_vec4);
  //使用get方法以索引作为参数访问元素，索引越界返回None
  println!("{:?}", _vec.get(0));
  // for 循环数组

  let _vec5 = vec![1,2,3,4,5];
  for i in _vec5{
    println!("{}",i);
  }
  let mut _vec6 = vec![10,20,30];
  for i in &mut _vec6{
    *i += 10;
    println!("{}",i)
  }

}