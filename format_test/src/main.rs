fn main() {
    println!("Formate test!");

    let s = format!("this is {}", 1);
    println!("s {}", s);

    println!(">>>>{}, {}, {}", 1, 2, 3);
    println!(">>>>{width}", width = "111");
    println!(">>>>, {1} {0}", 0, 1);
    println!(">>>{:?}", (1, 2, 3));
    println!(">>>{:#?}", (1111, 222, 33));
    println!(">>>>{:05}", 1);
    println!(">>>{} {1:.00001}", "x", 0.44);
    println!(">>>{} {1:.00002}", "x", 0.44);
    println!(">>>{} {1:.00003}", "x", 0.44);
    println!(">>>{} {1:.00103}", "x", 0.44);
    println!(">>>{} {:->5}", "x", "y");
    println!(">>>{} {:-<5}", "x", "y");
    println!(">>>{} {:-^5}", "x", "y");
    println!(">>>>{{}}");
    println!(">>>>{{");
    println!(">>>>{{111}}");
    println!(">>>{:#x}", 001);
    println!(">>>{:#x}", 011);
    println!(">>>{:#015X?}", 011);
}
