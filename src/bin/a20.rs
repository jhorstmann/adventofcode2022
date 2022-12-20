use adventofcode2022::Result;
use adventofcode2022::read_lines;

pub fn main() -> Result<()> {
    let mut input = read_lines("data/a20_example.txt")?.iter().map(|line| line.parse()).collect::<std::result::Result<Vec<i32>, _>>()?;

    let mut output = input.clone();
    // let mut input = vec![4, -2, 5, 6, 7, 8, 9];
    let mut permutations = Vec::with_capacity(input.len() * input.len());

    for i in 0..input.len() as u32 {
        create_permutation(&input, i, &mut permutations);
    }

    permutations.chunks(input.len()).for_each(|permutation| {
        apply_permutation(&input, &mut output, &permutations[0..input.len()]);
        dbg!(permutation, &input, &output);
        std::mem::swap(&mut input, &mut output);
    });

    Ok(())
}

fn create_permutation(input: &[i32], index: u32, permutations: &mut Vec<u32>) {
    let len = input.len() as u32;

    // assert!(len == output.len());
    assert!(index < len);

    let n = input[index as usize];
    let new_index = (index as i32 + n - ((n < 0) as i32)).rem_euclid(input.len() as i32) as u32;
    // dbg!(index, n, new_index);

    permutations.extend(
        (0..input.len() as u32).map(|i| {
            if i < index {
                i
            } else if i < new_index {
                i + 1
            } else if i == new_index {
                index
            } else {
                i
            }
        }))
}

fn apply_permutation(input: &[i32], output: &mut [i32], permutation: &[u32]) {
    output.iter_mut().zip(permutation.iter()).for_each(|(out, i)| {
        *out = input[*i as usize];
    })
}