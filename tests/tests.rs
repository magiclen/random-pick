#[test]
fn test_gen_index_with_weights_1() {
    let n = 1_000_000;
    let weights = [5, 10];

    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        result.push(random_pick::gen_usize_with_weights(2, &weights).unwrap());
    }

    let mut counter = [0usize; 2];

    for i in result {
        counter[i] += 1;
    }

    let a = (counter[0] as f64) * (weights[1] as f64) / (weights[0] as f64);

    let b = counter[1] as f64;

    let err = (b - a).abs() / b;

    assert!(err < 0.025);
}

#[test]
fn test_gen_index_with_weights_2() {
    let weights = [5, 10, 15, 20, 25];

    let n = 1_000_000;
    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        result.push(random_pick::gen_usize_with_weights(5, &weights).unwrap());
    }

    let mut counter = [0usize; 5];

    for i in result {
        counter[i] += 1;
    }

    for i in 0..5 {
        for j in i..5 {
            let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}

#[test]
fn test_gen_index_with_weights_3() {
    let n = 1_000_000;
    let weights = [5, 10];

    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        result.push(random_pick::gen_usize_with_weights(10, &weights).unwrap());
    }

    let mut counter = [0usize; 2];

    for i in result {
        if i < 5 {
            counter[0] += 1;
        } else {
            counter[1] += 1;
        }
    }

    let a = (counter[0] as f64) * (weights[1] as f64) / (weights[0] as f64);

    let b = counter[1] as f64;

    let err = (b - a).abs() / b;

    assert!(err < 0.025);
}

