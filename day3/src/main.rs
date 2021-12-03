use fs_err as fs;
use itertools::{self, Itertools};

fn main() -> anyhow::Result<()> {
    let lines = include_str!("input.txt").lines();

    /*let counts = counts(lines.clone());

    let most_common = counts
        .clone()
        .map(|(zeros, ones)| if ones >= zeros { '1' } else { '0' })
        .collect::<Vec<char>>();
    let least_common = counts
        .map(|(zeros, ones)| if ones >= zeros { '0' } else { '1' })
        .collect::<Vec<char>>();

    println!("most common bits: {:?}", &most_common);
    println!("least common bits: {:?}", &least_common);*/

    let oxy_rating = rating(lines.clone().collect::<Vec<&str>>(), true);
    println!("oxy_rating {}", oxy_rating);
    let co_rating = rating(lines.collect::<Vec<&str>>(), false);
    println!("co_rating {}", co_rating);

    // Code here
    let answer = oxy_rating * co_rating;

    println!("{}", answer);
    Ok(())
}

fn counts<'a>(lines: impl Iterator<Item = &'a &'a str>) -> Vec<(i32, i32)> {
    let mut zero_count = vec![0; 12];
    let mut one_count = vec![0; 12];
    for line in lines {
        let mut it = line.chars();
        for i in 0..12 {
            match it.next().unwrap() {
                '1' => {
                    one_count[i] += 1;
                }
                '0' => {
                    zero_count[i] += 1;
                }
                _ => panic!(),
            }
        }
    }
    zero_count
        .iter()
        .zip(one_count)
        .map(|(&a, b)| (a, b))
        .collect()
}

fn rating(mut lines: Vec<&str>, most_common: bool) -> usize {
    'outmost: for bit_pos in 0..12 {
        let criterion = if most_common {
            counts(lines.iter())
                .iter()
                .map(|(zeros, ones)| if ones >= zeros { '1' } else { '0' })
                .collect::<Vec<char>>()
        } else {
            counts(lines.iter())
                .iter()
                .map(|(zeros, ones)| if ones >= zeros { '0' } else { '1' })
                .collect::<Vec<char>>()
        };
        let crit = criterion[bit_pos];
        println!("[{}] -> c: {}", bit_pos, crit);

        let mut li = 0;
        while li < lines.len() {
            let char_at_i = lines[li].chars().nth(bit_pos).unwrap();
            print!(
                "considering [{}]: {}[{}] = {}",
                li, lines[li], bit_pos, char_at_i
            );
            if char_at_i != crit {
                println!(" -> R");
                lines.remove(li);
                if lines.len() == 1 {
                    break 'outmost;
                }
            } else {
                println!(" -> K");
                li += 1;
            }
        }
    }
    assert_eq!(lines.len(), 1);
    let r = lines[0];
    println!("bin rating {}", r);
    let r = usize::from_str_radix(&r, 2).unwrap();
    r
}
