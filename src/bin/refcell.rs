use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Employee {
    id: u32,
    tasks: Vec<String>,
}

fn main() {
    // 创建一个被 Rc<RefCell> 包装的员工对象。
    // 这使得它既能被共享又能被修改。
    let john = Rc::new(RefCell::new(Employee {
        id: 1,
        tasks: vec![],
    }));

    // 克隆 Rc 指针，创建另一个共享所有者的任务变量。
    let task1 = Rc::clone(&john);
    
    // 再克隆一次，创建第三个所有者。
    let task2 = Rc::clone(&john);

    // 从 task1 这个变量，我们可以获取内部的可变借用并添加一个任务。
    // borrow_mut() 会在运行时进行检查。
    {
        let mut john_mut = task1.borrow_mut();
        john_mut.tasks.push(String::from("完成项目一"));
    } // 可变借用在这里结束。

    // 从 task2 这个变量，我们也可以获取可变借用并添加任务。
    // 这是安全的，因为上一个可变借用已经结束了。
    {
        let mut john_mut = task2.borrow_mut();
        john_mut.tasks.push(String::from("编写第二季度报告"));
    }

    // 现在，无论从哪个变量来看，修改都已经被同步。
    println!("第一个任务变量的任务列表: {:?}", task1.borrow().tasks);
    println!("第二个任务变量的任务列表: {:?}", task2.borrow().tasks);
    println!("原始员工变量的任务列表: {:?}", john.borrow().tasks);
    
    // 所有输出都会显示完整的任务列表：["完成项目一", "编写第二季度报告"]
}