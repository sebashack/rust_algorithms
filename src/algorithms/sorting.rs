use std::cmp::Ordering::Less;
use std::cmp::{min, Ordering};

pub fn selection_sort_by<T, F>(v: &mut Vec<T>, compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = v.len();

    for i in 0..len {
        let mut min = i;

        for j in i + 1..len {
            if let Less = compare(&v[j], &v[min]) {
                min = j;
            }
        }

        v.swap(min, i);
    }
}

pub fn selection_sort<T>(v: &mut Vec<T>)
where
    T: Ord,
{
    selection_sort_by(v, |v0, v1| v0.cmp(v1))
}

pub fn insertion_sort_by<T, F>(v: &mut Vec<T>, compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = v.len();

    _insertion_sort_by(v, 0, len - 1, &compare);
}

fn _insertion_sort_by<T, F>(v: &mut Vec<T>, lo: usize, hi: usize, compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in lo..=hi {
        let mut j = i;

        while j > 0 {
            if let Less = compare(&v[j], &v[j - 1]) {
                v.swap(j, j - 1);
            } else {
                break;
            }

            j -= 1;
        }
    }
}

pub fn insertion_sort<T>(v: &mut Vec<T>)
where
    T: Ord,
{
    insertion_sort_by(v, |v0, v1| v0.cmp(v1))
}

pub fn shell_sort_by<T, F>(v: &mut Vec<T>, compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = v.len();

    let mut h: usize = 1;

    while h < len / 3 {
        h = (3 * h) + 1
    }

    while h >= 1 {
        for i in h..len {
            let mut j = i;

            while j >= h {
                if let Less = compare(&v[j], &v[j - h]) {
                    v.swap(j, j - 1);
                }

                j -= h;
            }
        }
        h /= 3;
    }
}

pub fn shell_sort<T>(v: &mut Vec<T>)
where
    T: Ord,
{
    shell_sort_by(v, |v0, v1| v0.cmp(v1));
}

pub fn merge_sort<T>(v: &mut Vec<T>)
where
    T: Ord,
{
    merge_sort_by(v, |v0, v1| v0.cmp(v1));
}

pub fn merge_sort_by<T, F>(v: &mut Vec<T>, compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = v.len();
    let mut aux = Vec::with_capacity(v.len());

    _merge_sort(v, &mut aux, 0, len - 1, &compare);
}

fn _merge_sort<T, F>(v: &mut Vec<T>, aux: &mut Vec<Option<T>>, lo: usize, hi: usize, compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if hi <= lo {
        return;
    }

    let CUTOFF = 20;

    if hi <= lo + CUTOFF - 1 {
        _insertion_sort_by(v, lo, hi, compare);
        return;
    }

    let mid = lo + ((hi - lo) / 2);
    _merge_sort(v, aux, lo, mid, compare);
    _merge_sort(v, aux, mid + 1, hi, compare);

    if let Less = compare(&v[mid + 1], &v[mid]) {
        return;
    }

    merge(v, aux, lo, mid, hi, compare);
}

pub fn bottom_up_merge_sort<T>(v: &mut Vec<T>)
where
    T: Ord,
{
    bottom_up_merge_sort_by(v, |n, m| n.cmp(m));
}

pub fn bottom_up_merge_sort_by<T, F>(v: &mut Vec<T>, compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = v.len();
    let mut aux = Vec::with_capacity(len);

    let mut sz = 1;

    while sz < len {
        let mut lo = 0;
        while lo < len - sz {
            merge(
                v,
                &mut aux,
                lo,
                lo + sz - 1,
                min(lo + sz + sz - 1, len - 1),
                &compare,
            );
            lo += sz * 2;
        }

        sz *= 2;
    }
}

fn merge<T, F>(
    v: &mut Vec<T>,
    aux: &mut Vec<Option<T>>,
    lo: usize,
    mid: usize,
    hi: usize,
    compare: &F,
) where
    F: Fn(&T, &T) -> Ordering,
{
    use std::mem::replace;
    use std::mem::MaybeUninit;

    unsafe {
        for k in lo..=hi {
            let uninit_val = MaybeUninit::<T>::uninit().assume_init();
            aux.insert(k, Some(replace(&mut v[k], uninit_val)));
        }
    }

    let mut i = lo;
    let mut j = mid + 1;

    for k in lo..=hi {
        if i > mid {
            v[k] = aux[j].take().unwrap();
            j += 1;
        } else if j > hi {
            v[k] = aux[i].take().unwrap();
            i += 1;
        } else if let Less = compare(aux[j].as_ref().unwrap(), aux[i].as_ref().unwrap()) {
            v[k] = aux[j].take().unwrap();
            j += 1;
        } else {
            v[k] = aux[i].take().unwrap();
            i += 1;
        }
    }
}

