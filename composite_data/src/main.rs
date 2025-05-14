  #[derive(Debug)] // 调试阶段可以打印出变量的值
  enum ColorNoParam{
    Red,
    Green,
    Blue,
  }
  #[derive(Debug)]
  enum ColorParam{
    Red(String),
    Green(String),
    Blue(String),
  }

fn main() {
  //  元组
  let tup: (i32,f32,bool) = (1,5.5,true);
  let tup1: (f32,(bool,i32)) = (7.7,(false,10));
  println!("{}",tup.0);
  println!("{}",tup.1);
  println!("{}",tup.2);
  println!("{}",tup1.0);
  println!("{}",tup1.1.0);
  println!("{}",tup1.1.1);

  // 数组
  let _arr = [1,2,3,4,5];
  let _arr1:[i32;5] = [1;5];
  for i in _arr1.iter(){
    println!("{}",i)
  }

  //结构体
  struct Person{
    name: &'static str,
    age: i32,
  }

  let name = "cty";
  let age = 26;

  let mut person = Person{
    name,
    age,
  };
  person.age = 18;
  println!("person.name:{},person.age:{}",person.name,person.age);

  let person1 = Person {
    age: 26,
    ..person
  };

  println!("person.name:{},person.age:{}",person1.name,person1.age);

  // 元祖结构体 没有字段名 只有类型
  struct Color(i32,i32,i32);
  let _white = Color(255,255,255);
  let _black = Color(0,0,0);
  println!("white color r:{}, g:{}, b:{}", _white.0, _white.1, _white.2);
  // 单元结构体 一般只用于特定结构体
  struct AlwaysEqual;
  let _ae = AlwaysEqual;

  // 枚举
  let _color_red = ColorNoParam::Red;
  let _color_green = ColorNoParam::Green;
  let _color_blue = ColorNoParam::Blue;
  // 使用match模式匹配来枚举所有的值，以处理不同值所对应的情况
  match _color_red {
    // {:?} 用于以调试格式打印变量的内容，通常用于输出结构体、枚举等复杂类型的详细信息
    ColorNoParam::Red => println!("{:?}",ColorNoParam::Red),
    ColorNoParam::Green => println!("{:?}",ColorNoParam::Green),
    ColorNoParam::Blue => println!("{:?}",ColorNoParam::Blue),
  }

  let color_param_red = ColorParam::Red("red".to_string());
  let color_param_blue = ColorParam::Blue("blue".to_string());
  let color_param_green = ColorParam::Green(String::from("green"));


  match color_param_red {
    ColorParam::Red(val) => println!("Red: {}", val),
    ColorParam::Green(val) => println!("Green: {}", val),
    ColorParam::Blue(val) => println!("Blue: {}", val),
  }
  match color_param_blue {
      ColorParam::Red(val) => println!("Red: {}", val),
      ColorParam::Green(val) => println!("Green: {}", val),
      ColorParam::Blue(val) => println!("Blue: {}", val),
  }
  match color_param_green {
      ColorParam::Red(val) => println!("Red: {}", val),
      ColorParam::Green(val) => println!("Green: {}", val),
      ColorParam::Blue(val) => println!("Blue: {}", val),
  }
}
