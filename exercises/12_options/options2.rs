fn main() {
    let range = 10;
    let mut optional_integers: Vec<Option<i8>> = vec![None];

    for i in 1..=range {
        optional_integers.push(Some(i));
    }

    let mut cursor = range;

    while let Some(integer) = optional_integers.pop() {
        // assert_eq!(integer.unwrap(), cursor);
        println!("{:?} huhhhh {:?}", integer, cursor);
        match integer {
            Some(integer) => {
                println!("Integer {:?}, but {:?}", integer, Some(cursor).unwrap());
                cursor -= 1
            }
            None => break,
        }
        if Some(integer) != None {
            cursor -= 1;
        }

        println!("{:?} huhhhh {:?}", integer, cursor);
    }

    print!("we currently at {cursor}")
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        while let Some(integer) = optional_integers.pop() {
            // assert_eq!(integer, cursor);
            match integer {
                Some(_) => cursor -= 1,
                None => print!("no"),
            }
        }

        assert_eq!(cursor, 0);
    }
}
