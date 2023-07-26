enum Shape {
    Circle { radius: f32 },
    Square { width: f32, height: f32 },
    Triangle { side_lenght: f32},
}
  
impl Shape {
    fn girth(&self) -> f32 {
        match self {
            Shape::Circle { radius } => std::f32::consts::PI * radius * 2.0,
            Shape::Square { width, height } => (width + height) * 2.0, 
            Shape::Triangle { side_lenght } => side_lenght * 3.0,
        }
    }
}




trait Girth {
    fn girth(&self) -> f32;
}

struct Circle {
    radius: f32,
}

impl Girth for Circle {
    fn girth(&self) -> f32 {
        return std::f32::consts::PI * self.radius * 2.0
    }
}

struct Square {
    width: f32,
    height: f32,
}
impl Girth for Square {
    fn girth(&self) -> f32 {
        return (self.width + self.height) * 2.0
    }
}

struct Triangle {
    side_lenght: f32,
}
impl Girth for Triangle {
    fn girth(&self) -> f32 {
        return self.side_lenght * 3.0
    }
}


  
fn main() {
    // 使用枚举包裹三个不同的类型，并放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    let shapes = vec![
        Shape::Circle { radius: 20.0 },
        Shape::Square { width: 30.0, height: 40.0 },
        Shape::Triangle { side_lenght: 50.0 },
    ];

    for shape in &shapes {
        println!("shape girth is: {}", shape.girth());
    }


    // 定义三个不同的类型，使用Trait Object，将其放入一个Vec中，对Vec进行遍历，调用三种不同类型的各自的方法。
    let trate_shapes = vec![
        Box::new(Circle { radius: 20.0 }) as Box<dyn Girth>,
        Box::new(Square { width: 30.0, height: 40.0 }) as Box<dyn Girth>,
        Box::new(Triangle { side_lenght: 50.0 }) as Box<dyn Girth>,
    ];

    for shape in trate_shapes {
        println!("trate shape girth is: {}", shape.girth());
    }

    // 区别：
    // 枚举比trait更直接一些，而且枚举表达类型比较直接，trait感觉更像是把具体的类型抹去，保留了类似接口的行为。
    // 但trait的抽象能力更加强大，也更加灵活，可以对已有的类型进行扩展，而枚举则不行。
}
