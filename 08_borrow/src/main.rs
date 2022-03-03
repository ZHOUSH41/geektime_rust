fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    // &data == data1  &&data != &data1
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    );
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    );
}

// 把data的类型改为 &[u32], data的地址会发生改变，变成堆上的地址
fn sum(data: &Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    // 这里的data是值的地址，&data是引用的地址，和上面的&&data，&data1都不一样的
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data);
    data.iter().sum()
}