pub fn quick_sort<T>(v: &mut Vec<T>)
where
    T: Ord,
{
    quick_sort_by(v, |v1, v0| v1.cmp(v0))
}

pub fn quick_sort_by<T, F>(v: &mut Vec<T>, compare: F)
where
    F: Fn(&T, &T) -> Ordering,
{
    shuffle_vec(v);
    _quick_sort(v, 0, v.len() - 1, &compare)
}

fn _quick_sort<T, F>(v: &mut Vec<T>, lo: usize, hi: usize, compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if hi <= lo {
        return;
    };

    let CUTOFF = 20;

    if hi <= lo + CUTOFF - 1 {
        _insertion_sort_by(v, lo, hi, compare);
        return;
    }

    let j = partition(v, lo, hi, compare);
    if lo == 0 && j == 0 {
        _quick_sort(v, j + 1, hi, compare);
    } else {
        _quick_sort(v, lo, j - 1, compare);
        _quick_sort(v, j + 1, hi, compare);
    }
}

pub fn shuffle_vec<T>(v: &mut Vec<T>) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;

    let mut rng = thread_rng();

    v.shuffle(&mut rng);
}

fn partition<T, F>(v: &mut Vec<T>, lo: usize, hi: usize, compare: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut i = lo;
    let mut j = hi + 1;

    loop {
        i += 1;
        while let Less = compare(&v[i], &v[lo]) {
            if i == hi {
                break;
            }

            i += 1;
        }

        println!("j out {}", j);
        j -= 1;
        while let Less = compare(&v[lo], &v[j]) {
            if j == lo {
                break;
            }

            println!("j in {}", j);
            j -= 1;
        }

        if i >= j {
            break;
        }

        v.swap(i, j);
    }

    v.swap(lo, j);

    j
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use crate::algorithms::sorting::{
        bottom_up_merge_sort_by, insertion_sort_by, merge_sort_by, quick_sort_by,
        selection_sort_by, shell_sort_by, shuffle_vec,
    };
    use rand::Rng;

    fn is_sorted<T>(v: &Vec<T>) -> bool
    where
        T: Ord,
    {
        let len = v.len();

        for (i, n) in v.iter().enumerate() {
            if i < len - 1 && n > &v[i + 1] {
                return false;
            }
        }

        true
    }

    fn gen_rand_vec(n: usize) -> Vec<usize> {
        let mut v = (0..n).collect();

        shuffle_vec(&mut v);

        v
    }

    #[test]
    fn selection_sort_by_should_sort_the_vector() {
        let mut v = vec![12, 0, 3, 12, 23, 1, 9, 9, 10, 45, 6, 12, 100, 45, 3, 1, 2];

        selection_sort_by(&mut v, |n, m| n.cmp(m));

        assert!(is_sorted(&v));
    }

    #[test]
    fn insertion_sort_by_should_sort_the_vector() {
        let mut v = vec![12, 0, 3, 12, 23, 1, 9, 9, 10, 45, 6, 12, 100, 45, 3, 1, 2];

        insertion_sort_by(&mut v, |n, m| n.cmp(m));

        assert!(is_sorted(&v));
    }

    #[test]
    fn shell_sort_by_should_sort_the_vector() {
        let mut v = gen_rand_vec(1000);

        shell_sort_by(&mut v, |n, m| n.cmp(m));

        assert!(is_sorted(&v));
    }

    #[test]
    fn merge_sort_by_should_sort_the_vector() {
        let mut v = gen_rand_vec(2701);

        merge_sort_by(&mut v, |n, m| n.cmp(m));

        assert!(is_sorted(&v));
    }

    #[test]
    fn bottom_up_merge_sort_by_should_sort_the_vector() {
        let mut v = gen_rand_vec(2877);

        bottom_up_merge_sort_by(&mut v, |n, m| n.cmp(m));

        assert!(is_sorted(&v));
    }

    #[test]
    fn quick_sort_by_should_sort_the_vector() {
        let mut v = gen_rand_vec(2877);

        quick_sort_by(&mut v, |n, m| n.cmp(m));

        assert!(is_sorted(&v));
    }
}
