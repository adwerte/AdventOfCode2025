use std::fs;


fn moving_paperoll_count(bank:&str, n : usize, required_weight : usize) -> usize{
    let weights = moving_paperoll_weights(bank, n);
    println!("{:?}", weights);
    weights.iter().filter(|weight| weight < &&(required_weight as isize) && weight != &&0).count()
}


fn moving_paperoll_weights(bank : &str, n : usize)-> Vec<isize>{
    let mut weights = vec![0; bank.len()];
    for (index, weight) in weights.iter_mut().enumerate(){
        let start = index.checked_sub(n);
        let cur_char = bank.chars().nth(index);
        if cur_char == Some('.') || cur_char.is_none(){
            continue;
        } else if cur_char == Some('@'){
            *weight -= 1;
        }
        let end = index + n + 1;
        let end = if end <= bank.len(){
            Some(end)
        } else{
            None
        };

        let weight = if let Some(start) = start && let Some(end) = end{
            &bank[start..end]
            
        } else if let Some(start) = start{
            &bank[start..]
        } else if let Some(end) = end {
            &bank[..end]
        } else {
            ""
        };
        for char in area.chars(){
            if char == '@'{
                *weight += 1;
            }
        }   
        println!("{}, {}, {:?}-{}-{:?}", area, weight, start, index, end);
    }
    weights
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example(){
        let input = "..@@.@@@@.@@@.@.@.@@@@@@@.@.@@@.@@@@..@.@@.@@@@.@@.@@@@@@@.@.@.@.@.@@@@.@@@.@@@@.@@@@@@@@.@.@.@@@.@.";
        let count = moving_paperoll_count(input, 4, 4);
        assert_eq!(count, 13)
    }
}