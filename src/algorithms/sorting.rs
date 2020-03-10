use std::cmp::Ordering;
use std::cmp::Ordering::Less;

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

    for i in 0..len {
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

pub fn merge_sort_by<T, F>(v: &mut Vec<T>, compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    let len = v.len();
    let mut aux = Vec::with_capacity(v.len());

    _merge_sort(v, &mut aux, 0, len - 1, compare);
}

fn _merge_sort<T, F>(v: &mut Vec<T>, aux: &mut Vec<Option<T>>, lo: usize, hi: usize, compare: &F)
where
    F: Fn(&T, &T) -> Ordering,
{
    if hi <= lo {
        return;
    }

    let mid = lo + ((hi - lo) / 2);
    _merge_sort(v, aux, lo, mid, compare);
    _merge_sort(v, aux, mid + 1, hi, compare);
    merge(v, aux, lo, mid, hi, compare);
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
    for i in lo..=hi {
        aux.insert(i, Some(v.remove(0)));
    }

    let mut i = lo;
    let mut j = mid + 1;

    for k in lo..=hi {
        if i > mid {
            v.insert(k, aux[j].take().unwrap());
            j += 1;
        } else if j > hi {
            v.insert(k, aux[i].take().unwrap());
            i += 1;
        } else if let Less = compare(aux[j].as_ref().unwrap(), aux[i].as_ref().unwrap()) {
            v.insert(k, aux[j].take().unwrap());
            j += 1;
        } else {
            v.insert(k, aux[i].take().unwrap());
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use rand::Rng;

    use crate::algorithms::sorting::{insertion_sort_by, selection_sort_by, shell_sort_by};

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

    fn gen_rand_vec() -> Vec<isize> {
        let mut v = Vec::with_capacity(1000);
        let mut rng = rand::thread_rng();

        for i in 0..999 {
            let n: isize = rng.gen_range(-999, 999);

            v.insert(i, n);
        }

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
        let mut v = gen_rand_vec();

        shell_sort_by(&mut v, |n, m| n.cmp(m));

        assert!(is_sorted(&v));
    }

    #[test]
    fn merge_sort_by_should_sort_the_vector() {
        let mut v = gen_rand_vec();

        shell_sort_by(&mut v, |n, m| n.cmp(m));

        assert!(is_sorted(&v));
    }
}
