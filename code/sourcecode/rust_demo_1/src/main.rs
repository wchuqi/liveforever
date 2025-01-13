// 在可执行的项日中，main()函数是必须的
fn hello() {
    let a = "aa"; // Rust中的赋值，更恰当的说法叫变量绑定
    let b = "bb"; // 支持unicode编码
    let c = [a, b]; // 数组字面量使用[]
    for item in c.iter() { // iter()方法返回一个迭代器，很多类型都有iter()方法
        println!("{}", &item); // &表示借用item的值，只读访问
    }
}
fn main() {
    println!("Hello, world!");
    hello(); // 函数调用
    handleCsv();
}

fn handleCsv() {
    // "\ -> 忽略掉末尾的换行符
    let data = "\
    name,length
    a,1
    b,2
    c,3
    d,4
    ";

    let rows = data.lines();

    for (i, item) in rows.enumerate() {
        // 跳过表头和空行
        if i == 0 || item.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = item
            .split(',') // 分割
            .map(|a| a.trim()) // 匿名函数，去掉字段两端空白符
            .collect(); // 构建集合

        // cfg! -> 用于编译时检查配置
        if cfg!(debug_assertions) {
            // 输出到标准错误（stderr）
            eprintln!("debug: {:?} -> {:?}", item, fields);
        }
        
        let name = fields[0];
        // 尝试着把fields[1]解析为一个32位浮点数，如果解析成功，把此浮点数赋值给变量
        if let Ok(length) = fields[1].parse::<f32>() { // 内嵌的类型注解
            println!("{}, {} cm", name, length);
        }
    }
}