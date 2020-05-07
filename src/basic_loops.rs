pub(crate) fn loop_to_10() {
    let mut n = 0;
    loop {
        n += 1;
        println!("{} - Loop", n);
        if n >= 10 {
            return;
        }
    }
}

pub(crate) fn loop_to_9() {
    for n in 1..10 {
        println!("Loop {}", n)
    }
}

pub(crate) fn array_loop() {
    let mut v = Vec::new();
    v.push(14);
    v.push(17);
    v.push(19);

    for n in v {
        println!("{}", n);
    }
}

pub(crate) fn array() {
    let v = vec![14, 17, 19, 100];

    for n in v {
        println!("{}", n);
    }
    
}