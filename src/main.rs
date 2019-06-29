// const GAPS: [usize; 9] = [1750, 701, 301, 132, 57, 23, 10, 4, 1];
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::prelude::*;
use rand::rngs::SmallRng;



fn shellsort(list: &mut [usize], gaps: &[usize]) -> usize {
    let mut comparisons = 0;
    for &gap in gaps.iter() {
        for i in gap..(list.len()) {
            let temp = list[i];
            let mut j = i;
            while j >= gap {
                comparisons += 1;
                if !(list[j - gap] > temp) {
                    break;
                }
                list[j] = list[j - gap];
                j -= gap;
            }
            list[j] = temp;
        }
    }
    comparisons
}

fn main() {
    let mut gaps = [701, 301, 132, 57, 23, 10, 4, 1];
    let mut best = 1000000000;
    for g0 in (1400..5000).into_iter().step_by(100) {
        let mut list: Vec<usize> = (0..g0).collect();
        for g1 in 600..1400 {
            gaps[0] = g1;
            let mut rng = SmallRng::seed_from_u64(1);
            let mut total_comparisons: usize = 0;
            for _ in 0..100000 {
                list.shuffle(&mut rng);
                let n = shellsort(&mut list, &gaps);
                total_comparisons += n;
            }
            if total_comparisons <= best {
                best = total_comparisons;
            }
            println!("n: {}, comps: {}, {:?}", g0, total_comparisons, gaps);
        }
    }
}