#[test]
fn test_gen_index_with_weights_4() {
    let n = 1_000_000;
    let weights = [5, 10, 15, 20, 25];

    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        result.push(random_pick::gen_usize_with_weights(10, &weights).unwrap());
    }

    let mut counter = [0usize; 5];

    for i in result {
        if i < 2 {
            counter[0] += 1;
        } else if i < 4 {
            counter[1] += 1;
        } else if i < 6 {
            counter[2] += 1;
        } else if i < 8 {
            counter[3] += 1;
        } else {
            counter[4] += 1;
        }
    }

    for i in 0..5 {
        for j in i..5 {
            let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}

#[test]
fn test_gen_index_with_weights_5() {
    let n = 1_000_000;
    let weights = [5];

    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        result.push(random_pick::gen_usize_with_weights(10, &weights).unwrap());
    }

    let mut counter = [0usize; 5];

    for i in result {
        if i < 2 {
            counter[0] += 1;
        } else if i < 4 {
            counter[1] += 1;
        } else if i < 6 {
            counter[2] += 1;
        } else if i < 8 {
            counter[3] += 1;
        } else {
            counter[4] += 1;
        }
    }

    for i in 0..5 {
        for j in i..5 {
            let a = counter[i] as f64;

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}

#[test]
fn test_gen_multiple_index_with_weights_1() {
    let n = 1_000_000;
    let weights = [5, 10];

    let result = random_pick::gen_multiple_usize_with_weights(2, &weights, n);

    let mut counter = [0usize; 2];

    for i in result {
        counter[i] += 1;
    }

    let a = (counter[0] as f64) * (weights[1] as f64) / (weights[0] as f64);

    let b = counter[1] as f64;

    let err = (b - a).abs() / b;

    assert!(err < 0.025);
}

#[test]
fn test_gen_multiple_index_with_weights_2() {
    let n = 1_000_000;
    let weights = [5, 10, 15, 20, 25];

    let result = random_pick::gen_multiple_usize_with_weights(5, &weights, n);

    let mut counter = [0usize; 5];

    for i in result {
        counter[i] += 1;
    }

    for i in 0..5 {
        for j in i..5 {
            let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}

#[test]
fn test_gen_multiple_index_with_weights_3() {
    let n = 1_000_000;
    let weights = [5, 10];

    let result = random_pick::gen_multiple_usize_with_weights(10, &weights, n);

    let mut counter = [0usize; 2];

    for i in result {
        if i < 5 {
            counter[0] += 1;
        } else {
            counter[1] += 1;
        }
    }

    let a = (counter[0] as f64) * (weights[1] as f64) / (weights[0] as f64);

    let b = counter[1] as f64;

    let err = (b - a).abs() / b;

    assert!(err < 0.025);
}

#[test]
fn test_gen_multiple_index_with_weights_4() {
    let n = 1_000_000;
    let weights = [5, 10, 15, 20, 25];

    let result = random_pick::gen_multiple_usize_with_weights(10, &weights, n);

    let mut counter = [0usize; 5];

    for i in result {
        if i < 2 {
            counter[0] += 1;
        } else if i < 4 {
            counter[1] += 1;
        } else if i < 6 {
            counter[2] += 1;
        } else if i < 8 {
            counter[3] += 1;
        } else {
            counter[4] += 1;
        }
    }

    for i in 0..5 {
        for j in i..5 {
            let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}

#[test]
fn test_gen_multiple_index_with_weights_5() {
    let n = 1_000_000;
    let weights = [5];

    let result = random_pick::gen_multiple_usize_with_weights(10, &weights, n);

    let mut counter = [0usize; 5];

    for i in result {
        if i < 2 {
            counter[0] += 1;
        } else if i < 4 {
            counter[1] += 1;
        } else if i < 6 {
            counter[2] += 1;
        } else if i < 8 {
            counter[3] += 1;
        } else {
            counter[4] += 1;
        }
    }

    for i in 0..5 {
        for j in i..5 {
            let a = counter[i] as f64;

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}

#[test]
fn test_pick_from_slice() {
    enum Prize {
        Legendary,
        Rare,
        Enchanted,
        Common,
    }

    let prize_list = [Prize::Legendary, Prize::Rare, Prize::Enchanted, Prize::Common];

    let weights = [1, 5, 15, 30];

    let n = 1_000_000;
    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        let picked_item = random_pick::pick_from_slice(&prize_list, &weights).unwrap();

        result.push(picked_item);
    }

    let mut counter = [0usize; 4];

    for picked_item in result {
        match picked_item {
            Prize::Legendary => {
                counter[0] += 1;
            },
            Prize::Rare => {
                counter[1] += 1;
            },
            Prize::Enchanted => {
                counter[2] += 1;
            },
            Prize::Common => {
                counter[3] += 1;
            },
        }
    }

    for i in 0..4 {
        for j in i..4 {
            let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}

#[test]
fn test_pick_multiple_from_slice() {
    enum Prize {
        Legendary,
        Rare,
        Enchanted,
        Common,
    }

    let prize_list = [Prize::Legendary, Prize::Rare, Prize::Enchanted, Prize::Common];

    let weights = [1, 5, 15, 30];

    let n = 1_000_000;
    let result = random_pick::pick_multiple_from_slice(&prize_list, &weights, n);

    let mut counter = [0usize; 4];

    for picked_item in result {
        match picked_item {
            Prize::Legendary => {
                counter[0] += 1;
            },
            Prize::Rare => {
                counter[1] += 1;
            },
            Prize::Enchanted => {
                counter[2] += 1;
            },
            Prize::Common => {
                counter[3] += 1;
            },
        }
    }

    for i in 0..4 {
        for j in i..4 {
            let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}

#[test]
fn test_pick_from_mutliple_slices() {
    enum Prize {
        Legendary,
        Rare,
        Enchanted,
        Common,
    }

    let prize_list_1 = [Prize::Legendary, Prize::Rare];
    let prize_list_2 = [Prize::Enchanted, Prize::Common];

    let weights = [1, 5, 15, 30];

    let n = 1_000_000;
    let result = random_pick::pick_multiple_from_multiple_slices(
        &[&prize_list_1, &prize_list_2],
        &weights,
        n,
    );

    let mut counter = [0usize; 4];

    for picked_item in result {
        match picked_item {
            Prize::Legendary => {
                counter[0] += 1;
            },
            Prize::Rare => {
                counter[1] += 1;
            },
            Prize::Enchanted => {
                counter[2] += 1;
            },
            Prize::Common => {
                counter[3] += 1;
            },
        }
    }

    for i in 0..4 {
        for j in i..4 {
            let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}

#[test]
fn test_pick_multiple_from_mutliple_slices() {
    enum Prize {
        Legendary,
        Rare,
        Enchanted,
        Common,
    }

    let prize_list_1 = [Prize::Legendary, Prize::Rare];
    let prize_list_2 = [Prize::Enchanted, Prize::Common];

    let weights = [1, 5, 15, 30];

    let n = 1_000_000;
    let mut result = Vec::with_capacity(n);

    for _ in 0..n {
        let picked_item =
            random_pick::pick_from_multiple_slices(&[&prize_list_1, &prize_list_2], &weights)
                .unwrap();

        result.push(picked_item);
    }

    let mut counter = [0usize; 4];

    for picked_item in result {
        match picked_item {
            Prize::Legendary => {
                counter[0] += 1;
            },
            Prize::Rare => {
                counter[1] += 1;
            },
            Prize::Enchanted => {
                counter[2] += 1;
            },
            Prize::Common => {
                counter[3] += 1;
            },
        }
    }

    for i in 0..4 {
        for j in i..4 {
            let a = (counter[i] as f64) * (weights[j] as f64) / (weights[i] as f64);

            let b = counter[j] as f64;

            let err = (b - a).abs() / b;

            assert!(err < 0.025);
        }
    }
}
