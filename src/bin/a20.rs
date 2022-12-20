use adventofcode2022::Result;
use adventofcode2022::read_lines;

pub fn main() -> Result<()> {
    // let mut numbers = read_lines("data/a20_example.txt")?.iter().map(|line| line.parse()).collect::<Result<Vec<i32>>>()?;

    let mut input = vec![4, 5, 6, 1, 7, 8, 9];
    let mut output = vec![-42; input.len()];

    mix(&mut input, &mut output, 3);

    dbg!(&output);


    Ok(())
}

fn mix(input: &mut Vec<i32>, output: &mut Vec<i32>, index: usize) {
    let len = input.len();

    // assert!(len == output.len());
    assert!(len < i32::MAX as usize);
    assert!(index < len);

    let n= input[index];
    let new_index = (index as i32 + n).rem_euclid(input.len() as i32) as usize;
dbg!(new_index);

    let mut j = 0;
    output.iter_mut().enumerate().for_each(|(i, out)| {
        *out = input[j];
        j += 1;
        if i == index {
            j+=1;
        }
    });
    output[new_index] = n;


    // std::mem::swap(input, output);
}